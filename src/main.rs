use anyhow::Result;
use std::time::Duration;
use tokio::{
    io::AsyncWriteExt,
    net::{TcpListener, TcpStream},
    sync::broadcast::{Receiver, Sender},
};

#[tokio::main]
async fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();

    let (tx, _): (Sender<String>, Receiver<String>) = tokio::sync::broadcast::channel(10);
    let tx1 = tx.clone();

    tokio::spawn(async move {
        for _ in 0..1000 {
             tx.send("hello".into())?;
            tokio::time::sleep(Duration::from_secs(5)).await;
        }
    });

    while let Ok((socket, _)) = listener.accept().await {
        let mut rx = tx1.subscribe();
        println!("new socket");
        tokio::spawn(async move { process_socket(socket, rx).await });
    }
    Ok(())
}

async fn process_socket(mut socket: TcpStream, mut rx: Receiver<String>) -> Result<()> {
    match socket.write_all("connected".as_bytes()).await {
        Ok(_) => {}
        Err(e) => {
            println!("e: {:?}", e);
        }
    }

    while let Ok(msg) = rx.recv().await {
         socket.write_all(msg.as_bytes()).await?
    }
    Ok(())
}
