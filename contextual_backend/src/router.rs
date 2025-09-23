use std::{collections::HashMap, sync::Arc};

use futures::future::BoxFuture;
use serde_json::Value;

use crate::{
    jsonrpc::{JsonRpcRequest, JsonRpcResponse, ResponseError},
    service::{CloneableService, Service},
};

pub struct RouterFactory {
    routes: Arc<HashMap<String, Arc<dyn CloneableService<JsonRpcRequest, Value, ResponseError>>>>,
}

impl RouterFactory {
    pub fn new() -> Self {
        Self {
            routes: Arc::new(HashMap::new()),
        }
    }

    pub fn with_route<S>(self, method: &str, svc: S) -> Self
    where
        S: CloneableService<JsonRpcRequest, Value, ResponseError> + 'static,
    {
        let mut routes = (*self.routes).clone();
        routes.insert(method.into(), Arc::new(svc));
        Self {
            routes: Arc::new(routes),
        }
    }

    pub fn service(&self) -> RouterService {
        RouterService {
            routes: self.routes.clone(),
        }
    }
}

pub struct RouterService {
    routes: Arc<HashMap<String, Arc<dyn CloneableService<JsonRpcRequest, Value, ResponseError>>>>,
}

impl Service<JsonRpcRequest> for RouterService {
    type Response = JsonRpcResponse;
    type Error = ();
    type Future = BoxFuture<'static, Result<JsonRpcResponse, Self::Error>>;

    fn call(&mut self, req: JsonRpcRequest) -> Self::Future {
        let id = req.id;

        if let Some(svc) = self.routes.get(&req.method) {
            let mut svc = svc.clone_box();
            Box::pin(async move {
                let response = match svc.call(req).await {
                    Ok(res) => JsonRpcResponse::ok(id, res),
                    Err(e) => JsonRpcResponse::from_error(id, e),
                };

                Ok(response)
            })
        } else {
            Box::pin(async move {
                Ok(JsonRpcResponse {
                    jsonrpc: "2.0".into(),
                    id,
                    result: None,
                    error: Some(ResponseError {
                        code: -32601,
                        message: format!("Method note found: {}", req.method),
                    }),
                })
            })
        }
    }
}
