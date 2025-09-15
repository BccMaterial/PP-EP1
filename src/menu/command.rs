use std::io::{Error, ErrorKind};
use std::str::FromStr;

fn new_error(kind: ErrorKind, message: &str) -> Result<DBCommand, Error> {
    return Err(Error::new(kind, message));
}

// Comandos suportados
pub enum DBCommand {
    ERROR = -1,
    EXIT = 0,
    ADD = 1,
    GET = 2,
    PRINT = 3,
    LOAD = 4,
    HELP = 5,
}

impl FromStr for DBCommand {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "ADD" => Ok(DBCommand::ADD),
            "GET" => Ok(DBCommand::GET),
            "PRINT" => Ok(DBCommand::PRINT),
            "HELP" => Ok(DBCommand::HELP),
            "LOAD" => Ok(DBCommand::LOAD),
            "EXIT" => Ok(DBCommand::EXIT),
            _ => new_error(ErrorKind::NotFound, "ERRO: Comando não encontrado."),
        }
    }
}
