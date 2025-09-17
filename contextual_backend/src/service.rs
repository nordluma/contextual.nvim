use std::task::{Context, Poll};

use futures::future::BoxFuture;

pub trait Service<Request> {
    type Response;
    type Future: Future<Output = Self::Response> + Send;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<()> {
        Poll::Ready(())
    }

    fn call(&mut self, req: Request) -> Self::Future;
}

pub trait CloneableService<Req, Res>:
    Service<Req, Response = Res, Future = BoxFuture<'static, Res>> + Send + Sync
{
    fn clone_box(&self) -> Box<dyn CloneableService<Req, Res>>;
}

impl<Req, Res, T> CloneableService<Req, Res> for T
where
    T: Service<Req, Response = Res, Future = BoxFuture<'static, Res>>
        + Clone
        + Send
        + Sync
        + 'static,
{
    fn clone_box(&self) -> Box<dyn CloneableService<Req, Res>> {
        Box::new(self.clone())
    }
}
