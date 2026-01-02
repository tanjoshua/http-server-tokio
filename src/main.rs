use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:6767").await?;
    loop {
        let (_socket, socket_addr) = listener.accept().await?;
        println!("New connection: {}", socket_addr);
    }
}
