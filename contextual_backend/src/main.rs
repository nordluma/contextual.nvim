use contextual_backend::{
    args::{Args, TransportType},
    router::RouterFactory,
    transport::{
        Server,
        codec::LengthDelimited,
        stdio::{CombinedStream, StdIoTransport},
        tcp::TcpTransport,
        unix_socket::UnixTransport,
    },
};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse_and_validate();

    let router = RouterFactory::new();

    match args.transport {
        TransportType::Unix { socket_path } => {
            type F = LengthDelimited<tokio::net::UnixStream>;

            Server::<_, F>::new(UnixTransport::<F>::new(socket_path))
                .start(router)
                .await
        }
        TransportType::Tcp { host, port } => {
            type F = LengthDelimited<tokio::net::TcpStream>;

            Server::<_, F>::new(TcpTransport::<F>::new(&host, port))
                .start(router)
                .await
        }
        TransportType::Stdio => {
            type F = LengthDelimited<CombinedStream<tokio::io::Stdin, tokio::io::Stdout>>;

            Server::<_, F>::new(StdIoTransport::<F>::new())
                .start(router)
                .await
        }
    }
}
