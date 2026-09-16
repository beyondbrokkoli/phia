// build.rs
use logos::Logos;
use std::collections::HashMap;
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
#[path = "src/optimizer.rs"] pub mod optimizer;       // 6.25 IR Optimization pass
#[path = "src/de_ssa.rs"] pub mod de_ssa;             // 6.5 Resolve Phis / Propagate Constants / Simplify
#[path = "src/reg_alloc.rs"] pub mod reg_alloc;       // 6.75 Allocate Registers
#[path = "src/backend.rs"] pub mod backend;           // 7. IR to Rust

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
                I::LoadString { target, val } => format!("v{target} = LoadString {val:?}"),
                I::Concat { target, left, right } => format!("v{target} = Concat v{left}, v{right}"),
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
                I::DebugProbe { tag, operands } => {
                    let ops: Vec<String> = operands.iter()
                        .map(|(r, t)| format!("v{r}:{t:?}")).collect();
                    format!("DebugProbe \"{tag}\" [{}]", ops.join(", "))
                }
            };
            out.push_str(&format!("    {s}\n"));
        }
        out.push_str(&format!("    {term}\n}}\n"));
    }
}

// PROBE MAP scan — one line per DebugProbe in stable emission order.
// Probes are never DCE'd and blocks are never reordered, so the ordinal
// joins the MID and FINAL scans of one build, and the tag joins the
// runtime print lines (the tag is the line's first tab-separated field
// when the first argument was a string literal). MID renders vreg ids
// and annotates an operand whose def is a (still intact) phi with its
// incoming edges; FINAL renders the pool-prefixed physical registers
// of each probe site — the register half of the join, since the
// runtime line itself now prints clean values only (Lua-style print).
fn scan_probes(blocks: &[ir::BasicBlock], mid: bool) -> Vec<String> {
    use ir::Instruction as I;
    // vreg -> its defining Phi. One hop only (direct phi defs) — the
    // loop-induction common case; move chains are not chased.
    let mut phi_defs: HashMap<ir::RegId, (usize, usize)> = HashMap::new();
    if mid {
        for (b, block) in blocks.iter().enumerate() {
            for (i, ins) in block.instrs.iter().enumerate() {
                if let I::Phi { target, .. } = ins { phi_defs.insert(*target, (b, i)); }
            }
        }
    }
    let uses_handles = backend::program_uses_handles(blocks);
    let mut lines = Vec::new();
    let mut ordinal = 0usize;
    for (b, block) in blocks.iter().enumerate() {
        for ins in &block.instrs {
            let I::DebugProbe { tag, operands } = ins else { continue };
            let ops: Vec<String> = operands.iter().map(|(r, t)| {
                if mid {
                    let mut s = format!("v{r}:{t:?}");
                    if let Some(&(pb, pi)) = phi_defs.get(r)
                        && let I::Phi { args, .. } = &blocks[pb].instrs[pi]
                    {
                        let incoming: Vec<String> = args.iter()
                            .map(|(bb, rr)| format!("b{bb}:v{rr}")).collect();
                        s.push_str(&format!(" [phi <- {}]", incoming.join(", ")));
                    }
                    s
                } else {
                    use ast::StaticType;
                    // A const operand renders c{n} (n = offset from the
                    // const base): its id names the reserved range, not a
                    // register — pre-remint it rendered i_r<stale-vreg-id>,
                    // naming a register that never existed.
                    if ir::is_const_reg(*r) {
                        format!("c{}", r - ir::CONST_REG_BASE)
                    } else {
                        match t {
                            StaticType::Integer => format!("i_r{r}"),
                            StaticType::Float => format!("f_r{r}"),
                            StaticType::Boolean => format!("b_r{r}"),
                            StaticType::String => format!("s_r{r}"),
                            StaticType::Table(_) | StaticType::UnknownTable(_) =>
                                if uses_handles { format!("t_r{r} len_r{r}") }
                                else { format!("len_r{r}") },
                        }
                    }
                }
            }).collect();
            lines.push(format!(
                "#{ordinal} tag=\"{tag}\" b{b} depth{} {}",
                block.depth, ops.join(" ")
            ));
            ordinal += 1;
        }
    }
    lines
}

