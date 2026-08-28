---
sidebar_position: 7
sidebar_label: Hot Reload
title: Hot Reload
---

# Hot Reload

While the client is running, saving the config applies changes immediately, **without restarting the client**.

- **Hot reload**: Keeps the control connection up; adds, removes, or updates tunnel registrations only.
- **Reconnect**: Drops the control connection and reconnects automatically; all tunnels are briefly interrupted.

:::info[Note]
Only **tunnel** changes use hot reload. Changes to connection-level fields (`server`, `user`, `auth`, `transport`, `udpPacketSize`) trigger a reconnect.
:::

## Usage

Start the client in the foreground (this opens the local control socket):

```shell
orbien -c conf/orbien.toml
```

In another terminal, reload the same config file:

```shell
orbien reload -c conf/orbien.toml
```

:::tip
`orbien reload` talks to the running `orbien -c` process over a Unix socket.
:::

Validate syntax and fields without connecting to the server or changing the running state:

```shell
orbien verify -c conf/orbien.toml
```

Run `verify` before `reload`:

```shell
orbien verify -c conf/orbien.toml && orbien reload -c conf/orbien.toml
```
