use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader, ReadHalf, WriteHalf};

use crate::transport::AsyncStream;

#[async_trait::async_trait]
pub trait Framer<S>
where
    S: AsyncStream,
{
    fn new(stream: S) -> Self
    where
        Self: Sized;

    async fn read_frame(&mut self) -> std::io::Result<String>;
    async fn write_frame(&mut self, frame: &[u8]) -> std::io::Result<()>;
}

pub trait Codec<Req, Res> {
    fn decode(&self, bytes: &[u8]) -> Result<Req, anyhow::Error>;
    fn encode(&self, res: &Res) -> Result<Vec<u8>, anyhow::Error>;
}

pub struct LengthDelimited<S> {
    reader: BufReader<ReadHalf<S>>,
    writer: WriteHalf<S>,
}

impl<S> LengthDelimited<S>
where
    S: AsyncStream,
{
    pub fn new(stream: S) -> Self {
        let (read, write) = tokio::io::split(stream);

        Self {
            reader: BufReader::new(read),
            writer: write,
        }
    }
}

#[async_trait::async_trait]
impl<S> Framer<S> for LengthDelimited<S>
where
    S: AsyncStream,
{
    fn new(stream: S) -> Self {
        Self::new(stream)
    }

    /// Read a single JSON-RPC message from reader.
    ///
    /// Expects a header section ending with and empty line (i.e. "\r\n") and then
    /// reads the message body based on the Content-Length header.
    async fn read_frame(&mut self) -> std::io::Result<String> {
        use std::io::{Error as IoError, ErrorKind as IoErrorKind};

        let mut header = String::new();
        loop {
            let mut line = String::new();
            let bytes_read = self.reader.read_line(&mut line).await?;
            if bytes_read == 0 {
                return Err(IoError::new(
                    IoErrorKind::UnexpectedEof,
                    "Connection closed",
                ));
            }

            if line.as_str() == "\r\n" {
                break;
            }
            header.push_str(&line);
        }

        let content_length = header
            .lines()
            .find_map(|l| {
                if l.to_ascii_lowercase().starts_with("content-length") {
                    l.split(':').nth(1).and_then(|s| s.trim().parse().ok())
                } else {
                    None
                }
            })
            .ok_or_else(|| {
                IoError::new(IoErrorKind::InvalidData, "Missing Content-Length header")
            })?;

        let mut buffer = vec![0u8; content_length];
        self.reader.read_exact(&mut buffer).await?;

        String::from_utf8(buffer)
            .map_err(|_| IoError::new(IoErrorKind::InvalidData, "Invalid UTF-8 message"))
    }

    /// Write a single JSON-RPC message to the writer.
    ///
    /// Write a header with the Content-Length followed by two CRLFs and then write
    /// the message payload.
    async fn write_frame(&mut self, frame: &[u8]) -> std::io::Result<()> {
        let content_len = frame.len();
        let header = format!("Content-Length: {content_len}\r\n\r\n");

        self.writer.write_all(header.as_bytes()).await?;
        self.writer.write_all(frame).await?;
        self.writer.flush().await?;

        Ok(())
    }
}
