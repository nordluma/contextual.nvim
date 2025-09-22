use std::marker::PhantomData;

use tokio::net::TcpListener;

use crate::{
    router::RouterFactory,
    transport::{Transport, codec::Framer, handle_client},
};

pub struct TcpTransport<F> {
    host: String,
    port: u16,
    _framer: PhantomData<F>,
}

impl<F> std::fmt::Display for TcpTransport<F> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.host, self.port)
    }
}

impl<F> TcpTransport<F> {
    pub fn new(host: &str, port: u16) -> Self {
        Self {
            host: host.to_string(),
            port,
            _framer: PhantomData,
        }
    }
}

impl<F> Transport for TcpTransport<F>
where
    F: Framer<tokio::net::TcpStream> + Send + 'static,
{
    async fn start(self, server: RouterFactory) -> Result<(), anyhow::Error> {
        let listener = TcpListener::bind(format!("{self}")).await?;
        println!("Server listening on {self}");

        loop {
            let (stream, client_addr) = listener.accept().await?;
            let service = server.service();
            eprintln!("New connection from: {client_addr}");

            tokio::spawn(async move {
                if let Err(e) = handle_client::<_, F>(stream, service).await {
                    eprintln!("Connection error: {e}");
                }
            });
        }
    }
}
