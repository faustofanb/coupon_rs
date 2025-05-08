use log::info;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub database: DatabaseConfig,
    pub server: ServerConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    #[serde(default = "default_server_port")]
    pub port: u16,
}

#[derive(Debug, Deserialize, Clone)] // Clone 可以方便传递
pub struct DatabaseConfig {
    pub url: String,
    #[serde(default = "default_max_connections")] // 为可选字段提供默认值
    pub max_connections: u32,
    #[serde(default = "default_min_connections")]
    pub min_connections: u32,
    #[serde(default = "default_connect_timeout")]
    pub connect_timeout_seconds: u64,
    #[serde(default = "default_idle_timeout")]
    pub idle_timeout_seconds: u64,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, config::ConfigError> {
        log4rs::init_file("config/log4rs.yaml", Default::default())
            .expect("Load log configuration failed.");
        info!("Load log configuration from config/log4rs.yaml.");
        let mut settings = config::Config::builder();

        // 1. 从默认配置文件加载 (config/default.yaml)
        //    路径是相对于可执行文件的。如果可执行文件在 target/debug/
        //    而配置文件在项目根目录的 config/ 下，你需要调整路径或确保工作目录正确。
        //    一个常见的做法是将配置文件放在与可执行文件相同的目录结构中，
        //    或者在运行时指定配置目录。
        //    为了简单起见，这里假设配置文件在 `config/` 目录下，
        //    并且程序从项目根目录运行（如使用 `cargo run`）。
        settings =
            settings.add_source(config::File::with_name("config/application").required(true));

        // 2. 从特定环境配置文件加载 (例如 config/development.yaml)
        //    这会覆盖 default.yaml 中的值。
        //    环境可以通过 RUN_MODE 环境变量设置 (config crate 默认行为)
        let run_mode = std::env::var("RUN_MODE").unwrap_or_else(|_| "dev".into());
        settings = settings.add_source(
            config::File::with_name(&format!("config/application-{}", run_mode)).required(false),
        );

        // 3. 从环境变量加载 (例如 APP_DATABASE_URL)
        //    环境变量会覆盖文件中的值。
        //    前缀 "APP" 会被移除，下划线会转换为结构体字段的分隔符 (例如 APP_DATABASE__URL -> database.url)
        settings = settings.add_source(config::Environment::with_prefix("APP").separator("_"));

        // 构建配置
        settings.build()?.try_deserialize()
    }
}

// 默认值函数，如果配置文件中没有提供这些值
fn default_server_port() -> u16 {
    8080
}
fn default_max_connections() -> u32 {
    10
}
fn default_min_connections() -> u32 {
    1
} // sqlx 默认 min_connections 是 0，但通常设为1或更高
fn default_connect_timeout() -> u64 {
    5
} // 5 seconds
fn default_idle_timeout() -> u64 {
    600
} // 10 minutes, sqlx 默认
