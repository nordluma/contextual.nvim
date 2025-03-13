use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize)]
struct Request {
    jsonrpc: String,
    id: u64,
    method: String,
    params: Value,
}

#[derive(Debug, Serialize)]
struct Response {
    jsonrpc: String,
    id: u64,
    result: Option<Value>,
    error: Option<ResponseError>,
}

#[derive(Debug, Serialize)]
struct ResponseError {
    code: i32,
    message: String,
}

pub struct JsonRpcServer {
    handlers:
        Arc<RwLock<HashMap<String, Box<dyn Fn(Value) -> Result<Value, String> + Send + Sync>>>>,
}

impl JsonRpcServer {
    pub fn new() -> Self {
        JsonRpcServer {
            handlers: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn register_method<F>(&mut self, method: String, handler: F)
    where
        F: Fn(Value) -> Result<Value, String> + Send + Sync + 'static,
    {
        let mut handlers = self.handlers.write().unwrap();
        handlers.insert(method, Box::new(handler));
    }

    pub fn handle_request(&self, request_text: &str) -> String {
        let request: Request = match serde_json::from_str(request_text) {
            Ok(req) => req,
            Err(e) => return self.create_error_response(0, -32700, &format!("Parse error: {e}")),
        };

        let handlers = self.handlers.read().unwrap();
        match handlers.get(&request.method) {
            Some(handler) => match handler(request.params) {
                Ok(res) => {
                    let response = Response {
                        jsonrpc: "2.0".to_string(),
                        id: request.id,
                        result: Some(res),
                        error: None,
                    };

                    serde_json::to_string(&response).expect("response json is valid")
                }
                Err(e) => self.create_error_response(request.id, -32603, &e),
            },
            None => self.create_error_response(request.id, -32601, "Method not found"),
        }
    }

    pub fn clone(&self) -> Self {
        Self {
            handlers: Arc::clone(&self.handlers),
        }
    }

    fn create_error_response(&self, id: u64, code: i32, message: &str) -> String {
        let response = Response {
            jsonrpc: "2.0".to_string(),
            id,
            result: None,
            error: Some(ResponseError {
                code,
                message: message.to_string(),
            }),
        };

        serde_json::to_string(&response).expect("error response is valid json")
    }
}
