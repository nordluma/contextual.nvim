use contextual_backend::{
    args::{Args, TransportType},
    jsonrpc::JsonRpcServer,
    transport::{Transport, tcp::TcpTransport},
};

#[tokio::main]
async fn main() {
    let args = Args::parse_and_validate();

    let mut server = JsonRpcServer::new();

    match args.transport {
        TransportType::Unix { socket_path } => {
            todo!()
        }
        TransportType::Tcp { host, port } => {
            let transport = TcpTransport::new(&host, port);
            transport.start(server).await.unwrap();
        }
        TransportType::Stdio => todo!(),
    }
}
