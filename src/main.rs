pub mod database;
pub mod menu;

use database::Database;
use menu::{command::DBCommand, user_input::UserInput};

fn main() {
    let mut db = Database::new();
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
            DBCommand::ADD => menu::add_key(&mut db.hashmap, input),
            DBCommand::GET => menu::get_key(&db.hashmap, input),
            DBCommand::HELP => menu::print_help(),
            DBCommand::PRINT => println!("{:?}", db.hashmap),
            DBCommand::ERROR => {
                println!("ERRO: Comando não reconhecido");
                println!("\tDica: HELP -> Mostra os comandos");
            }
        }
    }
}
