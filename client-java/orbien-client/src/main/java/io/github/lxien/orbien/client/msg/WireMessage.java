package io.github.lxien.orbien.client.msg;

public final class WireMessage {
    private final byte type;
    private final Object body;

    public WireMessage(byte type, Object body) {
        this.type = type;
        this.body = body;
    }

    public byte type() {
        return type;
    }

    @SuppressWarnings("unchecked")
    public <T> T body() {
        return (T) body;
    }
}
