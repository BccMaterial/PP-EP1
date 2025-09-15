pub mod command;
pub mod user_input;

use super::database::Database;
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
        .expect("ERRO: Falha ao ler a linha");
    UserInput::new(input)
}

pub fn print_help() {
    println!("ADD {{chave}} {{valor}} -> Adiciona uma chave");
    println!("GET {{chave}} -> Pega uma chave");
    println!("PRINT -> Printa os dados ou as extensões (Valores: [data]/extensions)");
    println!("LOAD {{caminho}} -> Carrega uma extensão");
    println!("EXIT -> Termina a execução");
    println!("HELP -> Mostra essa mensagem");
}

pub fn add_key(db: &mut Database, input: UserInput) {
    if input.options.len() < 2 {
        println!("ADD precisa de uma chave e um valor (Ex.: ADD nome thiago)");
        return;
    }
    // TODO:
    // for extension in db.extensions (Type = Write)
    // if regex match -> Apply
    let key = input.options[0].clone();
    let value = input.options[1..=(input.options.len() - 1)].join(" ");
    match db.add_data((&key, &value)) {
        Ok(value) => println!("{value}"),
        Err(err) => println!("{err}"),
    }
}

pub fn get_key(db: &Database, input: UserInput) {
    if input.options.len() < 1 {
        println!("GET precisa de uma chave (Ex.: GET nome)");
        return;
    }
    let key = &input.options[0];
    match db.get_data(key) {
        Ok(value) => println!("{key} = {value}"),
        Err(err) => println!("{err}"),
    }
}

pub fn print_info(db: &Database, input: UserInput) {
    if input.options.len() < 1 {
        println!("{:?}", db.data);
        return;
    }
    let argument = &input.options[0].to_string().to_uppercase();

    match argument.as_str() {
        "DATA" => println!("{:?}", db.data),
        "EXTENSIONS" => println!("{:?}", db.extensions.keys()),
        _ => println!("Erro: argumento {argument} não suportado (utilize data ou extensions)"),
    }
}

pub fn load_extension(db: &mut Database, input: UserInput) {
    if input.options.len() < 1 {
        println!("LOAD precisa de um caminho de arquivo (Ex.: ./lua/cpf.lua)");
        return;
    }

    let file_path = &input.options[0];
    match db.add_extension(file_path) {
        Ok(msg) => println!("{msg}"),
        Err(err) => println!("{err}"),
    };
}
