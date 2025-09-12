pub mod command;
pub mod user_input;

use super::database::DBData;
use std::io;
use user_input::UserInput;

// TODO: Adicionar função LOAD para o LUA

pub fn prompt_user(delimiter: Option<&str>) -> UserInput {
    let mut input = String::new();
    print!("{}", delimiter.unwrap_or("> "));
    // Garante que o prompt seja exibido
    io::Write::flush(&mut io::stdout()).expect("flush failed!");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line.");
    UserInput::new(input)
}

pub fn print_help() {
    println!("ADD {{chave}} {{valor}} -> Adiciona uma chave");
    println!("GET {{chave}} -> Pega uma chave");
    println!("PRINT -> Printa o KV");
    println!("EXIT -> Termina a execução");
    println!("HELP -> Mostra essa mensagem");
}

pub fn add_key(hashmap: &mut DBData, input: UserInput) {
    if input.options.len() < 2 {
        println!("GET precisa de uma chave e um valor (Ex.: ADD nome thiago)");
        return;
    }
    let key = input.options[0].to_string();
    let value = &input.options[1..=(input.options.len() - 1)].join(" ");
    hashmap.insert(key.clone(), value.clone());
    println!("ADDED {key} = {value}")
}

pub fn get_key(hashmap: &DBData, input: UserInput) {
    if input.options.len() < 1 {
        println!("GET precisa de uma chave (Ex.: GET nome)");
        return;
    }
    let key = &input.options[0];
    let some_value = hashmap.get(key);
    match some_value {
        Some(value) => println!("{key} = {value}"),
        None => println!("ERRO: Chave não encontrada"),
    }
}