// FINAL CFG renderer — like the dispatched renderer, but every register is
// pool-prefixed (i_r/b_r/f_r/s_r/t_r), naming the exact Rust variable
// emission generates. Raw physical ids made cross-pool sharing (i_r49 vs
// t_r49 — distinct variables, one number, by design) read as a register
// clobber. `v` marks const-skipped vregs: they keep their vreg id and never
// become Rust variables (their uses render as literals). Unhandled variants
// fall back to the raw Debug form, so a new IR op cannot break the dump.
fn render_final_ir(
    out: &mut String,
    program: &ir::IrProgram,
    alloc: &reg_alloc::AllocInfo,
    consts: &HashMap<ir::RegId, ir::ConstVal>,
) {
    use ir::Instruction as I;
    // pool_prefixed bound to this dump's data. One argument: the id
    // alone answers the pool (one global timeline, one range per
    // pool), so no StaticType hint rides along anymore.
    let pp = |r: ir::RegId| {
        backend::pool_prefixed(r, consts, alloc)
    };
    for b in &program.blocks {
        let term = match &b.terminator {
            Some(ir::Terminator::Jump(t)) => format!("Jump(b{t})"),
            Some(ir::Terminator::Branch { cond, true_block, false_block }) =>
                format!("Branch {{ cond: {}, true_block: {true_block}, false_block: {false_block} }}",
                    pp(*cond)),
            Some(ir::Terminator::Halt) => "Halt".to_string(),
            None => "None".to_string(),
        };
        out.push_str(&format!("BLOCK {} (depth {}) term=Some({term})\n", b.id, b.depth));
        for i in &b.instrs {
            let s = match i {
                I::LoadInt { target, val } => format!("LoadInt {{ target: {}, val: {val} }}", pp(*target)),
                I::LoadFloat { target, val } => format!("LoadFloat {{ target: {}, val: {val:?} }}", pp(*target)),
                I::LoadBool { target, val } => format!("LoadBool {{ target: {}, val: {val} }}", pp(*target)),
                I::LoadString { target, val } => format!("LoadString {{ target: {}, val: {val:?} }}", pp(*target)),
                I::NewTable { target, ty } => format!("NewTable {{ target: {}, ty: {ty:?} }}", pp(*target)),
                I::SetTable { table, key, val, ty } =>
                    format!("SetTable {{ table: {}, key: {}, val: {}, ty: {ty:?} }}",
                        pp(*table), pp(*key), pp(*val)),
                I::SetTableFast { table, key, val, ty } =>
                    format!("SetTableFast {{ table: {}, key: {}, val: {}, ty: {ty:?} }}",
                        pp(*table), pp(*key), pp(*val)),
                I::GetTable { target, table, key, ty } =>
                    format!("GetTable {{ target: {}, table: {}, key: {}, ty: {ty:?} }}",
                        pp(*target), pp(*table), pp(*key)),
                I::GetTableFast { target, table, key, ty } =>
                    format!("GetTableFast {{ target: {}, table: {}, key: {}, ty: {ty:?} }}",
                        pp(*target), pp(*table), pp(*key)),
                I::Move { target, source, ty } =>
                    format!("Move {{ target: {}, source: {}, ty: {ty:?} }}",
                        pp(*target), pp(*source)),
                I::Add { target, left, right } => arith(alloc, consts, "Add", *target, *left, *right),
                I::Sub { target, left, right } => arith(alloc, consts, "Sub", *target, *left, *right),
                I::Mul { target, left, right } => arith(alloc, consts, "Mul", *target, *left, *right),
                I::Div { target, left, right } => arith(alloc, consts, "Div", *target, *left, *right),
                I::IntDiv { target, left, right } => arith(alloc, consts, "IntDiv", *target, *left, *right),
                I::Mod { target, left, right } => arith(alloc, consts, "Mod", *target, *left, *right),
                I::Neg { target, source } =>
                    format!("Neg {{ target: {}, source: {} }}", pp(*target), pp(*source)),
                I::Less { target, left, right } =>
                    format!("Less {{ target: {}, left: {}, right: {} }}",
                        pp(*target), pp(*left), pp(*right)),
                I::Leq { target, left, right } =>
                    format!("Leq {{ target: {}, left: {}, right: {} }}",
                        pp(*target), pp(*left), pp(*right)),
                I::Geq { target, left, right } =>
                    format!("Geq {{ target: {}, left: {}, right: {} }}",
                        pp(*target), pp(*left), pp(*right)),
                I::Eq { target, left, right, ty } =>
                    format!("Eq {{ target: {}, left: {}, right: {}, ty: {ty:?} }}",
                        pp(*target), pp(*left), pp(*right)),
                I::Not { target, source } =>
                    format!("Not {{ target: {}, source: {} }}", pp(*target), pp(*source)),
                I::Concat { target, left, right } =>
                    format!("Concat {{ target: {}, left: {}, right: {} }}",
                        pp(*target), pp(*left), pp(*right)),
                I::Phi { target, ty, args } => {
                    let a: Vec<String> = args.iter()
                        .map(|(b, r)| format!("b{b}:{}", pp(*r)))
                        .collect();
                    format!("Phi {{ target: {}, ty: {ty:?}, args: [{}] }}",
                        pp(*target), a.join(", "))
                }
                I::EnsureCapacity { table, limit } =>
                    format!("EnsureCapacity {{ table: {}, limit: {} }}",
                        pp(*table), pp(*limit)),
                I::HoistRawPtr { table } =>
                    format!("HoistRawPtr {{ table: {} }}", pp(*table)),
                I::DebugProbe { tag, operands } => {
                    let ops: Vec<String> = operands.iter()
                        .map(|(r, t)| format!("({}, {t:?})", pp(*r)))
                        .collect();
                    format!("DebugProbe {{ tag: {tag:?}, operands: [{}] }}", ops.join(", "))
                }
            };
            out.push_str(&format!("   {s}\n"));
        }
    }
}

