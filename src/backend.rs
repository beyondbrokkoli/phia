// src/backend.rs
use crate::ir::{IrProgram, Instruction, Terminator, BasicBlock, BlockId, RegId};
use std::collections::{HashMap, HashSet};
use crate::ast::StaticType;
use crate::reg_alloc::{AllocInfo};

fn indent(d: usize) -> String { "    ".repeat(d) }

// DUAL-TEMPLATE GATE, shared with the build.rs probe-map sidecar (which
// must render the same table tokens the runtime PROBE line prints). A
// Table element type can only be born from a table-typed table op
// (checker unification is its only producer), so this predicate is
// exactly "the program nests tables": nested programs render handle-mode
// templates (nested_*/tier4_* locks), pure-integer programs the frozen
// pointer templates (all others).
pub fn program_uses_handles(blocks: &[BasicBlock]) -> bool {
    blocks.iter().any(|b| b.instrs.iter().any(|i| match i {
        Instruction::SetTable { ty, .. } | Instruction::SetTableFast { ty, .. }
        | Instruction::GetTable { ty, .. } | Instruction::GetTableFast { ty, .. } =>
            matches!(ty, StaticType::Table(_) | StaticType::UnknownTable(_)),
        _ => false,
    }))
}

pub struct IrBackend {
    pub program: IrProgram,
    alloc: AllocInfo,
    consts_i: HashMap<RegId, i64>,
    consts_b: HashMap<RegId, bool>,
}

impl IrBackend {
    pub fn new(
        program: IrProgram,
        alloc: AllocInfo,
        consts_i: HashMap<RegId, i64>,
        consts_b: HashMap<RegId, bool>
    ) -> Self {
        Self { program, alloc, consts_i, consts_b }
    }

    fn iop_str(&self, r: RegId) -> String {
        self.consts_i.get(&r).map(|v| v.to_string())
            .unwrap_or_else(|| format!("i_r{r}"))
    }
    fn bop_str(&self, r: RegId) -> String {
        self.consts_b.get(&r)
            .map(|v| if *v { "true" } else { "false" }.to_string())
            .unwrap_or_else(|| format!("b_r{r}"))
    }
    // Floats are never compile-time folded (no consts_f), so a float
    // operand is always a physical register.
    fn fop_str(&self, r: RegId) -> String { format!("f_r{r}") }
    // Strings likewise: no const folding (float precedent), so a string
    // operand is always a physical register.
    fn sop_str(&self, r: RegId) -> String { format!("s_r{r}") }
    // Pool membership is answered by the disjoint mint ranges, NOT a
    // phys-id -> pool map: Int/Bool/Table/String deliberately share one id
    // range, so such a map is last-writer-wins across those pools and any
    // future bare-id lookup would silently query whichever pool minted the
    // id last. The disjoint pools have exact ranges by construction —
    // Float mints [float_base, float_base + n_float), float_base sits
    // above the entire shared range, and no other pool mints into it — so
    // range membership equals "this pool minted this id", and vreg ids
    // (all < phys_base <= float_base) can never fall inside.
    pub fn is_float_reg(&self, r: RegId) -> bool {
        r >= self.alloc.float_base && r < self.alloc.float_base + self.alloc.n_float as RegId
    }

    // Storage side of a hoisted/EC'd table physical: float-ELEMENT tables
    // live in their own pool ([ftable_base, ftable_base + n_ftable)), so
    // the id alone answers farray vs array (and *mut f64 vs *mut i64 in
    // the decl block) unambiguously.
    fn is_ftable_reg(&self, r: RegId) -> bool {
        r >= self.alloc.ftable_base && r < self.alloc.ftable_base + self.alloc.n_ftable as RegId
    }

    // String-ELEMENT tables: same disjoint-range argument, sarray side.
    fn is_tstr_reg(&self, r: RegId) -> bool {
        r >= self.alloc.tstr_base && r < self.alloc.tstr_base + self.alloc.n_tstr as RegId
    }

    // Debug-dump helper: render an allocated register the way emission
    // would — pool prefix included. The final CFG dump prints raw physical
    // ids, and Int/Bool/Table/String physicals deliberately share one id
    // space (i_r49 and t_r49 are distinct variables), so a bare number
    // reads as a clobber that cannot happen. The operand's static type
    // disambiguates the shared pools (the same rule Eq/probe emission
    // uses); Float/TableFloat/TableString live in disjoint id ranges, so
    // their pool lookups are unambiguous. Const-skipped vregs keep their
    // vreg id at emission (uses render as literals) — mark those `v` so
    // the two namespaces are visually distinct in the dump.
    pub fn pool_prefixed(&self, r: RegId, t: &StaticType) -> String {
        if self.consts_i.contains_key(&r) || self.consts_b.contains_key(&r) {
            return format!("v{r}");
        }
        if self.is_float_reg(r) {
            return format!("f_r{r}");
        }
        if self.is_ftable_reg(r) || self.is_tstr_reg(r) {
            return format!("t_r{r}");
        }
        match t {
            StaticType::Integer => format!("i_r{r}"),
            StaticType::Float => format!("f_r{r}"),
            StaticType::Boolean => format!("b_r{r}"),
            StaticType::String => format!("s_r{r}"),
            StaticType::Table(_) | StaticType::UnknownTable(_) => format!("t_r{r}"),
        }
    }

