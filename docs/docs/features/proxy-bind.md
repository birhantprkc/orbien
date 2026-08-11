---
sidebar_position: 3
sidebar_label: 代理绑定地址
title: 代理绑定地址
---

# 代理绑定地址

服务端有两类监听：

| 用途   | 配置                                      | 谁连                    |
|------|-----------------------------------------|-----------------------|
| 控制通道 | `bindAddr` + `bindPort`（以及 QUIC/KCP 端口） | **客户端** orbien 连上来建隧道 |
| 对外代理 | `proxyBindAddr` + `remotePort` / 虚拟主机端口 | **公网访客** 访问你穿透出去的服务   |

`proxyBindAddr` 决定访客流量绑在哪张网卡 / 哪个地址上，与客户端连哪无关。

典型场景：机器多网卡时，控制通道只听内网，代理只挂公网 IP；或反过来隔离管理面与业务面。

默认 `0.0.0.0`（所有网卡）。留空则回退为 `bindAddr`。

## 示例

控制通道与代理都对所有网卡开放（默认行为）：

```toml
# orbien-server.toml
bindAddr = "0.0.0.0"
bindPort = 9527
proxyBindAddr = "0.0.0.0"
```

仅在公网网卡上对外提供代理（示例 IP）：

```toml
# orbien-server.toml
bindAddr = "10.0.0.2"
bindPort = 9527
proxyBindAddr = "203.0.113.10"
```

此时客户端连 `10.0.0.2:9527`；访客访问 `203.0.113.10` 上的 `remotePort` / `vhostHTTPPort` / `vhostHTTPSPort`。

## 参数

| 参数              | 必填 | 默认值       | 说明                        |
|-----------------|----|-----------|---------------------------|
| `proxyBindAddr` | 否  | `0.0.0.0` | 对外代理监听地址；空则回退为 `bindAddr` |
