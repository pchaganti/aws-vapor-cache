use tokio::{net::TcpListener, sync::watch, task::JoinHandle};

use crate::{database::Database, handler::Handler};

pub struct VaporCacheServer {
    task: JoinHandle<()>,
    shutdown_tx: watch::Sender<bool>,
}

impl VaporCacheServer {
    pub async fn start() -> Result<Self, lambda_extension::Error> {
        let listener = TcpListener::bind("127.0.0.1:6379").await?;
        let (stop_tx, _) = watch::channel(false);
        let shutdown_tx = stop_tx.clone();
        let database = Database::default();

        let task = tokio::spawn(async move {
            let mut listener_stop_rx = stop_tx.subscribe();
            while let Some((socket, _addr)) = tokio::select! {
                v = listener.accept() => if let Ok(v) = v { Some(v) } else { None },
                _ = listener_stop_rx.changed() => None,
            } {
                Handler::start(socket, database.clone(), stop_tx.subscribe()).await;
            }
            println!("Server stopped, braodcast stop");
        });
        Ok(Self { task, shutdown_tx })
    }

    pub async fn stop(self) -> Result<(), lambda_extension::Error> {
        if self.shutdown_tx.send(true).is_ok() {
            self.task.await?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use redis::AsyncCommands;

    #[tokio::test]
    async fn test_ping() {
        eprintln!("Starting server");

        let server = VaporCacheServer::start().await.unwrap();
        let client = redis::Client::open("redis://127.0.0.1/").unwrap();
        let mut con = client.get_connection_manager().await.unwrap();

        assert!(con.ping::<()>().await.is_ok());
        assert!(con.ping_message::<_, ()>("salam").await.is_ok());
        assert!(con.get::<_, String>("foo").await.is_err());
        server.stop().await.unwrap();
    }
}