    fn reg_uses(&self, r: RegId) -> usize {
        let mut n = 0;
        for b in &self.program.blocks {
            for i in &b.instrs { for u in i.use_regs() { if u == r { n += 1; } } }
            if let Some(Terminator::Branch { cond, .. }) = &b.terminator {
                if *cond == r { n += 1; }
            }
        }
        n
    }

    // a dominates b iff b is unreachable from the entry once a is removed.
    // Small structured CFGs: this DFS runs once per back-edge candidate.
    fn dominates(&self, a: BlockId, b: BlockId) -> bool {
        if a == b { return true; }
        let mut seen = HashSet::new();
        let mut stack = vec![0usize];
        while let Some(x) = stack.pop() {
            if x == a || !seen.insert(x) { continue; }
            match &self.program.blocks[x].terminator {
                Some(Terminator::Jump(t)) => stack.push(*t),
                Some(Terminator::Branch { true_block, false_block, .. }) => {
                    stack.push(*true_block);
                    stack.push(*false_block);
                }
                _ => {}
            }
        }
        !seen.contains(&b)
    }

    fn is_loop_header(&self, h: BlockId) -> bool {
        // A loop header has a back edge: a LATER block jumping to it that
        // the header DOMINATES — the natural-loop definition (every trip
        // to the back edge passes through the header). Mere id-space
        // backwardness is not enough: the lowerer mints if-arm blocks
        // (including nested ifs' joins) AFTER the enclosing join, so a
        // nested if's inner join jumps backward to the outer join. The old
        // reachability guard ("an if-join's jumper sits in a sibling arm
        // nothing downstream reaches") holds only OUTSIDE loops — inside a
        // while, a join's successors flow to the latch, around the back
        // edge, and back through the whole body, so every nested-if join
        // inside a loop graded as a header and emit_loop wrapped and
        // re-entered it (differential fuzzer seed 157: "header 15 reached
        // twice"). Dominance has no such blind spot: a join never
        // dominates its own then-arm's nested join, while a real while
        // header dominates its latch.
        let Some(Terminator::Branch { .. }) = &self.program.blocks[h].terminator else { return false };
        self.program.blocks[h + 1..].iter().enumerate().any(|(i, p)| {
            matches!(&p.terminator, Some(Terminator::Jump(t)) if *t == h)
                && self.dominates(h, h + 1 + i)
        })
    }

    fn emit_table_decl(&self, out: &mut String, r: RegId, uses_handles: bool, fast_phys: &HashSet<RegId>) {
        if uses_handles {
            out.push_str(&format!("    let mut t_r{r} = 0i64;\n"));
        } else {
            out.push_str(&format!("    let mut t_r{r}: *mut Table = std::ptr::null_mut();\n"));
        }
        if fast_phys.contains(&r) {
            let ptr_ty = if self.is_ftable_reg(r) { "*mut f64" }
                else if self.is_tstr_reg(r) { "*mut String" }
                else { "*mut i64" };
            out.push_str(&format!("    let mut p_r{r}: {ptr_ty} = std::ptr::null_mut();\n"));
            out.push_str(&format!("    let mut len_r{r} = 0usize;\n"));
        }
    }

