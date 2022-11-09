pub mod db;
pub mod string_builder;
pub mod utils;

use std::sync::Arc;
use once_cell::sync::OnceCell;
use serde::Deserialize;

pub use string_builder::*;
pub use utils::*;

pub static CONFIG: OnceCell<Arc<Configure>> = OnceCell::new();

#[derive(Debug, Deserialize)]
pub struct Configure {
    pub server: Server,
    pub database: Database,
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
    pub max: Option<usize>,
    pub host: Option<String>,
    pub port: Option<u16>,
    pub user: Option<String>,
    pub password: Option<String>,
    pub dbname: Option<String>,
}

#[allow(dead_code)]
pub fn global_cfg() -> &'static Arc<Configure> {
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