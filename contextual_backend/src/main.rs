use std::sync::Arc;

use contextual_backend::{
    args::{Args, TransportType},
    database::file::FileDatabase,
    handlers::Handler,
    jsonrpc::{HandlerFut, JsonRpcServer},
    transport::{Transport, stdio::StdIoTransport, tcp::TcpTransport, unix_socket::UnixTransport},
};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse_and_validate();

    let handler = Arc::new(Handler::new(FileDatabase::init().await));
    let mut server = JsonRpcServer::new();
    let handler_clone = Arc::clone(&handler);
    server.register_method(
        "contextual/saveNote",
        Box::new(move |params| -> HandlerFut {
            let handler = Arc::clone(&handler_clone);
            Box::pin(async move { handler.save_note(params).await })
        }),
    );

    let handler_clone = Arc::clone(&handler);
    server.register_method(
        "contextual/newTodo",
        Box::new(move |params| -> HandlerFut {
            let handler = Arc::clone(&handler_clone);
            Box::pin(async move { handler.save_todo_item(params).await })
        }),
    );

    match args.transport {
        TransportType::Unix { socket_path } => UnixTransport::new(socket_path).start(server).await,
        TransportType::Tcp { host, port } => TcpTransport::new(&host, port).start(server).await,
        TransportType::Stdio => StdIoTransport.start(server).await,
    }
}
