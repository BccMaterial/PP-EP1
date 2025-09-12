# PP-EP1

EP-1 De paradigmas de programação

## Exercício

Enunciado: https://crivelaro.notion.site/Banco-de-Dados-Rust-Lua-8517a7379cc94527be0147a21b504622

## Projeto

O projeto possui duas divisões:
- **rust-db:** Lida com as operações do banco de dados, é a engine do nosso banco
- **lua-extensions:** Diretório com exemplos de scripts em lua, para o Rust usar

## rust-db

Motor do banco, que vai armazenar os pares chave-valor e as extensões.

### Binários

O rust-db possui dois binários:
- default
- example_hashmap

Vale ressaltar que tudo que está na pasta `examples` não servirá como feature, apenas o que está
na pasta `src`.

#### example_hashmap

O `example_hashmap` é um exemplo de código que utiliza alguns recursos do Rust, como enumeráveis, 
o próprio `hashmap`, e `struct`. O objetivo desse binário é somente aprendizado.

#### default

É o projeto do motor do banco em Rust em si.

### Executar o projeto

Para executar o projeto, é possível executar:
```shell
cargo run
```

Caso seja necessário executar algum exemplo, podemos executar da seguinte forma:
```shell
cargo run --example hashmap
```

> [!TIP]
> É possível ver quais binários são executáveis no `Cargo.toml` do `rust-db`
