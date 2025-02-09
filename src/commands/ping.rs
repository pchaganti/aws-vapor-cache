use lambda_extension::tracing;
use redis::Value;

use crate::{
    commands::{Command, CommandError},
    database::Database,
};

pub struct PingCommand;

impl Command for PingCommand {
    async fn execute(request: &[Value], _: Database) -> Result<Value, CommandError> {
        tracing::info!("Executing 'ping' command");
        match request.len() {
            1 => Ok(Value::SimpleString("PONG".to_string())),
            2 => Ok(request[1].clone()),
            _ => Err("wrong number of arguments for 'ping' command".to_string()),
        }
    }
}
