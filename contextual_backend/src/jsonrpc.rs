use std::{
    collections::HashMap,
    fmt::Display,
    sync::{Arc, RwLock},
};

use futures::future::BoxFuture;
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub type HandlerFut = BoxFuture<'static, Result<Value, anyhow::Error>>;
type AsyncHandler = Box<dyn Fn(Value) -> HandlerFut + Send + Sync>;
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

    pub fn register_method<M, F>(&mut self, method: M, handler: F)
    where
        M: Into<String>,
        F: Fn(Value) -> HandlerFut + Send + Sync + 'static,
    {
        let mut handlers = self.handlers.write().unwrap();
        handlers.insert(method.into(), Box::new(handler));
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

                        serde_json::to_string(&response).expect("response json is valid")
                    }
                    Err(e) => self.create_error_response(request.id, -32603, e),
                }
            }
            None => self.create_error_response(request.id, -32601, "Method not found"),
        }
    }

    fn create_error_response<E: Display>(&self, id: u64, code: i32, message: E) -> String {
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
