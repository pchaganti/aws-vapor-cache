use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    sync::watch,
};

pub struct Handler;

impl Handler {
    pub async fn start(mut socket: tokio::net::TcpStream, stop_rx: watch::Receiver<bool>) {
        tokio::spawn(async move {
            while !stop_rx.has_changed().is_ok_and(|v| v) {
                let mut buf = [0u8; 1024];
                let len = socket.read(&mut buf).await.expect("error reading");
                if len == 0 {
                    break;
                }
                let mut redis_parser = redis::Parser::new();
                let value = redis_parser
                    .parse_value(&mut &buf[..len])
                    .expect("error parsing");
                // A client sends to the Redis server a RESP Array consisting of just Bulk Strings.
                if let redis::Value::Array(arr) = value {
                    let command: Vec<&[u8]> = arr
                        .iter()
                        .map(|v| {
                            if let redis::Value::BulkString(s) = v {
                                s.as_ref()
                            } else {
                                panic!("Expected BulkString");
                            }
                        })
                        .collect();
                    if command.len() == 1 && command[0] == b"PING" {
                        println!("Received PING");
                        let _ = socket.write_all(b"+PONG\r\n").await;
                    } else {
                        println!("Received unknown command");
                        let _ = socket.write_all(b"-ERR unknown command\r\n").await;
                    }
                } else {
                    println!("Received unknown command");
                    let _ = socket.write_all(b"-ERR unknown command\r\n").await;
                }
                socket.flush().await.expect("error flushing");
            }
        });
    }
}
