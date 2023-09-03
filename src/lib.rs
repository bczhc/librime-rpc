use std::sync::Mutex;
use once_cell::sync::Lazy;
use rime_api::Session;

pub mod cli;
pub mod methods;

#[macro_export]
macro_rules! mutex_lock {
    ($e:expr) => {
        $e.lock().unwrap()
    };
}

pub static RIME_SESSION: Lazy<Mutex<Option<Session>>> = Lazy::new(|| Mutex::new(None));
