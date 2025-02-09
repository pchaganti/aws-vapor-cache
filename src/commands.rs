use lambda_extension::tracing;
use redis::Value;

use crate::database::Database;

pub type CommandError = String;
pub trait Command {
    fn execute(request: &[Value], database: Database) -> Result<Value, CommandError>;
}

pub struct PingCommand;

impl Command for PingCommand {
    fn execute(request: &[Value], _: Database) -> Result<Value, CommandError> {
        tracing::info!("Executing 'ping' command");
        match request.len() {
            1 => Ok(Value::SimpleString("PONG".to_string())),
            2 => Ok(request[1].clone()),
            _ => Err("wrong number of arguments for 'ping' command".to_string()),
        }
    }
}

pub fn execute_command(request: &redis::Value, database: Database) -> Result<Value, CommandError> {
    let invalid_command = Err("invalid command".to_string());
    match request {
        redis::Value::Array(command) => {
            if command.is_empty() {
                return invalid_command;
            }
            match command[0].clone() {
                redis::Value::BulkString(command_name) => {
                    match command_name.to_ascii_uppercase().as_slice() {
                        b"PING" => PingCommand::execute(command, database),
                        _ => Err(format!(
                            "unsupported command '{}'",
                            String::from_utf8(command_name).unwrap_or("".to_string())
                        )),
                    }
                }
                _ => invalid_command,
            }
        }
        _ => invalid_command,
    }
}
