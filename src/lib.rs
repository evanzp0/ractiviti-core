use std::{cell::RefCell, rc::Rc};
use common::LocalTimeStamp;

pub mod common;
pub mod model;
pub mod dao;
pub mod error;
pub mod service;
pub mod dto;

pub type RcRefCell<T> = Rc<RefCell<T>>;

pub fn get_now() -> i64 {
    LocalTimeStamp::now().timestamp_millis()
}

pub fn gen_id() -> String {
    uuid::Uuid::new_v4().to_string()
}