use config::{Config, Environment, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Configuration {
    pub application: AppSettings,
    pub database: DatabaseSettings,
}

#[derive(Debug, Deserialize)]
pub struct AppSettings {
    pub port: String,
    pub host: String,
}

#[derive(Debug, Deserialize)]
pub struct DatabaseSettings {
    pub port: String,
}

pub fn get_config() -> Configuration {
    let env_type = std::env::var("ENV_TYPE").unwrap_or_else(|_| "dev".into());
    // 初始化配置
    let settings = Config::builder()
        .add_source(File::with_name("config/base.toml"))
        // 部署时为prod，本地开发时为dev
        .add_source(File::with_name(&format!("config/{}.toml", env_type)))
        // 从环境变量中读取 格式为 APP_APPLICATION_PORT=8001 前缀为APP区分cargo的环境变量 分隔符为_ 加载树形结构到Configuration
        .add_source(Environment::with_prefix("APP").separator("_"))
        .build()
        .unwrap();

    settings.try_deserialize::<Configuration>().unwrap()
}
