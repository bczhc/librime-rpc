#![feature(try_blocks)]

use jsonrpsee::core::Serialize;
use once_cell::sync::Lazy;
use rime_api::Session;
use std::sync::Mutex;

pub mod cli;
pub mod methods;

#[macro_export]
macro_rules! mutex_lock {
    ($e:expr) => {
        $e.lock().unwrap()
    };
}

pub static RIME_SESSION: Lazy<Mutex<Option<Session>>> = Lazy::new(|| Mutex::new(None));

#[derive(Serialize)]
pub struct RimeStatus {}
