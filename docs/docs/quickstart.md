---
sidebar_position: 3
title: 快速开始
---

# 快速开始

[下载](download.mdx)对应平台的二进制压缩包解压

## 服务端

```toml
# orbien-server.toml
listen = "0.0.0.0:9527"
```

```shell
./orbien-server -c orbien-server.toml
```

## 客户端

如果觉得命令行 CLI 操作麻烦，可以使用 [Orbien Desktop](download.mdx) 桌面客户端。

```toml
# orbien.toml
server = "127.0.0.1:9527"

[[tunnels]]
name = "mysql"
protocol = "tcp"
service = "127.0.0.1:3306"
remotePort = 9000
```

```shell
./orbien -c orbien.toml
```
