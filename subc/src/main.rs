#![warn(rustdoc::missing_crate_level_docs)]
extern crate pest;
#[macro_use]
extern crate pest_derive;

mod brackets;

use pest::Parser;

use std::env;
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct SubCParser;

/// Função principal do programa
///
/// # Panics
/// Se houver problemas na avaliação lexica o programa abortará com uma mensagem de erro
/// indicando o lugar onde aconteceu o erro no arquivo de entrada.
fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    // lendo o arquivo fonte a partir do caminho fornecido nos argumentos de command line
    let file_path = PathBuf::from(&args.get(1).expect("arquivo de entrada não foi especificado"));
    let sample_program = fs::read_to_string(file_path).unwrap();

    // testa se os (), [] e {} estão no mínimo balanceados
    match brackets::brackets_are_balanced(&sample_program) {
        brackets::IsBalanced::Balanced => (),
        brackets::IsBalanced::Unbalanced(position) => {
            panic!("Erro no balanceamento na posição {0}", position)
        }
    };

    // uma regra especial chamada lexer foi criada no intuito de formar uma corrente
    // de tokens válidos desconsiderando regras sintáticas (que serão posteriormente implementadas)
    let pairs = SubCParser::parse(Rule::lexer, &sample_program)
        .unwrap_or_else(|e| panic!("Erro na análise léxica:\n{}", e));

    // passa por todos os pares de regra/valor dos tokens reconhecidos
    for pair in pairs {
        // os pares são a combinação das regras e valores das strings que combinaram
        println!("Regra:    {:?}", pair.as_rule());
        println!("Span:    {:?}", pair.as_span());
        println!("Texto:    {}", pair.as_str());
    }


    println!("Arquivo de entrada está lexicamente correto!");
}