    fn emit_instr(&self, out: &mut String, instr: &Instruction, d: usize, uses_handles: bool) {
        // compile-time-computed defs emit nothing: their uses are literals
        if let Some(t) = instr.def_reg() {
            if self.consts_i.contains_key(&t) || self.consts_b.contains_key(&t) {
                return;
            }
        }
        let ind = indent(d);
        // In handle mode a table-typed operand renders as its t_r handle reg.
        // Checked arena resolution (tables.get/get_mut + nil panic) instead of
        // get_unchecked: even a hypothetical checker bug degrades to a clean
        // "Runtime Error", never UB. Handle mode only — the pointer templates
        // below are frozen, byte-identical to the milestone locks.
        let is_tbl = |ty: &StaticType| matches!(ty, StaticType::Table(_) | StaticType::UnknownTable(_));
        match instr {
            Instruction::LoadInt { target, val } =>
                out.push_str(&format!("{ind}i_r{target} = {val};\n")),
            Instruction::LoadFloat { target, val } =>
                out.push_str(&format!("{ind}f_r{target} = {val:?};\n")),
            Instruction::LoadBool { target, val } =>
                out.push_str(&format!("{ind}b_r{target} = {val};\n")),
            Instruction::LoadString { target, val } =>
                // {:?} on a &str renders a valid escaped Rust literal
                out.push_str(&format!("{ind}s_r{target} = {val:?}.to_string();\n")),
            Instruction::Concat { target, left, right } =>
                out.push_str(&format!(
                    "{ind}s_r{target} = format!(\"{{}}{{}}\", {}, {});\n",
                    self.sop_str(*left), self.sop_str(*right)
                )),
            Instruction::NewTable { target, ty } => {
                // A table is monomorphic: its static element type picks the
                // storage side at construction and it never changes.
                let float_tbl = matches!(ty, StaticType::Table(inner) if matches!(**inner, StaticType::Float));
                let str_tbl = matches!(ty, StaticType::Table(inner) if matches!(**inner, StaticType::String));
                if uses_handles {
                    // 1-based arena handle; 0 stays reserved for null
                    if float_tbl {
                        out.push_str(&format!(
                            "{ind}tables.push(Box::new(Table::new_float()));\n\
                             {ind}t_r{target} = tables.len() as i64;\n"
                        ));
                    } else if str_tbl {
                        out.push_str(&format!(
                            "{ind}tables.push(Box::new(Table::new_string()));\n\
                             {ind}t_r{target} = tables.len() as i64;\n"
                        ));
                    } else {
                        out.push_str(&format!(
                            "{ind}tables.push(Box::new(Table::new()));\n\
                             {ind}t_r{target} = tables.len() as i64;\n"
                        ));
                    }
                } else if float_tbl {
                    out.push_str(&format!(
                        "{ind}let mut new_table = Box::new(Table::new_float());\n\
                         {ind}t_r{target} = &mut *new_table as *mut Table;\n\
                         {ind}tables.push(new_table);\n"
                    ));
                } else if str_tbl {
                    out.push_str(&format!(
                        "{ind}let mut new_table = Box::new(Table::new_string());\n\
                         {ind}t_r{target} = &mut *new_table as *mut Table;\n\
                         {ind}tables.push(new_table);\n"
                    ));
                } else {
                    out.push_str(&format!(
                        "{ind}let mut new_table = Box::new(Table::new());\n\
                         {ind}t_r{target} = &mut *new_table as *mut Table;\n\
                         {ind}tables.push(new_table);\n"
                    ));
                }
            }
            Instruction::Move { target, source, ty } => match ty {
                StaticType::Integer => out.push_str(&format!("{ind}i_r{target} = {};\n", self.iop_str(*source))),
                StaticType::Boolean => out.push_str(&format!("{ind}b_r{target} = {};\n", self.bop_str(*source))),
                StaticType::Float => out.push_str(&format!("{ind}f_r{target} = {};\n", self.fop_str(*source))),
                // strings are owned: a Move clones (SSA semantics — the
                // source slot may still be read on another path)
                StaticType::String => out.push_str(&format!("{ind}s_r{target} = {}.clone();\n", self.sop_str(*source))),
                StaticType::Table(_) | StaticType::UnknownTable(_) => out.push_str(&format!("{ind}t_r{target} = t_r{source};\n")),
            },
            Instruction::Add { target, left, right } => {
                if self.is_float_reg(*target) {
                    out.push_str(&format!("{ind}f_r{target} = {} + {};\n", self.fop_str(*left), self.fop_str(*right)))
                } else {
                    out.push_str(&format!("{ind}i_r{target} = {} + {};\n", self.iop_str(*left), self.iop_str(*right)))
                }
            }
            Instruction::Sub { target, left, right } => {
                if self.is_float_reg(*target) {
                    out.push_str(&format!("{ind}f_r{target} = {} - {};\n", self.fop_str(*left), self.fop_str(*right)))
                } else {
                    out.push_str(&format!("{ind}i_r{target} = {} - {};\n", self.iop_str(*left), self.iop_str(*right)))
                }
            }
            Instruction::Mul { target, left, right } => {
                if self.is_float_reg(*target) {
                    out.push_str(&format!("{ind}f_r{target} = {} * {};\n", self.fop_str(*left), self.fop_str(*right)))
                } else {
                    out.push_str(&format!("{ind}i_r{target} = {} * {};\n", self.iop_str(*left), self.iop_str(*right)))
                }
            }
            Instruction::Div { target, left, right } => {
                if self.is_float_reg(*target) {
                    out.push_str(&format!("{ind}f_r{target} = {} / {};\n", self.fop_str(*left), self.fop_str(*right)))
                } else {
                    out.push_str(&format!("{ind}i_r{target} = {} / {};\n", self.iop_str(*left), self.iop_str(*right)))
                }
            }
            // Lua floor division: `//` rounds toward negative infinity
            // (-7 // 2 == -4), on both the integer and float sides. The
            // integer side is spelled explicitly (trunc quotient, minus one
            // when a remainder exists and disagrees with the divisor's
            // sign) rather than via div_floor — this toolchain predates
            // stabilized int_roundings. (`/` above is the pinned
            // divergence: Lua's `/` always yields a float and strict
            // typing forbids that, so integer `/` is truncating division
            // and float `/` is plain division.) /0 and MIN/-1 still panic
            // inside the leading L / R.
            Instruction::IntDiv { target, left, right } => {
                if self.is_float_reg(*target) {
                    out.push_str(&format!("{ind}f_r{target} = ({} / {}).floor();\n", self.fop_str(*left), self.fop_str(*right)))
                } else {
                    out.push_str(&format!(
                        "{ind}i_r{target} = {} / {} - i64::from({} % {} != 0 && ({} < 0) != ({} < 0));\n",
                        self.iop_str(*left), self.iop_str(*right),
                        self.iop_str(*left), self.iop_str(*right),
                        self.iop_str(*left), self.iop_str(*right)))
                }
            }
            // Lua modulo: result takes the divisor's sign (-7 % 3 == 2),
            // unlike Rust's truncated remainder — adjust the remainder by
            // the divisor exactly when the two signs disagree.
            Instruction::Mod { target, left, right } => {
                if self.is_float_reg(*target) {
                    out.push_str(&format!("{ind}f_r{target} = {} - ({} / {}).floor() * {};\n",
                        self.fop_str(*left), self.fop_str(*left), self.fop_str(*right), self.fop_str(*right)))
                } else {
                    out.push_str(&format!(
                        "{ind}i_r{target} = {} % {} + i64::from({} % {} != 0 && ({} % {} < 0) != ({} < 0)) * {};\n",
                        self.iop_str(*left), self.iop_str(*right),
                        self.iop_str(*left), self.iop_str(*right),
                        self.iop_str(*left), self.iop_str(*right),
                        self.iop_str(*right), self.iop_str(*right)))
                }
            }
            Instruction::Neg { target, source } => {
                if self.is_float_reg(*target) {
                    out.push_str(&format!("{ind}f_r{target} = -{};\n", self.fop_str(*source)))
                } else {
                    out.push_str(&format!("{ind}i_r{target} = -{};\n", self.iop_str(*source)))
                }
            }
            Instruction::Less { target, left, right } => {
                if self.is_float_reg(*left) {
                    out.push_str(&format!("{ind}b_r{target} = {} < {};\n", self.fop_str(*left), self.fop_str(*right)))
                } else {
                    out.push_str(&format!("{ind}b_r{target} = {} < {};\n", self.iop_str(*left), self.iop_str(*right)))
                }
            }
            Instruction::Leq { target, left, right } => {
                if self.is_float_reg(*left) {
                    out.push_str(&format!("{ind}b_r{target} = {} <= {};\n", self.fop_str(*left), self.fop_str(*right)))
                } else {
                    out.push_str(&format!("{ind}b_r{target} = {} <= {};\n", self.iop_str(*left), self.iop_str(*right)))
                }
            }
            Instruction::Geq { target, left, right } => {
                if self.is_float_reg(*left) {
                    out.push_str(&format!("{ind}b_r{target} = {} >= {};\n", self.fop_str(*left), self.fop_str(*right)))
                } else {
                    out.push_str(&format!("{ind}b_r{target} = {} >= {};\n", self.iop_str(*left), self.iop_str(*right)))
                }
            }
            // The instruction's ty is the single source of truth for the
            // rendering: Int and Bool physicals share one id range, so the
            // pool tables cannot tell an int operand from a bool operand
            // (a const-bool-on-the-left Eq used to render undeclared i_rN
            // registers, and an aliased physical bool rendered the wrong
            // variable entirely).
            Instruction::Eq { target, left, right, ty } => match ty {
                StaticType::Float =>
                    out.push_str(&format!("{ind}b_r{target} = {} == {};\n", self.fop_str(*left), self.fop_str(*right))),
                StaticType::String =>
                    out.push_str(&format!("{ind}b_r{target} = {} == {};\n", self.sop_str(*left), self.sop_str(*right))),
                StaticType::Boolean =>
                    out.push_str(&format!("{ind}b_r{target} = {} == {};\n", self.bop_str(*left), self.bop_str(*right))),
                _ =>
                    out.push_str(&format!("{ind}b_r{target} = {} == {};\n", self.iop_str(*left), self.iop_str(*right))),
            },
            Instruction::Not { target, source } =>
                out.push_str(&format!("{ind}b_r{target} = !{};\n", self.bop_str(*source))),

            // Runtime observation. One line per trip, naming each operand's
            // physical slot: the runtime line names the exact register
            // ir_final_cfg.txt shows, which is what joins a run back to its
            // dump. Determinism contract: values, handles and arena-derived
            // lengths only — never addresses — so PROBE lines stay
            // re-derivable pins. A table operand prints its handle (0 is
            // the one observable nil in the language) plus its materialized
            // length; a nil handle's len renders as MAX, a sentinel no real
            // table can collide with.
            Instruction::DebugProbe { tag, operands } => {
                let mut fmt_parts: Vec<String> = Vec::new();
                let mut args: Vec<String> = Vec::new();
                for &(r, ref t) in operands {
                    match t {
                        StaticType::Integer => {
                            fmt_parts.push(format!("i_r{r}={{}}"));
                            args.push(self.iop_str(r));
                        }
                        StaticType::Float => {
                            fmt_parts.push(format!("f_r{r}={{:?}}"));
                            args.push(self.fop_str(r));
                        }
                        StaticType::Boolean => {
                            fmt_parts.push(format!("b_r{r}={{}}"));
                            args.push(self.bop_str(r));
                        }
                        StaticType::String => {
                            fmt_parts.push(format!("s_r{r}={{:?}}"));
                            args.push(self.sop_str(r));
                        }
                        StaticType::Table(_) | StaticType::UnknownTable(_) => {
                            let fld = if self.is_ftable_reg(r) { "farray" }
                                else if self.is_tstr_reg(r) { "sarray" }
                                else { "array" };
                            if uses_handles {
                                fmt_parts.push(format!("t_r{r}={{}} len_r{r}={{}}"));
                                args.push(format!("t_r{r}"));
                                args.push(format!(
                                    "match tables.get((t_r{r} - 1) as usize) \
                                     {{ Some(t) => t.{fld}.len(), None => usize::MAX }}"
                                ));
                            } else {
                                // pointer mode: a table-typed operand is
                                // always NewTable-defined before use (only
                                // nested programs read tables out of tables,
                                // and those render handle mode), so the
                                // deref cannot see null
                                fmt_parts.push(format!("len_r{r}={{}}"));
                                args.push(format!("unsafe {{ (*t_r{r}).{fld}.len() }}"));
                            }
                        }
                    }
                }
                // a brace in the user tag would be a format directive
                let safe_tag = tag.replace('{', "{{").replace('}', "}}");
                if operands.is_empty() {
                    // probe("tag") with no operands: no {} placeholder, no args
                    out.push_str(&format!("{ind}println!(\"PROBE {safe_tag}:\");\n"));
                    return;
                }
                out.push_str(&format!(
                    "{ind}println!(\"PROBE {safe_tag}: {}\", {});\n",
                    fmt_parts.join(" "),
                    args.join(", ")
                ));
            }

            Instruction::EnsureCapacity { table, limit } => {
                // Storage side comes from the table's pool: float-element
                // tables resize the farray with 0.0 zeros, string-element
                // tables the sarray with empty strings, handle tables the
                // frozen integer template. Monomorphism guarantees the
                // fast ops riding this EC agree with the pool.
                let (fld, zero) = if self.is_ftable_reg(*table) {
                    ("farray", "0.0")
                } else if self.is_tstr_reg(*table) {
                    ("sarray", "String::new()")
                } else {
                    ("array", "0")
                };
                if uses_handles {
                    out.push_str(&format!(
                        "{ind}let lim = {lim};\n\
                         {ind}if lim > 0 {{\n\
                         {ind}    if t_r{table} == 0 {{ panic!(\"Runtime Error: table is nil\"); }}\n\
                         {ind}    let t = match tables.get_mut((t_r{table} - 1) as usize) {{ Some(t) => &mut **t, None => panic!(\"Runtime Error: table is nil\") }};\n\
                         {ind}    if (lim as usize) > t.{fld}.len() {{\n\
                         {ind}        t.{fld}.resize(lim as usize, {zero});\n\
                         {ind}    }}\n\
                         {ind}}}\n",
                        lim = self.iop_str(*limit)
                    ));
                } else {
                    out.push_str(&format!(
                        "{ind}let lim = {lim};\n\
                         {ind}if lim > 0 {{\n\
                         {ind}    let t = unsafe {{ &mut *t_r{table} }};\n\
                         {ind}    if (lim as usize) > t.{fld}.len() {{\n\
                         {ind}        t.{fld}.resize(lim as usize, {zero});\n\
                         {ind}    }}\n\
                         {ind}}}\n",
                        lim = self.iop_str(*limit)
                    ));
                }
            }
            Instruction::HoistRawPtr { table } => {
                let fld = if self.is_ftable_reg(*table) { "farray" }
                    else if self.is_tstr_reg(*table) { "sarray" }
                    else { "array" };
                if uses_handles {
                    out.push_str(&format!(
                        "{ind}if t_r{table} == 0 {{ panic!(\"Runtime Error: table is nil\"); }}\n\
                         {ind}let t = match tables.get_mut((t_r{table} - 1) as usize) {{ Some(t) => &mut **t, None => panic!(\"Runtime Error: table is nil\") }};\n\
                         {ind}len_r{table} = t.{fld}.len();\n\
                         {ind}p_r{table} = t.{fld}.as_mut_ptr();\n"
                    ));
                } else {
                    out.push_str(&format!(
                        "{ind}len_r{table} = unsafe {{ (*t_r{table}).{fld}.len() }};\n\
                         {ind}p_r{table} = unsafe {{ (*t_r{table}).{fld}.as_mut_ptr() }};\n"
                    ));
                }
            }

            Instruction::SetTable { table, key, val, ty } => {
                // Element-kind selects storage side and zero. The integer and
                // table cases render byte-identically to the frozen templates
                // (fld="array", zero="0"); floats take the f64 side.
                if uses_handles {
                    let (fld, zero, val_str) = if is_tbl(ty) {
                        ("array", "0", format!("t_r{val}"))
                    } else if matches!(ty, StaticType::Float) {
                        ("farray", "0.0", format!("f_r{val}"))
                    } else if matches!(ty, StaticType::String) {
                        // clone: a store COPIES the immutable value in
                        // (a move would kill the source slot — the borrow
                        // checker rejects it at build time, and the source
                        // is exactly what a loop-carried string is)
                        ("sarray", "String::new()", format!("s_r{val}.clone()"))
                    } else {
                        ("array", "0", self.iop_str(*val))
                    };
                    out.push_str(&format!(
                        "{ind}let k = {key};\n\
                         {ind}if k < 0 {{ panic!(\"Runtime Error: Negative table index\"); }}\n\
                         {ind}let idx = k as usize;\n\
                         {ind}if t_r{table} == 0 {{ panic!(\"Runtime Error: table is nil\"); }}\n\
                         {ind}let t = match tables.get_mut((t_r{table} - 1) as usize) {{ Some(t) => &mut **t, None => panic!(\"Runtime Error: table is nil\") }};\n\
                         {ind}if idx >= t.{fld}.len() {{ t.{fld}.resize(idx + 1, {zero}); }}\n\
                         {ind}unsafe {{ *t.{fld}.get_unchecked_mut(idx) = {val_str}; }}\n",
                        key = self.iop_str(*key)
                    ));
                } else {
                    let (fld, zero, val_str) = if matches!(ty, StaticType::Float) {
                        ("farray", "0.0", self.fop_str(*val))
                    } else if matches!(ty, StaticType::String) {
                        ("sarray", "String::new()", format!("{}.clone()", self.sop_str(*val)))
                    } else {
                        ("array", "0", self.iop_str(*val))
                    };
                    out.push_str(&format!(
                        "{ind}let k = {key};\n\
                         {ind}if k < 0 {{ panic!(\"Runtime Error: Negative table index\"); }}\n\
                         {ind}let idx = k as usize;\n\
                         {ind}let t = unsafe {{ &mut *t_r{table} }};\n\
                         {ind}if idx >= t.{fld}.len() {{ t.{fld}.resize(idx + 1, {zero}); }}\n\
                         {ind}unsafe {{ *t.{fld}.get_unchecked_mut(idx) = {val_str}; }}\n",
                        key = self.iop_str(*key)
                    ));
                }
            }
            Instruction::GetTable { target, table, key, ty } => {
                // String elements are owned, not Copy: every read CLONES
                // out of the Vec (dyn and fast), and the absent-key
                // default is the empty string — nil-as-absence compiled
                // to the pool's zero, exactly like 0 and 0.0.
                if matches!(ty, StaticType::String) {
                    if uses_handles {
                        out.push_str(&format!(
                            "{ind}let k = {key};\n\
                             {ind}if k < 0 {{ panic!(\"Runtime Error: Negative table index\"); }}\n\
                             {ind}let idx = k as usize;\n\
                             {ind}if t_r{table} == 0 {{ panic!(\"Runtime Error: table is nil\"); }}\n\
                             {ind}let t = match tables.get((t_r{table} - 1) as usize) {{ Some(t) => &**t, None => panic!(\"Runtime Error: table is nil\") }};\n\
                             {ind}s_r{target} = if idx < t.sarray.len() {{ unsafe {{ t.sarray.get_unchecked(idx).clone() }} }} else {{ String::new() }};\n",
                            key = self.iop_str(*key)
                        ));
                    } else {
                        out.push_str(&format!(
                            "{ind}let k = {key};\n\
                             {ind}if k < 0 {{ panic!(\"Runtime Error: Negative table index\"); }}\n\
                             {ind}let idx = k as usize;\n\
                             {ind}let t = unsafe {{ &*t_r{table} }};\n\
                             {ind}s_r{target} = if idx < t.sarray.len() {{ unsafe {{ t.sarray.get_unchecked(idx).clone() }} }} else {{ String::new() }};\n",
                            key = self.iop_str(*key)
                        ));
                    }
                } else if uses_handles {
                    let (fld, zero, target_str) = if is_tbl(ty) {
                        ("array", "0", format!("t_r{target}"))
                    } else if matches!(ty, StaticType::Float) {
                        ("farray", "0.0", format!("f_r{target}"))
                    } else {
                        ("array", "0", format!("i_r{target}"))
                    };
                    out.push_str(&format!(
                        "{ind}let k = {key};\n\
                         {ind}if k < 0 {{ panic!(\"Runtime Error: Negative table index\"); }}\n\
                         {ind}let idx = k as usize;\n\
                         {ind}if t_r{table} == 0 {{ panic!(\"Runtime Error: table is nil\"); }}\n\
                         {ind}let t = match tables.get((t_r{table} - 1) as usize) {{ Some(t) => &**t, None => panic!(\"Runtime Error: table is nil\") }};\n\
                         {ind}{target_str} = if idx < t.{fld}.len() {{ unsafe {{ *t.{fld}.get_unchecked(idx) }} }} else {{ {zero} }};\n",
                        key = self.iop_str(*key)
                    ));
                } else if matches!(ty, StaticType::Float) {
                    out.push_str(&format!(
                        "{ind}let k = {key};\n\
                         {ind}if k < 0 {{ panic!(\"Runtime Error: Negative table index\"); }}\n\
                         {ind}let idx = k as usize;\n\
                         {ind}let t = unsafe {{ &*t_r{table} }};\n\
                         {ind}f_r{target} = if idx < t.farray.len() {{ unsafe {{ *t.farray.get_unchecked(idx) }} }} else {{ 0.0 }};\n",
                        key = self.iop_str(*key)
                    ));
                } else {
                    out.push_str(&format!(
                        "{ind}let k = {key};\n\
                         {ind}if k < 0 {{ panic!(\"Runtime Error: Negative table index\"); }}\n\
                         {ind}let idx = k as usize;\n\
                         {ind}let t = unsafe {{ &*t_r{table} }};\n\
                         {ind}i_r{target} = if idx < t.array.len() {{ unsafe {{ *t.array.get_unchecked(idx) }} }} else {{ 0 }};\n",
                        key = self.iop_str(*key)
                    ));
                }
            }
            Instruction::SetTableFast { table, key, val, ty } => {
                let val_str = if uses_handles && is_tbl(ty) {
                    format!("t_r{val}")
                } else if matches!(ty, StaticType::Float) {
                    self.fop_str(*val)
                } else if matches!(ty, StaticType::String) {
                    // clone — same immutable-copy semantics as the dyn store
                    format!("{}.clone()", self.sop_str(*val))
                } else {
                    self.iop_str(*val)
                };
                out.push_str(&format!(
                    "{ind}let k = {key};\n\
                     {ind}if k < 0 {{ panic!(\"Runtime Error: Negative index in fast path\"); }}\n\
                     {ind}if (k as usize) < len_r{table} {{\n\
                     {ind}    unsafe {{ *p_r{table}.add(k as usize) = {val_str}; }}\n\
                     {ind}}} else {{\n\
                     {ind}    panic!(\"optimizer invariant violated: fast-path bounds check failed\");\n\
                     {ind}}}\n",
                    key = self.iop_str(*key)
                ));
            }
            Instruction::GetTableFast { target, table, key, ty } => {
                // String fast reads clone through the hoisted pointer —
                // the shared template's plain deref would move out of the
                // Vec, which the borrow checker (rightly) forbids.
                if matches!(ty, StaticType::String) {
                    out.push_str(&format!(
                        "{ind}let k = {key};\n\
                         {ind}if k < 0 {{ panic!(\"Runtime Error: Negative index in fast path\"); }}\n\
                         {ind}if (k as usize) < len_r{table} {{\n\
                         {ind}    s_r{target} = unsafe {{ (*p_r{table}.add(k as usize)).clone() }};\n\
                         {ind}}} else {{\n\
                         {ind}    panic!(\"optimizer invariant violated: fast-path bounds check failed\");\n\
                         {ind}}}\n",
                        key = self.iop_str(*key)
                    ));
                } else {
                    let target_str = if uses_handles && is_tbl(ty) {
                        format!("t_r{target}")
                    } else if matches!(ty, StaticType::Float) {
                        format!("f_r{target}")
                    } else {
                        format!("i_r{target}")
                    };
                    out.push_str(&format!(
                        "{ind}let k = {key};\n\
                         {ind}if k < 0 {{ panic!(\"Runtime Error: Negative index in fast path\"); }}\n\
                         {ind}if (k as usize) < len_r{table} {{\n\
                         {ind}    {target_str} = unsafe {{ *p_r{table}.add(k as usize) }};\n\
                         {ind}}} else {{\n\
                         {ind}    panic!(\"optimizer invariant violated: fast-path bounds check failed\");\n\
                         {ind}}}\n",
                        key = self.iop_str(*key)
                    ));
                }
            }
            Instruction::Phi { .. } => {} // deleted by resolve_phis
        }
    }

