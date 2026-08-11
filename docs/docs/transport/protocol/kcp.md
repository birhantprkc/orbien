---
sidebar_position: 4
sidebar_label: KCP
title: KCP
---

# KCP

基于 UDP 的可靠传输，弱网场景更稳。TLS 配置见 [TLS](../tls.md)。

## 示例

服务端：

```toml
# orbien-server.toml
bindAddr = "0.0.0.0"
bindPort = 9527
kcpBindPort = 9529

[transport]
tcpMux = true
```

客户端：

```toml
# orbien.toml
serverAddr = "YOUR_SERVER_IP"
serverPort = 9529

[transport]
protocol = "kcp"
tcpMux = true
heartbeatInterval = -1
```

`serverPort` 需指向服务端 `kcpBindPort`，且不可与 `quicBindPort` 相同。

## 客户端参数

| 参数                            | 必填 | 默认值    | 说明                                       |
|-------------------------------|----|--------|------------------------------------------|
| `transport.protocol`          | 是  | `tcp`  | 固定为 `kcp`                                |
| `serverPort`                  | 是  |        | 填服务端 `kcpBindPort`                       |
| `transport.tcpMux`            | 否  | `true` | TCP 多路复用；需与服务端一致                         |
| `transport.heartbeatInterval` | 否  | `-1`   | 应用心跳间隔（秒）；`-1` 关闭。关闭 `tcpMux` 时自动变为 `30` |

## 服务端参数

| 参数                 | 必填 | 默认值    | 说明                                           |
|--------------------|----|--------|----------------------------------------------|
| `kcpBindPort`      | 是  | `0`    | KCP 监听端口（UDP）；`0` 表示关闭；不可与 `quicBindPort` 相同 |
| `transport.tcpMux` | 否  | `true` | TCP 多路复用；需与客户端一致                             |
