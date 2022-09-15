
use std::sync::{Arc, RwLock};
use chrono::{Local, NaiveDateTime};

pub mod boot;
pub mod model;
pub mod dao;
pub mod error;
pub mod manager;

pub type ArcRw<T> = Arc<RwLock<T>>;

pub fn get_now() -> NaiveDateTime {
    Local::now().naive_local()
}

pub fn gen_id() -> String {
    uuid::Uuid::new_v4().to_string()
}