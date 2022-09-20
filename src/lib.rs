use std::{cell::RefCell, rc::Rc};
use chrono::{Local, NaiveDateTime};

pub mod common;
pub mod model;
pub mod dao;
pub mod error;
pub mod service;

pub type RcRefCell<T> = Rc<RefCell<T>>;

pub fn get_now() -> NaiveDateTime {
    Local::now().naive_local()
}

pub fn gen_id() -> String {
    uuid::Uuid::new_v4().to_string()
}