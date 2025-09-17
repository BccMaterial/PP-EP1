use mlua::Error as LuaError;
use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::io::Error as IOError;

#[derive(Debug)]
pub enum DBError {
    FileReadError(String, IOError),
    ExtensionError(String, LuaError),
    LoadFuncError(String, String, LuaError),
    ExecFuncError(String, String, LuaError),
    ConversionError(String, String, LuaError),
    InvalidFuncReturn(String, String, String),
    ClearFuncError(String, String, LuaError),
    DataNotFound(String),
}

impl Display for DBError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::FileReadError(file_path, err) => {
                write!(f, "Erro ao ler o arquivo '{file_path}': {err}")
            }
            Self::ExtensionError(file_path, lua_err) => {
                write!(f, "Erro ao interpretar o script '{file_path}': {lua_err}")
            }
            Self::LoadFuncError(func_name, file_path, lua_err) => write!(
                f,
                "Erro ao carregar a função '{func_name}' na extensão '{file_path}': {lua_err}"
            ),
            Self::ExecFuncError(func_name, ext_name, lua_err) => write!(
                f,
                "Erro ao executar '{func_name}' com a extensão '{ext_name}': {lua_err}"
            ),
            Self::ConversionError(type_name, ext_name, lua_err) => write!(
                f,
                "Erro ao converter o retorno para '{type_name}' na extensão '{ext_name}': {lua_err}"
            ),
            Self::InvalidFuncReturn(func_name, ext_name, reason) => write!(
                f,
                "ERRO: O retorno do '{func_name}' da extensão '{ext_name}' é inválido ({reason})"
            ),
            Self::ClearFuncError(func_name, file_path, lua_err) => write!(
                f,
                "Erro ao limpar '{func_name}' na extensão '{file_path}': {lua_err}"
            ),
            Self::DataNotFound(key) => write!(f, "ERRO: Chave '{key}' não encontrada"),
        }
    }
}

impl Error for DBError {}
