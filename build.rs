// build.rs
use logos::Logos;
use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use std::process::Command;

// LINEAR PIPELINE:
#[path = "src/lexer.rs"] pub mod lexer;               // 1. Text to Tokens
#[path = "src/ast.rs"] pub mod ast;                   // 2. AST Data Definitions
#[path = "src/parser.rs"] pub mod parser;             // 3. Tokens to AST
#[path = "src/type_checker.rs"] pub mod type_checker; // 4. AST Validation
#[path = "src/ir.rs"] pub mod ir;                     // 5. IR Data Definitions
#[path = "src/lowerer.rs"] pub mod lowerer;           // 6. AST to IR
#[path = "src/backend.rs"] pub mod backend;           // 7. IR to Rust (Optimize & Codegen)

fn main() {
    // Cargo will re-run build.rs for different test files
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

    // 3. IR Lowering (Now returns an IrProgram)
    let lowerer = lowerer::IrLowerer::new();
    let ir_program = lowerer.lower_program(&ast);

    // 4. Optimization & De-SSA
    let mut backend_engine = backend::IrBackend::new(ir_program);
    backend_engine.optimize();
    backend_engine.resolve_phis();
    backend_engine.simplify();            // NEW: copy-prop + dead-code elimination
    backend_engine.allocate_registers(); // NEW: linear-scan register allocation

    // 5. Code Generation
    let mut final_code = backend_engine.generate_rust_code();

    // Regression stats: computed from the FINAL CFG
    let (mut fs_, mut fg, mut ds, mut dg, mut ho) = (0, 0, 0, 0, 0);
    let mut ctx_list = Vec::new();

    for block in &backend_engine.program.blocks {
        for ins in &block.instrs {
            use ir::Instruction as I;
            match ins {
                I::SetTableFast { .. } => fs_ += 1,
                I::GetTableFast { .. } => fg += 1,
                I::SetTable { .. } => ds += 1,
                I::GetTable { .. } => dg += 1,
                I::HoistRawPtr { .. } => {
                    ho += 1;
                    // Pre-header depth exactly matches hoist context!
                    ctx_list.push(block.depth.to_string());
                }
                _ => {}
            }
        }
    }

    let hoist_ctx = ctx_list.join(",");
    final_code.push_str(&format!(
        "\npub const STATS: &str = \"fast_sets={fs_};fast_gets={fg};dyn_sets={ds};dyn_gets={dg};hoists={ho};hoist_ctx={hoist_ctx}\";\n"
    ));

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("baked_native.rs");

    // Write the raw, unformatted code to the file
    let mut f = BufWriter::new(File::create(&dest_path).unwrap());
    f.write_all(final_code.as_bytes()).unwrap();
    f.into_inner().unwrap(); // Ensure the file is completely flushed and closed before rustfmt reads it

    // Run rustfmt directly on the generated file
    let status = Command::new("rustfmt")
        .arg(&dest_path)
        .status();

    match status {
        Ok(stat) if !stat.success() => {
            println!("cargo:warning=rustfmt ran but failed to format 'baked_native.rs'. Check for syntax errors in the generated code.");
        }
        Err(_) => {
            println!("cargo:warning=rustfmt is not installed or not found in PATH. Code will remain unformatted.");
        }
        _ => {} // Success!
    }
}
