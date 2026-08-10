package io.github.lxien.orbien.client.msg;

import com.fasterxml.jackson.databind.PropertyNamingStrategies;
import com.fasterxml.jackson.databind.annotation.JsonNaming;

@JsonNaming(PropertyNamingStrategies.SnakeCaseStrategy.class)
public class StartWorkConn {
    public String proxyName = "";
    public String srcAddr = "";
    public int srcPort;
    public String dstAddr = "";
    public int dstPort;
    public String error = "";
}
