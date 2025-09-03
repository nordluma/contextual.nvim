use std::sync::Arc;

use contextual_backend::{
    args::{Args, TransportType},
    database::file::FileDatabase,
    handlers::Handler,
    jsonrpc::JsonRpcServer,
    transport::{Transport, stdio::StdIoTransport, tcp::TcpTransport, unix_socket::UnixTransport},
};
use futures::future::BoxFuture;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse_and_validate();

    let db = FileDatabase::init();
    let handler = Arc::new(Handler::new(db));
    let mut server = JsonRpcServer::new();
    server.register_method(
        "contextual/saveNote".to_string(),
        Box::new(
            move |params| -> BoxFuture<'static, Result<serde_json::Value, anyhow::Error>> {
                let handler = Arc::clone(&handler);
                Box::pin(async move { handler.save_note(params).await })
            },
        ),
    );

    match args.transport {
        TransportType::Unix { socket_path } => UnixTransport::new(socket_path).start(server).await,
        TransportType::Tcp { host, port } => TcpTransport::new(&host, port).start(server).await,
        TransportType::Stdio => StdIoTransport.start(server).await,
    }
}
