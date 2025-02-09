use std::collections::VecDeque;

use combine::{
    parser::combinator::AnySendSyncPartialState,
    stream::{Decoder, PointerOffset},
};
use redis::parse_redis_value_async;
use tokio::{
    io::{AsyncWriteExt, BufReader},
    net::tcp::{ReadHalf, WriteHalf},
};

use crate::commands::CommandError;

pub struct ParserError {
    pub error: String,
}

pub struct Parser<'a> {
    decoder: Decoder<AnySendSyncPartialState, PointerOffset<[u8]>>,
    reader: BufReader<ReadHalf<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(reader: ReadHalf<'a>) -> Self {
        Self {
            decoder: Decoder::new(),
            reader: BufReader::new(reader),
        }
    }

    pub async fn read(&mut self) -> Result<redis::Value, ParserError> {
        Ok(parse_redis_value_async(&mut self.decoder, &mut self.reader).await?)
    }

    pub async fn write(
        &self,
        value: Result<redis::Value, CommandError>,
        writer: &mut WriteHalf<'_>,
    ) -> Result<(), ParserError> {
        match value {
            Ok(value) => self.write_value(value, writer).await?,
            Err(error) => {
                writer.write_all(b"-ERR ").await?;
                writer.write_all(error.as_bytes()).await?;
                writer.write_all(b"\r\n").await?;
            }
        }
        Ok(())
    }

    pub async fn write_value(
        &self,
        value: redis::Value,
        writer: &mut WriteHalf<'_>,
    ) -> Result<(), ParserError> {
        let mut values: VecDeque<redis::Value> = VecDeque::new();
        values.push_back(value);
        while let Some(value) = values.pop_front() {
            match value {
                redis::Value::Nil => {
                    writer.write_all(b"_\r\n").await?;
                }
                redis::Value::Int(value) => {
                    writer
                        .write_all(format!(":{:+}\r\n", value).as_bytes())
                        .await?;
                }
                redis::Value::BulkString(bytes) => {
                    writer
                        .write_all(format!("${}\r\n", bytes.len()).as_bytes())
                        .await?;
                    writer.write_all(&bytes).await?;
                    writer.write_all(b"\r\n").await?;
                }
                redis::Value::SimpleString(string) => {
                    writer.write_all(b"+").await?;
                    writer.write_all(string.as_bytes()).await?;
                    writer.write_all(b"\r\n").await?;
                }
                _ => {
                    writer.write_all(b"-ERR unsupported response\r\n").await?;
                }
            }
        }
        Ok(())
    }
}

impl From<redis::RedisError> for ParserError {
    fn from(error: redis::RedisError) -> Self {
        Self {
            error: error.to_string(),
        }
    }
}
impl From<std::io::Error> for ParserError {
    fn from(error: std::io::Error) -> Self {
        Self {
            error: error.to_string(),
        }
    }
}
