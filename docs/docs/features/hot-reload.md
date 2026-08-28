---
sidebar_position: 7
sidebar_label: 热重载
title: 热重载
---

# 热重载

客户端运行期间修改配置并保存后，可让变更立即生效，**无需重启客户端**

- **热重载**：控制连接保持在线，只增删改隧道注册
- **重连**：断开控制连接后自动重连，期间全部隧道短暂中断

:::info[说明]
仅**隧道配置**变更走热重载，连接级字段（`server`、`user`、`auth`、`transport`、`udpPacketSize`
）变更会触发重连
:::

## 用法

先以前台方式启动客户端（会监听本地控制套接字）：

```shell
orbien -c conf/orbien.toml
```

另开终端，对同一配置文件执行 reload：

```shell
orbien reload -c conf/orbien.toml
```

:::tip
`orbien reload` 通过 Unix 套接字与运行中的 `orbien -c` 进程通信
:::

校验配置语法与字段（不连接服务端、不修改运行状态）：

```shell
orbien verify -c conf/orbien.toml
```

建议先 `verify` 再 `reload`：

```shell
orbien verify -c conf/orbien.toml && orbien reload -c conf/orbien.toml
```

