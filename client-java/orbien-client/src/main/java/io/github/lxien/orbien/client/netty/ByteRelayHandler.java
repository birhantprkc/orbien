package io.github.lxien.orbien.client.netty;

import io.netty.buffer.ByteBuf;
import io.netty.channel.Channel;
import io.netty.channel.ChannelHandlerContext;
import io.netty.channel.ChannelInboundHandlerAdapter;
import io.netty.util.ReferenceCountUtil;

public final class ByteRelayHandler extends ChannelInboundHandlerAdapter {
    private final Channel peer;

    public ByteRelayHandler(Channel peer) {
        this.peer = peer;
    }

    @Override
    public void channelRead(ChannelHandlerContext ctx, Object msg) {
        ByteBuf buf = (ByteBuf) msg;
        if (!peer.isActive()) {
            ReferenceCountUtil.release(buf);
            ctx.close();
            return;
        }
        peer.writeAndFlush(buf).addListener(
                f -> {
                    if (!f.isSuccess()) {
                        ctx.close();
                        peer.close();
                    }
                });
    }

    @Override
    public void channelInactive(ChannelHandlerContext ctx) {
        peer.close();
    }

    @Override
    public void exceptionCaught(ChannelHandlerContext ctx, Throwable cause) {
        ctx.close();
        peer.close();
    }
}
