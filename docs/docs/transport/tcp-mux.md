---
sidebar_position: 2.5
sidebar_label: TCP 多路复用
title: TCP 多路复用
---

# TCP 多路复用

一条物理连接承载多路逻辑流（Yamux），降低建连开销。适用于 `tcp` / `websocket` / `kcp`；**QUIC 自带多路复用，此项无效**。

客户端与服务端 `tcpMux` **必须一致**，否则无法握手。

## 示例：开启

TCP多路复用默认是开启状态。

服务端：

```toml
# orbien-server.toml
[transport]
tcpMux = true
```

客户端：

```toml
# orbien.toml
[transport]
tcpMux = true
```

开启后应用层心跳默认可关闭（`heartbeatInterval = -1`）。

## 示例：关闭

关闭后每条控制/工作流使用独立连接，需开启应用层心跳。

服务端：

```toml
# orbien-server.toml
[transport]
tcpMux = false
```

客户端：

```toml
# orbien.toml
[transport]
tcpMux = false
heartbeatInterval = 30
```

客户端若省略 `heartbeatInterval`，关闭 `tcpMux` 时会自动设为 `30`。

## 客户端参数

| 参数                            | 必填 | 默认值    | 说明                                       |
|-------------------------------|----|--------|------------------------------------------|
| `transport.tcpMux`            | 否  | `true` | 是否启用多路复用；需与服务端一致                         |
| `transport.heartbeatInterval` | 否  | `-1`   | 应用心跳间隔（秒）；`-1` 关闭。关闭 `tcpMux` 时自动变为 `30` |

## 服务端参数

| 参数                 | 必填 | 默认值    | 说明               |
|--------------------|----|--------|------------------|
| `transport.tcpMux` | 否  | `true` | 是否启用多路复用；需与客户端一致 |
