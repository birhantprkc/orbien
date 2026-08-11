---
sidebar_position: 2
sidebar_label: WebSocket
title: WebSocket
---

# WebSocket

与 TCP 共用服务端 `bindPort`。适用于需穿透 HTTP 代理、或仅开放 WebSocket 的网络环境。TLS 配置见 [TLS](../tls.md)。

## 示例

服务端：

```toml
# orbien-server.toml
bindAddr = "0.0.0.0"
bindPort = 9527

[transport]
tcpMux = true
```

客户端：

```toml
# orbien.toml
serverAddr = "YOUR_SERVER_IP"
serverPort = 9527

[transport]
protocol = "websocket"
tcpMux = true
heartbeatInterval = -1
```

## 客户端参数

| 参数                            | 必填 | 默认值    | 说明                                       |
|-------------------------------|----|--------|------------------------------------------|
| `transport.protocol`          | 是  | `tcp`  | 固定为 `websocket`（或 `ws`）                  |
| `transport.tcpMux`            | 否  | `true` | TCP 多路复用；需与服务端一致                         |
| `transport.heartbeatInterval` | 否  | `-1`   | 应用心跳间隔（秒）；`-1` 关闭。关闭 `tcpMux` 时自动变为 `30` |

## 服务端参数

| 参数                 | 必填 | 默认值    | 说明               |
|--------------------|----|--------|------------------|
| `bindPort`         | 是  | `9527` | 与 TCP 共用监听端口     |
| `transport.tcpMux` | 否  | `true` | TCP 多路复用；需与客户端一致 |
