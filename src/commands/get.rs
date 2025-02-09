use lambda_extension::tracing;
use redis::Value;

use crate::{
    commands::{Command, CommandError},
    database::Database,
};

pub struct GetCommand;

impl Command for GetCommand {
    async fn execute(request: &[Value], database: Database) -> Result<Value, CommandError> {
        tracing::info!("Executing 'set' command");
        if request.len() != 2 {
            return Err("invalid 'set' arguments".to_string());
        }
        let key = match &request[1] {
            Value::BulkString(key) => key,
            Value::SimpleString(key) => key.as_bytes(),
            _ => return Err("invalid 'set' key".to_string()),
        };
        if let Some(value) = database.storage.get(key).await {
            Ok(Value::BulkString(value))
        } else {
            Ok(Value::Nil)
        }
    }
}
