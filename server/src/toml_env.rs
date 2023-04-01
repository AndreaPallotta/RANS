use std::error::Error;
use log::LevelFilter;
use serde::{Deserialize, Deserializer};

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(rename = "Database")]
    pub db: DatabaseConfig,
    #[serde(rename = "Logs")]
    pub log: LogConfig,
}

impl Config {
    pub fn from(path: &str) -> Result<Self, Box<dyn Error>> {
        let contents = std::fs::read_to_string(path)?;
        let config: Self = toml::from_str(&contents)?;

        Ok(config)
    }
}

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub name: String,
    pub username: String,
    pub password: String,
}

impl DatabaseConfig {
    pub fn get_url(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

#[derive(Debug, Deserialize)]
pub struct LogConfig {
    pub path: String,
    #[serde(deserialize_with = "deserialize_log_level")]
    pub level: LevelFilter,
}

fn deserialize_log_level<'de, D>(deserializer: D) -> Result<LevelFilter, D::Error>
where D: Deserializer<'de> {
    let log_string = String::deserialize(deserializer)?;
    let level = match log_string.to_lowercase().as_str() {
        "off" => LevelFilter::Off,
        "error" => LevelFilter::Error,
        "warn" => LevelFilter::Warn,
        "info" => LevelFilter::Info,
        "debug" => LevelFilter::Debug,
        "trace" => LevelFilter::Trace,
        _ => LevelFilter::Info
    };

    Ok(level)
}
