use std::marker::PhantomData;

use tokio::io::{AsyncRead, AsyncWrite};

use crate::{
    router::RouterFactory,
    transport::{Transport, codec::Framer, handle_client},
};

pub struct StdIoTransport<F> {
    _framer: PhantomData<F>,
}

impl<F> StdIoTransport<F> {
    pub fn new() -> Self {
        Self {
            _framer: PhantomData,
        }
    }
}

impl<F> Transport for StdIoTransport<F>
where
    F: Framer<CombinedStream<tokio::io::Stdin, tokio::io::Stdout>> + Send + 'static,
{
    async fn start(self, server: RouterFactory) -> Result<(), anyhow::Error> {
        println!("Server listening on stdin/stdout");
        let service = server.service();
        let stream = CombinedStream::new(tokio::io::stdin(), tokio::io::stdout());

        handle_client::<_, F>(stream, service).await?;

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
