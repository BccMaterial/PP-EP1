# PP-EP1

EP-1 De paradigmas de programação

## Exercício

Enunciado: https://crivelaro.notion.site/Banco-de-Dados-Rust-Lua-8517a7379cc94527be0147a21b504622

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
