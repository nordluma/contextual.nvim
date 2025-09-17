use contextual_backend::{
    args::{Args, TransportType},
    router::RouterFactory,
    transport::{Transport, stdio::StdIoTransport, tcp::TcpTransport, unix_socket::UnixTransport},
};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse_and_validate();

    let router = RouterFactory::new();

    // let mut server = JsonRpcServer::new();
    // let handler_clone = Arc::clone(&handler);
    // server.register_method(
    //     "contextual/saveNote",
    //     Box::new(move |params| -> HandlerFut {
    //         let handler = Arc::clone(&handler_clone);
    //         Box::pin(async move { handler.save_note(params).await })
    //     }),
    // );
    //
    // let handler_clone = Arc::clone(&handler);
    // server.register_method(
    //     "contextual/newTodo",
    //     Box::new(move |params| -> HandlerFut {
    //         let handler = Arc::clone(&handler_clone);
    //         Box::pin(async move { handler.save_todo_item(params).await })
    //     }),
    // );
    //
    // let handler_clone = Arc::clone(&handler);
    // server.register_method(
    //     "contextual/syncTodos",
    //     Box::new(move |params| -> HandlerFut {
    //         let handler = Arc::clone(&handler_clone);
    //         Box::pin(async move { handler.sync_todos(params).await })
    //     }),
    // );

    match args.transport {
        TransportType::Unix { socket_path } => UnixTransport::new(socket_path).start(router).await,
        TransportType::Tcp { host, port } => TcpTransport::new(&host, port).start(router).await,
        TransportType::Stdio => StdIoTransport.start(router).await,
    }
}
