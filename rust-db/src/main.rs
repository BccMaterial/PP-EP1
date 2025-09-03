use std::collections::HashMap;

// Criação do tipo CPF
#[derive(Debug)]
struct Cpf(u128);

impl Cpf {
    fn new(digits: u128) -> Option<Self> {
        // Disclaimer: O ideal é retornar um Result no lugar do Option, mostrando o erro que deu
        if digits < 10_000_000_000 || digits > 99_999_999_999 {
            None
        } else {
            Some(Self(digits))
        }
    }

    // Formatação de CPF
    fn to_string(&self) -> String {
        format!("{:011}", self.0)
    }
}

// Valores que suportamos no banco
#[derive(Debug)]
enum DBTypes {
    String(String),
    Int(usize),
    Cpf(Cpf),
}

fn main() {
    println!("Criando HashMap...");
    let mut user_data: HashMap<&str, DBTypes> = HashMap::new();

    println!("Inserindo um dado...");
    user_data.insert("name", DBTypes::String(String::from("Jorge")));
    user_data.insert("age", DBTypes::Int(32));
    // Como é um Option, precisamos usar o "unwrap"
    user_data.insert("cpf", DBTypes::Cpf(Cpf::new(12345678900).unwrap()));

    println!("Exemplo de dado:");
    for (key, value) in user_data {
        match value {
            DBTypes::String(s) => println!("{key}: {s}"),
            DBTypes::Int(i) => println!("{key}: {i}"),
            DBTypes::Cpf(cpf) => println!("{}: {:?}", key, cpf.to_string()),
        }
    }
}
