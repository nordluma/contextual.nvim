use tokio::io::{
    AsyncBufReadExt, AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt, BufReader,
    Error as IoError, ErrorKind as IoErrorKind, Result as IoResult,
};

use crate::{
    jsonrpc::{JsonRpcRequest, JsonRpcResponse},
    router::{RouterFactory, RouterService},
    service::Service,
};

pub mod codec;
pub mod stdio;
pub mod tcp;
pub mod unix_socket;

/// Generic trait for any async read/write capable stream
pub trait AsyncStream: AsyncRead + AsyncWrite + Unpin + Send + 'static {}

// Implement for any type that satisfies the requirements
impl<T> AsyncStream for T where T: AsyncRead + AsyncWrite + Unpin + Send + 'static {}

pub trait Transport {
    fn start(self, server: RouterFactory)
    -> impl Future<Output = Result<(), anyhow::Error>> + Send;
}

async fn handle_client<S>(stream: S, mut server: RouterService) -> tokio::io::Result<()>
where
    S: AsyncStream,
{
    let (read_half, write_half) = tokio::io::split(stream);
    let mut reader = BufReader::new(read_half);
    let mut writer = write_half;

    loop {
        match read_message(&mut reader).await {
            Ok(message) => {
                println!("Received: {message}");
                let response = match serde_json::from_str::<JsonRpcRequest>(&message) {
                    Ok(msg) => server.call(msg).await,
                    Err(e) => JsonRpcResponse {
                        jsonrpc: "2.0".into(),
                        id: 0,
                        result: None,
                        error: Some(crate::jsonrpc::ResponseError {
                            code: -32700,
                            message: format!("Parse error: {e}"),
                        }),
                    },
                };

                let response = serde_json::to_string(&response).expect("response json is valid");
                write_message(&mut writer, &response).await?;
            }
            Err(e) => {
                eprintln!("Error reading message: {e}");
                break;
            }
        }
    }

    Ok(())
}

/// Read a single JSON-RPC message from reader.
///
/// Expects a header section ending with and empty line (i.e. "\r\n") and then
/// reads the message body based on the Content-Length header.
async fn read_message<R: AsyncBufReadExt + Unpin>(reader: &mut R) -> IoResult<String> {
    let mut header = String::new();
    loop {
        let mut line = String::new();
        let bytes_read = reader.read_line(&mut line).await?;
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
        .find_map(|line| {
            if line.to_ascii_lowercase().starts_with("content-length:") {
                line.split(':')
                    .nth(1)
                    .and_then(|s| s.trim().parse::<usize>().ok())
            } else {
                None
            }
        })
        .ok_or_else(|| IoError::new(IoErrorKind::InvalidData, "Missing Content-Length header"))?;

    let mut buffer = vec![0u8; content_length];
    reader.read_exact(&mut buffer).await?;

    String::from_utf8(buffer)
        .map_err(|_| IoError::new(IoErrorKind::InvalidData, "Invalid UTF-8 message"))
}

/// Write a single JSON-RPC message to the writer.
///
/// Write a header with the Content-Length followed by two CRLFs and then write
/// the message payload.
async fn write_message<W: AsyncWriteExt + Unpin>(
    writer: &mut W,
    message: &str,
) -> tokio::io::Result<()> {
    let content_length = message.len();
    let header = format!("Content-Length: {content_length}\r\n\r\n");
    writer.write_all(header.as_bytes()).await?;
    writer.write_all(message.as_bytes()).await?;
    writer.flush().await?;

    Ok(())
}
