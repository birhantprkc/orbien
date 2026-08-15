---
sidebar_position: 5
sidebar_label: 获取真实 IP
title: 获取真实 IP
---

# 获取真实 IP

穿透后本地服务默认看到的是隧道地址。可用以下方式拿到访客真实 IP。

## PROXY Protocol

客户端连接本地服务时写入 PROXY Protocol 头。本地服务需支持解析（如 Nginx、HAProxy）。

```toml
# orbien.toml
server = "YOUR_SERVER_IP:9527"

[[tunnels]]
name = "web"
protocol = "tcp"
service = "127.0.0.1:80"
remotePort = 9000
transport.proxyProtocolVersion = "v2"
```

适用于 `tcp` / `udp` / `http` / `https`（透传）。`tls-term`（TLS终止） 不可用。

| 参数                               | 必填 | 默认值 | 说明                |
|----------------------------------|----|-----|-------------------|
| `transport.proxyProtocolVersion` | 否  |     | `v1` / `v2`；空表示关闭 |

## X-Forwarded-For

`http` 由服务端自动注入；`https` + `tls-term` 由客户端插件自动注入，无需额外配置：

- `X-Forwarded-For`：访客 IP
- `X-Forwarded-Proto`：`http` 或 `https`

应用从请求头读取即可。

## 说明

服务端不解析前置负载均衡的 PROXY Protocol。访客源地址以直连 `proxyAddr` 的 TCP peer 为准。
