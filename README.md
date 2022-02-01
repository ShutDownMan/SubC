# SubC

Trabalho de Compiladores

O programa atualmente checka se o arquivo fonte fornecido
segue as regras lexicais da linguagem C.

Mostrando na tela os tokens mais relevantes encontrados se
nenhum erro for encontrado.

Este também verifica o balanceamento dos (), [] e {}.

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
