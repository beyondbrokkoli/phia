// build.rs
use logos::Logos;
use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

#[path = "src/compiler.rs"]
pub mod compiler;
#[path = "src/ir.rs"]
pub mod ir;
#[path = "src/lexer.rs"]
pub mod lexer;
#[path = "src/memory.rs"]
pub mod memory;

fn main() {
    println!("cargo:rerun-if-changed=test.lua");
    println!("cargo:rerun-if-changed=src/ir.rs");

    let source_code = std::fs::read_to_string("test.lua").expect("Failed to read test.lua");
    let tokens: Vec<lexer::Token> = lexer::Token::lexer(&source_code)
        .filter_map(|res| res.ok())
        .collect();

    let backend = ir::IrBackend::new();
    let mut comp = compiler::Compiler::new(tokens, backend);

    while !comp.is_done() {
        comp.compile_stmt();
    }

    let final_code = comp.backend.generate_rust_code();

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("baked_native.rs");
    let mut f = BufWriter::new(File::create(&dest_path).unwrap());
    f.write_all(final_code.as_bytes()).unwrap();
}
