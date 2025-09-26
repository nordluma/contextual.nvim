use crate::{
    jsonrpc::{JsonRpcRequest, JsonRpcResponse},
    router::RouterFactory,
    transport::{
        Transport,
        codec::{Codec, LengthDelimited},
        handle_client,
    },
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
    type Stream = tokio::net::TcpStream;
    type Framer = LengthDelimited<Self::Stream>;

    async fn start<C: Codec<JsonRpcRequest, JsonRpcResponse> + 'static>(
        self,
        server: RouterFactory,
        codec: C,
    ) -> Result<(), anyhow::Error> {
        let listener = tokio::net::TcpListener::bind(format!("{self}")).await?;
        println!("Server listening on {self}");

        loop {
            let (stream, client_addr) = listener.accept().await?;
            let service = server.service();
            eprintln!("New connection from: {client_addr}");

            let framer = Self::Framer::new(stream);
            tokio::spawn(async move {
                if let Err(e) = handle_client(framer, codec, service).await {
                    eprintln!("Connection error: {e}");
                }
            });
        }
    }
}
