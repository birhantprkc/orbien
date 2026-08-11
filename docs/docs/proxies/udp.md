---
sidebar_position: 2
sidebar_label: UDP
title: UDP
---

# UDP

将内网 UDP 服务映射到服务端公网端口。

两端 `udpPacketSize` 需一致（默认 `1500`）。

## 示例：穿透 DNS

服务端：

```toml
# orbien-server.toml
bindAddr = "0.0.0.0"
bindPort = 9527
udpPacketSize = 1500
```

客户端：

```toml
# orbien.toml
serverAddr = "YOUR_SERVER_IP"
serverPort = 9527
udpPacketSize = 1500

[[proxies]]
name = "dns"
type = "udp"
localIP = "127.0.0.1"
localPort = 53
remotePort = 9000
```

外网访问：

```shell
dig @YOUR_SERVER_IP -p 9000 example.com
```

## 参数

| 参数                               | 必填 | 默认值         | 说明                          |
|----------------------------------|----|-------------|-----------------------------|
| `udpPacketSize`                  | 否  | `1500`      | UDP 最大报文长度，需与服务端一致          |
| `name`                           | 是  |             | 代理名称，唯一                     |
| `type`                           | 是  |             | 固定为 `udp`                   |
| `localIP`                        | 否  | `127.0.0.1` | 本地服务地址                      |
| `localPort`                      | 是  |             | 本地服务端口                      |
| `remotePort`                     | 是  |             | 服务端对外监听端口                   |
| `transport.bandwidthLimit`       | 否  |             | 带宽限制，如 `1MB`、`100KB`；空表示不限制 |
| `transport.bandwidthLimitMode`   | 否  | `client`    | 限速端：`client` / `server`     |
| `transport.proxyProtocolVersion` | 否  |             | PROXY Protocol：`v1` / `v2`  |
