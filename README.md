# Phia 🌙

> 🌟 **Huge Shoutout to [logos](https://github.com/maciejhirsz/logos)!** 🌟
> This project would simply not be possible without the `logos` crate.

Phia is an ahead-of-time compiler for a statically typed Lua dialect: source in,
plain Rust out, `rustc` finishes the job. Compilation is a pure function from
source to bytes — the same input re-derives the same output, byte for byte,
which the test harness proves against frozen baselines on every run.

## The toolset

**Types** — Integer (i64, wrapping like Lua), Float (f64), Boolean, String,
Table. A table has Integer keys and a monomorphic element kind decided by the
checker on first use: Integer, Float, String, or Table (nesting).

**Statements** — `local` (lexically scoped, shadowing allowed), assignment,
table assignment (`t[i] = v`, nested lvalues like `t[0][j] = v`), `while`,
`if` / `elseif` / `else`, and `probe("tag", e1, e2, ...)` (a debug observation
point; prints one deterministic line per trip, naming each operand's physical
register — values, table handles and lengths only, never addresses).

**Operators** — `+ - * / // %` on both numeric kinds with Lua semantics
(`/` truncates on integers, `//` floors, `%` takes the divisor's sign);
comparisons `< > <= >= == ~=`; `==` / `~=` also work on two Booleans or two
Strings; unary `-` and `not`; string concatenation `..` (two Strings, chains
fold left); parentheses.

**Pinned divergences from Lua** (all enforced at build time, all locked by
tests): no numeric coercion anywhere — mixed Integer/Float arithmetic and
`2 .. "x"` are build errors; integer `/` is truncating (Lua's `/` always
yields a Float); no ordering on strings; Booleans are not storable in tables.

**nil** is a reserved keyword, not a value. Absence is compiled into the
element kind's zero: an absent table key reads as `0`, `0.0`, `""`, or a null
table handle — and *using* a null handle is a checked `Runtime Error: table
is nil`, never undefined behavior.

## Memory is static

Every memory decision is made at build time. Element kinds, storage sides
(`array` / `farray` / `sarray`), register allocation — scalars compile to
pre-declared Rust locals, so there is no stack machine and no heap traffic for
values. Tables live in one arena (`Vec<Box<Table>>`): a table is allocated
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

## The showcase

One program, every feature at least once ([showcase.lua](showcase.lua)):

```lua
-- showcase.lua — the entire toolset in one program.

-- scalars: strings (concat), integers, floats, booleans
local name = "phia" .. "/" .. "lua"
local version = 1
local ratio = 0.25
local tuned = true

probe("scalars", name, version, ratio, tuned)

-- Lua arithmetic semantics, integer side: trunc /, floor //, sign-of-divisor %
probe("int_sem", 7 / -2, 7 // -2, -7 % 3, 9 - 4, (1 + 2) * 3)
-- float side: plain /, floor //, adjusted %
probe("float_sem", 1.0 / 4.0, 0.75 // 0.5, 0.75 % 0.5, -ratio)
-- comparisons and boolean algebra
probe("cmp", version < 2, ratio >= 0.25, name == "phia/lua", tuned ~= false, not tuned)

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
    probe("iter", i, acc, acc[i])
    x = x + 0.25
    i = i + 1
end

-- taming: a non-affine key declines the fast path — runtime-checked store
local log = {}
log[i * 13] = grade

-- absent keys read as the element kind's zero: 0 and ""
probe("exit", acc[999], names[42])
probe("tables", acc, wave, names, grid, row, log)
probe("final", name, grade, row[0] == 99, acc[7] == 49)
```

What to look for when reading the output below:

- `len_r148=8` from the **first** trip: `while i <= 7` desugars to `i < 7+1`
  and the pre-header `EnsureCapacity` sizes all three tables to 8 *before*
  iteration zero — integer, float and string storage ride the same proof.
- The loop body writes `acc`/`wave`/`names` through hoisted raw pointers
  (`*p_r148.add(k)`, `*p_r172.add(k)`, `*p_r173.add(k)`), but `grid[0][i % 3]`
  and `log[i * 13]` stay on the fully checked dynamic path — non-affine keys
  decline the fast path; safety is never traded for speed the compiler cannot
  prove.
- `row[0] == 99` prints `false`: `grid[0]` *is* `row`, and the matrix store
  overwrote slot 0 — aliasing, observable in the program's own output.
- `acc[999]` reads `0` and `names[42]` reads `""` — absence as the element
  kind's zero.

Program output (the `TIME` line is wall clock, the only nondeterministic one):

```text
PROBE scalars: s_r146="phia/lua" i_r5=1 f_r157=0.25 b_r7=true
PROBE int_sem: i_r146=-3 i_r16=-4 i_r20=2 i_r24=5 i_r27=9
PROBE float_sem: f_r160=0.25 f_r161=1.0 f_r162=0.25 f_r159=-0.25
PROBE cmp: b_r43=true b_r146=true b_r147=true b_r149=true b_r56=false
PROBE iter: i_r147=0 t_r148=3 len_r148=8 i_r149=0
PROBE iter: i_r147=1 t_r148=3 len_r148=8 i_r149=1
PROBE iter: i_r147=2 t_r148=3 len_r148=8 i_r149=4
PROBE iter: i_r147=3 t_r148=3 len_r148=8 i_r149=9
PROBE iter: i_r147=4 t_r148=3 len_r148=8 i_r149=16
PROBE iter: i_r147=5 t_r148=3 len_r148=8 i_r149=25
PROBE iter: i_r147=6 t_r148=3 len_r148=8 i_r149=36
PROBE iter: i_r147=7 t_r148=3 len_r148=8 i_r149=49
PROBE exit: i_r149=0 s_r147=""
PROBE tables: t_r148=3 len_r148=8 t_r172=4 len_r172=8 t_r173=5 len_r173=8 t_r146=1 len_r146=1 t_r147=2 len_r147=3 t_r149=6 len_r149=105
PROBE final: s_r146="phia/lua" i_r146=50 b_r149=false b_r148=true
TABLE 0 LEN 1 NZ 1 CHECKSUM 2
TABLE 1 LEN 3 NZ 3 CHECKSUM 3500
TABLE 2 LEN 8 NZ 7 CHECKSUM 924
TABLE 3 LEN 8 NZ 7 CHECKSUM -4760304806130614272 SUM 7
TABLE 4 LEN 8 NZ 8 CHECKSUM 4463244024491530904
TABLE 5 LEN 105 NZ 1 CHECKSUM 5250
STATS fast_sets=3;fast_gets=1;dyn_sets=4;dyn_gets=5;hoists=3;hoist_ctx=0,0,0
```

# Build Notice

For v4 architecture, link missing /usr/bin/x86_64-linux-gnu-gcc and /usr/bin/x86_64-linux-gnu-g++
```
sudo ln -sf /usr/bin/x86_64_v4-linux-gnu-gcc /usr/bin/x86_64-linux-gnu-gcc
sudo ln -sf /usr/bin/x86_64_v4-linux-gnu-g++ /usr/bin/x86_64-linux-gnu-g++
```

## The compiled result

The generated Rust, 1:1 from `target/release/build/phia-*/out/baked_native.rs`:

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

## Building and the harness

```sh
PHIA_SOURCE=showcase.lua cargo build --release
./target/release/phia
```

`lua tests/run_boss.lua` runs the full invariant analysis (positive / negative
/ panic tests plus a byte-diff of every generated `baked_native.rs` against
the frozen baselines in `tests/lock/`); `tests/milestone_lockdown.lua`
re-issues baselines at milestones. `PHIA_DEBUG_DUMP=mid|final|all` writes IR
dumps beside `baked_native.rs`, and any program containing `probe` statements
also gets `probe_map.txt` there — the sidecar that joins runtime `PROBE` lines
to both dumps (tag → runtime line, MID vregs + phi ancestry, FINAL physical
registers).
