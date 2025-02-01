use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
    sync::oneshot,
    task::JoinHandle,
};

pub struct VaporCacheServer {
    task: JoinHandle<()>,
    close_tx: oneshot::Sender<()>,
}

impl VaporCacheServer {
    pub async fn start() -> Result<Self, lambda_extension::Error> {
        let listener = TcpListener::bind("127.0.0.1:6379").await?;
        let (close_tx, mut close_rx) = oneshot::channel();
        type TryRecvError = oneshot::error::TryRecvError;

        let task = tokio::spawn(async move {
            while close_rx
                .try_recv()
                .is_err_and(|err| err == TryRecvError::Empty)
            {
                let (mut socket, addr) = listener.accept().await.expect("error message");

                tokio::spawn(async move {
                    let mut buf = [0; 1024];

                    // Read data from socket
                    match socket.read(&mut buf).await {
                        Ok(0) => println!("Connection closed by {}", addr),
                        Ok(n) => {
                            println!("Received: {:?}", &buf[..n]);
                            let _ = socket.write_all(&buf[..n]).await;
                        }
                        Err(e) => eprintln!("Failed to read from socket: {:?}", e),
                    }
                });
            }
        });
        Ok(Self { task, close_tx })
    }

    pub async fn stop(self) -> Result<(), lambda_extension::Error> {
        if self.close_tx.send(()).is_ok() {
            self.task.await?;
        }
        Ok(())
    }
}
