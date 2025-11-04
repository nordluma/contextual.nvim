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

impl JsonRpcResponse {
    pub fn ok(id: u64, response: Value) -> Self {
        Self {
            jsonrpc: "2.0".into(),
            id,
            result: Some(response),
            error: None,
        }
    }

    pub fn from_error(id: u64, error: ResponseError) -> Self {
        Self {
            jsonrpc: "2.0".into(),
            id,
            result: None,
            error: Some(error),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct ResponseError {
    pub code: i32,
    pub message: String,
}
