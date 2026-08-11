---
sidebar_position: 3
sidebar_label: TLS
title: TLS
---

# TLS

控制通道 TLS。适用于 `tcp` / `websocket` / `kcp`（QUIC 自身已加密，但证书路径字段仍可用于 QUIC 身份校验）。

三种方式：

1. **仅加密**：默认，不校验证书
2. **校验服务端证书**：客户端信任 CA，校验服务端身份
3. **双向认证（mTLS）**：双方互验证书

## 方式一：仅加密

不填 `trustedCaFile`，客户端跳过证书校验；服务端证书可省略（临时自签）。

服务端：

```toml
# orbien-server.toml
[transport.tls]
force = false
```

客户端：

```toml
# orbien.toml
[transport.tls]
enable = true
disableCustomTLSFirstByte = true
```

## 方式二：校验服务端证书

服务端提供证书；客户端用 CA 校验，并设置 SNI。

服务端：

```toml
# orbien-server.toml
[transport.tls]
force = true
certFile = "/path/to/server.crt"
keyFile = "/path/to/server.key"
```

客户端：

```toml
# orbien.toml
[transport.tls]
enable = true
trustedCaFile = "/path/to/ca.crt"
serverName = "orbien.example.com"
disableCustomTLSFirstByte = true
```

## 方式三：双向认证（mTLS）

双方均校验对方证书。服务端配置 `trustedCaFile` 时会自动 `force = true`。

服务端：

```toml
# orbien-server.toml
[transport.tls]
force = true
certFile = "/path/to/server.crt"
keyFile = "/path/to/server.key"
trustedCaFile = "/path/to/ca.crt"
```

客户端：

```toml
# orbien.toml
[transport.tls]
enable = true
certFile = "/path/to/client.crt"
keyFile = "/path/to/client.key"
trustedCaFile = "/path/to/ca.crt"
serverName = "orbien.example.com"
disableCustomTLSFirstByte = true
```

## 客户端参数

| 参数                                        | 必填 | 默认值    | 说明                                                    |
|-------------------------------------------|----|--------|-------------------------------------------------------|
| `transport.tls.enable`                    | 否  | `true` | 启用 TLS；QUIC 本身已加密，此项无效                                |
| `transport.tls.certFile`                  | 否  |        | 客户端证书（mTLS）                                           |
| `transport.tls.keyFile`                   | 否  |        | 客户端私钥（mTLS）                                           |
| `transport.tls.trustedCaFile`             | 否  |        | 校验服务端证书；空则跳过校验（仅加密）                                   |
| `transport.tls.serverName`                | 否  |        | TLS SNI；空则使用 `serverAddr`                             |
| `transport.tls.disableCustomTLSFirstByte` | 否  | `true` | 禁用自定义 TLS 首字节；若 HTTPS 虚拟主机与 `bindPort` 共用，需设为 `false` |

## 服务端参数

| 参数                            | 必填 | 默认值     | 说明                                 |
|-------------------------------|----|---------|------------------------------------|
| `transport.tls.force`         | 否  | `false` | 强制 TLS，拒绝非 TLS 控制连接                |
| `transport.tls.certFile`      | 否  |         | 服务端证书；空则临时自签                       |
| `transport.tls.keyFile`       | 否  |         | 服务端私钥；空则临时自签                       |
| `transport.tls.trustedCaFile` | 否  |         | 校验客户端证书（mTLS）；非空时自动 `force = true` |
