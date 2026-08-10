package io.github.lxien.orbien.client.msg;

import com.fasterxml.jackson.databind.DeserializationFeature;
import com.fasterxml.jackson.databind.ObjectMapper;

import java.io.IOException;

public final class MsgCodec {
    public static final int MAX_BODY = 4 * 1024 * 1024;

    private static final ObjectMapper MAPPER =
            new ObjectMapper().configure(DeserializationFeature.FAIL_ON_UNKNOWN_PROPERTIES, false);

    private MsgCodec() {
    }

    public static ObjectMapper mapper() {
        return MAPPER;
    }

    public static byte[] encodeBody(Object body) {
        try {
            if (body == null) {
                return new byte[]{'{', '}'};
            }
            return MAPPER.writeValueAsBytes(body);
        } catch (IOException e) {
            throw new IllegalArgumentException("json encode failed", e);
        }
    }

    public static WireMessage decode(byte type, byte[] body) throws IOException {
        Object parsed =
                switch (type) {
                    case MsgType.LOGIN -> MAPPER.readValue(body, Login.class);
                    case MsgType.LOGIN_RESP -> MAPPER.readValue(body, LoginResp.class);
                    case MsgType.NEW_PROXY -> MAPPER.readValue(body, NewProxy.class);
                    case MsgType.NEW_PROXY_RESP -> MAPPER.readValue(body, NewProxyResp.class);
                    case MsgType.NEW_WORK_CONN -> MAPPER.readValue(body, NewWorkConn.class);
                    case MsgType.REQ_WORK_CONN -> MAPPER.readValue(body, ReqWorkConn.class);
                    case MsgType.START_WORK_CONN -> MAPPER.readValue(body, StartWorkConn.class);
                    case MsgType.PING -> MAPPER.readValue(body, Ping.class);
                    case MsgType.PONG -> MAPPER.readValue(body, Pong.class);
                    case MsgType.KICK_OUT -> MAPPER.readValue(body, KickOut.class);
                    default -> throw new IOException("unknown message type: " + (type & 0xff));
                };
        return new WireMessage(type, parsed);
    }
}
