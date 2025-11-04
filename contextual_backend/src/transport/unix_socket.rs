use std::path::PathBuf;

use crate::{
    jsonrpc::{JsonRpcRequest, JsonRpcResponse},
    router::RouterFactory,
    transport::{
        Transport,
        codec::{Codec, LengthDelimited},
        handle_client,
    },
};

pub struct UnixTransport {
    socket: PathBuf,
}

impl UnixTransport {
    pub fn new(socket: impl Into<PathBuf>) -> Self {
        Self {
            socket: socket.into(),
        }
    }
}

impl std::fmt::Display for UnixTransport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.socket.display())
    }
}

impl Transport for UnixTransport {
    type Stream = tokio::net::UnixStream;
    type Framer = LengthDelimited<Self::Stream>;

    async fn start<C: Codec<JsonRpcRequest, JsonRpcResponse>>(
        self,
        server: RouterFactory,
        codec: C,
    ) -> Result<(), anyhow::Error> {
        if self.socket.exists() {
            std::fs::remove_file(&self.socket)?;
        }

        let listener = tokio::net::UnixListener::bind(&self.socket)?;
        println!("Server listening on {self}");

        loop {
            match listener.accept().await {
                Ok((stream, client_addr)) => {
                    eprintln!("New connection from: {client_addr:?}");
                    let service = server.service();
                    let framer = Self::Framer::new(stream);
                    tokio::spawn(async move {
                        if let Err(e) = handle_client(framer, codec, service).await {
                            eprintln!("Connection error: {e}");
                        }
                    })
                    .await?;
                }
                Err(e) => eprintln!("Error accepting client: {e}"),
            }
        }
    }
}
