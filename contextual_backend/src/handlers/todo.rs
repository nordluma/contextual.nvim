use futures::future::BoxFuture;

use crate::{
    database::TodoStorage,
    jsonrpc::{JsonRpcRequest, ResponseError},
    service::Service,
    types::todo::NewTodoItem,
};

#[derive(Debug, Clone)]
pub struct NewTodoService<S> {
    storage: S,
}

impl<S> NewTodoService<S> {
    pub fn new(storage: S) -> Self {
        Self { storage }
    }
}

impl<S> Service<JsonRpcRequest> for NewTodoService<S>
where
    S: TodoStorage + Clone + Send + 'static,
{
    type Response = serde_json::Value;
    type Error = ResponseError;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn call(&mut self, req: JsonRpcRequest) -> Self::Future {
        let storage = self.storage.clone();

        Box::pin(async move {
            let params = req.params.clone();
            let new_todo = match NewTodoItem::try_from(params) {
                Ok(t) => t,
                Err(e) => {
                    return Err(ResponseError {
                        code: -32000, // TODO: use right error code
                        message: e.to_string(),
                    });
                }
            };

            match storage.save_todo(new_todo).await {
                Ok(id) => Ok(serde_json::Value::String(id.to_string())),
                Err(e) => Err(ResponseError {
                    code: -32000,
                    message: e.to_string(),
                }),
            }
        })
    }
}
