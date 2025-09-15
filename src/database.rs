use mlua;
use std::io::{Error, ErrorKind};
use std::{collections::HashMap, fs};

// Usado para simplificar o Err(Error::new()) em algumas partes do código
fn create_error(kind: ErrorKind, message: String) -> Result<String, Error> {
    return Err(Error::new(kind, message));
}

pub struct LuaExtension {
    pub get: mlua::Function,
    pub add: mlua::Function,
}

// HashMap não pode ser &str, pois precisamos guardar na memória
pub type DBData = HashMap<String, String>;
pub type DBExtensions = HashMap<String, LuaExtension>;

pub struct Database {
    pub data: DBData,
    pub lua_vm: mlua::Lua,
    pub extensions: DBExtensions,
}

impl Database {
    pub fn new() -> Self {
        let lua_vm = mlua::Lua::new();
        let extensions_hashmap: HashMap<String, LuaExtension> = HashMap::new();

        Self {
            data: DBData::new(),
            lua_vm,
            extensions: extensions_hashmap,
        }
    }

    pub fn add_extension(self: &mut Self, file_path: &str) -> Result<String, Error> {
        let lua_file = fs::read_to_string(file_path.to_string()).map_err(|file_err| {
            Error::new(
                file_err.kind(),
                format!("Erro ao abrir o arquivo {file_path}: {file_err}"),
            )
        })?;

        self.lua_vm.load(&lua_file).exec().map_err(|lua_err| {
            Error::new(
                ErrorKind::InvalidData,
                format!("Erro ao interpretar o script {file_path}: {lua_err}"),
            )
        })?;

        let get_func: mlua::Function = self.lua_vm.globals().get("get").map_err(|lua_err| {
            Error::new(
                ErrorKind::InvalidData,
                format!("Erro ao carregar função get: {lua_err}"),
            )
        })?;

        let add_func: mlua::Function = self.lua_vm.globals().get("add").map_err(|lua_err| {
            Error::new(
                ErrorKind::InvalidData,
                format!("Erro ao carregar função add: {lua_err}"),
            )
        })?;

        self.extensions.insert(
            file_path.to_string(),
            LuaExtension {
                get: get_func,
                add: add_func,
            },
        );

        Ok(String::from("Extensão adicionada com sucesso!"))
    }

    pub fn get_data(self: &Self, key: &str) -> Result<String, Error> {
        let value: String = match self.data.get(key) {
            Some(v) => v.to_string(),
            None => {
                return create_error(
                    ErrorKind::NotFound,
                    String::from("Erro: Chave não encontrada"),
                );
            }
        };

        for (ext_name, ext_funcs) in &self.extensions {
            let result: mlua::MultiValue =
                ext_funcs
                    .get
                    .call((key, value.as_str()))
                    .map_err(|lua_err| {
                        Error::new(
                            ErrorKind::InvalidData,
                            format!(
                                "Erro ao executar \"get\" com a extensão {ext_name}: {lua_err}"
                            ),
                        )
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
                                return Err(Error::new(
                                    ErrorKind::InvalidData,
                                    format!("Erro ao converter o retorno para string: {lua_err}"),
                                ));
                            }
                        }
                    } else if lua_value.is_boolean() {
                        if lua_value.as_boolean().unwrap_or(false) {
                            continue;
                        }
                    } else {
                        return Err(Error::new(
                            ErrorKind::InvalidData,
                            format!(
                                "ERRO: O retorno do \"get\" da extensão {ext_name} não é uma string"
                            ),
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
                                return create_error(
                                    ErrorKind::InvalidData,
                                    format!("Erro ao converter o retorno para string: {lua_err}"),
                                );
                            }
                        }
                    } else {
                        return create_error(
                            ErrorKind::InvalidData,
                            format!(
                                "ERRO: Os retornos do \"get\" da extensão {ext_name} não seguem o formato string, bool"
                            ),
                        );
                    }
                }
                _ => {
                    return create_error(
                        ErrorKind::InvalidData,
                        format!(
                            "Erro ao executar \"get\" com a extensão {ext_name}: Quantidade de retornos da função inválida (Deve retornar string, bool ou string)"
                        ),
                    );
                }
            }
        }

        Ok(value)
    }

    pub fn add_data(self: &mut Self, data: (&str, &str)) -> Result<String, Error> {
        for (ext_name, ext_funcs) in &self.extensions {
            let result: mlua::MultiValue = ext_funcs
                .add
                .call((data.0.to_string(), data.1.to_string()))
                .map_err(|lua_err| {
                    Error::new(
                        ErrorKind::InvalidData,
                        format!("Erro ao executar \"add\" com a extensão {ext_name}: {lua_err}"),
                    )
                })?;

            if result.len() != 1 {
                return Err(Error::new(
                    ErrorKind::InvalidData,
                    format!(
                        "Erro ao executar \"add\" com a extensão {ext_name}: Quantidade de retornos da função inválida (Deve retornar apenas bool)"
                    ),
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
