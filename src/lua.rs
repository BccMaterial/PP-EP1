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
 * no hashmap do banco (Exemplo: cpf_* pra aplicar o código em qualquer extensão)
 * -> Code: Filepath do código em Lua
 * */

#![allow(dead_code)]

pub struct LuaExtension {
    pattern: String,
    code: String,
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
    pub fn execute_code() {
        panic!("execute_code não implementado")
    }

    // Por que duas funções?
    // Caso queiramos mostrar o código importado
    // Se não usarmos o read_code, inserimos ele no execute_code
}
