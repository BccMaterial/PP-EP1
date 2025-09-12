use std::str::FromStr;

// DISCLAIMER: Fiz isso pra brincar com ENUM
pub struct ParseDBCommandError;

pub enum DBCommand {
    ERROR = -1,
    ADD = 0,
    GET = 1,
    PRINT = 2,
    HELP = 3,
    EXIT = 4,
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
