use std::env;
use std::fs;
use std::io;
use std::io::Read;
use std::path::PathBuf;

mod ast;
mod compiler;
mod lexer;
mod optimizer;
mod parser;

use compiler::Compiler;
use parser::Parser;

fn main() {
    let (input, output_target) = read_input_and_output();
    let mut parser = Parser::new(&input);
    let program = match parser.parse_program() {
        Ok(program) => program,
        Err(err) => {
            eprintln!("parse error: {err}");
            std::process::exit(1);
        }
    };

    let mut compiler = Compiler::new(program);
    let asm = match compiler.compile() {
        Ok(asm) => asm,
        Err(err) => {
            eprintln!("compile error: {err}");
            std::process::exit(1);
        }
    };

    match output_target {
        OutputTarget::Stdout => {
            print!("{asm}");
        }
        OutputTarget::File(path) => {
            if let Err(err) = fs::write(&path, asm) {
                eprintln!("failed to write {}: {err}", path.display());
                std::process::exit(1);
            }
        }
    }
}

enum OutputTarget {
    Stdout,
    File(PathBuf),
}

fn read_input_and_output() -> (String, OutputTarget) {
    let mut args = env::args().skip(1);
    let input_path = args.next().unwrap_or_else(|| {
        eprintln!("usage: fovia-c <input.fv> [output.fvo]");
        std::process::exit(1);
    });

    let output_target = if let Some(path) = args.next() {
        if path == "-" {
            OutputTarget::Stdout
        } else {
            OutputTarget::File(PathBuf::from(path))
        }
    } else {
        let mut out = PathBuf::from(&input_path);
        out.set_extension("fvo");
        OutputTarget::File(out)
    };

    let input = if input_path == "-" {
        let mut buf = String::new();
        io::stdin()
            .read_to_string(&mut buf)
            .expect("failed to read stdin");
        buf
    } else {
        fs::read_to_string(&input_path).expect("failed to read source file")
    };

    (input, output_target)
}
