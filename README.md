# PP-EP1

EP-1 De paradigmas de programação.

Este EP é um pequeno banco de dados, que armazena pares chave-valor do Rust, junto com extensões em lua.

O EP possui dois binários:
- default
- example_hashmap

Vale ressaltar que tudo que está na pasta `examples` não servirá como feature, apenas como aprendizado.

## Requisitos

- Lua 5.4
- Rust >= 1.88

## Comandos do banco

O banco possui alguns comandos simples, sendo eles:
- ADD -> Adiciona uma chave (Ex.: ADD {chave} {valor});
- GET -> Retorna uma chave (Ex.: GET chave valor);
- PRINT -> Imprime informações (Ex.: PRINT data, PRINT extensions);
- LOAD -> Carrega uma extensão lua (Ex.: LOAD ./lua/cpf.lua);
- HELP -> Mostra um overview dos comandos;
- EXIT -> Finaliza a execução.

## Execuçao

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

Sua extensão deve possuir duas funções no escopo dos `globals`, um `add` e um `get`, onde:
- O `add` retorna um booleano ou joga um erro, dizendo se a entrada é válida;
- O `get` retorna `string, bool`, `string`, `bool` ou joga um erro.

> [!TIP]
> Caso o `get` retorne um booleano, não será alterado o valor.

## Exercício

Enunciado: https://crivelaro.notion.site/Banco-de-Dados-Rust-Lua-8517a7379cc94527be0147a21b504622


