# 技术选型方案（Rust 实现）

| 技术点         | Rust 推荐库/方案                                                                                   |
|----------------|---------------------------------------------------------------------------------------------------|
| 数据库         | PostgreSQL：[sqlx](https://github.com/launchbadge/sqlx)、[sea-orm](https://www.sea-ql.org/SeaORM/) |
| ORM            | [sea-orm](https://www.sea-ql.org/SeaORM/)、[diesel](https://diesel.rs/)                            |
| 缓存           | [redis-rs](https://github.com/redis-rs/redis-rs)、[deadpool-redis](https://github.com/bikeshedder/deadpool) |
| 分布式锁       | [redlock-rs](https://github.com/mitsuhiko/redlock-rs)                                              |
| WEB 框架       | [actix-web](https://actix.rs/)                                                                     |
| 配置           | [config](https://github.com/mehcode/config-rs)                                                     |
| 日志           | [log4rs](https://github.com/estk/log4rs)、[tracing](https://tokio.rs/tokio/topics/tracing)         |
| 分库分表       | 暂不实现                                                                                           |
| RPC            | 暂不实现                                                                                           |
| 消息队列       | [rocketmq-client-rust](https://github.com/apache/rocketmq-clients/tree/develop/rust)、[rust-rdkafka](https://github.com/fede1024/rust-rdkafka)（Kafka） |
| JSON           | [serde](https://serde.rs/)                                                                         |
| JWT            | [jsonwebtoken](https://github.com/Keats/jsonwebtoken)                                              |
| Bean 复制      | [copy_struct](https://github.com/estk/copy_struct)、[derive_more](https://github.com/JelteF/derive_more) 的 `From` 派生，或用 serde 的序列化/反序列化 |
| 依赖注入       | [shaku](https://github.com/ivanceras/shaku)、[injekt](https://github.com/udoprog/injekt)（可选）    |
| 单元测试       | Rust 内置测试框架（`cargo test`），可用 [mockall](https://github.com/asomers/mockall) 做 mock        |
| 任务调度       | [cron](https://github.com/zslayton/cron)、[tokio-cron-scheduler](https://github.com/mvniekerk/tokio-cron-scheduler) |
| 热重载         | [cargo-watch](https://github.com/watchexec/cargo-watch)                                            |

## 说明

- ORM：Rust 没有 MyBatis 这种 XML 映射 ORM，推荐 sea-orm 或 diesel，类型安全且支持代码生成。
- Bean 复制：Rust 通常用结构体的 `From/Into` trait、serde 序列化/反序列化或 copy_struct 实现。
- 配置库 config 支持 yaml、json、toml、env 等多种格式，灵活易用。
- 依赖注入不是 Rust 的主流，但 shaku/injekt 可选用。
- 任务调度、热重载等可根据实际需求选用。