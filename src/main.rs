use io::Write;
use std::fs::File;
use std::io;
use std::{error::Error, io::Read};

use compiler::lexer::Lexer;

use clap::Parser;
use compiler::lexer;

#[derive(Parser, Debug)]
#[command(author = "Oscar M", version = "0.1.0beta", about = "FuckScript compiler that compiles to Brainf*ck", long_about = None)]
struct Args {
    /// The input file to be compiled
    #[arg()]
    input: Option<std::path::PathBuf>,

    /// Prefer loops over constant values
    #[arg(long, short)]
    prefeer_loops: bool,

    #[arg(value_enum, long, default_value_t = CompileMode::None)]
    compile_mode: CompileMode,

    /// Output file path
    #[arg(default_value = Some("output.bf"), short, long)]
    output: Option<std::path::PathBuf>,
}
#[derive(Debug, clap::ValueEnum, Clone)]
enum CompileMode {
    None,
    Sizem,
    Speed,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    match args.input {
        Some(input) => {
            let file = File::open(input.clone());
            match file {
                Ok(mut file) => {
                    let mut buffer = String::new();
                    file.read_to_string(&mut buffer)?;
                    let mut lexer = Lexer::new(
                        input.as_path().to_str().unwrap().to_owned(),
                        buffer,
                    );
                    let tokens = lexer.make_tokens();
                    tokens2parser(tokens)
                }
                Err(err) => Err(Box::new(io::Error::new(
                    io::ErrorKind::Other,
                    err.to_string(),
                ))),
            }
        }
        None => {
            println!("Using REPL");
            let mut buffer = String::new();
            let stdin = io::stdin();
            loop {
                print!("repl> ");
                io::stdout().flush()?;
                stdin.read_line(&mut buffer)?;
                let mut lexer = Lexer::new("stdout".to_owned(), buffer.clone());
                let tokens = lexer.make_tokens();
                tokens2parser(tokens)?;
                buffer.clear();
            }
        }
    }
}

fn tokens2parser(tokens: Result<Vec<lexer::Token>, compiler::errors::LexerError>) -> Result<(), Box<dyn Error>> {
    return match tokens {
        Ok(tokens) => {
            let mut parser = compiler::parser::Parser::new(tokens);
            let bin = parser.parse();
            println!("{:?}", bin);
            Ok(())
        }
        Err(err) => {
            println!("Error: {}", err);
            Ok(())
        }
    }
}