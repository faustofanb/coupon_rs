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
const LOG_CONFIG_PATH: &str = "log4rs.yaml";
const APP_CONFIG_PATH: &str = "admin/application";

impl AppConfig {
    pub fn from_env() -> Result<Self, config::ConfigError> {
        log4rs::init_file(LOG_CONFIG_PATH, Default::default())
            .expect("Load log configuration failed.");
        info!("Load log configuration from /log4rs.yaml.");
        let mut settings = config::Config::builder();

        settings =
            settings.add_source(config::File::with_name(APP_CONFIG_PATH).required(true));

        let run_mode = std::env::var("RUN_MODE").unwrap_or_else(|_| "dev".into());
        settings = settings.add_source(
            config::File::with_name(&format!("application-{}", run_mode)).required(false),
        );

        settings = settings.add_source(config::Environment::with_prefix("APP").separator("_"));

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
