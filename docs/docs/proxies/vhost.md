---
sidebar_position: 5
sidebar_label: 虚拟主机
title: 虚拟主机
---

# 虚拟主机

HTTP / HTTPS 按域名路由。服务端开启 `vhostHTTPPort` / `vhostHTTPSPort` 后，客户端用 `customDomains` 或 `subdomain` 绑定域名。

`subdomain` 需配合服务端 `subDomainHost`：最终域名为 `{subdomain}.{subDomainHost}`。

## 示例：自定义域名

服务端：

```toml
# orbien-server.toml
vhostHTTPPort = 80
vhostHTTPSPort = 443
```

客户端：

```toml
# orbien.toml
[[proxies]]
name = "web"
type = "http"
localIP = "127.0.0.1"
localPort = 8080
customDomains = ["web.example.com"]
```

将 `web.example.com` 解析到服务端 IP 后访问。

## 示例：子域名

服务端：

```toml
# orbien-server.toml
vhostHTTPPort = 80
subDomainHost = "example.com"
```

客户端：

```toml
# orbien.toml
[[proxies]]
name = "web"
type = "http"
localIP = "127.0.0.1"
localPort = 8080
subdomain = "blog"
```

访问域名为 `blog.example.com`。

## 参数

| 参数               | 必填 | 默认值 | 说明                                     |
|------------------|----|-----|----------------------------------------|
| `vhostHTTPPort`  | 否  | `0` | HTTP 虚拟主机端口；`0` 表示关闭                   |
| `vhostHTTPSPort` | 否  | `0` | HTTPS 虚拟主机端口；`0` 表示关闭                  |
| `subDomainHost`  | 否  |     | 子域名根域；客户端填 `subdomain` 时必填，拼接为 `前缀.根域` |
