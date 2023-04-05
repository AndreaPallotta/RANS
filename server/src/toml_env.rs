use std::{error::Error, net::{Ipv4Addr, SocketAddr, IpAddr}};
use log::LevelFilter;
use serde::{Deserialize, Deserializer};

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(rename = "Database")]
    pub db: DatabaseConfig,
    #[serde(rename = "Logs")]
    pub log: LogConfig,
    #[serde(rename = "Server")]
    pub server: ServerConfig,
}

impl Config {
    pub fn parse(path: &str) -> Result<Self, Box<dyn Error>> {
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

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    #[serde(deserialize_with = "deserialize_host")]
    pub host: IpAddr,
    pub port: u16,
}

impl ServerConfig {
    pub fn socket_addr(&self) -> SocketAddr {
        SocketAddr::new(self.host, self.port)
    }
}

fn deserialize_host<'de, D>(deserializer: D) -> Result<IpAddr, D::Error>
where D: Deserializer<'de> {
    let host = String::deserialize(deserializer)?;
    match host.parse() {
        Ok(ip) => Ok(ip),
        Err(_) => Ok(Ipv4Addr::new(0, 0, 0, 0).into()),
    }
}
