---
sidebar_position: 4
sidebar_label: 带宽限制
title: 带宽限制
---

# 带宽限制

按代理限制转发带宽。配置在 `[[proxies]]` 的 `transport` 下。

- `bandwidthLimitMode = "client"`：在客户端限速
- `bandwidthLimitMode = "server"`：在服务端限速

单位仅支持 `KB`、`MB`（如 `100KB`、`1MB`）。留空表示不限制。

## 示例：客户端限速

```toml
# orbien.toml
[[proxies]]
name = "web"
type = "tcp"
localIP = "127.0.0.1"
localPort = 80
remotePort = 9000
transport.bandwidthLimit = "1MB"
transport.bandwidthLimitMode = "client"
```

## 示例：服务端限速

```toml
# orbien.toml
[[proxies]]
name = "web"
type = "tcp"
localIP = "127.0.0.1"
localPort = 80
remotePort = 9000
transport.bandwidthLimit = "500KB"
transport.bandwidthLimitMode = "server"
```

## 参数

| 参数                             | 必填 | 默认值      | 说明                          |
|--------------------------------|----|----------|-----------------------------|
| `transport.bandwidthLimit`     | 否  |          | 带宽上限，如 `1MB`、`100KB`；空表示不限制 |
| `transport.bandwidthLimitMode` | 否  | `client` | 限速端：`client` / `server`     |
