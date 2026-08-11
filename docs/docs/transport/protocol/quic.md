---
sidebar_position: 3
sidebar_label: QUIC
title: QUIC
---

# QUIC

基于 UDP。自带加密与多路复用，`tcpMux` / `transport.tls.enable` 无效。证书校验仍可通过 `transport.tls`
配置，详见 [TLS](../tls.md)。

## 示例

服务端：

```toml
# orbien-server.toml
bindAddr = "0.0.0.0"
bindPort = 9527
quicBindPort = 9528

[transport.quic]
keepalivePeriod = 10
maxIdleTimeout = 30
maxIncomingStreams = 100000
```

客户端：

```toml
# orbien.toml
serverAddr = "YOUR_SERVER_IP"
serverPort = 9528

[transport]
protocol = "quic"

[transport.quic]
keepalivePeriod = 10
maxIdleTimeout = 30
```

`serverPort` 需指向服务端 `quicBindPort`。

## 客户端参数

| 参数                               | 必填 | 默认值   | 说明                  |
|----------------------------------|----|-------|---------------------|
| `transport.protocol`             | 是  | `tcp` | 固定为 `quic`          |
| `serverPort`                     | 是  |       | 填服务端 `quicBindPort` |
| `transport.quic.keepalivePeriod` | 否  | `10`  | QUIC 保活周期（秒）        |
| `transport.quic.maxIdleTimeout`  | 否  | `30`  | QUIC 空闲超时（秒）        |

## 服务端参数

| 参数                                  | 必填 | 默认值      | 说明                                           |
|-------------------------------------|----|----------|----------------------------------------------|
| `quicBindPort`                      | 是  | `0`      | QUIC 监听端口（UDP）；`0` 表示关闭；不可与 `kcpBindPort` 相同 |
| `transport.quic.keepalivePeriod`    | 否  | `10`     | QUIC 保活周期（秒）                                 |
| `transport.quic.maxIdleTimeout`     | 否  | `30`     | QUIC 空闲超时（秒）                                 |
| `transport.quic.maxIncomingStreams` | 否  | `100000` | QUIC 最大入站流                                   |
