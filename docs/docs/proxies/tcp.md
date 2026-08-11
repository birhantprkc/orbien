---
sidebar_position: 1
sidebar_label: TCP
title: TCP
---

# TCP

将内网 TCP 服务映射到服务端公网端口。

## 示例：穿透 SSH

```toml
# orbien.toml
serverAddr = "YOUR_SERVER_IP"
serverPort = 9527

[[proxies]]
name = "ssh"
type = "tcp"
localIP = "127.0.0.1"
localPort = 22
remotePort = 9000
```

```shell
./orbien -c orbien.toml
```

外网访问：

```shell
ssh -p 9000 user@YOUR_SERVER_IP
```

## 参数

| 参数                               | 必填 | 默认值         | 说明                          |
|----------------------------------|----|-------------|-----------------------------|
| `name`                           | 是  |             | 代理名称，唯一                     |
| `type`                           | 是  |             | 固定为 `tcp`                   |
| `localIP`                        | 否  | `127.0.0.1` | 本地服务地址                      |
| `localPort`                      | 是  |             | 本地服务端口                      |
| `remotePort`                     | 是  |             | 服务端对外监听端口                   |
| `transport.bandwidthLimit`       | 否  |             | 带宽限制，如 `1MB`、`100KB`；空表示不限制 |
| `transport.bandwidthLimitMode`   | 否  | `client`    | 限速端：`client` / `server`     |
| `transport.proxyProtocolVersion` | 否  |             | PROXY Protocol：`v1` / `v2`  |
