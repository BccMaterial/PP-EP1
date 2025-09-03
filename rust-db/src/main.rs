use std::collections::HashMap;

fn main() {
    let mut map: HashMap<&str, u8> = HashMap::new();

    map.insert("id", 1);
    map.insert("count", 28);

    // O copied transforma o Option<&T> em Option<T>
    let count = map.get("count").copied().unwrap_or(0);

    println!("{map:?}");
    println!("Count: {count}");
}
