use tokio::io::{AsyncRead, AsyncWrite};

use crate::{
    jsonrpc::{JsonRpcRequest, JsonRpcResponse, ResponseError},
    router::{RouterFactory, RouterService},
    service::Service,
    transport::codec::{Codec, Framer},
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
    type Stream: AsyncStream + Send + 'static;
    type Framer: Framer<Self::Stream> + Send + 'static;

    fn start<C: Codec<JsonRpcRequest, JsonRpcResponse>>(
        self,
        server: RouterFactory,
        codec: C,
    ) -> impl Future<Output = Result<(), anyhow::Error>> + Send;
}

pub struct Server<T, C>
where
    T: Transport,
    C: Codec<JsonRpcRequest, JsonRpcResponse>,
{
    transport: T,
    codec: C,
}

impl<T, C> Server<T, C>
where
    T: Transport,
    C: Codec<JsonRpcRequest, JsonRpcResponse>,
{
    pub fn new(transport: T, codec: C) -> Self {
        Self { transport, codec }
    }

    pub fn start(
        self,
        router_factory: RouterFactory,
    ) -> impl Future<Output = Result<(), anyhow::Error>> + Send {
        self.transport.start(router_factory, self.codec)
    }
}

pub async fn handle_client<S, F, C>(
    mut framer: F,
    codec: C,
    mut server: RouterService,
) -> anyhow::Result<()>
where
    S: AsyncStream,
    F: Framer<S>,
    C: Codec<JsonRpcRequest, JsonRpcResponse>,
{
    loop {
        let message = match framer.read_frame().await {
            Ok(msg) => msg,
            Err(e) => {
                eprintln!("Error reading frame: {e}");
                break;
            }
        };

        println!("Received: {message}");

        let response = match codec.decode(message.as_bytes()) {
            Ok(msg) => match server.call(msg).await {
                Ok(res) => res,
                Err(_) => unreachable!("all error should be handled by router service"),
            },
            Err(e) => JsonRpcResponse::from_error(
                0,
                ResponseError {
                    code: -32700,
                    message: format!("Parse error: {e}"),
                },
            ),
        };

        let response = codec.encode(&response)?;
        if let Err(e) = framer.write_frame(&response).await {
            eprintln!("Error writing frame: {e}");
            break;
        }
    }

    Ok(())
}
