use jsonrpsee::types::Params;
use jsonrpsee::IntoResponse;
use std::ffi::CStr;

pub fn version(_p: Params) -> impl IntoResponse {
    unsafe {
        let api = librime_sys::rime_get_api();
        CStr::from_ptr((*api).get_version.unwrap()())
            .to_str()
            .expect("Invalid UTF-8")
    }
}
