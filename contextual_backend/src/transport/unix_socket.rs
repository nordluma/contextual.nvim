use std::{path::PathBuf, sync::Arc};

use tokio::{
    io::BufReader,
    net::{UnixListener, UnixStream},
};

use crate::{jsonrpc::JsonRpcServer, transport::write_message};

use super::{GenError, Transport, read_message};

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

async fn handle_client(
    mut stream: UnixStream,
    server: Arc<JsonRpcServer>,
) -> tokio::io::Result<()> {
    let (read_half, mut write_half) = stream.split();
    let mut reader = BufReader::new(read_half);

    loop {
        match read_message(&mut reader).await {
            Ok(message) => {
                println!("Received: {message}");
                let response = server.handle_request(&message);
                write_message(&mut write_half, &response).await?;
            }
            Err(e) => {
                eprintln!("Error reading message: {e}");
                break;
            }
        }
    }

    Ok(())
}
