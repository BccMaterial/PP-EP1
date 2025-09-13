use mlua;
use std::io::{Error, ErrorKind};
use std::path::Path;
use std::{collections::HashMap, fs};

fn get_lua_filename_no_ext(file_path: &str) -> String {
    let path = Path::new(file_path);
    path.file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("")
        .trim_end_matches(".lua")
        .to_string()
}

pub struct LuaExtension {
    pub get: mlua::Function,
    pub add: mlua::Function,
}

// HashMap não pode ser &str, pois precisamos guardar na memória
pub type DBData = HashMap<String, String>;
pub type DBExtensions = HashMap<String, LuaExtension>;

pub struct Database {
    pub hashmap: DBData,
    pub lua_vm: mlua::Lua,
    pub extensions_table: DBExtensions,
}

impl Database {
    pub fn new() -> Self {
        let lua_vm = mlua::Lua::new();
        let extensions_table: HashMap<String, LuaExtension> = HashMap::new();

        Self {
            hashmap: DBData::new(),
            lua_vm,
            extensions_table,
        }
    }

    pub fn add_extension(self: &mut Self, file_path: &str) -> Result<String, Error> {
        let lua_file = fs::read_to_string(file_path.to_string()).map_err(|file_err| {
            Error::new(
                file_err.kind(),
                format!("Erro ao abrir o arquivo:\n{file_err:?}"),
            )
        })?;

        let filename = get_lua_filename_no_ext(file_path);

        self.lua_vm.load(&lua_file).exec().map_err(|lua_err| {
            Error::new(
                ErrorKind::InvalidData,
                format!("Erro ao interpretar o script {filename}:\n{lua_err:?}"),
            )
        })?;

        let get_func: mlua::Function = self.lua_vm.globals().get("get").map_err(|lua_err| {
            Error::new(
                ErrorKind::InvalidData,
                format!("Erro ao carregar função get: {lua_err:?}"),
            )
        })?;

        let add_func: mlua::Function = self.lua_vm.globals().get("get").map_err(|lua_err| {
            Error::new(
                ErrorKind::InvalidData,
                format!("Erro ao carregar função add: {lua_err:?}"),
            )
        })?;

        self.extensions_table.insert(
            filename,
            LuaExtension {
                get: get_func,
                add: add_func,
            },
        );
        Ok(String::from("Extensão adicionada com sucesso!"))
    }
}
