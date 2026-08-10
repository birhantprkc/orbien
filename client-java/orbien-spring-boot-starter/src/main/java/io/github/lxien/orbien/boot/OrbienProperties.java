package io.github.lxien.orbien.boot;

import io.github.lxien.orbien.client.OrbienClientConfig;

import java.util.ArrayList;
import java.util.List;

import org.springframework.boot.context.properties.ConfigurationProperties;
import org.springframework.boot.context.properties.NestedConfigurationProperty;
import org.springframework.util.StringUtils;

@ConfigurationProperties(prefix = "orbien")
public class OrbienProperties {
    private static final String DEFAULT_LOCAL_IP = "127.0.0.1";

    private boolean enabled = true;
    private String serverAddr = "127.0.0.1";
    private int serverPort = 9527;
    private String token = "";
    private boolean tcpMux = false;
    private int poolCount = 1;
    private String user = "";
    private int heartbeatIntervalSecs = 30;
    private String runId = "";
    private String runIdFile = "";

    @NestedConfigurationProperty
    private final Proxy proxy = new Proxy();

    public boolean isEnabled() {
        return enabled;
    }

    public void setEnabled(boolean enabled) {
        this.enabled = enabled;
    }

    public String getServerAddr() {
        return serverAddr;
    }

    public void setServerAddr(String serverAddr) {
        this.serverAddr = serverAddr;
    }

    public int getServerPort() {
        return serverPort;
    }

    public void setServerPort(int serverPort) {
        this.serverPort = serverPort;
    }

    public String getToken() {
        return token;
    }

    public void setToken(String token) {
        this.token = token;
    }

    public boolean isTcpMux() {
        return tcpMux;
    }

    public void setTcpMux(boolean tcpMux) {
        this.tcpMux = tcpMux;
    }

    public int getPoolCount() {
        return poolCount;
    }

    public void setPoolCount(int poolCount) {
        this.poolCount = poolCount;
    }

    public String getUser() {
        return user;
    }

    public void setUser(String user) {
        this.user = user;
    }

    public int getHeartbeatIntervalSecs() {
        return heartbeatIntervalSecs;
    }

    public void setHeartbeatIntervalSecs(int heartbeatIntervalSecs) {
        this.heartbeatIntervalSecs = heartbeatIntervalSecs;
    }

    public String getRunId() {
        return runId;
    }

    public void setRunId(String runId) {
        this.runId = runId == null ? "" : runId;
    }

    public String getRunIdFile() {
        return runIdFile;
    }

    public void setRunIdFile(String runIdFile) {
        this.runIdFile = runIdFile == null ? "" : runIdFile;
    }

    public Proxy getProxy() {
        return proxy;
    }

    /** Whether a proxy tunnel is configured (name / port / domains / subdomain). */
    public boolean hasProxy() {
        return StringUtils.hasText(proxy.getName())
                || proxy.getLocalPort() > 0
                || proxy.getRemotePort() > 0
                || (proxy.getCustomDomains() != null && !proxy.getCustomDomains().isEmpty())
                || StringUtils.hasText(proxy.getSubdomain());
    }

    public OrbienClientConfig toClientConfig() {
        OrbienClientConfig cfg = new OrbienClientConfig();
        cfg.setServerAddr(serverAddr);
        cfg.setServerPort(serverPort);
        cfg.setToken(token);
        cfg.setTcpMux(tcpMux);
        cfg.setPoolCount(poolCount);
        cfg.setUser(user);
        cfg.setHeartbeatIntervalSecs(heartbeatIntervalSecs);
        cfg.setRunId(runId);
        cfg.setRunIdFile(runIdFile);
        if (hasProxy()) {
            OrbienClientConfig.ProxyConfig p = new OrbienClientConfig.ProxyConfig();
            String name = proxy.getName();
            if (!StringUtils.hasText(name)) {
                String type = StringUtils.hasText(proxy.getType()) ? proxy.getType() : "tcp";
                name = "orbien-" + type.toLowerCase();
            }
            p.setName(name);
            p.setType(proxy.getType());
            String localIp =
                    StringUtils.hasText(proxy.getLocalIp()) ? proxy.getLocalIp() : DEFAULT_LOCAL_IP;
            p.setLocalIp(localIp);
            p.setLocalPort(proxy.getLocalPort());
            p.setRemotePort(proxy.getRemotePort());
            p.setCustomDomains(new ArrayList<>(proxy.getCustomDomains()));
            p.setSubdomain(proxy.getSubdomain());
            cfg.getProxies().add(p);
        }
        return cfg;
    }

    public static class Proxy {
        private String name;
        private String type = "tcp";
        private String localIp = DEFAULT_LOCAL_IP;
        private int localPort;
        private int remotePort;
        private List<String> customDomains = new ArrayList<>();
        private String subdomain = "";

        public String getName() {
            return name;
        }

        public void setName(String name) {
            this.name = name;
        }

        public String getType() {
            return type;
        }

        public void setType(String type) {
            this.type = type;
        }

        public String getLocalIp() {
            return localIp;
        }

        public void setLocalIp(String localIp) {
            this.localIp = localIp;
        }

        public int getLocalPort() {
            return localPort;
        }

        public void setLocalPort(int localPort) {
            this.localPort = localPort;
        }

        public int getRemotePort() {
            return remotePort;
        }

        public void setRemotePort(int remotePort) {
            this.remotePort = remotePort;
        }

        public List<String> getCustomDomains() {
            return customDomains;
        }

        public void setCustomDomains(List<String> customDomains) {
            this.customDomains = customDomains == null ? new ArrayList<>() : customDomains;
        }

        public String getSubdomain() {
            return subdomain;
        }

        public void setSubdomain(String subdomain) {
            this.subdomain = subdomain == null ? "" : subdomain;
        }
    }
}
