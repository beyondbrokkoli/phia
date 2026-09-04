// build.rs
use logos::Logos;
use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

#[path = "src/lexer.rs"] pub mod lexer;
#[path = "src/ast.rs"] pub mod ast;
#[path = "src/parser.rs"] pub mod parser;
#[path = "src/type_checker.rs"] pub mod type_checker;
#[path = "src/lowerer.rs"] pub mod lowerer;
#[path = "src/ir.rs"] pub mod ir;

fn main() {
    println!("cargo:rerun-if-changed=main.lua");

    let source = std::fs::read_to_string("main.lua").expect("Failed to read main.lua");
    let mut tokens = Vec::new();
    let mut lexer = lexer::Token::lexer(&source);

    while let Some(res) = lexer.next() {
        match res {
            Ok(token) => tokens.push(token),
            Err(_) => {
                let span = lexer.span();
                let snippet = &source[span.clone()];
                panic!(
                    "Lexer Error: Unrecognized token or invalid literal '{}' at bytes {:?}", 
                    snippet, span
                );
            }
        }
    }

    // 1. AST Generation
    let mut parser = parser::Parser::new(tokens);
    let ast = parser.parse_program();

    // 2. Semantic Analysis & Type Checking
    let mut checker = type_checker::TypeChecker::new();
    checker.check_program(&ast);

    // 3. IR Lowering
    let mut lowerer = lowerer::IrLowerer::new();
    lowerer.lower_program(&ast);

    // 4. Optimization
    let mut backend = ir::IrBackend::new();
    backend.ir = lowerer.ir; // Transfer the flat IR
    backend.optimize();

    // 5. Code Generation
    let final_code = backend.generate_rust_code();

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("baked_native.rs");
    let mut f = BufWriter::new(File::create(&dest_path).unwrap());
    f.write_all(final_code.as_bytes()).unwrap();
}
