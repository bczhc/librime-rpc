use jsonrpsee::IntoResponse;
use jsonrpsee::types::Params;

pub fn version(_p: Params) -> impl IntoResponse {
    "TODO"
}