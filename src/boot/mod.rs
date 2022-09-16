pub mod db;

use std::sync::Arc;
use once_cell::sync::OnceCell;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Configure {
    pub server: Server,
    pub database: Database,
    pub log: Log,
    pub profile: String,
}

#[derive(Debug, Deserialize)]
pub struct Server {
    pub ip: String,
    pub port: u32,
    pub thread_per_worker: u32,
    pub workers: u32,
}

#[derive(Debug, Deserialize)]
pub struct Database {
    pub url: String,
    pub min: u32,
    pub max: u32,
    // pub host: Option<String>,
    // pub port: Option<String>,
    // pub user: Option<String>,
    // pub password: Option<String>,
    // pub dbname: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Log {
    pub file: String,
    pub level: String,
}

#[allow(dead_code)]
pub fn global() -> &'static Arc<Configure> {
    static CONFIG: OnceCell<Arc<Configure>> = OnceCell::new();
    CONFIG.get_or_init(|| {
        let s = std::fs::read_to_string(&"config.yaml").unwrap();
        Arc::new(serde_yaml::from_str(&s).unwrap())
    })
}

impl Server {
    #[allow(dead_code)]
    pub fn addr(&self) -> String {
        format!("{}:{}", self.ip, self.port)
    }
}