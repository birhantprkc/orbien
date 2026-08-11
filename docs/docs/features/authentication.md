---
sidebar_position: 1
sidebar_label: 身份认证
title: 身份认证
---

# 身份认证

客户端连接服务端时的 Token 鉴权。服务端未配置 `token`（或为空）时不校验。

两端 `token` 必须一致，否则登录失败（`authorization failed`）。

## 示例

服务端：

```toml
# orbien-server.toml
[auth]
token = "YOUR_TOKEN"
```

客户端：

```toml
# orbien.toml
[auth]
token = "YOUR_TOKEN"
```

## 参数

| 参数           | 必填 | 默认值 | 说明                     |
|--------------|----|-----|------------------------|
| `auth.token` | 否  |     | 共享密钥；服务端为空表示关闭鉴权；两端需一致 |

## 命令行

服务端也可通过参数设置（会覆盖配置文件中的 `token`）：

```shell
./orbien-server -c orbien-server.toml -t YOUR_TOKEN
```

| 参数               | 默认值 | 说明   |
|------------------|-----|------|
| `-t` / `--token` |     | 共享密钥 |
