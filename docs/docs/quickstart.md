---
sidebar_position: 3
title: 快速开始
---
# 安装

[下载](download.mdx)对应平台的二进制压缩包解压。若使用 Docker，见 [Docker 安装](./docker.mdx)。

## 服务端

```toml
# orbien-server.toml
bindAddr = "0.0.0.0"
bindPort = 9527
```

```shell
./orbien-server -c orbien-server.toml
```


## 客户端

如果觉得命令行CLI操作麻烦，可以使用[Orbien-Desktop](download.mdx)桌面端

```toml
# orbien.toml
serverAddr = "127.0.0.1"
serverPort = 9527

[[proxies]]
name = "mysql"
type = "tcp"
localIP = "127.0.0.1"
localPort = 3306
remotePort = 6050
```

```shell
./orbien -c orbien.toml
```

## Docker

服务端 / 客户端镜像的独立部署写法见 **[Docker 安装](./docker.mdx)**。

