use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use futures::future::BoxFuture;
use serde::{Deserialize, Serialize};
use serde_json::Value;

type AsyncHandler = Box<dyn Fn(Value) -> BoxFuture<'static, Result<Value, String>> + Send + Sync>;

type HandlerRegister = Arc<RwLock<HashMap<String, AsyncHandler>>>;

#[derive(Debug, Deserialize)]
pub struct Request {
    #[allow(unused)]
    pub jsonrpc: String,
    pub id: u64,
    pub method: String,
    pub params: Value,
}

#[derive(Debug, Serialize)]
pub struct Response {
    pub jsonrpc: String,
    pub id: u64,
    pub result: Option<Value>,
    pub error: Option<ResponseError>,
}

#[derive(Debug, Serialize)]
pub struct ResponseError {
    pub code: i32,
    pub message: String,
}

pub struct JsonRpcServer {
    handlers: HandlerRegister,
}

impl Default for JsonRpcServer {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for JsonRpcServer {
    fn clone(&self) -> Self {
        Self {
            handlers: Arc::clone(&self.handlers),
        }
    }
}

impl JsonRpcServer {
    pub fn new() -> Self {
        JsonRpcServer {
            handlers: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn register_method<F>(&mut self, method: String, handler: F)
    where
        F: Fn(Value) -> BoxFuture<'static, Result<Value, String>> + Send + Sync + 'static,
    {
        let mut handlers = self.handlers.write().unwrap();
        handlers.insert(method, Box::new(handler));
    }

    pub async fn handle_request(&self, request_text: String) -> String {
        let request: Request = match serde_json::from_str(&request_text) {
            Ok(req) => req,
            Err(e) => return self.create_error_response(0, -32700, &format!("Parse error: {e}")),
        };

        let future = {
            let handlers = self.handlers.read().unwrap();
            match handlers.get(&request.method) {
                Some(handler) => Some(handler(request.params)),
                None => None,
            }
        };

        match future {
            Some(future) => {
                let response_res = future.await;
                match response_res {
                    Ok(res) => {
                        let response = Response {
                            jsonrpc: "2.0".to_string(),
                            id: request.id,
                            result: Some(res),
                            error: None,
                        };

                        return serde_json::to_string(&response).expect("response json is valid");
                    }
                    Err(e) => return self.create_error_response(request.id, -32603, &e),
                }
            }
            None => self.create_error_response(request.id, -32601, "Method not found"),
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
