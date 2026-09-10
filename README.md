# Phia 🌙

> 🌟 **[logos](https://github.com/maciejhirsz/logos)!** 🌟
> This project would simply not be possible without the `logos` crate.

Phia is an ahead-of-time compiler for a statically typed Lua subset.

### Why bother?

Fair question: why build an ahead-of-time (AOT) compiler for a scripting language, especially when LuaJIT exists? The answer lies in the architectural limits of runtime execution, demonstrated by [main.lua](main.lua).

`main.lua` is not a general-purpose benchmark; it is a deliberately constructed stress test designed to isolate the exact boundaries where a Just-In-Time (JIT) compiler mathematically hits a wall, and where AOT static analysis takes over.

For the same program, executed on the same machine:

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
This 10x performance gap is not because LuaJIT is slow—18 seconds for 8+ billion dynamic iterations is an incredible feat of engineering. The gap exists because trace-based JIT compilers and AOT static analyzers operate under fundamentally different constraints:

1. **Proofs vs. Speculation:** Because Lua tables can grow dynamically, a JIT compiler must insert bailout guards and bounds checks into its generated machine code. If a loop writes past an array's capacity, the trace must abort, reallocate memory, and resume. **Phia** uses static Scalar Evolution (SCEV) to prove the exact capacity a loop will need *before* it runs, hoisting a single, exact memory allocation into the pre-header and entirely eliminating runtime bounds checks.
2. **The SIMD Barrier:** A trace JIT operates on sequential, scalar instructions interspersed with bailout guards. Because Phia proves memory safety ahead of time, it emits clean, branchless, unaliased Rust code (using `*mut T` pointers). This allows the LLVM backend to auto-vectorize the innermost loops, processing 4 to 8 array slots simultaneously per CPU cycle via hardware SIMD instructions.
3. **The Goal:** Phia is not designed to replace Lua for highly dynamic, metatable-heavy logic. Instead, it is an exploration of a specific thesis: **Can we achieve C/Rust-level performance (zero-cost abstractions, static memory guarantees, and SIMD vectorization) while writing purely in Lua syntax?**

Phia exists to prove that the performance tax on scripting languages is not in the syntax, but in the dynamic runtime semantics. By enforcing a strict typed subset and replacing runtime guessing with compile-time mathematical proofs, the engine unlocks an entirely different performance tier.

### Missing Implementations (TODOs)

* **Hash Maps and Sparse Tables:** Phia does not yet implement the hash-map half of standard Lua tables. Currently, table storage is backed by strictly dense, 0-indexed `Vec`s.
* *Warning:* Because of this, a sparse insert like `t[1000000] = 1` will instantly allocate and zero-fill a million elements, likely causing an OOM. Negative keys (`t[-1] = 42`) are a checked runtime panic, as they cannot currently overflow into a hash map.

* **Table Dot Notation:** Table property access via dot notation (e.g., `t.field`) and string keys in general are not yet supported.
* **Globals:** Global variable definitions are missing (all variables must currently be `local`).
* **Control Flow:** `for` loops and standard library functions are missing.
* **Optimized String Memory:** String operations currently generate heavy heap traffic. String concatenation (`..`) emits Rust `format!()` calls, and string table stores emit `.clone()`.
* **Boolean Storage:** Booleans cannot currently be stored in tables.

### Missing Memory Management

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
pointer hoists.
### Current Strict Semantics

To maintain performance and safety during this development phase, a few behaviors currently diverge from standard Lua and are strictly locked by tests:

* **No Numeric Coercion:** Mixed Integer/Float arithmetic and implicit string conversions (e.g., `2 .. "x"`) are strict build errors.
* **Integer Division:** The `/` operator performs truncating division for integers. In standard Lua, `/` always yields a Float.
* **No String Ordering:** Relational operators (`<`, `>`, etc.) cannot be used on strings.
* **The `nil` Keyword:** `nil` is treated as a reserved keyword rather than an actual value. Absence is compiled into the element kind's zero: an absent table key reads as `0`, `0.0`, `""`, or a null table handle. *Using* a null handle is a checked `Runtime Error: table is nil`, never undefined behavior.

### Optimizer Safety

The compiler relies on strict static analysis to emit unsafe, bare-metal Rust without introducing undefined behavior. The proofs cover notoriously difficult edge cases in ahead-of-time compilation for dynamic semantics, dynamically tiering between zero-cost raw pointer arithmetic and fully bounds-checked memory access.
* **Lexical Allocation Identity:** While memory is arena-bound and never freed, table literals (`{}`) inside loops do not collapse into a single static handle. The compiler maps loop-scoped literals to an emitted arena `.push()` *inside* the generated Rust `loop {}` block. This guarantees that each iteration receives a fresh, distinct table handle, preserving standard Lua object identity and preventing cross-trip mutation bugs.
* **Scalar Evolution (SCEV) & Pre-Header Allocation:** The affine analyzer does not just look for simple `t[i] = v` assignments. It statically evaluates linear math on loop induction variables (e.g., `t[i + 10] = v` or `t[i * 13] = v`). By projecting the loop bounds, the optimizer calculates the absolute maximum index the loop will ever touch and emits a single, exact `.resize(max_limit)` in the loop's pre-header. This ensures the backing vector capacity is guaranteed before the loop even starts.
* **Flow-Sensitive Alias Analysis:** Because tables are pre-sized in the pre-header, the optimizer can confidently hoist raw pointers (`*mut T`) and emit `unsafe { *ptr.add(...) }` direct writes inside the loop. The escape analysis is highly precise: if a table escapes into a parent structure (e.g., `history[i] = snapshot`), the analyzer does *not* automatically revoke the affine proof. It only revokes the proof if a subsequent dynamic write could trigger a `.resize()` *while* the raw pointer is still live. If the pointer's lifetime is cleanly scoped, the emitted `unsafe` block stays.
* **Data-Dependent Indirection Demotion:** When a loop contains memory-dependent indexing that cannot be solved statically (e.g., `t[other_t[i]] = v`), the optimizer instantly recognizes that the target index is arbitrary at runtime. It explicitly revokes the affine proof for that specific table and demotes its stores to the fully checked dynamic path. The generated Rust natively falls back to bounds-checked `.get_mut()` and on-the-fly `.resize()` calls, guaranteeing memory safety without crashing the generated binary.

```lua
-- showcase.lua — Pascal's Triangle (Optimizer Tripwire Edition)
-- This program is deliberately written to lay UB traps.

local pascal = {}
local n = 6
local row_idx = 0

while row_idx <= n do

    local row = {}
    local col_idx = 0

    -- TRIPWIRE: Dynamic Loop Bounds & Strict SCEV
    -- What could go wrong: A reckless optimizer sees `col_idx` incrementing
    -- and blindly hoists a raw pointer to skip bounds checks. But the loop limit
    -- `row_idx` mutates in the outer loop! If the compiler guesses the pre-size
    -- wrong, the inner loop causes a buffer overflow.
    -- How Phia handles it: Phia's Scalar Evolution (SCEV) engine detects that
    -- the inner loop bound is dynamic. Because it cannot mathematically prove
    -- the absolute maximum limit, it deliberately surrenders the affine proof.
    -- It emits safe, on-the-fly `.resize()` and `.get_mut()` checks inside
    -- the inner loop. (Check the STATS: hoists=0, fast_sets=0).

    while col_idx <= row_idx do
        if col_idx == 0 then
            row[col_idx] = 1
        elseif col_idx == row_idx then
            row[col_idx] = 1
        else
            local prev_row = pascal[row_idx - 1]
            row[col_idx] = prev_row[col_idx - 1] + prev_row[col_idx]
        end
        col_idx = col_idx + 1
    end

    pascal[row_idx] = row
    row_idx = row_idx + 1
end

print("pascal_mid", pascal[3][1], pascal[6][3])

-- TRIPWIRE: Data-Dependent Indirection Demotion
local unprovable_read = pascal[3][1] -- Evaluates to 3 at runtime
local fallback_table = {}

-- What could go wrong: If a data-dependent read is used as a table key, the
-- compiler has no way to predict the required memory size.
-- How Phia handles it: It instantly recognizes the indirection (`fallback_table[x]`).
-- Because runtime memory dictates the index, it explicitly demotes this store
-- to the fully checked dynamic path. No panics, no UB—just memory-safe Rust.
fallback_table[unprovable_read] = 99

print("safe_fallback", fallback_table[3])
```
```text
$ ./target/release/phia
PROBE pascal_mid: i_r69=3 i_r70=20
PROBE safe_fallback: i_r70=99
TABLE 0 LEN 7 NZ 7 CHECKSUM 168
TABLE 1 LEN 1 NZ 1 CHECKSUM 1
TABLE 2 LEN 2 NZ 2 CHECKSUM 3
TABLE 3 LEN 3 NZ 3 CHECKSUM 8
TABLE 4 LEN 4 NZ 4 CHECKSUM 20
TABLE 5 LEN 5 NZ 5 CHECKSUM 48
TABLE 6 LEN 6 NZ 6 CHECKSUM 112
TABLE 7 LEN 7 NZ 7 CHECKSUM 256
TABLE 8 LEN 4 NZ 1 CHECKSUM 396
STATS fast_sets=0;fast_gets=0;dyn_sets=5;dyn_gets=10;hoists=0;hoist_ctx=
TIME 32.445µs
```
### Basic Features

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

```lua
-- available_toolset.lua — the entire toolset in one program.

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

## Build
```
# 1. V4 DISTRO LINKER FIX

# Run this once if building on a v4-optimized Linux distro (CachyOS)
# fails with: `Error: linker x86_64-linux-gnu-gcc not found`
sudo ln -sf /usr/bin/x86_64_v4-linux-gnu-gcc /usr/bin/x86_64-linux-gnu-gcc
sudo ln -sf /usr/bin/x86_64_v4-linux-gnu-g++ /usr/bin/x86_64-linux-gnu-g++


# 2. COMPILING PROGRAMS

# If PHIA_SOURCE is unset or empty, it defaults to compiling main.lua
cargo build --release
./target/release/phia

# Compile a specific file (provide the env var on the same line)
PHIA_SOURCE=showcase.lua cargo build --release
./target/release/phia

# BULLETPROOF: 
# If Cargo refuses to recompile, touch the file first to update the timestamp
touch showcase.lua && PHIA_SOURCE=showcase.lua cargo build --release


# 3. DEBUG & IR DUMPS (Options: mid | final | all)

# Writes IR dumps and the probe_map.txt sidecar directly beside the 
# generated baked_native.rs file in the target/release/build/... folder.
PHIA_DEBUG_DUMP=all PHIA_SOURCE=main.lua cargo build --release
```
## Known Quirks: `print`

### 1. Strict Tag Requirement

Phia strictly requires a string literal tag as the first argument in a `print` statement (e.g., `print("tag", x)`).  
A plain `print(x)` is a syntax error.

**Why is the tag required?**
The compiler relies on this mandatory string to generate `probe_map.txt`.

**Why hijack `print` instead of adding a `probe()` keyword?**
Compatibility. By keeping the name `print`, you can run the exact same `.lua` script in standard Lua or LuaJIT without modification.

### 2. Ghost Registers (Optimize First, Print Later)

`print` does not act as a liveness barrier. Printing a DCE'd variable lets you watch the register allocator recycle memory in real-time.

**Example:**

```lua
local ghost = {}
local ghost_val = ghost[1] -- Dead code: never affects the final arena state

if true then
    local shadow = 42      -- Reuses ghost_val's physical register (r5)!
end

print("ghost_test", ghost_val)

```

**Execution Comparison:**

```text
$ luajit ghost_register.lua
ghost_test    nil

$ ./target/release/phia
PROBE ghost_test: i_r5=42

```

*(Here, `ghost_val` was optimized away, the allocator gave its physical slot to `shadow`, and the `print` probe blindly read the recycled memory containing `42`.)*
