use std::sync::RwLock;
use serde::{Serialize, Deserialize};
use hbb_common::config as hbb_config;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    pub web: WebConfig,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WebConfig {
    #[serde(default = "default_web_port")]
    pub port: u16,
}

fn default_web_port() -> u16 {
    58087
}

lazy_static::lazy_static! {
    static ref CONFIG: RwLock<Config> = RwLock::new(Config::default());
}

impl Config {
    fn config_path() -> std::path::PathBuf {
        hbb_config::Config::path("config.toml")
    }

    pub fn load() -> Self {
        let path = Self::config_path();
        if !path.exists() {
            let _ = std::fs::create_dir_all(path.parent().unwrap());
            return Config::default();
        }
        // 使用 hbb_common 的配置加载方式
        let content = std::fs::read_to_string(path).unwrap_or_default();
        serde_json::from_str(&content).unwrap_or_default()
    }

    pub fn validate_port(&self) -> Result<(), String> {
        if self.web.port < 1024 {
            Err("Port must be >= 1024".to_string())
        } else {
            Ok(())
        }
    }
}