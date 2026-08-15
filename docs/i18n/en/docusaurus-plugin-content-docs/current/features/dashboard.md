---
sidebar_position: 2
sidebar_label: Dashboard
title: Dashboard
---

# Dashboard

Server web dashboard. Enable it by configuring `[dashboard]` in `orbien-server.toml` with `port > 0`.

## Example

```toml
# orbien-server.toml
listen = "0.0.0.0:9527"

[dashboard]
addr = "0.0.0.0"
port = 8020
user = "admin"
password = "123456"
```

Open `http://SERVER_IP:8020` in a browser and log in with Basic Auth (`user` / `password`).

`addr` defaults to `127.0.0.1` (localhost only). Set it to `0.0.0.0` for remote access. `staticDir` may be omitted; if omitted, the built-in frontend is used.

## Parameters

| Parameter               | Required | Default       | Description                                  |
|-------------------------|----------|---------------|----------------------------------------------|
| `dashboard.addr`        | No       | `127.0.0.1`   | Listen address; use `0.0.0.0` for remote access |
| `dashboard.port`        | Yes      | `0`           | Listen port; `0` disables the dashboard      |
| `dashboard.user`        | No       |               | Basic Auth username                          |
| `dashboard.password`    | No       |               | Basic Auth password                          |
| `dashboard.staticDir`   | No       |               | Static assets directory; empty uses the built-in frontend |

## Command line

You can also enable it with flags when there is no config file:

```shell
./orbien-server --dashboard_port 8020 --dashboard_user admin --dashboard_pwd 123456
```

| Flag               | Default     | Description                    |
|--------------------|-------------|--------------------------------|
| `--dashboard_addr` | `0.0.0.0`   | Listen address                 |
| `--dashboard_port` | `0`         | Listen port; `0` disables it   |
| `--dashboard_user` | `admin`     | Username                       |
| `--dashboard_pwd`  | `admin`     | Password                       |
