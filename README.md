# Phia 🌙

> 🌟 **Huge Shoutout to [logos](https://github.com/maciejhirsz/logos)!** 🌟
> This project would simply not be possible without the `logos` crate.

## ✨ Supported Features

**Data Types**
- ✅ Integers
- ✅ "Tables" (keys and values must be integers)

**Control Flow**
- ✅ `while`
- ✅ `do` / `end` blocks
- ✅ Local variable scoping (`local`)

**Operators**
- ✅ Addition (`+`)
- ✅ Subtraction (`-`)
- ✅ Less Than (`<`)
- ✅ Assignment (`=`)

**Table Operations**
- ✅ Table creation (`{}`)
- ✅ Array access (`t[1]`)
- ✅ Table assignment (`t[i] = value`)

# Input
```lua
-- main.lua — "The Gauntlet"
-- Strict Phia subset: integers, tables, local, while, +, -, <.
--
-- 13 phases, one per optimizer outcome class, annotated against the
-- full post-ABCD pipeline: literal-bound materialization (lowerer),
-- region-wide poison (A), constant-offset key analysis (B), root-keyed
-- hoisting (C), region-wide upgrade (D).

-- PHASE A — The Stride. Non-unit IV step (+2) and the analysis doesn't
-- blink: the induction variable is still the raw phi (offset 0), the
-- bound pa_n is a pre-header LoadInt (def-block < header — loop-
-- invariant by dominance), so the gate opens on first contact. The
-- capacity contract pays once: EC(pa_n) zero-fills the whole span up
-- front, the ramp store lands only on even indices, and the odd slots
-- keep their EC zeros — half the table is written by the ABSENCE of a
-- store. One fast store per trip through the cached raw pointer; one
-- HR at depth 0. Final: LEN 8000000, NZ 4000000,
-- CHECKSUM 16000000000000 (the first 4,000,000 odd integers: sum = N^2).
local pa_t = {}
local pa_n = 8000000
local pa_i = 0
while pa_i < pa_n do
    pa_t[pa_i] = pa_i + 1
    pa_i = pa_i + 2
end

-- PHASE B — The Staircase. The inner bound is the OUTER induction phi
-- (pb_i): invariant FOR THE INNER LOOP, because dominance is checked
-- per-header — defined before the inner header, stable across the whole
-- inner trip since the outer bump lands in the tail, after. The outer
-- pass abstains on its own region scan: the write pb_t[φ_pb_j] does not
-- trace to φ_pb_i (wrong phi) → poison → no outer contract. The inner
-- pass owns the write: its own phi, offset 0, root defined pre-loop →
-- upgrade. EC+HR land in the outer BODY (hoist_ctx=1) and re-arm every
-- outer trip — this is the file's only EC whose limit is a live
-- register (`let lim = i_pb;`), un-foldable by constant propagation, so
-- the capacity guarantee is re-derived per iteration. Sound because EC
-- is idempotent (amortized Vec growth) and nothing resizes in-region.
-- O(pb_n^2/2) ramp stores into an L2-resident table — first hand of
-- the runtime clock. Final: LEN 99999, NZ 99999, CHECKSUM 4999950000.
local pb_t = {}
local pb_n = 100000
local pb_i = 0
while pb_i < pb_n do
    local pb_j = 0
    while pb_j < pb_i do
        pb_t[pb_j] = pb_j + 1
        pb_j = pb_j + 1
    end
    pb_i = pb_i + 1
end

-- PHASE C — The Polisher. Two sibling loops, one root, two contracts:
-- cross-loop dedup is deliberately out of scope, so each pass mints its
-- own EC+HR in its own pre-header (hoist_ctx=0,0). The second EC finds
-- len ≥ n already — a capacity no-op — but the HR re-derives p_r/len_r
-- anyway: the pointer cache is per-contract, not per-object. Loop 2 is
-- read-modify-write at the SAME safe key (φ_pc_j): the get folds to a
-- raw-pointer load (fast_get), the +1 is a scalar add, the set writes
-- back through the same cached pointer. Same table, hoisted twice, on
-- purpose. Final: LEN 2500000, NZ 2500000, CHECKSUM 15000000 (6s).
local pc_t = {}
local pc_n = 2500000
local pc_i = 0
while pc_i < pc_n do
    pc_t[pc_i] = 5
    pc_i = pc_i + 1
end
local pc_j = 0
while pc_j < pc_n do
    pc_t[pc_j] = pc_t[pc_j] + 1
    pc_j = pc_j + 1
end

-- PHASE D — The Sentinel. NUMBERS FROZEN — this is the phase that bites
-- when scaled blindly. The fill converts: the literal 30000 is
-- materialized into the pre-header by the lowerer (the LoadInt the
-- bounds gate needs, def-block < header), one EC, one fast store per
-- trip. The search loop does NOT convert, twice over: (1) its only
-- table op is the GetTable living in its own HEADER, and headers are
-- excluded from every scan region by construction — they hold phis and
-- condition lowering, never body candidates; (2) its key is the loop's
-- live-out, and idx_reg is that very read's RESULT — a def-use cycle,
-- structurally untraceable. So the scan runs on the dynamic path, which
-- is exactly what you want: termination is data-dependent (stop at the
-- first slot ≥ pd_lim; the sentinel pd_t[30] = 60 is planted by the
-- pre-header dyn write). RAISE pd_lim WITHOUT MOVING THE SENTINEL and
-- the reads walk past the array returning defined zeros forever —
-- 0 < pd_lim never fails, no panic fires, the program hangs. The
-- pre-header write and post-loop witness are dyn for free: neither
-- pre-headers nor post-loop code are ever scan candidates.
-- Final: pd_t LEN 30000, NZ 30000, CHECKSUM 30059; pd_w LEN 1, NZ 1,
-- CHECKSUM 30.
local pd_t = {}
local pd_fill = 0
while pd_fill < 30000 do
    pd_t[pd_fill] = 1
    pd_fill = pd_fill + 1
end
pd_t[30] = 60
local pd_lim = 60
local pd_i = 0
while pd_t[pd_i] < pd_lim do
    pd_i = pd_i + 1
end
local pd_w = {}
pd_w[0] = pd_i

-- PHASE E — The Frozen Handoff (the Patch-D flip, live in codegen).
-- pe_d = Move(φ_pe_i): SSA pins that register for the entire outer
-- iteration — the header's Less proved φ_pe_i < pe_n, and nothing in
-- the region can alter the register without minting a new vreg, which
-- would break the def-use trace and auto-decline. So the OUTER pass
-- upgrades a write that lives in the INNER body: region-wide PASS 2
-- walks into the nest, and EC(pe_n) + HR land BEFORE the outer loop
-- (hoist_ctx=0) — 2M outer iterations, 6M stores, one capacity payment.
-- The inner pass declines the very same instruction: pe_d does not
-- trace to φ_pe_j — and should not, since pe_d ranges over [0, pe_n)
-- against an inner limit of 3; converting there would arm the fast
-- path's invariant panic. One store, two verdicts, both correct: the
-- outer bound subsumes the key, the inner bound does not.
-- Final: LEN 2000000, NZ 2000000, CHECKSUM 14000000.
local pe_t = {}
local pe_n = 2000000
local pe_m = 3
local pe_i = 0
while pe_i < pe_n do
    local pe_d = pe_i
    local pe_j = 0
    while pe_j < pe_m do
        pe_t[pe_d] = 7
        pe_j = pe_j + 1
    end
    pe_i = pe_i + 1
end

-- PHASE F — The Carried Alias. Identical topology to E, opposite
-- resolution — and the difference is one def-use edge. Here the key IS
-- the inner phi (pf_d is the inner IV, reassigned in-region), so the
-- outer pass cannot trace it: loop-carried, poisoned, abstain. The
-- inner pass converts — offset 0 against its own limit pf_n — and its
-- EC+HR land in the outer BODY (hoist_ctx=1), re-armed every outer
-- trip: idempotent, sound, cheap. E vs F is the showcase pair: same
-- block structure, same nesting depth, hoists on opposite sides of the
-- outer loop — placement decided purely by WHICH phi the key's chain
-- terminates at. A frozen copy converts at the outer level; the live
-- counter converts at the inner. O(pf_n^2/2) stores, L2-resident —
-- second hand of the clock. Final: LEN 60000, NZ 60000,
-- CHECKSUM 1800030000.
local pf_t = {}
local pf_n = 60000
local pf_i = 0
while pf_i < pf_n do
    local pf_d = pf_i
    while pf_d < pf_n do
        pf_t[pf_d] = pf_d + 1
        pf_d = pf_d + 1
    end
    pf_i = pf_i + 1
end

-- PHASE G — The Mirror. Loop 1: textbook ramp fill, fast, HR at depth
-- 0. Loop 2 runs fast and dynamic SIDE BY SIDE, per-root: the read
-- pg_src[φ_pg_j] traces clean (fast_get, fresh HR for the src root);
-- the write pg_dst[pg_n - 1 - φ_pg_j] has a key whose trace dies in
-- non-constant arithmetic — the subtrahend IS the phi, so no constant
-- offset exists — unsafe write, poison lands on pg_dst's root only.
-- Alias analysis at root granularity: one poisoned chalice does not
-- spill into a sibling root. The dynamic write grows pg_dst by
-- resize-per-store — the exact per-access capacity tax the fast
-- contract exists to abolish, kept here on purpose as the contrast
-- specimen. Final: pg_src LEN 180, NZ 180, CHECKSUM 16290; pg_dst
-- LEN 180, NZ 180, CHECKSUM 16290 — mirror images, slot for slot.
local pg_src = {}
local pg_dst = {}
local pg_n = 180
local pg_i = 0
while pg_i < pg_n do
    pg_src[pg_i] = pg_i + 1
    pg_i = pg_i + 1
end
local pg_j = 0
while pg_j < pg_n do
    local pg_rev = pg_n - 1 - pg_j
    pg_dst[pg_rev] = pg_src[pg_j]
    pg_j = pg_j + 1
end

-- PHASE H — The Abacus. Fill loop converts (fast set, HR at 0). The
-- reduce loop's read keys off its own phi → fast_get through the cached
-- pointer; the accumulator is a loop-carried scalar (phi → coalesced to
-- one register; the add is the phase's only loop-carried dependence).
-- The witness store is post-loop → dyn, by design. Closed form
-- sum 1..n = n(n+1)/2, computed with + alone.
-- Final: ph_t LEN 1000000, NZ 1000000, CHECKSUM 500000500000;
-- ph_w LEN 1, NZ 1, CHECKSUM 500000500000.
local ph_t = {}
local ph_n = 1000000
local ph_i = 0
while ph_i < ph_n do
    ph_t[ph_i] = ph_i + 1
    ph_i = ph_i + 1
end
local ph_sum = 0
local ph_j = 0
while ph_j < ph_n do
    ph_sum = ph_sum + ph_t[ph_j]
    ph_j = ph_j + 1
end
local ph_w = {}
ph_w[0] = ph_sum

-- PHASE I — The One-Shot. The branch condition is a phi, not a Less:
-- def_map[cond] lands on the Phi node and the gate never opens — no
-- induction variable, no limit, no contract even formulable. Shape, not
-- conservativeness: there is nothing to prove. The loop runs once (flag
-- armed pre-loop, cleared in-body — codegen keeps the folded
-- `b = 0 < 0`). It is also the only genuine `while` in the emitted
-- file: every Less-headed loop could render in pretty form, but all
-- conditions share one physical bool register, so the uses==1
-- eligibility check fails and they render as loop{if{...}} — identical
-- semantics, identical machine-code shape, purely cosmetic demotion.
-- Final: LEN 1, NZ 1, CHECKSUM 100.
local pi_t = {}
local pi_flag = 0 < 1
local pi_k = 0
while pi_flag do
    pi_t[pi_k] = 100
    pi_flag = 0 < 0
    pi_k = pi_k + 1
end

-- PHASE J — The Terraces. One root, two epochs, two contracts, one
-- REALLOC: epoch 2's EC(350) outgrows the amortized capacity bought by
-- EC(200), the Vec moves, and the second HR re-derives the raw pointer
-- from the fresh allocation — the pointer cache is rebuilt, never
-- assumed. The non-zero IV origin (i = 100) stresses offset-blindness:
-- the key is the raw phi over [100, 350), and EC sizes to the LIMIT,
-- not the span — the unwritten prefix [0, 100) keeps its EC zeros. The
-- four witnesses probe the gap (t[99] = 0), both terraces, and the far
-- edge; post-loop code is never a scan candidate → 4 dyn reads, 4 dyn
-- writes, by design. Final: pj_t LEN 350, NZ 249, CHECKSUM 46125;
-- pj_w LEN 4, NZ 3, CHECKSUM 649 (50, 250, 349, 0 — the gap reads zero).
local pj_t = {}
local pj_hi1 = 200
local pj_i = 100
while pj_i < pj_hi1 do
    pj_t[pj_i] = pj_i - 100
    pj_i = pj_i + 1
end
local pj_hi2 = 350
local pj_j = 200
while pj_j < pj_hi2 do
    pj_t[pj_j] = pj_j
    pj_j = pj_j + 1
end
local pj_w = {}
pj_w[0] = pj_t[150]
pj_w[1] = pj_t[250]
pj_w[2] = pj_t[349]
pj_w[3] = pj_t[99]

-- PHASE K — The Cube. Triple nesting, single conversion, deepest hoist
-- in the file. The innermost write keys off φ_pk_k; that trace misses
-- φ_pk_j and φ_pk_i, so the region scan poisons pk_t for BOTH enclosing
-- passes — a poison minted at depth 2 propagates upward through every
-- enclosing candidate (their regions contain the inner body). The
-- innermost pass alone converts: EC+HR sit in the MIDDLE body
-- (hoist_ctx=2 — the deepest context in STATS), re-armed pk_n^2 =
-- 4,000,000 times to underwrite pk_n^3 = 8,000,000,000 stores — the
-- amortization ratio the whole contract exists to buy. The stored value
-- k + j is a ramp-plus-broadcast add, a vectorizable lane pattern: the
-- reason the cube finishes in seconds, not minutes.
-- Final: LEN 2000, NZ 2000, CHECKSUM 5997000 (every slot k + 1999).
local pk_t = {}
local pk_n = 2000
local pk_i = 0
while pk_i < pk_n do
    local pk_j = 0
    while pk_j < pk_n do
        local pk_k = 0
        while pk_k < pk_n do
            pk_t[pk_k] = pk_k + pk_j
            pk_k = pk_k + 1
        end
        pk_j = pk_j + 1
    end
    pk_i = pk_i + 1
end

-- PHASE L — The Poisoned Chalice, post-Patch-B: the chalice is empty
-- and the drink is safe. All three keys trace to φ_pl_i — pl_a and
-- pl_c by Move chains, pl_b by `+ 0`, a constant-offset fold to offset
-- 0. Note what tier-1 did NOT do: the Add still executes at runtime.
-- key_offset proves equality analytically; it does not need the add
-- folded dead — proof, not rewriting. No unsafe write in the region →
-- no poison → one EC + one HR underwrite all three stores. The file's
-- dyn_sets residue now lives in D, G, H, I, J: the chalice flipped to
-- fast and left the poison ledger entirely.
-- Final: LEN 8000, NZ 8000, CHECKSUM 24000 (last store wins: 3).
local pl_t = {}
local pl_n = 8000
local pl_i = 0
while pl_i < pl_n do
    local pl_a = pl_i
    pl_t[pl_a] = 1
    local pl_b = pl_a
    pl_b = pl_b + 0
    pl_t[pl_b] = 2
    local pl_c = pl_i
    pl_t[pl_c] = 3
    pl_i = pl_i + 1
end

-- PHASE M — The Handoff, post-Patch-C. pm_a and pm_b are two SSA
-- registers over ONE allocation — the Table Move between them is
-- root-traceable, so the alias is provably the same object, not an
-- assumed one. Both stores key off φ_pm_i and upgrade; the hoist set is
-- keyed by ROOT, so one EC + one HR cover both views (pre-C: two full
-- pairs, one per register — sound because EC is idempotent, but
-- structurally redundant). The fast stores are emitted against the root
-- register, pm_b's alias Move loses its last use, and simplify() reaps
-- it. Epilogue flourish: the physical table slot is recycled straight
-- from phase L — disjoint live ranges, 13 tables, 3 table registers,
-- zero spills.
-- Final: LEN 2000000, NZ 2000000, CHECKSUM 2000003000000 (pm_b's i+2
-- wins the same-slot write race).
local pm_a = {}
local pm_b = pm_a
local pm_n = 2000000
local pm_i = 0
while pm_i < pm_n do
    pm_a[pm_i] = pm_i + 1
    pm_b[pm_i] = pm_i + 2
    pm_i = pm_i + 1
end
```

# Output
```rust
// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r326 = 0i64;
    let mut i_r327 = 0i64;
    let mut i_r328 = 0i64;
    let mut i_r329 = 0i64;
    let mut i_r330 = 0i64;
    let mut b_r326 = false;
    let mut t_r326: *mut Table = std::ptr::null_mut();
    let mut p_r326: *mut i64 = std::ptr::null_mut();
    let mut len_r326 = 0usize;
    let mut t_r327: *mut Table = std::ptr::null_mut();
    let mut p_r327: *mut i64 = std::ptr::null_mut();
    let mut len_r327 = 0usize;
    let mut t_r328: *mut Table = std::ptr::null_mut();
    let mut p_r328: *mut i64 = std::ptr::null_mut();
    let mut len_r328 = 0usize;
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    let mut new_table = Box::new(Table::new());
    t_r326 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let lim = 8000000;
    if lim > 0 {
        let t = unsafe { &mut *t_r326 };
        if (lim as usize) > t.array.len() {
            t.array.resize(lim as usize, 0);
        }
    }
    len_r326 = unsafe { (*t_r326).array.len() };
    p_r326 = unsafe { (*t_r326).array.as_mut_ptr() };
    i_r326 = 0;
    loop {
        b_r326 = i_r326 < 8000000;
        if b_r326 {
            i_r327 = i_r326 + 1;
            let k = i_r326;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r326 {
                unsafe {
                    *p_r326.add(k as usize) = i_r327;
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            i_r326 = i_r326 + 2;
        } else {
            break;
        }
    }
    let mut new_table = Box::new(Table::new());
    t_r326 = &mut *new_table as *mut Table;
    tables.push(new_table);
    i_r327 = 0;
    loop {
        b_r326 = i_r327 < 100000;
        if b_r326 {
            let lim = i_r327;
            if lim > 0 {
                let t = unsafe { &mut *t_r326 };
                if (lim as usize) > t.array.len() {
                    t.array.resize(lim as usize, 0);
                }
            }
            len_r326 = unsafe { (*t_r326).array.len() };
            p_r326 = unsafe { (*t_r326).array.as_mut_ptr() };
            i_r326 = 0;
            loop {
                b_r326 = i_r326 < i_r327;
                if b_r326 {
                    i_r329 = i_r326 + 1;
                    let k = i_r326;
                    if k < 0 {
                        panic!("Runtime Error: Negative index in fast path");
                    }
                    if (k as usize) < len_r326 {
                        unsafe {
                            *p_r326.add(k as usize) = i_r329;
                        }
                    } else {
                        panic!("optimizer invariant violated: fast-path bounds check failed");
                    }
                    i_r326 = i_r326 + 1;
                } else {
                    break;
                }
            }
            i_r327 = i_r327 + 1;
        } else {
            break;
        }
    }
    let mut new_table = Box::new(Table::new());
    t_r327 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let lim = 2500000;
    if lim > 0 {
        let t = unsafe { &mut *t_r327 };
        if (lim as usize) > t.array.len() {
            t.array.resize(lim as usize, 0);
        }
    }
    len_r327 = unsafe { (*t_r327).array.len() };
    p_r327 = unsafe { (*t_r327).array.as_mut_ptr() };
    i_r328 = 0;
    loop {
        b_r326 = i_r328 < 2500000;
        if b_r326 {
            let k = i_r328;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r327 {
                unsafe {
                    *p_r327.add(k as usize) = 5;
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            i_r328 = i_r328 + 1;
        } else {
            break;
        }
    }
    let lim = 2500000;
    if lim > 0 {
        let t = unsafe { &mut *t_r327 };
        if (lim as usize) > t.array.len() {
            t.array.resize(lim as usize, 0);
        }
    }
    len_r327 = unsafe { (*t_r327).array.len() };
    p_r327 = unsafe { (*t_r327).array.as_mut_ptr() };
    i_r329 = 0;
    loop {
        b_r326 = i_r329 < 2500000;
        if b_r326 {
            let k = i_r329;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r327 {
                i_r328 = unsafe { *p_r327.add(k as usize) };
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            i_r326 = i_r328 + 1;
            let k = i_r329;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r327 {
                unsafe {
                    *p_r327.add(k as usize) = i_r326;
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            i_r329 = i_r329 + 1;
        } else {
            break;
        }
    }
    let mut new_table = Box::new(Table::new());
    t_r327 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let lim = 30000;
    if lim > 0 {
        let t = unsafe { &mut *t_r327 };
        if (lim as usize) > t.array.len() {
            t.array.resize(lim as usize, 0);
        }
    }
    len_r327 = unsafe { (*t_r327).array.len() };
    p_r327 = unsafe { (*t_r327).array.as_mut_ptr() };
    i_r326 = 0;
    loop {
        b_r326 = i_r326 < 30000;
        if b_r326 {
            let k = i_r326;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r327 {
                unsafe {
                    *p_r327.add(k as usize) = 1;
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            i_r326 = i_r326 + 1;
        } else {
            break;
        }
    }
    let k = 30;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r327 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = 60;
    }
    i_r326 = 0;
    loop {
        let k = i_r326;
        if k < 0 {
            panic!("Runtime Error: Negative table index");
        }
        let idx = k as usize;
        let t = unsafe { &*t_r327 };
        i_r328 = if idx < t.array.len() {
            unsafe { *t.array.get_unchecked(idx) }
        } else {
            0
        };
        b_r326 = i_r328 < 60;
        if b_r326 {
            i_r326 = i_r326 + 1;
        } else {
            break;
        }
    }
    let mut new_table = Box::new(Table::new());
    t_r327 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r327 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = i_r326;
    }
    let mut new_table = Box::new(Table::new());
    t_r327 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let lim = 2000000;
    if lim > 0 {
        let t = unsafe { &mut *t_r327 };
        if (lim as usize) > t.array.len() {
            t.array.resize(lim as usize, 0);
        }
    }
    len_r327 = unsafe { (*t_r327).array.len() };
    p_r327 = unsafe { (*t_r327).array.as_mut_ptr() };
    i_r328 = 0;
    loop {
        b_r326 = i_r328 < 2000000;
        if b_r326 {
            i_r326 = i_r328;
            i_r329 = 0;
            loop {
                b_r326 = i_r329 < 3;
                if b_r326 {
                    let k = i_r326;
                    if k < 0 {
                        panic!("Runtime Error: Negative index in fast path");
                    }
                    if (k as usize) < len_r327 {
                        unsafe {
                            *p_r327.add(k as usize) = 7;
                        }
                    } else {
                        panic!("optimizer invariant violated: fast-path bounds check failed");
                    }
                    i_r329 = i_r329 + 1;
                } else {
                    break;
                }
            }
            i_r328 = i_r328 + 1;
        } else {
            break;
        }
    }
    let mut new_table = Box::new(Table::new());
    t_r326 = &mut *new_table as *mut Table;
    tables.push(new_table);
    i_r327 = 0;
    loop {
        b_r326 = i_r327 < 60000;
        if b_r326 {
            i_r329 = i_r327;
            let lim = 60000;
            if lim > 0 {
                let t = unsafe { &mut *t_r326 };
                if (lim as usize) > t.array.len() {
                    t.array.resize(lim as usize, 0);
                }
            }
            len_r326 = unsafe { (*t_r326).array.len() };
            p_r326 = unsafe { (*t_r326).array.as_mut_ptr() };
            i_r326 = i_r329;
            loop {
                b_r326 = i_r326 < 60000;
                if b_r326 {
                    i_r328 = i_r326 + 1;
                    let k = i_r326;
                    if k < 0 {
                        panic!("Runtime Error: Negative index in fast path");
                    }
                    if (k as usize) < len_r326 {
                        unsafe {
                            *p_r326.add(k as usize) = i_r328;
                        }
                    } else {
                        panic!("optimizer invariant violated: fast-path bounds check failed");
                    }
                    i_r326 = i_r326 + 1;
                } else {
                    break;
                }
            }
            i_r327 = i_r327 + 1;
        } else {
            break;
        }
    }
    let mut new_table = Box::new(Table::new());
    t_r327 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let mut new_table = Box::new(Table::new());
    t_r328 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let lim = 180;
    if lim > 0 {
        let t = unsafe { &mut *t_r327 };
        if (lim as usize) > t.array.len() {
            t.array.resize(lim as usize, 0);
        }
    }
    len_r327 = unsafe { (*t_r327).array.len() };
    p_r327 = unsafe { (*t_r327).array.as_mut_ptr() };
    i_r329 = 0;
    loop {
        b_r326 = i_r329 < 180;
        if b_r326 {
            i_r328 = i_r329 + 1;
            let k = i_r329;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r327 {
                unsafe {
                    *p_r327.add(k as usize) = i_r328;
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            i_r329 = i_r329 + 1;
        } else {
            break;
        }
    }
    let lim = 180;
    if lim > 0 {
        let t = unsafe { &mut *t_r327 };
        if (lim as usize) > t.array.len() {
            t.array.resize(lim as usize, 0);
        }
    }
    len_r327 = unsafe { (*t_r327).array.len() };
    p_r327 = unsafe { (*t_r327).array.as_mut_ptr() };
    i_r328 = 0;
    loop {
        b_r326 = i_r328 < 180;
        if b_r326 {
            i_r329 = 179 - i_r328;
            let k = i_r328;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r327 {
                i_r326 = unsafe { *p_r327.add(k as usize) };
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            let k = i_r329;
            if k < 0 {
                panic!("Runtime Error: Negative table index");
            }
            let idx = k as usize;
            let t = unsafe { &mut *t_r328 };
            if idx >= t.array.len() {
                t.array.resize(idx + 1, 0);
            }
            unsafe {
                *t.array.get_unchecked_mut(idx) = i_r326;
            }
            i_r328 = i_r328 + 1;
        } else {
            break;
        }
    }
    let mut new_table = Box::new(Table::new());
    t_r328 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let lim = 1000000;
    if lim > 0 {
        let t = unsafe { &mut *t_r328 };
        if (lim as usize) > t.array.len() {
            t.array.resize(lim as usize, 0);
        }
    }
    len_r328 = unsafe { (*t_r328).array.len() };
    p_r328 = unsafe { (*t_r328).array.as_mut_ptr() };
    i_r326 = 0;
    loop {
        b_r326 = i_r326 < 1000000;
        if b_r326 {
            i_r329 = i_r326 + 1;
            let k = i_r326;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r328 {
                unsafe {
                    *p_r328.add(k as usize) = i_r329;
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            i_r326 = i_r326 + 1;
        } else {
            break;
        }
    }
    let lim = 1000000;
    if lim > 0 {
        let t = unsafe { &mut *t_r328 };
        if (lim as usize) > t.array.len() {
            t.array.resize(lim as usize, 0);
        }
    }
    len_r328 = unsafe { (*t_r328).array.len() };
    p_r328 = unsafe { (*t_r328).array.as_mut_ptr() };
    i_r329 = 0;
    i_r326 = 0;
    loop {
        b_r326 = i_r329 < 1000000;
        if b_r326 {
            let k = i_r329;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r328 {
                i_r328 = unsafe { *p_r328.add(k as usize) };
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            i_r326 = i_r326 + i_r328;
            i_r329 = i_r329 + 1;
        } else {
            break;
        }
    }
    let mut new_table = Box::new(Table::new());
    t_r328 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r328 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = i_r326;
    }
    let mut new_table = Box::new(Table::new());
    t_r328 = &mut *new_table as *mut Table;
    tables.push(new_table);
    b_r326 = true;
    i_r328 = 0;
    while b_r326 {
        let k = i_r328;
        if k < 0 {
            panic!("Runtime Error: Negative table index");
        }
        let idx = k as usize;
        let t = unsafe { &mut *t_r328 };
        if idx >= t.array.len() {
            t.array.resize(idx + 1, 0);
        }
        unsafe {
            *t.array.get_unchecked_mut(idx) = 100;
        }
        b_r326 = 0 < 0;
        i_r328 = i_r328 + 1;
    }
    let mut new_table = Box::new(Table::new());
    t_r328 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let lim = 200;
    if lim > 0 {
        let t = unsafe { &mut *t_r328 };
        if (lim as usize) > t.array.len() {
            t.array.resize(lim as usize, 0);
        }
    }
    len_r328 = unsafe { (*t_r328).array.len() };
    p_r328 = unsafe { (*t_r328).array.as_mut_ptr() };
    i_r328 = 100;
    loop {
        b_r326 = i_r328 < 200;
        if b_r326 {
            i_r326 = i_r328 - 100;
            let k = i_r328;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r328 {
                unsafe {
                    *p_r328.add(k as usize) = i_r326;
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            i_r328 = i_r328 + 1;
        } else {
            break;
        }
    }
    let lim = 350;
    if lim > 0 {
        let t = unsafe { &mut *t_r328 };
        if (lim as usize) > t.array.len() {
            t.array.resize(lim as usize, 0);
        }
    }
    len_r328 = unsafe { (*t_r328).array.len() };
    p_r328 = unsafe { (*t_r328).array.as_mut_ptr() };
    i_r326 = 200;
    loop {
        b_r326 = i_r326 < 350;
        if b_r326 {
            let k = i_r326;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r328 {
                unsafe {
                    *p_r328.add(k as usize) = i_r326;
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            i_r326 = i_r326 + 1;
        } else {
            break;
        }
    }
    let mut new_table = Box::new(Table::new());
    t_r327 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let k = 150;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &*t_r328 };
    i_r326 = if idx < t.array.len() {
        unsafe { *t.array.get_unchecked(idx) }
    } else {
        0
    };
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r327 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = i_r326;
    }
    let k = 250;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &*t_r328 };
    i_r326 = if idx < t.array.len() {
        unsafe { *t.array.get_unchecked(idx) }
    } else {
        0
    };
    let k = 1;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r327 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = i_r326;
    }
    let k = 349;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &*t_r328 };
    i_r326 = if idx < t.array.len() {
        unsafe { *t.array.get_unchecked(idx) }
    } else {
        0
    };
    let k = 2;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r327 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = i_r326;
    }
    let k = 99;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &*t_r328 };
    i_r326 = if idx < t.array.len() {
        unsafe { *t.array.get_unchecked(idx) }
    } else {
        0
    };
    let k = 3;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    let t = unsafe { &mut *t_r327 };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = i_r326;
    }
    let mut new_table = Box::new(Table::new());
    t_r327 = &mut *new_table as *mut Table;
    tables.push(new_table);
    i_r326 = 0;
    loop {
        b_r326 = i_r326 < 2000;
        if b_r326 {
            i_r328 = 0;
            loop {
                b_r326 = i_r328 < 2000;
                if b_r326 {
                    let lim = 2000;
                    if lim > 0 {
                        let t = unsafe { &mut *t_r327 };
                        if (lim as usize) > t.array.len() {
                            t.array.resize(lim as usize, 0);
                        }
                    }
                    len_r327 = unsafe { (*t_r327).array.len() };
                    p_r327 = unsafe { (*t_r327).array.as_mut_ptr() };
                    i_r327 = 0;
                    loop {
                        b_r326 = i_r327 < 2000;
                        if b_r326 {
                            i_r330 = i_r327 + i_r328;
                            let k = i_r327;
                            if k < 0 {
                                panic!("Runtime Error: Negative index in fast path");
                            }
                            if (k as usize) < len_r327 {
                                unsafe {
                                    *p_r327.add(k as usize) = i_r330;
                                }
                            } else {
                                panic!(
                                    "optimizer invariant violated: fast-path bounds check failed"
                                );
                            }
                            i_r327 = i_r327 + 1;
                        } else {
                            break;
                        }
                    }
                    i_r328 = i_r328 + 1;
                } else {
                    break;
                }
            }
            i_r326 = i_r326 + 1;
        } else {
            break;
        }
    }
    let mut new_table = Box::new(Table::new());
    t_r328 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let lim = 8000;
    if lim > 0 {
        let t = unsafe { &mut *t_r328 };
        if (lim as usize) > t.array.len() {
            t.array.resize(lim as usize, 0);
        }
    }
    len_r328 = unsafe { (*t_r328).array.len() };
    p_r328 = unsafe { (*t_r328).array.as_mut_ptr() };
    i_r329 = 0;
    loop {
        b_r326 = i_r329 < 8000;
        if b_r326 {
            i_r330 = i_r329;
            let k = i_r330;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r328 {
                unsafe {
                    *p_r328.add(k as usize) = 1;
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            i_r327 = i_r330 + 0;
            let k = i_r327;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r328 {
                unsafe {
                    *p_r328.add(k as usize) = 2;
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            i_r327 = i_r329;
            let k = i_r327;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r328 {
                unsafe {
                    *p_r328.add(k as usize) = 3;
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            i_r329 = i_r329 + 1;
        } else {
            break;
        }
    }
    let mut new_table = Box::new(Table::new());
    t_r328 = &mut *new_table as *mut Table;
    tables.push(new_table);
    let lim = 2000000;
    if lim > 0 {
        let t = unsafe { &mut *t_r328 };
        if (lim as usize) > t.array.len() {
            t.array.resize(lim as usize, 0);
        }
    }
    len_r328 = unsafe { (*t_r328).array.len() };
    p_r328 = unsafe { (*t_r328).array.as_mut_ptr() };
    i_r327 = 0;
    loop {
        b_r326 = i_r327 < 2000000;
        if b_r326 {
            i_r329 = i_r327 + 1;
            let k = i_r327;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r328 {
                unsafe {
                    *p_r328.add(k as usize) = i_r329;
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            i_r329 = i_r327 + 2;
            let k = i_r327;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r328 {
                unsafe {
                    *p_r328.add(k as usize) = i_r329;
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            i_r327 = i_r327 + 1;
        } else {
            break;
        }
    }
    return tables;
}

pub const STATS: &str = "fast_sets=17;fast_gets=3;dyn_sets=9;dyn_gets=5;hoists=16;hoist_ctx=0,1,0,0,0,0,1,0,0,0,0,0,0,0,2,0";
```

## Proof Of Concept

```bash
Benchmark 1: ./target/release/phia > /dev/null
  Time (mean ± σ):      1.876 s ±  0.037 s    [User: 1.852 s, System: 0.008 s]
  Range (min … max):    1.850 s …  1.902 s    2 runs

Benchmark 2: luajit main.lua > /dev/null
  Time (mean ± σ):     13.173 s ±  0.137 s    [User: 13.033 s, System: 0.023 s]
  Range (min … max):   13.076 s … 13.270 s    2 runs

Summary
  ./target/release/phia > /dev/null ran
    7.02 ± 0.16 times faster than luajit main.lua > /dev/null
```