    /// Every block reachable from `b` via any terminator edge.
    fn reachable_from(&self, b: BlockId) -> HashSet<BlockId> {
        let mut seen = HashSet::new();
        let mut stack = vec![b];
        while let Some(x) = stack.pop() {
            if !seen.insert(x) { continue; }
            match &self.program.blocks[x].terminator {
                Some(Terminator::Jump(t)) => stack.push(*t),
                Some(Terminator::Branch { true_block, false_block, .. }) => {
                    stack.push(*true_block);
                    stack.push(*false_block);
                }
                _ => {}
            }
        }
        seen
    }

    /// The block both if-arms converge on (None if they never rejoin, i.e.
    /// both paths halt — not producible by this language, defensive only).
    /// The join is minted immediately after both arms, and every other block
    /// reachable from BOTH arms sits behind the join — so it is exactly the
    /// smallest common block id above both arms.
    fn common_join(&self, tb: BlockId, fb: BlockId) -> Option<BlockId> {
        let rf = self.reachable_from(fb);
        self.reachable_from(tb).into_iter()
            .filter(|b| *b > tb.max(fb) && rf.contains(b))
            .min()
    }

    /// Emit block `b` and everything that follows it, staying inside the loop
    /// whose header is `hdr` (a back edge to `hdr` closes the loop body).
    /// `stop` is the join block of an enclosing structured if: reaching it
    /// ends this arm — the parent emits the join after both arms.
    fn emit_seq(&self, out: &mut String, b: BlockId, hdr: Option<BlockId>, stop: Option<BlockId>, d: usize, emitted: &mut [bool], uses_handle: bool) {
        if Some(b) == stop { return; }
        if emitted[b] { panic!("structured codegen: block {b} reached twice — CFG is not a tree"); }
        emitted[b] = true;
        let block = &self.program.blocks[b];
        for i in &block.instrs { self.emit_instr(out, i, d, uses_handle); }
        match &block.terminator {
            None | Some(Terminator::Halt) => {
                // early return == the dispatcher's `break 'cfg`: there is no
                // code after Halt, so jumping to the end is exactly a return
                out.push_str(&format!("{}return tables;\n", indent(d)));
            }
            Some(Terminator::Jump(t)) => {
                if Some(*t) == hdr {
                    // back edge: this loop body is complete
                } else if Some(*t) == stop {
                    // if-arm reached its join: the parent continues
                } else if *t < b {
                    panic!("structured codegen: stray backward jump {b} -> {t}");
                } else if self.is_loop_header(*t) {
                    // forward jump into a loop header = entering a loop
                    let (cond, tb, fb) = match &self.program.blocks[*t].terminator {
                        Some(Terminator::Branch { cond, true_block, false_block }) =>
                            (*cond, *true_block, *false_block),
                        _ => panic!("structured codegen: block {t} has a back edge but no Branch"),
                    };
                    self.emit_loop(out, *t, cond, tb, d, emitted, uses_handle);
                    self.emit_seq(out, fb, hdr, stop, d, emitted, uses_handle);
                } else {
                    self.emit_seq(out, *t, hdr, stop, d, emitted, uses_handle);
                }
            }
            Some(Terminator::Branch { cond, true_block, false_block }) if self.is_loop_header(b) => {
                // a header reached directly (not via its pre-header Jump):
                // same handling as the Jump-into-header case
                self.emit_loop(out, b, *cond, *true_block, d, emitted, uses_handle);
                self.emit_seq(out, *false_block, hdr, stop, d, emitted, uses_handle);
            }
            Some(Terminator::Branch { cond, true_block, false_block }) => {
                // non-header branch = structured if/else. Both arms converge
                // on the join block, which the parent emits after the arms.
                let join = self.common_join(*true_block, *false_block);
                let ind = indent(d);
                out.push_str(&format!("{ind}if {} {{\n", self.bop_str(*cond)));
                self.emit_seq(out, *true_block, hdr, join, d + 1, emitted, uses_handle);
                // skip an `else` that would be empty: bare else-block with
                // no instructions jumping straight to the join. The block
                // is still CONSUMED — mark it emitted, or the orphan check
                // below fires on the common `if c then flag = true end`
                // inside a loop (the coalesced loop phi turns the else
                // arm's join Move into a removable self-copy, re-emptying
                // the block; found by probe_ops_loop_forms).
                let trivial_else = self.program.blocks[*false_block].instrs.is_empty()
                    && matches!(&self.program.blocks[*false_block].terminator,
                                Some(Terminator::Jump(t)) if Some(*t) == join);
                if !trivial_else {
                    out.push_str(&format!("{ind}}} else {{\n"));
                    self.emit_seq(out, *false_block, hdr, join, d + 1, emitted, uses_handle);
                } else {
                    emitted[*false_block] = true;
                }
                out.push_str(&format!("{ind}}}\n"));
                if let Some(j) = join {
                    self.emit_seq(out, j, hdr, stop, d, emitted, uses_handle);
                }
            }
        }
    }

