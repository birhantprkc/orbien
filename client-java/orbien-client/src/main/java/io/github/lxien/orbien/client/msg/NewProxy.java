package io.github.lxien.orbien.client.msg;

import com.fasterxml.jackson.databind.PropertyNamingStrategies;
import com.fasterxml.jackson.databind.annotation.JsonNaming;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

@JsonNaming(PropertyNamingStrategies.SnakeCaseStrategy.class)
public class NewProxy {
    public String proxyName;
    public String proxyType;
    public int remotePort;
    public String localIp = "";
    public int localPort;
    public List<String> customDomains = new ArrayList<>();
    public String subdomain = "";
    public List<String> locations = new ArrayList<>();
    public String httpUser = "";
    public String httpPwd = "";
    public String hostHeaderRewrite = "";
    public Map<String, String> headers = new HashMap<>();
    public Map<String, String> responseHeaders = new HashMap<>();
    public String routeByHttpUser = "";
    public String bandwidthLimit = "";
    public String bandwidthLimitMode = "";
}
