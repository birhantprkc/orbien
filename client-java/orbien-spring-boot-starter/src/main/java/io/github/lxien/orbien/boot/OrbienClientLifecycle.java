package io.github.lxien.orbien.boot;

import io.github.lxien.orbien.client.OrbienClient;
import io.github.lxien.orbien.client.OrbienClientConfig;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.boot.context.event.ApplicationReadyEvent;
import org.springframework.boot.web.context.WebServerApplicationContext;
import org.springframework.context.ApplicationContext;
import org.springframework.context.event.EventListener;
import org.springframework.core.env.Environment;
import org.springframework.util.StringUtils;

public class OrbienClientLifecycle {
    private static final Logger log = LoggerFactory.getLogger(OrbienClientLifecycle.class);
    private static final String DEFAULT_LOCAL_IP = "127.0.0.1";

    private final OrbienClient client;
    private final OrbienProperties properties;

    public OrbienClientLifecycle(OrbienClient client, OrbienProperties properties) {
        this.client = client;
        this.properties = properties;
    }

    @EventListener(ApplicationReadyEvent.class)
    public void onReady(ApplicationReadyEvent event) {
        if (!properties.isEnabled()) {
            return;
        }
        if (properties.isTcpMux()) {
            throw new IllegalStateException(
                    "orbien.tcp-mux=true is not supported; set false on client and server");
        }
        applyLocalDefaults(event.getApplicationContext());
        log.info("starting Orbien client");
        client.start();
    }

    private void applyLocalDefaults(ApplicationContext applicationContext) {
        OrbienProperties.Proxy proxyProps = properties.getProxy();
        if (!properties.hasProxy()) {
            return;
        }

        String localIp = proxyProps.getLocalIp();
        if (!StringUtils.hasText(localIp)) {
            localIp = DEFAULT_LOCAL_IP;
            proxyProps.setLocalIp(localIp);
        }

        int localPort = proxyProps.getLocalPort();
        if (localPort <= 0) {
            localPort = resolveLocalPort(applicationContext);
            proxyProps.setLocalPort(localPort);
            log.info("orbien.proxy.local-port not set; using Spring Boot port {}", localPort);
        }

        String name = proxyProps.getName();
        if (!StringUtils.hasText(name)) {
            name = defaultProxyName(applicationContext.getEnvironment(), proxyProps.getType());
            proxyProps.setName(name);
            log.info("orbien.proxy.name not set; using {}", name);
        }

        syncClientProxy(name, localIp, localPort);
    }

    private void syncClientProxy(String name, String localIp, int localPort) {
        OrbienClientConfig cfg = client.config();
        OrbienClientConfig.ProxyConfig proxy;
        if (cfg.getProxies().isEmpty()) {
            proxy = properties.toClientConfig().getProxies().stream().findFirst().orElse(null);
            if (proxy == null) {
                return;
            }
            cfg.getProxies().add(proxy);
        } else {
            proxy = cfg.getProxies().get(0);
        }
        proxy.setName(name);
        proxy.setLocalIp(localIp);
        proxy.setLocalPort(localPort);
    }

    static int resolveLocalPort(ApplicationContext applicationContext) {
        if (applicationContext instanceof WebServerApplicationContext webContext) {
            try {
                int port = webContext.getWebServer().getPort();
                if (port > 0) {
                    return port;
                }
            } catch (IllegalStateException ex) {
                log.debug("web server not ready while resolving local port: {}", ex.getMessage());
            }
        }

        Environment env = applicationContext.getEnvironment();
        Integer localServerPort = env.getProperty("local.server.port", Integer.class);
        if (localServerPort != null && localServerPort > 0) {
            return localServerPort;
        }

        Integer serverPort = env.getProperty("server.port", Integer.class);
        if (serverPort != null && serverPort > 0) {
            return serverPort;
        }

        throw new IllegalStateException(
                "orbien.proxy.local-port is not set and the Spring Boot web server port could not be"
                        + " determined; set orbien.proxy.local-port explicitly");
    }

    static String defaultProxyName(Environment env, String type) {
        String appName = env.getProperty("spring.application.name");
        if (StringUtils.hasText(appName)) {
            return appName.trim();
        }
        String proxyType = StringUtils.hasText(type) ? type.trim().toLowerCase() : "tcp";
        return "orbien-" + proxyType;
    }
}
