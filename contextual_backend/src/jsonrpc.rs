use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Deserialize)]
pub struct JsonRpcRequest {
    #[allow(unused)]
    pub jsonrpc: String,
    pub id: u64,
    pub method: String,
    pub params: Value,
}

#[derive(Debug, Clone, Serialize)]
pub struct JsonRpcResponse {
    pub jsonrpc: String,
    pub id: u64,
    pub result: Option<Value>,
    pub error: Option<ResponseError>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ResponseError {
    pub code: i32,
    pub message: String,
}
