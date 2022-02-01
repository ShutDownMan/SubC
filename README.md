# SubC

Trabalho de Compiladores

Nome dos integrantes da equipe: 

Jedson Gabriel Ferreira de Paula
e Pedro Sergio de Lima Junior.

O programa atualmente checka se o arquivo fonte fornecido
segue as regras lexicais da linguagem C.

Mostrando na tela os tokens mais relevantes encontrados se
nenhum erro for encontrado.

Este também verifica o balanceamento dos (), [] e {}.

Se encontrado algum erro lexical em algum dos tokens,
é apontado a posição da linha e coluna que o erro se situa.

## Compilação e Execução do Programa

Para compilar será necessário ter instalado a utilidade `cargo` e
executar o seguinte comando na pasta subc:

```bash
cargo build --release
```

O binário será criado a partir de suas dependências

Agora utilizando o binário dentro da pasta `target` ou novamente o cargo
você pode executar na linha de comando:

```bash
cargo run caminho_arquivo_fonte.subc
OU
./target/release/subc caminho_arquivo_fonte.subc
```

## Referências

Código para balanceamento dos parênteses.
https://codereview.stackexchange.com/questions/253279/matching-brackets-in-rust

Biblioteca de parsing utilizada:
https://docs.rs/pest/2.1.3/pest/index.html
https://pest.rs/book/

