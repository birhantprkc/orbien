package io.github.lxien.orbien.client.netty;

import io.github.lxien.orbien.client.OrbienClientConfig;
import io.github.lxien.orbien.client.msg.MsgCodec;
import io.github.lxien.orbien.client.msg.MsgType;
import io.github.lxien.orbien.client.msg.StartWorkConn;
import io.github.lxien.orbien.client.msg.WireMessage;
import io.netty.bootstrap.Bootstrap;
import io.netty.buffer.ByteBuf;
import io.netty.channel.Channel;
import io.netty.channel.ChannelFutureListener;
import io.netty.channel.ChannelHandlerContext;
import io.netty.channel.ChannelInboundHandlerAdapter;
import io.netty.channel.ChannelInitializer;
import io.netty.channel.ChannelOption;
import io.netty.channel.EventLoopGroup;
import io.netty.channel.socket.SocketChannel;
import io.netty.channel.socket.nio.NioSocketChannel;
import io.netty.util.ReferenceCountUtil;

import java.util.Map;
import java.util.concurrent.ConcurrentHashMap;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

public final class WorkHandshakeHandler extends ChannelInboundHandlerAdapter {
    private static final Logger log = LoggerFactory.getLogger(WorkHandshakeHandler.class);

    private final OrbienClientConfig config;
    private final EventLoopGroup group;
    private final Map<String, OrbienClientConfig.ProxyConfig> byName;

    private ByteBuf cumulation;
    private boolean headerDone;

    public WorkHandshakeHandler(OrbienClientConfig config, EventLoopGroup group) {
        this.config = config;
        this.group = group;
        this.byName = new ConcurrentHashMap<>();
        for (OrbienClientConfig.ProxyConfig p : config.getProxies()) {
            if (p.getName() != null) {
                byName.put(p.getName(), p);
            }
        }
    }

    @Override
    public void channelRead(ChannelHandlerContext ctx, Object msg) {
        ByteBuf buf = (ByteBuf) msg;
        if (headerDone) {
            ReferenceCountUtil.release(buf);
            return;
        }
        if (cumulation == null) {
            cumulation = ctx.alloc().buffer(buf.readableBytes());
        }
        cumulation.writeBytes(buf);
        buf.release();

        if (cumulation.readableBytes() < 5) {
            return;
        }
        cumulation.markReaderIndex();
        byte type = cumulation.readByte();
        long len = cumulation.readUnsignedIntLE();
        if (len > MsgCodec.MAX_BODY) {
            log.error("StartWorkConn body too large: {}", len);
            ctx.close();
            return;
        }
        if (cumulation.readableBytes() < len) {
            cumulation.resetReaderIndex();
            return;
        }
        byte[] body = new byte[(int) len];
        cumulation.readBytes(body);

        WireMessage wire;
        try {
            wire = MsgCodec.decode(type, body);
        } catch (Exception e) {
            log.error("failed to decode StartWorkConn", e);
            ctx.close();
            return;
        }
        if (wire.type() != MsgType.START_WORK_CONN) {
            log.warn("unexpected work message type={}", (char) wire.type());
            ctx.close();
            return;
        }

        StartWorkConn start = wire.body();
        ByteBuf leftover = cumulation.isReadable() ? cumulation.readRetainedSlice(cumulation.readableBytes()) : null;
        cumulation.release();
        cumulation = null;
        headerDone = true;

        bridge(ctx, start, leftover);
    }

    private void bridge(ChannelHandlerContext ctx, StartWorkConn start, ByteBuf leftover) {
        if (start.error != null && !start.error.isEmpty()) {
            log.error("StartWorkConn rejected: {}", start.error);
            ReferenceCountUtil.release(leftover);
            ctx.close();
            return;
        }
        OrbienClientConfig.ProxyConfig proxy = byName.get(start.proxyName);
        if (proxy == null) {
            log.error("unknown proxy name={}", start.proxyName);
            ReferenceCountUtil.release(leftover);
            ctx.close();
            return;
        }

        Channel work = ctx.channel();
        work.config().setAutoRead(false);

        if (ctx.pipeline().get(MsgFrameEncoder.class) != null) {
            ctx.pipeline().remove(MsgFrameEncoder.class);
        }

        Bootstrap b = new Bootstrap();
        b.group(group)
                .channel(NioSocketChannel.class)
                .option(ChannelOption.TCP_NODELAY, true)
                .option(ChannelOption.AUTO_READ, false)
                .handler(
                        new ChannelInitializer<SocketChannel>() {
                            @Override
                            protected void initChannel(SocketChannel ch) {
                            }
                        });

        b.connect(proxy.getLocalIp(), proxy.getLocalPort())
                .addListener(
                        (ChannelFutureListener)
                                f -> {
                                    if (!f.isSuccess()) {
                                        log.error(
                                                "failed to connect local {}:{} for proxy={}",
                                                proxy.getLocalIp(),
                                                proxy.getLocalPort(),
                                                start.proxyName,
                                                f.cause());
                                        ReferenceCountUtil.release(leftover);
                                        work.close();
                                        return;
                                    }
                                    Channel local = f.channel();
                                    local.pipeline().addLast(new ByteRelayHandler(work));
                                    ctx.pipeline()
                                            .replace(
                                                    WorkHandshakeHandler.this,
                                                    "work-relay",
                                                    new ByteRelayHandler(local));

                                    int leftoverBytes = leftover == null ? 0 : leftover.readableBytes();
                                    if (leftover != null) {
                                        local.writeAndFlush(leftover)
                                                .addListener(
                                                        wf -> {
                                                            if (!wf.isSuccess()) {
                                                                work.close();
                                                                local.close();
                                                            }
                                                        });
                                    }
                                    work.config().setAutoRead(true);
                                    local.config().setAutoRead(true);

                                    log.debug(
                                            "work connection bridged proxy={} local={}:{} remote={}:{} leftoverBytes={}",
                                            start.proxyName,
                                            proxy.getLocalIp(),
                                            proxy.getLocalPort(),
                                            start.srcAddr,
                                            start.srcPort,
                                            leftoverBytes);
                                });
    }

    @Override
    public void channelInactive(ChannelHandlerContext ctx) {
        if (cumulation != null) {
            cumulation.release();
            cumulation = null;
        }
    }

    @Override
    public void exceptionCaught(ChannelHandlerContext ctx, Throwable cause) {
        log.error("work connection error", cause);
        ctx.close();
    }
}
