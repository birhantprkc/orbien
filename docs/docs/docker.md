---
sidebar_position: 4
sidebar_label: Docker 安装
title: Docker 安装
---

# Docker 安装

服务端跑在**公网机器**，客户端跑在**内网机器**

```shell
docker pull ghcr.io/orbien-org/orbien-server:latest  # 服务端
docker pull ghcr.io/orbien-org/orbien:latest         # 客户端
```

---

## 服务端（公网）

### 挂载配置

准备 `orbien-server.toml`：

```toml
bindAddr = "0.0.0.0"
bindPort = 9527

# 可选：HTTP / HTTPS 虚拟主机
# vhostHTTPPort = 80
# vhostHTTPSPort = 443

# 可选：客户端鉴权
# [auth]
# method = "token"
# token = "YOUR_TOKEN"

# 可选：Web 管理面板
[webServer]
addr = "0.0.0.0"
port = 8020
user = "admin"
password = "123456"
```

:::warning
`webServer.addr` 需为 `0.0.0.0`，否则宿主机无法通过端口映射访问管理面板
:::

### 方式一：配置文件启动
```shell
docker run -d --name orbien-server --restart unless-stopped \
  -p 9527:9527 \
  -p 8020:8020 \
  -v "$PWD/orbien-server.toml:/etc/orbien/orbien-server.toml:ro" \
  ghcr.io/orbien-org/orbien-server:latest
```

开启虚拟主机时再映射 `80` / `443`：

```shell
docker run -d --name orbien-server --restart unless-stopped \
  -p 9527:9527 -p 8020:8020 -p 80:80 -p 443:443 \
  -v "$PWD/orbien-server.toml:/etc/orbien/orbien-server.toml:ro" \
  ghcr.io/orbien-org/orbien-server:latest
```

管理面板：`http://<公网IP>:8020`，账号密码见配置中的 `user` / `password`

### 方式二：命令行启动

```shell
docker run -d --name orbien-server --restart unless-stopped \
  -p 9527:9527 -p 8020:8020 \
  ghcr.io/orbien-org/orbien-server:latest \
  --bind_port 9527 \
  --dashboard_addr 0.0.0.0 \
  --dashboard_port 8020 \
  --dashboard_user admin \
  --dashboard_pwd admin
```

### 方式三：Compose启动

```yaml
# docker-compose.yaml
services:
  orbien-server:
    image: ghcr.io/orbien-org/orbien-server:latest
    container_name: orbien-server
    restart: unless-stopped
    ports:
      - "9527:9527"
      - "8020:8020"
      # - "80:80"
      # - "443:443"
    volumes:
      - ./orbien-server.toml:/etc/orbien/orbien-server.toml:ro
```

```shell
docker compose up -d
```

---

## 客户端（内网）

把 `serverAddr` 写成公网服务端地址即可

### 挂载配置

准备 `orbien.toml`：

```toml
serverAddr = "YOUR_SERVER_IP"
serverPort = 9527

# 若服务端开启了 Token，需保持一致
# [auth]
# method = "token"
# token = "YOUR_TOKEN"

[[proxies]]
name = "mysql"
type = "tcp"
localIP = "127.0.0.1"
localPort = 3306
remotePort = 6050
```

### 方式一：配置文件启动

```shell
docker run -d --name orbien --restart unless-stopped \
  -v "$PWD/orbien.toml:/etc/orbien/orbien.toml:ro" \
  ghcr.io/orbien-org/orbien:latest
```

:::tip
容器内 `127.0.0.1` 是容器自己。若要穿透**宿主机**上的服务，把 `localIP` 改成宿主机 IP，或使用下方 host 网络
:::

**host 网络（Linux）**

与宿主机共用网络，配置里可直接写 `127.0.0.1` 访问本机服务：

```shell
docker run -d --name orbien --restart unless-stopped \
  --network host \
  -v "$PWD/orbien.toml:/etc/orbien/orbien.toml:ro" \
  ghcr.io/orbien-org/orbien:latest
```

### 方式二：Compose启动

```yaml
# docker-compose.yaml
services:
  orbien:
    image: ghcr.io/orbien-org/orbien:latest
    container_name: orbien
    restart: unless-stopped
    # Linux 穿透本机服务时可打开：
    # network_mode: host
    volumes:
      - ./orbien.toml:/etc/orbien/orbien.toml:ro
```

```shell
docker compose up -d
```

