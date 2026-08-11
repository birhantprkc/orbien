---
sidebar_position: 1
sidebar_label: TCP
title: TCP
---

# TCP

默认传输协议。客户端通过服务端 `bindPort` 建立控制通道。TLS 配置见 [TLS](../tls.md)。

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
protocol = "tcp"
tcpMux = true
heartbeatInterval = -1
```

## 客户端参数

| 参数                            | 必填 | 默认值    | 说明                                       |
|-------------------------------|----|--------|------------------------------------------|
| `transport.protocol`          | 否  | `tcp`  | 固定为 `tcp`                                |
| `transport.tcpMux`            | 否  | `true` | TCP 多路复用；需与服务端一致                         |
| `transport.heartbeatInterval` | 否  | `-1`   | 应用心跳间隔（秒）；`-1` 关闭。关闭 `tcpMux` 时自动变为 `30` |

## 服务端参数

| 参数                 | 必填 | 默认值    | 说明               |
|--------------------|----|--------|------------------|
| `bindPort`         | 是  | `9527` | TCP 监听端口         |
| `transport.tcpMux` | 否  | `true` | TCP 多路复用；需与客户端一致 |
