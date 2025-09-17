pub mod error;
pub mod util;

use error::DBError;
use mlua;
use std::fs;
use util::{DBData, DBExtensions, LuaExtension};

pub struct Database {
    pub data: DBData,
    pub lua_vm: mlua::Lua,
    pub extensions: DBExtensions,
}

impl Database {
    pub fn new() -> Self {
        Self {
            data: DBData::new(),
            lua_vm: mlua::Lua::new(),
            extensions: DBExtensions::new(),
        }
    }

    pub fn add_extension(&mut self, file_path: &str) -> Result<String, DBError> {
        let lua_file = fs::read_to_string(file_path)
            .map_err(|file_err| DBError::FileReadError(file_path.to_string(), file_err))?;

        self.lua_vm
            .load(&lua_file)
            .exec()
            .map_err(|lua_err| DBError::ExtensionError(file_path.to_string(), lua_err))?;

        let get_func: mlua::Function = self.lua_vm.globals().get("get").map_err(|lua_err| {
            DBError::LoadFuncError(String::from("get"), file_path.to_string(), lua_err)
        })?;

        let add_func: mlua::Function = self.lua_vm.globals().get("add").map_err(|lua_err| {
            DBError::LoadFuncError(String::from("add"), file_path.to_string(), lua_err)
        })?;

        self.extensions.insert(
            file_path.to_string(),
            LuaExtension {
                get: get_func,
                add: add_func,
            },
        );

        // Limpamos as variáveis, para conseguirmos validar em outras extensões se "add" ou "get"
        // existem
        self.lua_vm
            .globals()
            .set("get", mlua::Nil)
            .map_err(|lua_err| {
                DBError::ClearFuncError(String::from("get"), file_path.to_string(), lua_err)
            })?;

        self.lua_vm
            .globals()
            .set("add", mlua::Nil)
            .map_err(|lua_err| {
                DBError::ClearFuncError(String::from("add"), file_path.to_string(), lua_err)
            })?;

        Ok(String::from("Extensão adicionada com sucesso!"))
    }

    pub fn get_data(&self, key: &str) -> Result<String, DBError> {
        let value: String = match self.data.get(key) {
            Some(v) => v.to_string(),
            None => {
                return Err(DBError::DataNotFound(String::from(key)));
            }
        };

        for (ext_name, ext_funcs) in &self.extensions {
            let result: mlua::MultiValue =
                ext_funcs
                    .get
                    .call((key, value.as_str()))
                    .map_err(|lua_err| {
                        DBError::ExecFuncError(String::from("get"), ext_name.to_string(), lua_err)
                    })?;

            let results_vec = result.into_vec();

            // Se retorna só string, consideramos como true
            // Se retorna um booleano, validamos se é true
            match results_vec.len() {
                1 => {
                    let lua_value = &results_vec[0];
                    if lua_value.is_string() {
                        match lua_value.to_string() {
                            Ok(s) => return Ok(s),
                            Err(lua_err) => {
                                return Err(DBError::ConversionError(
                                    String::from("string"),
                                    ext_name.to_string(),
                                    lua_err,
                                ));
                            }
                        }
                    } else if lua_value.is_boolean() {
                        // Caso o get seja um bool, apenas paramos o loop
                        // MOTIVO: Sempre paramos a iteração na primeira extensão
                        // que for válida, para garantir performance
                        if lua_value.as_boolean().unwrap_or(false) {
                            break;
                        }
                    } else {
                        return Err(DBError::InvalidFuncReturn(
                            String::from("get"),
                            ext_name.to_string(),
                            String::from("deve ser string + bool, string ou bool"),
                        ));
                    }
                }
                2 => {
                    let lua_value = &results_vec[0];
                    let passed = &results_vec[1];
                    if lua_value.is_string() && passed.is_boolean() {
                        if !passed.as_boolean().unwrap_or(false) {
                            continue;
                        }
                        match lua_value.to_string() {
                            Ok(s) => return Ok(s),
                            Err(lua_err) => {
                                return Err(DBError::ConversionError(
                                    String::from("string"),
                                    ext_name.to_string(),
                                    lua_err,
                                ));
                            }
                        }
                    } else {
                        return Err(DBError::InvalidFuncReturn(
                            String::from("get"),
                            ext_name.to_string(),
                            String::from("deve ser string + bool, string ou bool"),
                        ));
                    }
                }
                _ => {
                    return Err(DBError::InvalidFuncReturn(
                        String::from("get"),
                        ext_name.to_string(),
                        String::from("deve ser string + bool, string ou bool"),
                    ));
                }
            }
        }

        Ok(value)
    }

    pub fn add_data(&mut self, data: (&str, &str)) -> Result<String, DBError> {
        for (ext_name, ext_funcs) in &self.extensions {
            let result: mlua::MultiValue = ext_funcs
                .add
                .call((data.0.to_string(), data.1.to_string()))
                .map_err(|lua_err| {
                    DBError::ExecFuncError(String::from("add"), ext_name.to_string(), lua_err)
                })?;

            if result.len() != 1 {
                return Err(DBError::InvalidFuncReturn(
                    String::from("add"),
                    ext_name.to_string(),
                    String::from("deve retornar bool"),
                ));
            }

            if result[0].as_boolean().unwrap_or(false) {
                break;
            }
        }

        // Uso do match por conta do insert:
        // Se tinha valor na key, retorna Some(valor)
        // Caso contrário, retorna "None"
        match self.data.insert(data.0.to_string(), data.1.to_string()) {
            Some(value) => {
                return Ok(String::from(format!(
                    "UPDATED {} = {} (was {value})",
                    data.0, data.1
                )));
            }
            None => {
                return Ok(String::from(format!("ADDED {} = {}", data.0, data.1)));
            }
        };
    }
}