fn arith(
    alloc: &reg_alloc::AllocInfo,
    consts: &HashMap<ir::RegId, ir::ConstVal>,
    name: &str,
    target: ir::RegId,
    left: ir::RegId,
    right: ir::RegId,
) -> String {
    // The untyped arith ops render every operand by its own pool range:
    // the id alone answers float vs int (const operands render through
    // the map lookup inside pool_prefixed). The old version had to
    // probe the TARGET's pool first and smear that hint over the
    // operands — a workaround for the days when ids carried no pools.
    let p = |r: ir::RegId| {
        backend::pool_prefixed(r, consts, alloc)
    };
    format!("{name} {{ target: {}, left: {}, right: {} }}", p(target), p(left), p(right))
}

fn main() {
    // Cargo will re-run build.rs for different test files. An empty
    // PHIA_SOURCE counts as unset: `PHIA_SOURCE= cargo build` must fall
    // back to the project-root main.lua, not try to read "".
    println!("cargo:rerun-if-env-changed=PHIA_SOURCE");
    let source_path = env::var("PHIA_SOURCE")
        .ok()
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "main.lua".to_string());
    println!("cargo:rerun-if-changed={}", source_path);

    let source = std::fs::read_to_string(&source_path).expect("Failed to read source");

    // CONTENT-HASH STALENESS STAMP — deliberately a sidecar file, NOT part
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
    let mut ir_program = lowerer.lower_program(&ast, type_map);

    // 4. Optimization
    optimizer::optimize(&mut ir_program);

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
        render_dispatched_ir(&mut s, &ir_program.blocks);
        std::fs::write(Path::new(&dump_dir).join("ir_dispatched.txt"), s).unwrap();
    }

    // MID probe scan must run while phis are still intact (pre-resolve_phis).
    let probe_mid = scan_probes(&ir_program.blocks, true);

    // Phase 6.5
    de_ssa::resolve_phis(&mut ir_program);
    let consts = de_ssa::propagate_constants(&mut ir_program);
    de_ssa::simplify(&mut ir_program);

    // Phase 6.75
    let alloc_info = reg_alloc::allocate_registers(&mut ir_program, &consts);

    if dump_final {
        let mut s = String::from(
            "== FINAL CFG — post-allocate_registers (pool-prefixed physical ids), the structured codegen's input ==\n\
             == i_r/b_r/f_r/s_r/t_r name the exact Rust variable emission generates; every physical id is unique ==\n\
             == (one global timeline, one pool per range — no two variables share a number). c{n} marks const-folded ==\n\
             == regs (layer-B ids; their uses render as literals). ==\n",
        );
        render_final_ir(&mut s, &ir_program, &alloc_info, &consts);
        std::fs::write(Path::new(&dump_dir).join("ir_final_cfg.txt"), s).unwrap();
    }

    // PROBE MAP sidecar — written whenever the program contains probes
    // (tiny, and the runtime echo is always live, unlike the dump-gated
    // files). Deliberately OUTSIDE baked_native.rs so the byte-frozen
    // lock baselines never see it (source_stamp.rs precedent). This is
    // the join: tag -> runtime PROBE line; ordinal -> MID/FINAL pair;
    // FINAL tokens -> physical registers in ir_final_cfg.txt; MID
    // vregs + phi ancestry -> ir_dispatched.txt.
    let probe_final = scan_probes(&ir_program.blocks, false);
    if !probe_final.is_empty() {
        let mut s = String::from(
            "== PROBE MAP — joins runtime print lines to both IR dumps ==\n\
             == MID: vreg ids, phis intact (pairs with ir_dispatched.txt); [phi <- ...] marks an operand whose def is that phi ==\n",
        );
        for l in &probe_mid { s.push_str(l); s.push('\n'); }
        s.push_str(
            "== FINAL: the pool-prefixed physical registers of each probe site (pairs with ir_final_cfg.txt) ==\n\
             == the runtime line prints clean values only — the registers live HERE ==\n",
        );
        for l in &probe_final { s.push_str(l); s.push('\n'); }
        std::fs::write(Path::new(&dump_dir).join("probe_map.txt"), s).unwrap();
    }

    // 5. Code Generation
    let mut final_code = backend::generate_rust_code(&ir_program, &alloc_info, &consts);

    // Regression stats: computed from the FINAL CFG
    let (mut fs_, mut fg, mut ds, mut dg, mut ho) = (0, 0, 0, 0, 0);
    let mut ctx_list = Vec::new();

    for block in &ir_program.blocks {
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
    // consts_* = per-kind fold counts (layer-B sizes): the per-pass
    // observability rider — with the float/string folds the fold's
    // effect on emitted code is pinned by locks, and these counters
    // pin the fold's REACH (EXPECT: consts_f=3 style pins). One map,
    // four variants — the counts come off the variant discriminant.
    let (mut ci_n, mut cb_n, mut cf_n, mut cs_n) = (0usize, 0usize, 0usize, 0usize);
    for v in consts.values() {
        match v {
            ir::ConstVal::Int(_) => ci_n += 1,
            ir::ConstVal::Bool(_) => cb_n += 1,
            ir::ConstVal::Float(_) => cf_n += 1,
            ir::ConstVal::String(_) => cs_n += 1,
        }
    }
    final_code.push_str(&format!(
        "\npub const STATS: &str = \"fast_sets={fs_};fast_gets={fg};dyn_sets={ds};dyn_gets={dg};hoists={ho};hoist_ctx={hoist_ctx};consts_i={ci_n};consts_b={cb_n};consts_f={cf_n};consts_s={cs_n}\";\n"
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
