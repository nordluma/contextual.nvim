use futures::future::BoxFuture;

use crate::{
    jsonrpc::{JsonRpcRequest, ResponseError},
    service::Service,
};

#[derive(Clone)]
pub struct EchoService;

impl Service<JsonRpcRequest> for EchoService {
    type Response = serde_json::Value;
    type Error = ResponseError;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn call(&mut self, req: JsonRpcRequest) -> Self::Future {
        Box::pin(async move { Ok(req.params) })
    }
}
