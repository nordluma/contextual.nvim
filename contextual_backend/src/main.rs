use contextual_backend::{
    args::{Args, TransportType},
    jsonrpc::JsonRpcServer,
    transport::{Transport, stdio::StdIoTransport, tcp::TcpTransport, unix_socket::UnixTransport},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse_and_validate();

    // TODO: define handlers and register them to the jsonrpc server
    let server = JsonRpcServer::new();

    match args.transport {
        TransportType::Unix { socket_path } => UnixTransport::new(socket_path).start(server).await,
        TransportType::Tcp { host, port } => TcpTransport::new(&host, port).start(server).await,
        TransportType::Stdio => StdIoTransport.start(server).await,
    }
}
