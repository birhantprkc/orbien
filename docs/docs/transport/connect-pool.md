---
sidebar_position: 2
sidebar_label: 连接池
title: 连接池
---

# 连接池

登录时预创建工作连接/流，降低新建连接延迟。客户端 `poolCount` 受服务端 `maxPoolCount` 限制。

## 示例

服务端：

```toml
# orbien-server.toml
[transport]
maxPoolCount = 5
```

客户端：

```toml
# orbien.toml
[transport]
poolCount = 3
```

实际生效数量为 `min(poolCount, maxPoolCount)`。

## 客户端参数

| 参数                    | 必填 | 默认值 | 说明              |
|-----------------------|----|-----|-----------------|
| `transport.poolCount` | 否  | `1` | 登录时预创建的工作连接/流数量 |

## 服务端参数

| 参数                       | 必填 | 默认值 | 说明                                      |
|--------------------------|----|-----|-----------------------------------------|
| `transport.maxPoolCount` | 否  | `5` | 限制客户端 `poolCount` 上限；配置为 `0` 时会自动变为 `5` |
