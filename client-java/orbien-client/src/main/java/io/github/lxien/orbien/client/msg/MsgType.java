package io.github.lxien.orbien.client.msg;

public final class MsgType {
    public static final byte LOGIN = 'o';
    public static final byte LOGIN_RESP = '1';
    public static final byte NEW_PROXY = 'p';
    public static final byte NEW_PROXY_RESP = '2';
    public static final byte CLOSE_PROXY = 'c';
    public static final byte NEW_WORK_CONN = 'w';
    public static final byte REQ_WORK_CONN = 'r';
    public static final byte START_WORK_CONN = 's';
    public static final byte PING = 'h';
    public static final byte PONG = '4';
    public static final byte UDP_PACKET = 'u';
    public static final byte KICK_OUT = 'k';

    private MsgType() {}
}