    fn emit_loop(&self, out: &mut String, h: BlockId, cond: RegId, body: BlockId, d: usize, emitted: &mut [bool], uses_handle: bool) {
        if !self.is_loop_header(h) {
            panic!("structured codegen: Branch in block {h} is not a loop header");
        }
        if emitted[h] { panic!("structured codegen: header {h} reached twice"); }
        emitted[h] = true;

        let block = &self.program.blocks[h];
        let ind = indent(d);

        // Pretty form: the header holds nothing (identifier condition, e.g.
        // phase I / bug16a) or exactly the Less computing the branch
        // condition with no other readers of its result. The Less folds into
        // the while-condition — still evaluated every iteration.
        let pretty = if block.instrs.is_empty() {
            Some(self.bop_str(cond))
        } else if block.instrs.len() == 1 {
            match &block.instrs[0] {
                Instruction::Less { target, left, right }
                    if *target == cond && self.reg_uses(cond) == 1 =>
                    Some(format!("{} < {}", self.iop_str(*left), self.iop_str(*right))),
                _ => None,
            }
        } else { None };

        if let Some(c) = pretty {
            out.push_str(&format!("{ind}while {c} {{\n"));
            self.emit_seq(out, body, Some(h), None, d + 1, emitted, uses_handle);
            out.push_str(&format!("{ind}}}\n"));
        } else {
            // General fallback: everything in the header runs every
            // iteration. Never fires on the current corpus — it exists so a
            // surprising CFG degrades to correct-but-ugly, not wrong.
            out.push_str(&format!("{ind}loop {{\n"));
            for i in &block.instrs { self.emit_instr(out, i, d + 1, uses_handle); }
            out.push_str(&format!("{}if {} {{\n", indent(d + 1), self.bop_str(cond)));
            self.emit_seq(out, body, Some(h), None, d + 2, emitted, uses_handle);
            out.push_str(&format!("{}    }} else {{\n", indent(d + 1)));
            out.push_str(&format!("{}        break;\n", indent(d + 1)));
            out.push_str(&format!("{}    }}\n{ind}}}\n", indent(d + 1)));
        }
    }

