/* TODO:
 * - Função pra carregar arquivo
 * - Função pra executar função
 * - Hashmap com as funções
 * DICA: Seria interessante reconhecer a chave por regex, para aplicar a função
 * */

/*
 * Estrutura das nossas extensões
 * Teremos um HashMap <String, LuaExtension>
 * -> Pattern: é o regex que queremos detectar
 * no hashmap do banco (Exemplo: cpf_* pra
 * aplicar o código em qualquer chave que começa com "cpf_")
 * -> Code: Filepath do código em Lua
 * */

// IMPORTANTE: Remover quando começar a desenvolver
#![allow(dead_code)]

pub enum ExtensionType {
    READ = 0,
    WRITE = 1,
}

pub struct LuaExtension {
    regex_pattern: String,
    code_path: String,
    ext_type: ExtensionType,
}

impl LuaExtension {
    pub fn new() {
        panic!("new não implementado")
    }

    // Lemos o self.code
    // Retornamos uma String com o código
    pub fn read_code() {
        panic!("read_code não implementado")
    }

    // Chamamos o read_code
    // Executamos o código
    // Deve retornar um Result,
    // com o retorno do Lua, ou com Erro do Lua
    pub fn execute_code() {
        panic!("execute_code não implementado")
    }

    // Por que duas funções?
    // Caso queiramos mostrar o código importado
    // Se não usarmos o read_code, inserimos ele no execute_code
}
