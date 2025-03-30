use lambda_extension::tracing;
use redis::Value;

use crate::{
    commands::{Command, CommandError},
    database::Database,
};

pub struct DelCommand;

impl Command for DelCommand {
    async fn execute(request: &[Value], database: Database) -> Result<Value, CommandError> {
        tracing::info!("Executing 'del' command");
        if request.len() < 2 {
            return Err("invalid 'del' arguments".to_string());
        }
        let key = match &request[1] {
            Value::BulkString(key) => key,
            Value::SimpleString(key) => key.as_bytes(),
            _ => return Err("invalid 'set' key".to_string()),
        };
        database.storage.remove(key).await;
        Ok(Value::SimpleString("OK".to_string()))
    }
}
