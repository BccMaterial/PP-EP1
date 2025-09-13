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

// HashMap não pode ser &str, pois precisamos guardar na memória
pub type DBData = HashMap<String, String>;

#[allow(dead_code)]
pub struct Database {
    pub hashmap: DBData,
    pub lua_vm: mlua::Lua,
    pub extensions_table: mlua::Table,
}

impl Database {
    pub fn new() -> Self {
        let lua_vm = mlua::Lua::new();
        let extensions_table = lua_vm.create_table().expect("Could not create Lua table.");

        Self {
            hashmap: DBData::new(),
            lua_vm,
            extensions_table,
        }
    }

    pub fn add_extension(self: &mut Self, file_path: &str) -> Result<(), Error> {
        let lua_file = fs::read_to_string(file_path.to_string())
            .map_err(|_| Error::new(ErrorKind::NotFound, "File not found"))?;

        let filename = get_lua_filename_no_ext(file_path);

        let _result = self
            .lua_vm
            .load(&lua_file)
            .exec()
            .map_err(|err| Error::new(ErrorKind::InvalidData, err.to_string()))?;

        let get_func: mlua::Function = self
            .lua_vm
            .globals()
            .get("get")
            .expect("Função get não encontrada");

        let add_func: mlua::Function = self
            .lua_vm
            .globals()
            .get("get")
            .expect("Função add não encontrada");

        let inner_table = self
            .lua_vm
            .create_table()
            .expect("Falha ao criar tabela interna");

        inner_table
            .set("get", get_func)
            .expect("Erro ao adicionar get");
        inner_table
            .set("add", add_func)
            .expect("Erro ao adicionar add");

        self.extensions_table
            .set(filename, inner_table)
            .expect("Erro ao adicionar extensão");
        Ok(())
    }
}
