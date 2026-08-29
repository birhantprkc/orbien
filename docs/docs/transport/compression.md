---
sidebar_position: 4
sidebar_label: 数据压缩
title: 数据压缩
---

# 数据压缩

开启数据压缩可以节省带宽，开启后，数据连接上转发的数据按块压缩，仅在体积更小时发送压缩结果，否则原样转发。

- `compression = "none"`：关闭（默认）
- `compression = "lz4"`：启用 LZ4

## 示例：启用 LZ4

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

- 已加密或已压缩内容通常压缩不会带来收益，还会增加 CPU，建议不要开启压缩
- 文本、日志等可压流量，在带宽受限链路上开启压缩更有收益
  :::

## 参数

| 参数                      | 必填 | 默认值    | 说明                  |
|-------------------------|----|--------|---------------------|
| `transport.compression` | 否  | `none` | 压缩算法：`none` / `lz4` |
