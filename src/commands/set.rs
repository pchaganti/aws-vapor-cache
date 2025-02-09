use lambda_extension::tracing;
use redis::Value;

use crate::{
    commands::{Command, CommandError},
    database::Database,
};

pub struct SetCommand;

impl Command for SetCommand {
    async fn execute(request: &[Value], database: Database) -> Result<Value, CommandError> {
        tracing::info!("Executing 'set' command");
        if request.len() < 3 {
            return Err("invalid 'set' arguments".to_string());
        }
        let key = match &request[1] {
            Value::BulkString(key) => key,
            Value::SimpleString(key) => key.as_bytes(),
            _ => return Err("invalid 'set' key".to_string()),
        };
        let value = match &request[2] {
            Value::BulkString(value) => value,
            Value::SimpleString(value) => value.as_bytes(),
            _ => return Err("invalid 'set' value".to_string()),
        };
        database.storage.insert(key.to_vec(), value.to_vec()).await;
        Ok(Value::SimpleString("OK".to_string()))
    }
}
