use contextual_backend::{
    args::{Args, TransportType},
    router::RouterFactory,
    transport::{Server, stdio::StdIoTransport, tcp::TcpTransport, unix_socket::UnixTransport},
};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse_and_validate();

    let router = RouterFactory::new();

    match args.transport {
        TransportType::Unix { socket_path } => {
            Server::new(UnixTransport::new(socket_path))
                .start(router)
                .await
        }
        TransportType::Tcp { host, port } => {
            Server::new(TcpTransport::new(&host, port))
                .start(router)
                .await
        }
        TransportType::Stdio => Server::new(StdIoTransport).start(router).await,
    }
}
