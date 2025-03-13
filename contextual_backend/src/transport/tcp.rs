use std::sync::Arc;

use tokio::{
    io::BufReader,
    net::{TcpListener, TcpStream},
};

use crate::{jsonrpc::JsonRpcServer, transport::write_message};

use super::{GenError, Transport, read_message};

pub struct TcpTransport {
    host: String,
    port: u16,
}

impl std::fmt::Display for TcpTransport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.host, self.port)
    }
}

impl TcpTransport {
    pub fn new(host: &str, port: u16) -> Self {
        Self {
            host: host.to_string(),
            port,
        }
    }
}

impl Transport for TcpTransport {
    async fn start(self, server: JsonRpcServer) -> Result<(), GenError> {
        let listener = TcpListener::bind(format!("{self}")).await?;
        println!("Server listening on {self}");
        let server = Arc::new(server);

        loop {
            let (stream, client_addr) = listener.accept().await?;
            let server_clone = Arc::clone(&server);
            eprintln!("New connection from: {client_addr}");

            tokio::spawn(async move {
                if let Err(e) = handle_client(stream, server_clone).await {
                    eprintln!("Connection error: {e}");
                }
            })
            .await?;
        }
    }
}

async fn handle_client(mut stream: TcpStream, server: Arc<JsonRpcServer>) -> tokio::io::Result<()> {
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
