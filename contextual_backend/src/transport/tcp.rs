use tokio::net::TcpListener;

use crate::{
    router::RouterFactory,
    transport::{Transport, handle_client},
};

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
    async fn start(self, server: RouterFactory) -> Result<(), anyhow::Error> {
        let listener = TcpListener::bind(format!("{self}")).await?;
        println!("Server listening on {self}");

        loop {
            let (stream, client_addr) = listener.accept().await?;
            let service = server.service();
            eprintln!("New connection from: {client_addr}");

            tokio::spawn(async move {
                if let Err(e) = handle_client(stream, service).await {
                    eprintln!("Connection error: {e}");
                }
            });
        }
    }
}
