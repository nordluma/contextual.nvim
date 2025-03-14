use std::{path::PathBuf, sync::Arc};

use tokio::net::UnixListener;

use crate::{
    jsonrpc::JsonRpcServer,
    transport::{Transport, handle_client},
};

use super::GenError;

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
    async fn start(self, server: JsonRpcServer) -> Result<(), GenError> {
        if self.socket.exists() {
            std::fs::remove_file(&self.socket)?;
        }

        let listener = UnixListener::bind(&self.socket)?;
        println!("Server listening on {self}");
        let server = Arc::new(server);

        loop {
            match listener.accept().await {
                Ok((stream, client_addr)) => {
                    eprintln!("New connection from: {client_addr:?}");
                    let server_clone = server.clone();
                    tokio::spawn(async move {
                        if let Err(e) = handle_client(stream, server_clone).await {
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
