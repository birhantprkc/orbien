---
sidebar_position: 4
sidebar_label: HTTPS
title: HTTPS
---

# HTTPS

通过域名将内网服务暴露到公网。服务端需配置虚拟主机，见 [虚拟主机](./vhost.md)。

两种模式：

- **透传**：按 SNI 转发，证书在内网 HTTPS 服务上
- **TLS 终止**：客户端插件 `https2http` 终止 TLS，再转发到本地 HTTP

## 示例：透传

服务端：

```toml
# orbien-server.toml
bindAddr = "0.0.0.0"
bindPort = 9527
vhostHTTPSPort = 443
```

客户端：

```toml
# orbien.toml
serverAddr = "YOUR_SERVER_IP"
serverPort = 9527

[[proxies]]
name = "web-ssl"
type = "https"
localIP = "127.0.0.1"
localPort = 443
customDomains = ["web.example.com"]
```

将 `web.example.com` 解析到服务端 IP 后访问：

```shell
curl https://web.example.com
```

## 示例：TLS 终止

客户端终止 TLS，后端只需提供 HTTP：

```toml
# orbien.toml
serverAddr = "YOUR_SERVER_IP"
serverPort = 9527

[[proxies]]
name = "web-ssl"
type = "https"
customDomains = ["web.example.com"]

[proxies.plugin]
type = "https2http"
localAddr = "127.0.0.1:80"
crtPath = "/path/to/cert.pem"
keyPath = "/path/to/key.pem"
hostHeaderRewrite = "127.0.0.1"

[proxies.plugin.requestHeaders.set]
X-From = "orbien"
```

`crtPath` / `keyPath` 可省略，省略时使用临时自签证书（浏览器会提示不受信任）。启用插件后不再使用 `localIP` / `localPort`
，且不可配置 PROXY Protocol。

## 参数

| 参数                               | 必填 | 默认值         | 说明                                                    |
|----------------------------------|----|-------------|-------------------------------------------------------|
| `name`                           | 是  |             | 代理名称，唯一                                               |
| `type`                           | 是  |             | 固定为 `https`                                           |
| `localIP`                        | 否  | `127.0.0.1` | 本地服务地址（透传）                                            |
| `localPort`                      | 条件 |             | 本地服务端口（透传必填）                                          |
| `customDomains`                  | 条件 |             | 自定义域名，可多个；与 `subdomain` 至少填一项                         |
| `subdomain`                      | 条件 |             | 子域名前缀，与服务端 `subDomainHost` 拼接；与 `customDomains` 至少填一项 |
| `plugin.type`                    | 否  |             | `https2http`：客户端终止 TLS                                |
| `plugin.localAddr`               | 条件 |             | https2http 时必填，如 `127.0.0.1:80`                       |
| `plugin.crtPath`                 | 否  |             | 证书路径；空则临时自签                                           |
| `plugin.keyPath`                 | 否  |             | 私钥路径；空则临时自签                                           |
| `plugin.hostHeaderRewrite`       | 否  |             | 改写转发到本地服务的 Host；空表示不改                                 |
| `plugin.requestHeaders.set`      | 否  |             | 向后端追加请求头，键值对                                          |
| `transport.bandwidthLimit`       | 否  |             | 带宽限制，如 `1MB`、`100KB`；空表示不限制                           |
| `transport.bandwidthLimitMode`   | 否  | `client`    | 限速端：`client` / `server`                               |
| `transport.proxyProtocolVersion` | 否  |             | PROXY Protocol：`v1` / `v2`（https2http 不可用）            |
