use crate::http::{Request, Response, decode_http_request};
use bytes::BytesMut;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

mod http;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:6767").await?;
    loop {
        let (stream, socket_addr) = listener.accept().await?;
        tokio::spawn(async move {
            println!("New connection: {}", socket_addr);
            if let Err(e) = process(stream).await {
                eprintln!("Could not process connection: {}", e);
            }
        });
    }
}

async fn process(mut stream: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    let mut buf = BytesMut::with_capacity(4096);
    stream.read_buf(&mut buf).await?;

    let request = decode_http_request(buf);

    match request {
        Ok(request) => {
            println!("{}", request);
            let response = handle_request(request);
            let response_bytes: Vec<u8> = response.into();
            if stream.write_all(&response_bytes).await.is_err() {
                eprintln!("Error writing response");
            }
        }
        Err(_) => {
            eprintln!("Error occurred");
        }
    }

    Ok(())
}

fn handle_request(_request: Request) -> Response {
    http::Response { code: 200 }
}
