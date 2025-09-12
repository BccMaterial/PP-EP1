// HashMap não pode ser &str, pois precisamos guardar na memória
use super::lua::LuaExtension;
use std::collections::HashMap;

pub type DBData = HashMap<String, String>;
pub type LuaExtensions = HashMap<String, LuaExtension>;

#[allow(dead_code)]
pub struct Database {
    pub hashmap: DBData,
    pub extensions: LuaExtensions,
}

impl Database {
    pub fn new() -> Self {
        Self {
            hashmap: DBData::new(),
            extensions: LuaExtensions::new(),
        }
    }
}
