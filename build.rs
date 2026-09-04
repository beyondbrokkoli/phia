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
    // CRITICAL: without this, cargo won't re-run build.rs for different test files
    println!("cargo:rerun-if-env-changed=PHIA_SOURCE");
    let source_path = env::var("PHIA_SOURCE").unwrap_or_else(|_| "main.lua".to_string());
    println!("cargo:rerun-if-changed={}", source_path);

    let source = std::fs::read_to_string(&source_path).expect("Failed to read source");
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
    let mut final_code = backend.generate_rust_code();

    // Regression stats: computed from the FINAL IR, grep-able via the binary.
    let (mut fs_, mut fg, mut ds, mut dg, mut ho, mut dep) = (0, 0, 0, 0, 0, 0i32);
    let mut ctx = String::new();
    for ins in &backend.ir {
        use ir::Instruction as I;
        match ins {
            I::SetTableFast { .. } => fs_ += 1,
            I::GetTableFast { .. } => fg += 1,
            I::SetTable { .. } => ds += 1,
            I::GetTable { .. } => dg += 1,
            I::HoistRawPtr { .. } => { ho += 1; ctx.push_str(&format!("{dep},")); }
            I::BeginWhile => dep += 1,
            I::EndWhile => dep -= 1,
            _ => {}
        }
    }
    final_code.push_str(&format!(
        "\npub const STATS: &str = \"fast_sets={fs_};fast_gets={fg};dyn_sets={ds};dyn_gets={dg};hoists={ho};hoist_ctx={}\";\n",
        ctx.trim_end_matches(',')
    ));

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("baked_native.rs");
    let mut f = BufWriter::new(File::create(&dest_path).unwrap());
    f.write_all(final_code.as_bytes()).unwrap();
}
