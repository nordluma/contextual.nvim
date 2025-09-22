use std::{marker::PhantomData, path::PathBuf};

use tokio::net::UnixListener;

use crate::{
    router::RouterFactory,
    transport::{Transport, codec::Framer, handle_client},
};

pub struct UnixTransport<F> {
    socket: PathBuf,
    _framer: PhantomData<F>,
}

impl<F> UnixTransport<F> {
    pub fn new(socket: impl Into<PathBuf>) -> Self {
        Self {
            socket: socket.into(),
            _framer: PhantomData,
        }
    }
}

impl<F> std::fmt::Display for UnixTransport<F> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.socket.display())
    }
}

impl<F> Transport for UnixTransport<F>
where
    F: Framer<tokio::net::UnixStream> + Send,
{
    async fn start(self, server: RouterFactory) -> Result<(), anyhow::Error> {
        if self.socket.exists() {
            std::fs::remove_file(&self.socket)?;
        }

        let listener = UnixListener::bind(&self.socket)?;
        println!("Server listening on {self}");

        loop {
            match listener.accept().await {
                Ok((stream, client_addr)) => {
                    eprintln!("New connection from: {client_addr:?}");
                    let service = server.service();
                    tokio::spawn(async move {
                        if let Err(e) = handle_client::<_, F>(stream, service).await {
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
