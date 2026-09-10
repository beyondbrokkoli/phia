# Phia 🌙

> 🌟 **Huge Shoutout to [logos](https://github.com/maciejhirsz/logos)!** 🌟
> This project would simply not be possible without the `logos` crate.

Phia is an ahead-of-time compiler for a statically typed Lua dialect: source in,
plain Rust out, `rustc` finishes the job.

## Why bother?

Fair question: why would anyone want a vibecoded Lua compiler? The honest
answer is pinned at the project root — [main.lua](main.lua).
It is a deliberately unfair synthetic workload, hammered through the patterns the optimizer
can prove — induction variables, affine keys, aliasing chains, hoist placement
across loop nests. Same program, two executables, reference machine:

```text
$ unset PHIA_SOURCE
$ cargo build --release
   Compiling phia v0.1.0 (/home/halim/phia)
    Finished `release` profile [optimized] target(s) in 0.35s
$ hyperfine './target/release/phia >/dev/null' 'luajit main.lua >/dev/null'
Benchmark 1: ./target/release/phia >/dev/null
  Time (mean ± σ):      1.809 s ±  0.035 s    [User: 1.798 s, System: 0.009 s]
  Range (min … max):    1.775 s …  1.860 s    10 runs

Benchmark 2: luajit main.lua >/dev/null
  Time (mean ± σ):     18.107 s ±  0.524 s    [User: 18.069 s, System: 0.020 s]
  Range (min … max):   17.536 s … 19.116 s    10 runs

Summary
  ./target/release/phia >/dev/null ran
   10.01 ± 0.35 times faster than luajit main.lua >/dev/null
```

## Known Limitations

A few standard Lua behaviors are currently unsupported:

*   **No Sparse Tables:** Phia does not implement the hash-map half of standard Lua tables. Table storage is strictly dense `Vec`s padded with the element kind's zero (`0`, `0.0`, or `""`). Writing to `t[1000000]` will instantly allocate and zero-fill a million slots.
*   **No Negative Table Indices:** Because tables are backed strictly by 0-indexed vectors, negative keys (e.g., `t[-1] = 42`) cannot overflow into a hash map like they do in Lua. Attempting to use a negative key is a checked runtime panic. 
*   **Strings and Heap Traffic:** String concatenation (`..`) emits Rust `format!()` calls, and string table stores emit `.clone()`. Heavy string manipulation in loops *will* hit the global allocator. 

 **Pinned divergences from Lua** (all enforced at build time, all locked by tests): no numeric coercion anywhere — mixed Integer/Float arithmetic and `2 .. "x"` are build errors; integer `/` is truncating (Lua's `/` always yields a Float); no ordering on strings;  
**Booleans are not storable in tables; negative table keys are a runtime panic (`t[-1]` is forbidden); and tables are strictly dense zero-indexed vectors, meaning sparse inserts like `t[1000000] = 1` will instantly zero-fill a million elements and likely OOM.**
 
**nil** is a reserved keyword, not a value. Absence is compiled into the
element kind's zero: an absent table key reads as `0`, `0.0`, `""`, or a null
table handle — and *using* a null handle is a checked `Runtime Error: table
is nil`, never undefined behavior.

`for` loops, `table.entry`, string keys in general and lots of other core lua features are still missing.

```lua
-- showcase.lua — the entire toolset in one program.

-- scalars: strings (concat), integers, floats, booleans
local name = "phia" .. "/" .. "lua"
local version = 1
local ratio = 0.25
local tuned = true

print("scalars", name, version, ratio, tuned)

-- Lua arithmetic semantics, integer side: trunc /, floor //, sign-of-divisor %
print("int_sem", 7 / -2, 7 // -2, -7 % 3, 9 - 4, (1 + 2) * 3)
-- float side: plain /, floor //, adjusted %
print("float_sem", 1.0 / 4.0, 0.75 // 0.5, 0.75 % 0.5, -ratio)
-- comparisons and boolean algebra
print("cmp", version < 2, ratio >= 0.25, name == "phia/lua", tuned ~= false, not tuned)

-- structured control: if / elseif / else (grade merges through a join phi)
local grade = 0
if version >= 2 then
    grade = 100
elseif version == 1 then
    grade = 50
else
    grade = 9
end

-- nested tables flip the program to arena handles (0 = nil, checked)
local grid = {}
local row = {}
row[0] = 99
grid[0] = row

-- one loop, three element kinds riding the same affine-store proof:
-- EC sizes all three to the limit in the pre-header, pointers hoist,
-- stores and the probe's read take the unsafe fast path.
local acc = {}
local wave = {}
local names = {}
local x = 0.0
local i = 0
while i <= 7 do
    local shadow = i * 100        -- scoped per-trip local
    acc[i] = i * i                -- integers
    wave[i] = x                   -- floats
    names[i] = name               -- strings (stored by clone)
    grid[0][i % 3] = shadow       -- non-affine key through a child: the
                                  -- proof declines, the store stays dyn
                                  -- and checked
    print("iter", i, acc, acc[i])
    x = x + 0.25
    i = i + 1
end

-- taming: a non-affine key declines the fast path — runtime-checked store
local log = {}
log[i * 13] = grade

-- absent keys read as the element kind's zero: 0 and ""
print("exit", acc[999], names[42])
print("tables", acc, wave, names, grid, row, log)
print("final", name, grade, row[0] == 99, acc[7] == 49)
```

## What works

### Optimizer Proofs & Safety

The compiler relies on strict static analysis to emit unsafe Rust without introducing undefined behavior. The proofs cover two notoriously difficult edge cases in ahead-of-time compilation for dynamic semantics:

*   **Lexical Allocation Identity:** While memory is arena-bound and never freed, table literals (`{}`) inside loops do not collapse into a single static handle. The compiler maps loop-scoped literals to an emitted arena `.push()` *inside* the generated Rust `loop {}` block. This guarantees that each iteration receives a fresh, distinct table handle, preserving standard Lua object identity and preventing cross-trip mutation bugs.
*   **Alias-Proofed Pointer Hoisting:** The optimizer can hoist raw pointers (`*mut T`) for affine loop inserts (e.g., `t[i] = v`) to bypass bounds checking. However, this is gated by strict escape analysis. If a hoisted table reference escapes into another structure (e.g., `wrap[1] = t`), a dynamic write to that parent (`wrap[1][k] = v`) could trigger a runtime `.resize()`, which would reallocate the backing vector and leave the hoisted pointer dangling. The analyzer detects this containment, explicitly revokes the affine proof, and demotes all stores for that table to the fully checked dynamic path.

**Types** — Integer (i64, wrapping like Lua), Float (f64), Boolean, String,
Table. A table has Integer keys and a monomorphic element kind decided by the
checker on first use: Integer, Float, String, or Table (nesting).

**Statements** — `local` (lexically scoped, shadowing allowed), assignment,
table assignment (`t[i] = v`, nested lvalues like `t[0][j] = v`), `while`,
`if` / `elseif` / `else`, and `print("tag", e1, e2, ...)` (a debug observation
point; prints one deterministic line per trip, naming each operand's physical
register — values, table handles and lengths only, never addresses). The
keyword is deliberate: `print` is Lua's own, so any Phia source runs as-is
under a real Lua interpreter — values agree except at the pinned divergences
(Lua's `/` is float division; `//` and `%` agree), and Lua's stable table
address across loop trips mirrors Phia's stable arena handle.

**Operators** — `+ - * / // %` on both numeric kinds with Lua semantics
(`/` truncates on integers, `//` floors, `%` takes the divisor's sign);
comparisons `< > <= >= == ~=`; `==` / `~=` also work on two Booleans or two
Strings; unary `-` and `not`; string concatenation `..` (two Strings, chains
fold left); parentheses.

## Memory is static

Every memory decision is made at build time. Element kinds, storage sides
(`array` / `farray` / `sarray`), register allocation — scalars compile to pre-declared Rust locals, so there is no stack machine and no heap traffic for **numeric or boolean** values **(strings, however, still rely on the global allocator)**. Tables live in one arena (`Vec<Box<Table>>`): a table is allocated
exactly once at its literal, never moves, never frees. There is no garbage
collector, no reference counting, no runtime type tags — nothing executes per
operation except the operation itself.

A table reference is a 1-based arena handle (0 = null, checked at use).
Because a table never moves and only ever grows, the optimizer can hoist raw
`(pointer, len)` pairs out of loops and emit `unsafe` direct writes — but only
behind a static proof; whenever the proof fails (non-affine keys, aliases it
cannot bound), the store stays on the fully checked dynamic path with nil
checks, bounds checks and explicit resizes. The `unsafe` blocks you see in
generated code are paid for by compiler-side proofs, not trusted.

The *result* of a program is its final arena state: for every table ever
created, `TABLE <id> LEN <n> NZ <n> CHECKSUM <n>` (position-weighted; `SUM`
additionally for floats — string elements are FNV-1a hashed into the same
formula), plus a `STATS` line counting fast/dynamic table operations and
pointer hoists. Interactive I/O is not part of this development phase.

## The compiled result

```rust
// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r146 = 0i64;
    let mut i_r147 = 0i64;
    let mut i_r148 = 0i64;
    let mut i_r149 = 0i64;
    let mut b_r146 = false;
    let mut b_r147 = false;
    let mut b_r148 = false;
    let mut b_r149 = false;
    let mut f_r157 = 0f64;
    let mut f_r158 = 0f64;
    let mut f_r159 = 0f64;
    let mut f_r160 = 0f64;
    let mut f_r161 = 0f64;
    let mut f_r162 = 0f64;
    let mut s_r146 = String::new();
    let mut s_r147 = String::new();
    let mut s_r148 = String::new();
    let mut t_r146 = 0i64;
    let mut t_r147 = 0i64;
    let mut t_r148 = 0i64;
    let mut p_r148: *mut i64 = std::ptr::null_mut();
    let mut len_r148 = 0usize;
    let mut t_r149 = 0i64;
    let mut t_r172 = 0i64;
    let mut p_r172: *mut f64 = std::ptr::null_mut();
    let mut len_r172 = 0usize;
    let mut t_r173 = 0i64;
    let mut p_r173: *mut String = std::ptr::null_mut();
    let mut len_r173 = 0usize;
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    s_r146 = "phia".to_string();
    s_r147 = "/".to_string();
    s_r148 = format!("{}{}", s_r146, s_r147);
    s_r147 = "lua".to_string();
    s_r146 = format!("{}{}", s_r148, s_r147);
    f_r157 = 0.25;
    println!(
        "PROBE scalars: s_r146={:?} i_r5={} f_r157={:?} b_r7={}",
        s_r146, 1, f_r157, true
    );
    i_r146 = 7 / -2;
    println!(
        "PROBE int_sem: i_r146={} i_r16={} i_r20={} i_r24={} i_r27={}",
        i_r146, -4, 2, 5, 9
    );
    f_r158 = 1.0;
    f_r159 = 4.0;
    f_r160 = f_r158 / f_r159;
    f_r159 = 0.75;
    f_r158 = 0.5;
    f_r161 = (f_r159 / f_r158).floor();
    f_r158 = 0.75;
    f_r159 = 0.5;
    f_r162 = f_r158 - (f_r158 / f_r159).floor() * f_r159;
    f_r159 = -f_r157;
    println!(
        "PROBE float_sem: f_r160={:?} f_r161={:?} f_r162={:?} f_r159={:?}",
        f_r160, f_r161, f_r162, f_r159
    );
    f_r159 = 0.25;
    b_r146 = f_r157 >= f_r159;
    s_r147 = "phia/lua".to_string();
    b_r147 = s_r146 == s_r147;
    b_r148 = true == false;
    b_r149 = !b_r148;
    println!(
        "PROBE cmp: b_r43={} b_r146={} b_r147={} b_r149={} b_r56={}",
        true, b_r146, b_r147, b_r149, false
    );
    if false {
        i_r146 = 100;
    } else {
        if true {
            i_r146 = 50;
        } else {
            i_r146 = 9;
        }
    }
    tables.push(Box::new(Table::new()));
    t_r146 = tables.len() as i64;
    tables.push(Box::new(Table::new()));
    t_r147 = tables.len() as i64;
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r147 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r147 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = 99;
    }
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r146 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r146 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = t_r147;
    }
    tables.push(Box::new(Table::new()));
    t_r148 = tables.len() as i64;
    tables.push(Box::new(Table::new_float()));
    t_r172 = tables.len() as i64;
    tables.push(Box::new(Table::new_string()));
    t_r173 = tables.len() as i64;
    f_r159 = 0.0;
    let lim = 8;
    if lim > 0 {
        if t_r148 == 0 {
            panic!("Runtime Error: table is nil");
        }
        let t = match tables.get_mut((t_r148 - 1) as usize) {
            Some(t) => &mut **t,
            None => panic!("Runtime Error: table is nil"),
        };
        if (lim as usize) > t.array.len() {
            t.array.resize(lim as usize, 0);
        }
    }
    if t_r148 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r148 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    len_r148 = t.array.len();
    p_r148 = t.array.as_mut_ptr();
    let lim = 8;
    if lim > 0 {
        if t_r172 == 0 {
            panic!("Runtime Error: table is nil");
        }
        let t = match tables.get_mut((t_r172 - 1) as usize) {
            Some(t) => &mut **t,
            None => panic!("Runtime Error: table is nil"),
        };
        if (lim as usize) > t.farray.len() {
            t.farray.resize(lim as usize, 0.0);
        }
    }
    if t_r172 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r172 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    len_r172 = t.farray.len();
    p_r172 = t.farray.as_mut_ptr();
    let lim = 8;
    if lim > 0 {
        if t_r173 == 0 {
            panic!("Runtime Error: table is nil");
        }
        let t = match tables.get_mut((t_r173 - 1) as usize) {
            Some(t) => &mut **t,
            None => panic!("Runtime Error: table is nil"),
        };
        if (lim as usize) > t.sarray.len() {
            t.sarray.resize(lim as usize, String::new());
        }
    }
    if t_r173 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r173 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    len_r173 = t.sarray.len();
    p_r173 = t.sarray.as_mut_ptr();
    f_r157 = f_r159;
    i_r147 = 0;
    loop {
        b_r149 = i_r147 < 8;
        if b_r149 {
            i_r148 = i_r147 * 100;
            i_r149 = i_r147 * i_r147;
            let k = i_r147;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r148 {
                unsafe {
                    *p_r148.add(k as usize) = i_r149;
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            let k = i_r147;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r172 {
                unsafe {
                    *p_r172.add(k as usize) = f_r157;
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            let k = i_r147;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r173 {
                unsafe {
                    *p_r173.add(k as usize) = s_r146.clone();
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            let k = 0;
            if k < 0 {
                panic!("Runtime Error: Negative table index");
            }
            let idx = k as usize;
            if t_r146 == 0 {
                panic!("Runtime Error: table is nil");
            }
            let t = match tables.get((t_r146 - 1) as usize) {
                Some(t) => &**t,
                None => panic!("Runtime Error: table is nil"),
            };
            t_r149 = if idx < t.array.len() {
                unsafe { *t.array.get_unchecked(idx) }
            } else {
                0
            };
            i_r149 = i_r147 % 3 + i64::from(i_r147 % 3 != 0 && (i_r147 % 3 < 0) != (3 < 0)) * 3;
            let k = i_r149;
            if k < 0 {
                panic!("Runtime Error: Negative table index");
            }
            let idx = k as usize;
            if t_r149 == 0 {
                panic!("Runtime Error: table is nil");
            }
            let t = match tables.get_mut((t_r149 - 1) as usize) {
                Some(t) => &mut **t,
                None => panic!("Runtime Error: table is nil"),
            };
            if idx >= t.array.len() {
                t.array.resize(idx + 1, 0);
            }
            unsafe {
                *t.array.get_unchecked_mut(idx) = i_r148;
            }
            let k = i_r147;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r148 {
                i_r149 = unsafe { *p_r148.add(k as usize) };
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            println!(
                "PROBE iter: i_r147={} t_r148={} len_r148={} i_r149={}",
                i_r147,
                t_r148,
                match tables.get((t_r148 - 1) as usize) {
                    Some(t) => t.array.len(),
                    None => usize::MAX,
                },
                i_r149
            );
            f_r159 = 0.25;
            f_r157 = f_r157 + f_r159;
            i_r147 = i_r147 + 1;
        } else {
            break;
        }
    }
    tables.push(Box::new(Table::new()));
    t_r149 = tables.len() as i64;
    i_r149 = i_r147 * 13;
    let k = i_r149;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r149 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r149 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = i_r146;
    }
    let k = 999;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r148 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get((t_r148 - 1) as usize) {
        Some(t) => &**t,
        None => panic!("Runtime Error: table is nil"),
    };
    i_r149 = if idx < t.array.len() {
        unsafe { *t.array.get_unchecked(idx) }
    } else {
        0
    };
    let k = 42;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r173 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get((t_r173 - 1) as usize) {
        Some(t) => &**t,
        None => panic!("Runtime Error: table is nil"),
    };
    s_r147 = if idx < t.sarray.len() {
        unsafe { t.sarray.get_unchecked(idx).clone() }
    } else {
        String::new()
    };
    println!("PROBE exit: i_r149={} s_r147={:?}", i_r149, s_r147);
    println!("PROBE tables: t_r148={} len_r148={} t_r172={} len_r172={} t_r173={} len_r173={} t_r146={} len_r146={} t_r147={} len_r147={} t_r149={} len_r149={}", t_r148, match tables.get((t_r148 - 1) as usize) { Some(t) => t.array.len(), None => usize::MAX }, t_r172, match tables.get((t_r172 - 1) as usize) { Some(t) => t.farray.len(), None => usize::MAX }, t_r173, match tables.get((t_r173 - 1) as usize) { Some(t) => t.sarray.len(), None => usize::MAX }, t_r146, match tables.get((t_r146 - 1) as usize) { Some(t) => t.array.len(), None => usize::MAX }, t_r147, match tables.get((t_r147 - 1) as usize) { Some(t) => t.array.len(), None => usize::MAX }, t_r149, match tables.get((t_r149 - 1) as usize) { Some(t) => t.array.len(), None => usize::MAX });
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r147 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get((t_r147 - 1) as usize) {
        Some(t) => &**t,
        None => panic!("Runtime Error: table is nil"),
    };
    i_r149 = if idx < t.array.len() {
        unsafe { *t.array.get_unchecked(idx) }
    } else {
        0
    };
    b_r149 = i_r149 == 99;
    let k = 7;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r148 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get((t_r148 - 1) as usize) {
        Some(t) => &**t,
        None => panic!("Runtime Error: table is nil"),
    };
    i_r149 = if idx < t.array.len() {
        unsafe { *t.array.get_unchecked(idx) }
    } else {
        0
    };
    b_r148 = i_r149 == 49;
    println!(
        "PROBE final: s_r146={:?} i_r146={} b_r149={} b_r148={}",
        s_r146, i_r146, b_r149, b_r148
    );
    return tables;
}

pub const STATS: &str = "fast_sets=3;fast_gets=1;dyn_sets=4;dyn_gets=5;hoists=3;hoist_ctx=0,0,0";
```

## Build

For v4 architecture, linux packaging symlinks might be missing

```
sudo ln -sf /usr/bin/x86_64_v4-linux-gnu-gcc /usr/bin/x86_64-linux-gnu-gcc
sudo ln -sf /usr/bin/x86_64_v4-linux-gnu-g++ /usr/bin/x86_64-linux-gnu-g++
```

```sh
cargo build --release        # no PHIA_SOURCE (or empty) -> compiles main.lua
./target/release/phia

PHIA_SOURCE=showcase.lua cargo build --release   # any other program
./target/release/phia
```

`lua tests/run_boss.lua` runs the full invariant analysis (positive / negative
/ panic tests plus a byte-diff of every generated `baked_native.rs` against
the frozen baselines in `tests/lock/`); `tests/milestone_lockdown.lua`
re-issues baselines at milestones. `PHIA_DEBUG_DUMP=mid|final|all` writes IR
dumps beside `baked_native.rs`, and any program containing `print` statements
also gets `probe_map.txt` there — the sidecar that joins runtime `PROBE` lines
to both dumps (tag → runtime line, MID vregs + phi ancestry, FINAL physical
registers).
