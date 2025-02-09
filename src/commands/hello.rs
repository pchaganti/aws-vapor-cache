use lambda_extension::tracing;
use redis::Value;

use crate::{
    commands::{Command, CommandError},
    database::Database,
};

pub struct HelloCommand;

impl Command for HelloCommand {
    async fn execute(request: &[Value], _: Database) -> Result<Value, CommandError> {
        tracing::info!("Executing 'set' command");
        if let Some(protover) = request.get(1) {
            match protover {
                Value::BulkString(version) if version == b"3".as_slice() => Ok(()),
                Value::SimpleString(version) if version == "3" => Ok(()),
                Value::Int(version) if *version == 3 => Ok(()),
                _ => Err("requested version is not supported."),
            }?;
        }
        Ok(Value::Map(vec![
            (
                Value::SimpleString("server".to_string()),
                Value::SimpleString(env!("CARGO_PKG_NAME").to_string()),
            ),
            (
                Value::SimpleString("version".to_string()),
                Value::SimpleString(env!("CARGO_PKG_VERSION").to_string()),
            ),
            (Value::SimpleString("proto".to_string()), Value::Int(3)),
            (
                Value::SimpleString("mode".to_string()),
                Value::SimpleString("standalone".to_string()),
            ),
            (
                Value::SimpleString("role".to_string()),
                Value::SimpleString("master".to_string()),
            ),
            (
                Value::SimpleString("modules".to_string()),
                Value::Array(vec![]),
            ),
        ]))
    }
}
