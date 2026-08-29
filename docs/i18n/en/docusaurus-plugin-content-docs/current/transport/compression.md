---
sidebar_position: 4
sidebar_label: Data Compression
title: Data Compression
---

# Data Compression

Data compression saves bandwidth. When enabled, data forwarded on data connections is compressed in chunks; the
compressed form is sent only when it is smaller, otherwise the original bytes are forwarded unchanged.

- `compression = "none"`: off (default)
- `compression = "lz4"`: enable LZ4

## Example: Enable LZ4

```toml
# orbien.toml
server = "YOUR_SERVER_IP:9527"

[[tunnels]]
name = "ssh"
protocol = "tcp"
service = "127.0.0.1:22"
remotePort = 9000
transport.compression = "lz4"
```

:::tip

- Already encrypted or compressed payloads usually gain little from compression and cost extra CPU; leave compression
  off
- Text, logs, and other compressible traffic benefit more on bandwidth-constrained links
  :::

## Parameters

| Parameter               | Required | Default | Description                           |
|-------------------------|----------|---------|---------------------------------------|
| `transport.compression` | No       | `none`  | Compression algorithm: `none` / `lz4` |
