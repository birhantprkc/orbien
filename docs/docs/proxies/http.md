---
sidebar_position: 3
sidebar_label: HTTP
title: HTTP
---

# HTTP

通过域名将内网 HTTP 服务暴露到公网。服务端需配置虚拟主机，见 [虚拟主机](./vhost.md)。

## 示例：穿透 Web 服务

服务端：

```toml
# orbien-server.toml
bindAddr = "0.0.0.0"
bindPort = 9527
vhostHTTPPort = 80
```

客户端：

```toml
# orbien.toml
serverAddr = "YOUR_SERVER_IP"
serverPort = 9527

[[proxies]]
name = "web"
type = "http"
localIP = "127.0.0.1"
localPort = 8080
customDomains = ["web.example.com"]
```

将 `web.example.com` 解析到服务端 IP 后访问：

```shell
curl http://web.example.com
```

## 参数

| 参数                               | 必填  | 默认值         | 说明                                                    |
|----------------------------------|-----|-------------|-------------------------------------------------------|
| `name`                           | 是   |             | 代理名称，唯一                                               |
| `type`                           | 是   |             | 固定为 `http`                                            |
| `localIP`                        | 否   | `127.0.0.1` | 本地服务地址                                                |
| `localPort`                      | 是   |             | 本地服务端口                                                |
| `customDomains`                  | 条件  |             | 自定义域名，可多个；与 `subdomain` 至少填一项                         |
| `subdomain`                      | 条件  |             | 子域名前缀，与服务端 `subDomainHost` 拼接；与 `customDomains` 至少填一项 |
| `locations`                      | 否   |             | 路径前缀，如 `/api`；空表示全部                                   |
| `hostHeaderRewrite`              | 否   |             | 改写转发到本地服务的 Host；空表示不改                                 |
| `transport.bandwidthLimit`       | 否   |             | 带宽限制，如 `1MB`、`100KB`；空表示不限制                           |
| `transport.bandwidthLimitMode`   | 否   | `client`    | 限速端：`client` / `server`                               |
| `transport.proxyProtocolVersion` | 否   |             | PROXY Protocol：`v1` / `v2`                            |
