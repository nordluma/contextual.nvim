use std::marker::PhantomData;

use tokio::io::{AsyncRead, AsyncWrite};

use crate::{
    jsonrpc::{JsonRpcRequest, JsonRpcResponse, ResponseError},
    router::{RouterFactory, RouterService},
    service::Service,
    transport::codec::Framer,
};

pub mod codec;
pub mod stdio;
pub mod tcp;
pub mod unix_socket;

/// Generic trait for any async read/write capable stream
pub trait AsyncStream: AsyncRead + AsyncWrite + Unpin + Send + 'static {}

// Implement for any type that satisfies the requirements
impl<T> AsyncStream for T where T: AsyncRead + AsyncWrite + Unpin + Send + 'static {}

pub trait Transport {
    fn start(self, server: RouterFactory)
    -> impl Future<Output = Result<(), anyhow::Error>> + Send;
}

pub struct Server<T, F>
where
    T: Transport,
{
    transport: T,
    _marker: PhantomData<F>,
}

impl<T, F> Server<T, F>
where
    T: Transport,
{
    pub fn new(transport: T) -> Self {
        Self {
            transport,
            _marker: PhantomData,
        }
    }

    pub fn start(
        self,
        router_factory: RouterFactory,
    ) -> impl Future<Output = Result<(), anyhow::Error>> + Send {
        self.transport.start(router_factory)
    }
}

pub async fn handle_client<S, F>(stream: S, mut server: RouterService) -> anyhow::Result<()>
where
    S: AsyncStream,
    F: Framer<S>,
{
    let mut framer = F::new(stream);

    loop {
        let message = match framer.read_frame().await {
            Ok(msg) => msg,
            Err(e) => {
                eprintln!("Error reading frame: {e}");
                break;
            }
        };

        println!("Received: {message}");

        let response = match serde_json::from_str::<JsonRpcRequest>(&message) {
            Ok(msg) => server.call(msg).await,
            Err(e) => JsonRpcResponse {
                jsonrpc: "2.0".into(),
                id: 0,
                result: None,
                error: Some(ResponseError {
                    code: -32700,
                    message: format!("Parse error: {e}"),
                }),
            },
        };

        let response = serde_json::to_vec(&response)?;
        if let Err(e) = framer.write_frame(&response).await {
            eprintln!("Error writing frame: {e}");
            break;
        }
    }

    Ok(())
}
