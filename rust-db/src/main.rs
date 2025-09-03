use std::collections::HashMap;

// Criação do tipo CPF
#[derive(Debug)]
struct Cpf(u128);

impl Cpf {
    fn new(digits: u128) -> Option<Self> {
        if digits < 10_000_000_000 || digits > 99_999_999_999 {
            None
        } else {
            Some(Self(digits))
        }
    }

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
    let mut user_data: HashMap<&str, DBTypes> = HashMap::new();

    user_data.insert("name", DBTypes::String(String::from("Jorge")));
    user_data.insert("age", DBTypes::Int(32));
    user_data.insert("cpf", DBTypes::Cpf(Cpf::new(12345678900).unwrap()));

    // O copied transforma o Option<&T> em Option<T>
    for (key, value) in &user_data {
        match value {
            DBTypes::String(s) => println!("{key}: {s}"),
            DBTypes::Int(i) => println!("{key}: {i}"),
            DBTypes::Cpf(cpf) => println!("{}: {:?}", key, cpf.to_string()),
        }
    }
}
