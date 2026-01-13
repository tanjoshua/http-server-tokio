mod h1;
use h1::{Content, Method, Request, Response, decode_http_request};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

use bytes::{Buf, BytesMut};

use crate::h1::Encoding;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("0.0.0.0:8080").await?;
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

    loop {
        let n = stream.read_buf(&mut buf).await?;
        if n == 0 {
            break;
        }

        let request = decode_http_request(&mut buf);

        match request {
            Ok((request, bytes_read)) => {
                println!("{:?} request received at {}", request.method, request.uri);

                // advance buffer
                buf.advance(bytes_read);

                // get encodings for response before request passed to handler
                let encodings = request.headers.get("Accept-Encoding").cloned();

                // check whether tcp connection should be persisted
                let should_close = request
                    .headers
                    .get("Connection")
                    .is_some_and(|v| v == "close");

                let mut response = handle_request(request);

                // set close header
                if should_close {
                    response.headers.insert("Connection".into(), "close".into());
                }

                // set encoding,
                if let Some(encodings) = encodings
                    && encodings.split(",").map(|s| s.trim()).any(|s| s == "gzip")
                {
                    response.content_encoding = Some(Encoding::Gzip);
                }

                let response_bytes: Vec<u8> = response.into();
                if stream.write_all(&response_bytes).await.is_err() {
                    eprintln!("Error writing response");
                }

                // close connection if header was set
                if should_close {
                    break;
                }
            }
            Err(e) => {
                eprintln!("Error occurred {}", e);
            }
        }
    }

    Ok(())
}

fn handle_request(request: Request) -> Response {
    match (request.method, request.uri.as_str()) {
        (Method::Get, "/") => main_page_handler(request),
        (_, _) => Response::new(404, Content::Empty),
    }
}

fn main_page_handler(_request: Request) -> Response {
    let Ok(file) = std::fs::read("public/index.html") else {
        return Response::new(404, Content::Text("File not found".into()));
    };

    Response::new(200, Content::Html(file))
}
