---
sidebar_position: 1
sidebar_label: Authentication
title: Authentication
---

# Authentication

Token authentication when the client connects to the server. If the server has no `token` (or it is empty), authentication is skipped.

The `token` on both sides must match, or login fails (`authorization failed`).

## Example

Server:

```toml
# orbien-server.toml
[auth]
token = "YOUR_TOKEN"
```

Client:

```toml
# orbien.toml
[auth]
token = "YOUR_TOKEN"
```

## Parameters

| Parameter    | Required | Default | Description                                                          |
|--------------|----------|---------|----------------------------------------------------------------------|
| `auth.token` | No       |         | Shared secret; empty on the server disables auth; both sides must match |

## Command line

The server can also set this via a flag (it overrides `token` in the config file):

```shell
./orbien-server -c orbien-server.toml -t YOUR_TOKEN
```

| Flag             | Default | Description   |
|------------------|---------|---------------|
| `-t` / `--token` |         | Shared secret |
