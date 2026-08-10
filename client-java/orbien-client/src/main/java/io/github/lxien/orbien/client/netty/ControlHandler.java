package io.github.lxien.orbien.client.netty;

import io.github.lxien.orbien.client.OrbienClientConfig;
import io.github.lxien.orbien.client.auth.AuthKeys;
import io.github.lxien.orbien.client.msg.KickOut;
import io.github.lxien.orbien.client.msg.LoginResp;
import io.github.lxien.orbien.client.msg.MsgType;
import io.github.lxien.orbien.client.msg.NewProxy;
import io.github.lxien.orbien.client.msg.NewProxyResp;
import io.github.lxien.orbien.client.msg.Ping;
import io.github.lxien.orbien.client.msg.WireMessage;
import io.netty.channel.ChannelHandlerContext;
import io.netty.channel.SimpleChannelInboundHandler;
import io.netty.util.concurrent.ScheduledFuture;
import java.util.concurrent.TimeUnit;
import java.util.concurrent.atomic.AtomicBoolean;
import java.util.function.Consumer;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

public final class ControlHandler extends SimpleChannelInboundHandler<WireMessage> {
    private static final Logger log = LoggerFactory.getLogger(ControlHandler.class);

    private final OrbienClientConfig config;
    private final Consumer<String> onLoginOk;
    private final Consumer<Throwable> onFailure;
    private final WorkConnFactory workFactory;
    private final AtomicBoolean proxiesRegistered = new AtomicBoolean(false);
    private volatile String runId = "";
    private ScheduledFuture<?> heartbeat;

    public ControlHandler(
            OrbienClientConfig config,
            WorkConnFactory workFactory,
            Consumer<String> onLoginOk,
            Consumer<Throwable> onFailure) {
        this.config = config;
        this.workFactory = workFactory;
        this.onLoginOk = onLoginOk;
        this.onFailure = onFailure;
    }

    public String runId() {
        return runId;
    }

    @Override
    protected void channelRead0(ChannelHandlerContext ctx, WireMessage msg) {
        switch (msg.type()) {
            case MsgType.LOGIN_RESP -> handleLoginResp(ctx, msg.body());
            case MsgType.REQ_WORK_CONN -> workFactory.openWorkConn(runId);
            case MsgType.NEW_PROXY_RESP -> handleNewProxyResp(msg.body());
            case MsgType.PONG -> log.trace("received Pong");
            case MsgType.KICK_OUT -> {
                KickOut k = msg.body();
                log.warn("disconnected by server: {}", k.reason);
                ctx.close();
            }
            default -> log.warn("unsupported control message type={}", (char) msg.type());
        }
    }

    private void handleLoginResp(ChannelHandlerContext ctx, LoginResp resp) {
        if (resp.error != null && !resp.error.isEmpty()) {
            onFailure.accept(new IllegalStateException("login failed: " + resp.error));
            ctx.close();
            return;
        }
        this.runId = resp.runId == null ? "" : resp.runId;
        log.debug("login succeeded, runId={}", runId);
        registerProxies(ctx);
        startHeartbeat(ctx);
        onLoginOk.accept(runId);
    }

    private void registerProxies(ChannelHandlerContext ctx) {
        if (!proxiesRegistered.compareAndSet(false, true)) {
            return;
        }
        for (OrbienClientConfig.ProxyConfig p : config.getProxies()) {
            String type = p.getType() == null ? "" : p.getType().toLowerCase();
            if (!"tcp".equals(type) && !"http".equals(type)) {
                log.warn("unsupported proxy type={} name={}", type, p.getName());
                continue;
            }
            NewProxy np = new NewProxy();
            np.proxyName = p.getName();
            np.proxyType = type;
            np.localIp = p.getLocalIp() == null ? "" : p.getLocalIp();
            np.localPort = p.getLocalPort();
            if ("tcp".equals(type)) {
                np.remotePort = p.getRemotePort();
            } else {
                np.remotePort = 0;
                np.customDomains = p.getCustomDomains();
                np.subdomain = p.getSubdomain();
            }
            ctx.writeAndFlush(new WireMessage(MsgType.NEW_PROXY, np));
            log.debug(
                    "NewProxy requested name={} type={} local={}:{} remotePort={} domains={}",
                    p.getName(),
                    type,
                    p.getLocalIp(),
                    p.getLocalPort(),
                    np.remotePort,
                    np.customDomains);
        }
    }

    private void handleNewProxyResp(NewProxyResp resp) {
        if (resp.error != null && !resp.error.isEmpty()) {
            log.error("proxy registration failed name={} error={}", resp.proxyName, resp.error);
            return;
        }
        OrbienClientConfig.ProxyConfig proxy = findProxy(resp.proxyName);
        String local =
                proxy == null ? "?" : proxy.getLocalIp() + ":" + proxy.getLocalPort();
        String remote = formatRemoteAddr(resp.remoteAddr, proxy);
        log.info(
                """
                
                ============================================================
                 Tunnel ready: {} -> {}
                ============================================================
                """,
                local,
                remote);
    }

    private OrbienClientConfig.ProxyConfig findProxy(String name) {
        if (name == null) {
            return null;
        }
        for (OrbienClientConfig.ProxyConfig p : config.getProxies()) {
            if (name.equals(p.getName())) {
                return p;
            }
        }
        return null;
    }

    private String formatRemoteAddr(String remoteAddr, OrbienClientConfig.ProxyConfig proxy) {
        String remote = remoteAddr == null ? "" : remoteAddr.trim();
        if (remote.startsWith(":")) {
            remote = config.getServerAddr() + remote;
        } else if (remote.isEmpty()) {
            if (proxy != null && proxy.getRemotePort() > 0) {
                remote = config.getServerAddr() + ":" + proxy.getRemotePort();
            } else {
                return "?";
            }
        }
        if (remote.startsWith("http://") || remote.startsWith("https://")) {
            return remote;
        }
        return "http://" + remote;
    }

    private void startHeartbeat(ChannelHandlerContext ctx) {
        int interval = Math.max(config.getHeartbeatIntervalSecs(), 0);
        if (interval <= 0) {
            return;
        }
        heartbeat =
                ctx.executor()
                        .scheduleAtFixedRate(
                                () -> {
                                    if (!ctx.channel().isActive()) {
                                        return;
                                    }
                                    long ts = System.currentTimeMillis() / 1000;
                                    Ping ping = new Ping();
                                    ping.timestamp = ts;
                                    ping.privilegeKey = AuthKeys.getAuthKey(config.getToken(), ts);
                                    ctx.writeAndFlush(new WireMessage(MsgType.PING, ping));
                                },
                                interval,
                                interval,
                                TimeUnit.SECONDS);
    }

    @Override
    public void channelInactive(ChannelHandlerContext ctx) {
        if (heartbeat != null) {
            heartbeat.cancel(false);
        }
        log.info("control connection closed");
    }

    @Override
    public void exceptionCaught(ChannelHandlerContext ctx, Throwable cause) {
        log.error("control connection error", cause);
        onFailure.accept(cause);
        ctx.close();
    }
}
