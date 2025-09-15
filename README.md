# PP-EP1

EP-1 De paradigmas de programação

## Exercício

Enunciado: https://crivelaro.notion.site/Banco-de-Dados-Rust-Lua-8517a7379cc94527be0147a21b504622

## Requisitos

- Lua 5.4
- Rust >= 1.88

## Projeto

O projeto possui duas divisões:
- **rust-db:** Lida com as operações do banco de dados, é a engine do nosso banco
- **lua-extensions:** Diretório com scripts em lua, para o Rust usar

## rust-db

Motor do banco, que vai armazenar os pares chave-valor e as extensões em lua.

O rust-db possui dois binários:
- default
- example_hashmap

Vale ressaltar que tudo que está na pasta `examples` não servirá como feature, apenas o que está
na pasta `src`.

### Executar o projeto

Para executar o projeto, utilizamos:
```shell
cargo run
```

Para rodar algum código do diretório `examples`, utilizamos o seguinte comando:
```shell
cargo run --example hashmap
```
## Criação de uma extensão

Para criar uma extensão para o banco, deve-se seguir o template encontrado em [./lua/template.lua](https://github.com/BccMaterial/PP-EP1/blob/main/lua/template.lua).

Sua função deve possuir duas funções no escopo dos `globals`, um `add` e um `get`, onde:
- O `add` retorna um booleano ou joga um erro, dizendo se a entrada é válida
- O `get` retorna `string, bool`, `string` ou joga um erro
