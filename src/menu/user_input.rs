use super::command::DBCommand;

pub struct UserInput {
    pub command: DBCommand,
    pub options: Vec<String>,
}

impl UserInput {
    pub fn new(input: String) -> Self {
        let values: Vec<&str> = input.split_whitespace().collect();
        let command: DBCommand = match values[0].parse() {
            Ok(v) => v,
            Err(_) => DBCommand::ERROR,
        };
        let options = values[1..=(values.len() - 1)]
            .iter()
            .map(|v| v.to_string())
            .collect();
        Self { command, options }
    }
}
