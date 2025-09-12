use std::str::FromStr;

// DISCLAIMER: Fiz isso pra brincar com ENUM
pub struct ParseDBCommandError;

// TODO: Adicionar comando LUA, p/ carregar extensão
pub enum DBCommand {
    ERROR = -1,
    EXIT = 0,
    ADD = 1,
    GET = 2,
    PRINT = 3,
    HELP = 4,
}

impl FromStr for DBCommand {
    type Err = ParseDBCommandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "ADD" => Ok(DBCommand::ADD),
            "GET" => Ok(DBCommand::GET),
            "PRINT" => Ok(DBCommand::PRINT),
            "HELP" => Ok(DBCommand::HELP),
            "EXIT" => Ok(DBCommand::EXIT),
            _ => Err(ParseDBCommandError),
        }
    }
}
