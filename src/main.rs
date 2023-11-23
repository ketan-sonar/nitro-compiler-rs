mod tokenizer;
mod parser;
mod generator;

use std::{env::args, process::exit, fs::File, io::{Read, Write}};

use tokenizer::Tokenizer;

use crate::{parser::Parser, generator::Generator};

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() != 3 {
        eprintln!("Incorrect usage. Correct usage...\nntc <input.nt> <output.s>");
        exit(1);
    }

    let input_filename = &args[1];

    let mut input_file = File::open(input_filename)
        .unwrap_or_else(|err| {
            eprintln!("ERROR: could not open `{}`: {}", input_filename, err);
            exit(1);
        });

    let mut src = String::new();
    let _bytes_read = input_file.read_to_string(&mut src)
        .unwrap_or_else(|err| {
            eprintln!("ERROR: could not read from `{}`: {}", input_filename, err);
            exit(1);
        });

    let mut tokenizer = Tokenizer::new(src.into_bytes());
    let tokens = tokenizer.tokenize();
    dbg!(&tokens);

    let mut parser = Parser::new(tokens);
    let ast = parser.parse();
    dbg!(&ast);

    let mut generator = Generator::new(ast);
    let asm_string = generator.generate();
    dbg!(&asm_string);

    let output_filename = &args[2];
    let mut output_file = File::create(output_filename)
        .unwrap_or_else(|err| {
            eprintln!("ERROR: could not open `{}`: {}", output_filename, err);
            exit(1);
        });

    output_file.write_all(asm_string.as_bytes())
        .unwrap_or_else(|err| {
            eprintln!("ERROR: could not write to `{}`: {}", output_filename, err);
            exit(1);
        });
}
