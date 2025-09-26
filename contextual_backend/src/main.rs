use contextual_backend::{
    args::{Args, TransportType},
    database::file::FileDatabase,
    handlers::{echo::EchoService, todo::NewTodoService},
    router::RouterFactory,
    transport::{
        Server, codec::JsonRpcCodec, stdio::StdIoTransport, tcp::TcpTransport,
        unix_socket::UnixTransport,
    },
};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse_and_validate();
    let storage = FileDatabase::init().await;

    let router = RouterFactory::new()
        .with_route("contextual/echo", EchoService)
        .with_route("contextual/new_todo", NewTodoService::new(storage.clone()));

    let codec = JsonRpcCodec;

    match args.transport {
        TransportType::Unix { socket_path } => {
            Server::new(UnixTransport::new(socket_path), codec)
                .start(router)
                .await
        }
        TransportType::Tcp { host, port } => {
            Server::new(TcpTransport::new(&host, port), codec)
                .start(router)
                .await
        }
        TransportType::Stdio => Server::new(StdIoTransport, codec).start(router).await,
    }
}
