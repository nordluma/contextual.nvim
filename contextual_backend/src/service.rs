use std::task::{Context, Poll};

use futures::future::BoxFuture;

pub trait Service<Request> {
    type Response;
    type Error;
    type Future: Future<Output = Result<Self::Response, Self::Error>> + Send;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<()> {
        Poll::Ready(())
    }

    fn call(&mut self, req: Request) -> Self::Future;
}

pub trait CloneableService<Req, Res, Err>:
    Service<Req, Response = Res, Error = Err, Future = BoxFuture<'static, Result<Res, Err>>>
    + Send
    + Sync
{
    fn clone_box(&self) -> Box<dyn CloneableService<Req, Res, Err>>;
}

impl<Req, Res, Err, T> CloneableService<Req, Res, Err> for T
where
    T: Service<Req, Response = Res, Error = Err, Future = BoxFuture<'static, Result<Res, Err>>>
        + Clone
        + Send
        + Sync
        + 'static,
{
    fn clone_box(&self) -> Box<dyn CloneableService<Req, Res, Err>> {
        Box::new(self.clone())
    }
}