    pub fn generate_rust_code(&self) -> String {
        let uses_handles = program_uses_handles(&self.program.blocks);

        let mut out = String::new();
        out.push_str("// target/release/build/phia-*/out/baked_native.rs\n\n");
        out.push_str("use crate::memory::Table;\n\n");
        out.push_str("#[allow(unused_variables, unused_mut, unused_assignments)]\n");
        out.push_str("pub fn run_baked() -> Vec<Box<Table>> {\n");

        let mut fast_phys: HashSet<RegId> = HashSet::new();
        for b in &self.program.blocks {
            for i in &b.instrs {
                if let Instruction::HoistRawPtr { table }
                    | Instruction::SetTableFast { table, .. }
                    | Instruction::GetTableFast { table, .. } = i {
                    fast_phys.insert(*table);
                }
            }
        }

        let (
            n_i,
            n_b,
            n_f,
            n_s,
            n_t,
            n_tf,
            n_ts,
            base,
            fbase,
            tfbase,
            tsbase
        ) = (
            self.alloc.n_int,
            self.alloc.n_bool,
            self.alloc.n_float,
            self.alloc.n_str,
            self.alloc.n_table,
            self.alloc.n_ftable,
            self.alloc.n_tstr,
            self.alloc.phys_base as usize,
            self.alloc.float_base as usize,
            self.alloc.ftable_base as usize,
            self.alloc.tstr_base as usize
        );

        for r in base..base + n_i { out.push_str(&format!("    let mut i_r{r} = 0i64;\n")); }
        for r in base..base + n_b { out.push_str(&format!("    let mut b_r{r} = false;\n")); }
        for r in fbase..fbase + n_f { out.push_str(&format!("    let mut f_r{r} = 0f64;\n")); }
        for r in base..base + n_s { out.push_str(&format!("    let mut s_r{r} = String::new();\n")); }
        // handle tables: the shared id range first, then the disjoint
        // float-table and string-table ranges — same decl shape, pointer
        // type from the pool
        for r in base..base + n_t { self.emit_table_decl(&mut out, r as RegId, uses_handles, &fast_phys); }
        for r in tfbase..tfbase + n_tf { self.emit_table_decl(&mut out, r as RegId, uses_handles, &fast_phys); }
        for r in tsbase..tsbase + n_ts { self.emit_table_decl(&mut out, r as RegId, uses_handles, &fast_phys); }
        out.push_str("    let mut tables = Vec::<Box<Table>>::with_capacity(128);\n\n");

        let mut emitted = vec![false; self.program.blocks.len()];
        self.emit_seq(&mut out, 0, None, None, 1, &mut emitted, uses_handles);
        let orphans: Vec<usize> = emitted.iter().enumerate()
            .filter(|(_, e)| !**e).map(|(i, _)| i).collect();
        if !orphans.is_empty() {
            panic!("structured codegen: blocks never reached: {orphans:?}");
        }

        out.push_str("}\n");
        out
    }
}
