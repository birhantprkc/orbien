package io.github.lxien.orbien.client;

import java.util.ArrayList;
import java.util.List;
import java.util.Objects;

public final class OrbienClientConfig {
    private String serverAddr = "127.0.0.1";
    private int serverPort = 9527;
    private String token = "";
    private boolean tcpMux = false;
    private int poolCount = 1;
    private String user = "";
    private String runId = "";
    private String runIdFile = "";
    private int heartbeatIntervalSecs = 30;
    private final List<ProxyConfig> proxies = new ArrayList<>();

    public String getServerAddr() {
        return serverAddr;
    }

    public void setServerAddr(String serverAddr) {
        this.serverAddr = Objects.requireNonNull(serverAddr, "serverAddr");
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
        this.token = token == null ? "" : token;
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
        this.user = user == null ? "" : user;
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

    public int getHeartbeatIntervalSecs() {
        return heartbeatIntervalSecs;
    }

    public void setHeartbeatIntervalSecs(int heartbeatIntervalSecs) {
        this.heartbeatIntervalSecs = heartbeatIntervalSecs;
    }

    public List<ProxyConfig> getProxies() {
        return proxies;
    }

    public static final class ProxyConfig {
        private String type = "tcp";
        private String name;
        private String localIp = "127.0.0.1";
        private int localPort;
        private int remotePort;
        private List<String> customDomains = new ArrayList<>();
        private String subdomain = "";

        public String getType() {
            return type;
        }

        public void setType(String type) {
            this.type = type;
        }

        public String getName() {
            return name;
        }

        public void setName(String name) {
            this.name = name;
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
