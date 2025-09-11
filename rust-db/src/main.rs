use std::collections::HashMap;
use std::io;

// HashMap não pode ser &str, pois precisamos guardar na memória
type KeyValues = HashMap<String, String>;

fn prompt_user(delimiter: Option<&str>) -> String {
    let mut input = String::new();
    print!("{}", delimiter.unwrap_or("> "));
    // Garante que o prompt seja exibido
    io::Write::flush(&mut io::stdout()).expect("flush failed!");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line.");
    input.trim().to_string()
}

fn main() {
    let mut key_values = KeyValues::new();
    println!("Bem-vindo ao rust-db!");
    println!("GET -> Pega uma chave");
    println!("ADD -> Adiciona uma chave");
    println!("PRINT -> Printa o KV");
    println!("EXIT -> Termina a execução");
    loop {
        let user_input = prompt_user(None);
        let values: Vec<&str> = user_input.split_whitespace().collect();
        let command = values[0].to_uppercase();
        // Aqui precisamos converter para &str
        match command.as_str() {
            "ADD" => {
                if values.len() < 3 {
                    println!("GET precisa de uma chave e um valer (Ex.: ADD nome thiago)");
                    continue;
                }
                let key = values[1].to_string();
                let value = &values[2..=(values.len() - 1)].join(" ");
                key_values.insert(key.clone(), value.clone());
                println!("ADDED {key} = {value}")
            }
            "GET" => {
                if values.len() < 2 {
                    println!("GET precisa de uma chave (Ex.: GET nome)");
                    continue;
                }
                let key = values[1];
                let some_value = key_values.get(key);
                match some_value {
                    Some(value) => println!("{key} = {value}"),
                    None => println!("Not Found"),
                }
            }
            "PRINT" => println!("{key_values:?}"),
            "EXIT" => break,
            _ => {
                println!("Comando inválido.");
                println!("GET chave -> Pega uma chave");
                println!("ADD chave valor -> Adiciona uma chave");
                println!("PRINT -> Printa o KV");
                println!("EXIT -> Termina a execução");
            }
        }
    }
}
