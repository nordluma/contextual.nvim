use std::{collections::HashMap, sync::Arc};

use futures::future::BoxFuture;
use serde_json::Value;

use crate::{
    jsonrpc::{JsonRpcRequest, JsonRpcResponse, ResponseError},
    service::{CloneableService, Service},
};

pub struct RouterFactory {
    routes: Arc<HashMap<String, Arc<dyn CloneableService<JsonRpcRequest, Value>>>>,
}

impl RouterFactory {
    pub fn new() -> Self {
        Self {
            routes: Arc::new(HashMap::new()),
        }
    }

    pub fn with_route<S>(self, method: &str, svc: S) -> Self
    where
        S: CloneableService<JsonRpcRequest, Value> + 'static,
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
    routes: Arc<HashMap<String, Arc<dyn CloneableService<JsonRpcRequest, Value>>>>,
}

impl Service<JsonRpcRequest> for RouterService {
    type Response = JsonRpcResponse;
    type Future = BoxFuture<'static, JsonRpcResponse>;

    fn call(&mut self, req: JsonRpcRequest) -> Self::Future {
        let id = req.id;

        if let Some(svc) = self.routes.get(&req.method) {
            let mut svc = svc.clone_box();
            Box::pin(async move {
                let res = svc.call(req).await;
                JsonRpcResponse {
                    jsonrpc: "2.0".into(),
                    id,
                    result: Some(res),
                    error: None,
                }
            })
        } else {
            Box::pin(async move {
                JsonRpcResponse {
                    jsonrpc: "2.0".into(),
                    id,
                    result: None,
                    error: Some(ResponseError {
                        code: -32601,
                        message: format!("Method note found: {}", req.method),
                    }),
                }
            })
        }
    }
}
