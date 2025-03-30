use redis::Value;

use crate::database::Database;

pub type CommandError = String;
pub trait Command {
    async fn execute(request: &[Value], database: Database) -> Result<Value, CommandError>;
}

mod del;
mod get;
mod hello;
mod ping;
mod set;

pub async fn execute_command(
    request: &redis::Value,
    database: Database,
) -> Result<Value, CommandError> {
    let invalid_command = Err("invalid command".to_string());
    match request {
        redis::Value::Array(command) => {
            if command.is_empty() {
                return invalid_command;
            }
            match command[0].clone() {
                redis::Value::BulkString(command_name) => {
                    match command_name.to_ascii_uppercase().as_slice() {
                        b"PING" => ping::PingCommand::execute(command, database).await,
                        b"SET" => set::SetCommand::execute(command, database).await,
                        b"GET" => get::GetCommand::execute(command, database).await,
                        b"HELLO" => hello::HelloCommand::execute(command, database).await,
                        b"DEL" => del::DelCommand::execute(command, database).await,
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
