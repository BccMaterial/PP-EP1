use std::collections::HashMap;

pub struct LuaExtension {
    pub get: mlua::Function,
    pub add: mlua::Function,
}

// HashMap não pode ser &str, pois precisamos guardar na memória
pub type DBData = HashMap<String, String>;
pub type DBExtensions = HashMap<String, LuaExtension>;
