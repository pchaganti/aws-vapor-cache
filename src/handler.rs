use lambda_extension::tracing;
use tokio::sync::watch;

use crate::{commands::execute_command, database::Database, parser::Parser};

pub struct Handler;

impl Handler {
    pub async fn start(
        mut socket: tokio::net::TcpStream,
        database: Database,
        stop_rx: watch::Receiver<bool>,
    ) {
        tokio::spawn(async move {
            let (reader, mut writer) = socket.split();
            let mut parser = Parser::new(reader);

            while !stop_rx.has_changed().is_ok_and(|v| v) {
                match parser.read().await {
                    Ok(request) => {
                        let response = execute_command(&request, database.clone()).await;
                        if parser.write(response, &mut writer).await.is_err() {
                            tracing::error!("Error writing response");
                            break;
                        }
                    }
                    Err(error) => {
                        tracing::error!("Error reading request. {}", error.error);
                        break;
                    }
                }
            }
        });
    }
}
