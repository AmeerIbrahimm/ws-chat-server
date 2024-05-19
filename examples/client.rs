use std::io::Result;
use tokio::{io::AsyncReadExt, net::TcpStream};

#[tokio::main]
async fn main() -> Result<()> {
    let mut socket = TcpStream::connect("127.0.0.1:8080").await?;
    loop {
        let mut buf = [0; 1028];
        match socket.read(&mut buf).await {
            Ok(0) => break,
            Ok(_) => {
                if let Ok(s) = String::from_utf8(buf.to_vec()) {
                    println!("GOT:{}", s);
                };
            }
            Err(_) => break,
        }
    }
    Ok(())
}
