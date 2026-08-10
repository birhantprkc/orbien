package io.github.lxien.orbien.client.netty;

@FunctionalInterface
public interface WorkConnFactory {
    void openWorkConn(String runId);
}
