use tokio::io::{AsyncRead, AsyncWrite, Stdin, Stdout};

use crate::{
    jsonrpc::{JsonRpcRequest, JsonRpcResponse},
    router::RouterFactory,
    transport::{
        Transport,
        codec::{Codec, LengthDelimited},
        handle_client,
    },
};

pub struct StdIoTransport;

impl Transport for StdIoTransport {
    type Stream = CombinedStream<Stdin, Stdout>;
    type Framer = LengthDelimited<Self::Stream>;

    async fn start<C: Codec<JsonRpcRequest, JsonRpcResponse> + Send>(
        self,
        server: RouterFactory,
        codec: C,
    ) -> Result<(), anyhow::Error> {
        use tokio::io::{stdin, stdout};

        println!("Server listening on stdin/stdout");
        let service = server.service();
        let framer = Self::Framer::new(CombinedStream::new(stdin(), stdout()));

        handle_client(framer, codec, service).await?;

        Ok(())
    }
}

/// Combined stream for stdin/stdout
pub struct CombinedStream<R, W> {
    reader: R,
    writer: W,
}

impl<R, W> CombinedStream<R, W>
where
    R: AsyncRead + Unpin,
    W: AsyncWrite + Unpin,
{
    pub fn new(reader: R, writer: W) -> Self {
        Self { reader, writer }
    }
}

impl<R, W> AsyncRead for CombinedStream<R, W>
where
    R: AsyncRead + Unpin,
    W: AsyncWrite + Unpin,
{
    fn poll_read(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        std::pin::Pin::new(&mut self.reader).poll_read(cx, buf)
    }
}

impl<R, W> AsyncWrite for CombinedStream<R, W>
where
    R: AsyncRead + Unpin,
    W: AsyncWrite + Unpin,
{
    fn poll_write(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &[u8],
    ) -> std::task::Poll<Result<usize, std::io::Error>> {
        std::pin::Pin::new(&mut self.writer).poll_write(cx, buf)
    }

    fn poll_flush(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), std::io::Error>> {
        std::pin::Pin::new(&mut self.writer).poll_flush(cx)
    }

    fn poll_shutdown(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), std::io::Error>> {
        std::pin::Pin::new(&mut self.writer).poll_shutdown(cx)
    }
}
