use std::ffi::CStr;

use jsonrpsee::types::Params;
use jsonrpsee::IntoResponse;
use serde::Serialize;

use crate::{mutex_lock, RIME_SESSION};

pub fn version(_p: Params) -> impl IntoResponse {
    unsafe {
        let api = librime_sys::rime_get_api();
        CStr::from_ptr((*api).get_version.unwrap()())
            .to_str()
            .expect("Invalid UTF-8")
    }
}

#[derive(Serialize)]
pub struct Response<'a> {
    status: rime_api::Status,
    commit: Option<String>,
    composition: rime_api::Composition<'a>,
    menu: rime_api::Menu<'a>,
    select_labels: Option<Vec<&'a str>>,
}

pub fn simulate_key_sequence(p: Params) -> impl IntoResponse {
    let result: anyhow::Result<_> = try {
        let keys = p.parse::<String>()?;
        let guard = mutex_lock!(RIME_SESSION);
        let session = guard.as_ref().unwrap();
        session.simulate_key_sequence(&keys)?;

        let context = session.context().unwrap();
        let response = Response {
            status: session.status()?,
            commit: session.commit().map(|x| String::from(x.text())),
            composition: context.composition(),
            menu: context.menu(),
            select_labels: context.select_labels(),
        };
        serde_json::to_value(&response).unwrap()
    };
    match result {
        Err(e) => serde_json::Value::String(format!("Error: {}", e)).into_response(),
        Ok(r) => r.into_response(),
    }
}
