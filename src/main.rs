pub mod lua;
pub mod menu;

use menu::{command::DBCommand, user_input::UserInput};
use std::collections::HashMap;

// HashMap não pode ser &str, pois precisamos guardar na memória
type KeyValues = HashMap<String, String>;

fn main() {
    let mut hashmap = KeyValues::new();
    println!("Comandos:");
    println!("- ADD -> Adiciona uma chave (Ex.: ADD chave valor)");
    println!("- GET -> Pega uma chave (Ex.: GET chave)");
    println!("- PRINT -> Printa o KV");
    println!("- HELP -> Mostra novamente os comandos");
    println!("- EXIT -> Termina a execução");
    loop {
        let input: UserInput = menu::prompt_user(None);
        // Aqui precisamos converter para &str
        match input.command {
            DBCommand::EXIT => {
                println!("Valeu falô!");
                break;
            }
            DBCommand::ADD => menu::add_key(&mut hashmap, input),
            DBCommand::GET => menu::get_key(&hashmap, input),
            DBCommand::HELP => menu::print_help(),
            DBCommand::PRINT => println!("{hashmap:?}"),
            DBCommand::ERROR => {
                println!("ERRO: Comando não reconhecido");
                println!("\tDica: HELP -> Mostra os comandos");
            }
        }
    }
}
