// src/network.rs
mod network;

pub use self::network::Network;

pub mod network {
    use std::sync::Arc;
    use tokio::net::TcpListener;
    use tokio::stream::StreamExt;
    use tokio::task;
    use crate::blockchain::Blockchain;

    pub struct Network {
        peers: Vec<Arc<String>>,
        blockchain: Arc<Blockchain>,
    }

    impl Network {
        pub async fn new(blockchain: Arc<Blockchain>) -> Self {
            Network {
                peers: Vec::new(),
                blockchain,
            }
        }

        pub async fn start(&self) {
            let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();

            println!("Listening on 127.0.0.1:8080");

            while let Some((socket, _)) = listener.next().await {
                match socket {
                    Ok(socket) => {
                        task::spawn(async move {
                            println!("New connection from {:?}", socket.peer_addr().unwrap());
                            let mut stream = socket.into_stream();
                            while let Some(result) = stream.next().await {
                                match result {
                                    Ok(buf) => {
                                        println!("Received message: {:?}", buf);
                                    },
                                    Err(e) => {
                                        eprintln!("Error reading from socket: {}", e);
                                        break;
                                    },
                                }
                            }
                        });
                    },
                    Err(e) => {
                        eprintln!("Failed to accept connection: {}", e);
                    },
                }
            }
        }
    }
}