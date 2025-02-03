use combine::{
    parser::combinator::AnySendSyncPartialState,
    stream::{Decoder, PointerOffset},
};
use redis::parse_redis_value_async;

use tokio::{
    io::{AsyncWriteExt, BufReader},
    sync::watch,
};

pub struct Handler;

impl Handler {
    pub async fn start(mut socket: tokio::net::TcpStream, stop_rx: watch::Receiver<bool>) {
        tokio::spawn(async move {
            let (reader, mut writer) = socket.split();
            let mut stream = BufReader::new(reader);
            let mut decoder = Decoder::<AnySendSyncPartialState, PointerOffset<[u8]>>::default();
            while !stop_rx.has_changed().is_ok_and(|v| v) {
                let value = parse_redis_value_async(&mut decoder, &mut stream)
                    .await
                    .expect("error parsing");
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
                        let _ = writer.write_all(b"+PONG\r\n").await;
                    } else {
                        println!("Received unknown command");
                        let _ = writer.write_all(b"-ERR unknown command\r\n").await;
                    }
                } else {
                    println!("Received unknown command");
                    let _ = writer.write_all(b"-ERR unknown command\r\n").await;
                }
                writer.flush().await.expect("error flushing");
            }
        });
    }
}
