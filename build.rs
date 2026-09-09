// build.rs
use logos::Logos;
use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use std::process::Command;

// LINEAR PIPELINE:
#[path = "src/lexer.rs"] pub mod lexer;               // 1. Text to Tokens
#[path = "src/fnv.rs"] pub mod fnv;                   // content hashing (staleness stamp)
#[path = "src/ast.rs"] pub mod ast;                   // 2. AST Data Definitions
#[path = "src/parser.rs"] pub mod parser;             // 3. Tokens to AST
#[path = "src/type_checker.rs"] pub mod type_checker; // 4. AST Validation
#[path = "src/ir.rs"] pub mod ir;                     // 5. IR Data Definitions
#[path = "src/lowerer.rs"] pub mod lowerer;           // 6. AST to IR
#[path = "src/backend.rs"] pub mod backend;           // 7. IR to Rust (Optimize & Codegen)

// DISPATCHED IR renderer — one arm per block, explicit control flow, in
// the shape of the archived dispatched codegen's match arms (the `bN:`
// bodies below were literally its `N =>` arms). Deliberately exhaustive:
// a new IR op fails this match until the dump learns it, so the facility
// can never silently go stale.
fn render_dispatched_ir(out: &mut String, blocks: &[ir::BasicBlock]) {
    use ir::Instruction as I;
    out.push_str("== DISPATCHED IR — post-optimize, pre-resolve_phis (vreg ids; phis intact) ==\n");
    for b in blocks {
        let term = match &b.terminator {
            Some(ir::Terminator::Jump(t)) => format!("goto b{t}"),
            Some(ir::Terminator::Branch { cond, true_block, false_block }) =>
                format!("branch v{cond} ? b{true_block} : b{false_block}"),
            Some(ir::Terminator::Halt) | None => "halt".to_string(),
        };
        out.push_str(&format!("b{}: {{  // depth {}\n", b.id, b.depth));
        for i in &b.instrs {
            let s = match i {
                I::LoadInt { target, val } => format!("v{target} = LoadInt {val}"),
                I::LoadFloat { target, val } => format!("v{target} = LoadFloat {val:?}"),
                I::LoadBool { target, val } => format!("v{target} = LoadBool {val}"),
                I::NewTable { target, ty } => format!("v{target} = NewTable : {ty:?}"),
                I::SetTable { table, key, val, ty } =>
                    format!("SetTable v{table}[v{key}] = v{val} : {ty:?}"),
                I::SetTableFast { table, key, val, ty } =>
                    format!("SetTableFast! v{table}[v{key}] = v{val} : {ty:?}"),
                I::GetTable { target, table, key, ty } =>
                    format!("v{target} = GetTable v{table}[v{key}] : {ty:?}"),
                I::GetTableFast { target, table, key, ty } =>
                    format!("v{target} = GetTableFast! v{table}[v{key}] : {ty:?}"),
                I::Move { target, source, ty } => format!("v{target} = Move v{source} : {ty:?}"),
                I::Add { target, left, right } => format!("v{target} = Add v{left}, v{right}"),
                I::Sub { target, left, right } => format!("v{target} = Sub v{left}, v{right}"),
                I::Mul { target, left, right } => format!("v{target} = Mul v{left}, v{right}"),
                I::Div { target, left, right } => format!("v{target} = Div v{left}, v{right}"),
                I::IntDiv { target, left, right } => format!("v{target} = IntDiv v{left}, v{right}"),
                I::Mod { target, left, right } => format!("v{target} = Mod v{left}, v{right}"),
                I::Neg { target, source } => format!("v{target} = Neg v{source}"),
                I::Less { target, left, right } => format!("v{target} = Less v{left}, v{right}"),
                I::Leq { target, left, right } => format!("v{target} = Leq v{left}, v{right}"),
                I::Geq { target, left, right } => format!("v{target} = Geq v{left}, v{right}"),
                I::Eq { target, left, right, ty } =>
                    format!("v{target} = Eq v{left}, v{right} : {ty:?}"),
                I::Not { target, source } => format!("v{target} = Not v{source}"),
                I::Phi { target, ty, args } => {
                    let a: Vec<String> = args.iter()
                        .map(|(b, r)| format!("b{b}:v{r}")).collect();
                    format!("v{target} = Phi [{}] : {ty:?}", a.join(", "))
                }
                I::EnsureCapacity { table, limit } =>
                    format!("EnsureCapacity v{table} >= v{limit}"),
                I::HoistRawPtr { table } => format!("HoistRawPtr v{table}"),
            };
            out.push_str(&format!("    {s}\n"));
        }
        out.push_str(&format!("    {term}\n}}\n"));
    }
}

