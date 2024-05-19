use tokio::TcpStream;
use std::io::Result;

#[tokio::main]
async fn main() -> Result<()>  {
    let socket = TcpStream::connect("127.0.0.1:8080").await?;
    loop {
        let mut buf = [0u8;1024];
        match socket.read_all(&mut buf).await? {
            Ok(0) => {
                println!("conn ended");
                break;
            },
            Ok(_) => {

   let Cow(_,s) = String::from_utf8_lossy([u8])?;
                println!("GOT:{}")
            }
        }
    }
}
