#![allow(dead_code)]
use std::fmt::Display;

use chrono::{FixedOffset, Local, DateTime, NaiveDateTime};
use crypto::{md5::Md5, digest::Digest};


pub fn gen_random_str(len: usize) -> String {
    use rand::Rng;
    const CHARSET: &[u8] = b"0123456789";
    let mut rng = rand::thread_rng();

    let password: String = (0..len)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();

    password
}

pub fn md5<S:Into<String>>(input: S) -> String {
    let mut md5 = Md5::new();
    md5.input_str(&input.into());
    md5.result_str()
}


pub fn set_working_dir() 
{
    let exec_path = std::env::current_exe().expect("Can't get the execution path");

    let work_dir = exec_path
        .parent()
        .expect("Can't get the working directory")
        .to_string_lossy()
        .into_owned();

    std::env::set_current_dir(work_dir).unwrap();
}

pub struct LocalTimeStamp {
    pub nsecs: i64,
    pub tz: FixedOffset,
}

impl LocalTimeStamp {
    pub fn new(nsecs: i64) -> Self {
        Self {
            nsecs,
            tz: Local::now().offset().clone()
        }
    }

    pub fn now() -> Self {
        Self::new(Local::now().timestamp_millis())
    }

    pub fn timestamp_millis(&self) -> i64 {
        self.nsecs
    }

    pub fn timestamp(&self) -> i64 {
        self.nsecs / 1000
    }

    pub fn local_dt(&self) -> DateTime<Local> {
        let sec: i64 = self.nsecs / 1000;
        let nsec: u32 = (self.nsecs % 1000 * 1000000) as u32;
        let ndt = NaiveDateTime::from_timestamp(sec, nsec);
        let dt: DateTime<Local> = DateTime::from_utc(ndt, self.tz);
        dt
    }
}

impl Display for LocalTimeStamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.local_dt().format("%Y-%m-%d %H:%M:%S").to_string())
    }
}