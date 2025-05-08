# 技术选型方案（Rust 实现）

| 技术点         | Rust 推荐库/方案                                                                                   |
|----------------|---------------------------------------------------------------------------------------------------|
| 数据库         | PostgreSQL：[sqlx](https://github.com/launchbadge/sqlx) |
| ORM            | [sea-orm](https://www.sea-ql.org/SeaORM/)                        |
| 缓存           | [redis-rs](https://github.com/redis-rs/redis-rs) |
| 分布式锁       | [redlock-rs](https://github.com/mitsuhiko/redlock-rs)                                              |
| WEB 框架       | [actix-web](https://actix.rs/)                                                                     |
| 配置           | [config](https://github.com/mehcode/config-rs)                                                     |
| 日志           | [log4rs](https://github.com/estk/log4rs)         |
| 分库分表       | 暂不实现                                                                                           |
| RPC            | 暂不实现                                                                                           |
| 消息队列       | [rocketmq-client-rust](https://github.com/apache/rocketmq-clients/tree/develop/rust) |
| JSON           | [serde](https://serde.rs/)                                                                         |
| JWT            | [jsonwebtoken](https://github.com/Keats/jsonwebtoken)                                              |
| Bean 复制      | [copy_struct](https://github.com/estk/copy_struct)|
| 单元测试       | Rust 内置测试框架（`cargo test`），可用 [mockall](https://github.com/asomers/mockall) 做 mock        |
| 任务调度       |[tokio-cron-scheduler](https://github.com/mvniekerk/tokio-cron-scheduler) |
| 热重载         | [cargo-watch](https://github.com/watchexec/cargo-watch)                                            |