fn main() {
    // Cargo will re-run build.rs for different test files
    println!("cargo:rerun-if-env-changed=PHIA_SOURCE");
    let source_path = env::var("PHIA_SOURCE").unwrap_or_else(|_| "main.lua".to_string());
    println!("cargo:rerun-if-changed={}", source_path);

    let source = std::fs::read_to_string(&source_path).expect("Failed to read source");

    // CONTENT-HASH STALeness STAMP — deliberately a sidecar file, NOT part
    // of baked_native.rs, so the byte-frozen lock baselines never see it.
    // Cargo's rerun triggers are mtime-based: replacing a source file with
    // same-path content carrying an older mtime (the classic `mv`) makes
    // cargo skip build.rs entirely and link a stale program. main.rs
    // re-hashes the source at startup and fails LOUD on mismatch.
    {
        let out_dir = env::var("OUT_DIR").unwrap();
        let stamp = Path::new(&out_dir).join("source_stamp.rs");
        std::fs::write(&stamp, format!(
            "// generated by build.rs — content-hash staleness guard, do not edit\n\
             pub const SOURCE_HASH: &str = \"{}\";\n\
             pub const SOURCE_PATH: &str = {:?};\n",
            fnv::hex(source.as_bytes()),
            source_path,
        )).expect("Failed to write source stamp");
    }
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
    let type_map = checker.get_type_map().clone();

    // 3. IR Lowering
    let lowerer = lowerer::IrLowerer::new();
    let ir_program = lowerer.lower_program(&ast, type_map);

    // 4. Optimization & De-SSA
    let mut backend_engine = backend::IrBackend::new(ir_program);

    backend_engine.optimize();

    // DEBUG DUMPS (PHIA_DEBUG_DUMP=<mode>) — written as files into OUT_DIR,
    // beside baked_native.rs, so they survive cargo's build-script stderr
    // capture (stderr only surfaces when the build FAILS; files work for
    // healthy builds too — `ls -t target/release/build/phia-*/out/ir_*.txt`):
    //   mid   — DISPATCHED IR -> ir_dispatched.txt: the post-optimize,
    //           pre-resolve_phis program, one numbered arm per block with
    //           explicit goto/branch/halt, in the shape of the archived
    //           dispatched codegen's match arms. The only window where
    //           loop/if phis are still intact AND the tier4 rewrites (fast
    //           ops, EC/Hoist, pre-header mints) are already applied —
    //           phi-coalescing bugs are visible here in their original
    //           form; the final dump below shows their aftermath.
    //   final — (also 1 / any other nonempty value) -> ir_final_cfg.txt:
    //           the FINAL CFG after allocate_registers, the exact graph the
    //           structured codegen walks, physical register ids. For
    //           tracing structured-codegen panics (reached-twice /
    //           never-reached blocks) and codegen sanity checks.
    //   all   — both.
    let dump_mode = std::env::var("PHIA_DEBUG_DUMP").unwrap_or_default();
    let (dump_mid, dump_final) = match dump_mode.as_str() {
        "mid" => (true, false),
        "all" => (true, true),
        "" => (false, false),
        _ => (false, true),
    };
    let dump_dir = std::env::var("OUT_DIR").unwrap();
    if dump_mid {
        let mut s = String::new();
        render_dispatched_ir(&mut s, &backend_engine.program.blocks);
        std::fs::write(Path::new(&dump_dir).join("ir_dispatched.txt"), s).unwrap();
    }

    backend_engine.resolve_phis();
    backend_engine.propagate_constants();
    backend_engine.simplify();
    backend_engine.allocate_registers();

    if dump_final {
        let mut s = String::from(
            "== FINAL CFG — post-allocate_registers (physical ids), the structured codegen's input ==\n",
        );
        for b in &backend_engine.program.blocks {
            s.push_str(&format!("BLOCK {} (depth {}) term={:?}\n", b.id, b.depth, b.terminator));
            for i in &b.instrs { s.push_str(&format!("   {:?}\n", i)); }
        }
        std::fs::write(Path::new(&dump_dir).join("ir_final_cfg.txt"), s).unwrap();
    }

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
    f.into_inner().unwrap();

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
        _ => {}
    }
}
