package io.github.lxien.orbien.client.msg;

import com.fasterxml.jackson.databind.PropertyNamingStrategies;
import com.fasterxml.jackson.databind.annotation.JsonNaming;

@JsonNaming(PropertyNamingStrategies.SnakeCaseStrategy.class)
public class NewWorkConn {
    public String runId = "";
    public String privilegeKey = "";
    public long timestamp;
}
