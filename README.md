# Phia 🌙

[![License: MIT/Apache-2.0](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)
[![Status: Experimental AOT](https://img.shields.io/badge/status-experimental-orange.svg)]()

> 🌟 **[logos](https://github.com/maciejhirsz/logos)!** 🌟
> This project would simply not be possible without the `logos` crate.

Phia is an ahead-of-time compiler for a statically typed Lua subset.

### Why bother?

Fair question: why build an ahead-of-time (AOT) compiler for a scripting language, especially when LuaJIT exists? The answer lies in the architectural limits of runtime execution, demonstrated by [main.lua](main.lua).

`main.lua` is not a general-purpose benchmark; it is a deliberately constructed stress test designed to isolate the exact boundaries where a Just-In-Time (JIT) compiler mathematically hits a wall, and where AOT static analysis takes over.

For the same program, executed on the same machine:

```text
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
This performance gap is not because LuaJIT is slow, it exists because trace-based JIT compilers and AOT static analyzers operate under fundamentally different constraints:

1. **Proofs vs. Speculation:** Because Lua tables can grow dynamically, a JIT compiler must insert bailout guards and bounds checks into its generated machine code. If a loop writes past an array's capacity, the trace must abort, reallocate memory, and resume. **Phia** uses static Scalar Evolution (SCEV) to prove the exact capacity a loop will need *before* it runs, hoisting a single, exact memory allocation into the pre-header and entirely eliminating runtime bounds checks.
2. **The SIMD Barrier:** A trace JIT operates on sequential, scalar instructions interspersed with bailout guards. Because Phia proves memory safety ahead of time, it emits clean, branchless, unaliased Rust code (using `*mut T` pointers). This allows the LLVM backend to auto-vectorize the innermost loops, processing 4 to 8 array slots simultaneously per CPU cycle via hardware SIMD instructions.
3. **The Goal:** Phia is not designed to replace Lua for highly dynamic, metatable-heavy logic. Instead, it is an exploration of a specific thesis: **Can we achieve C/Rust-level performance (zero-cost abstractions, static memory guarantees, and SIMD vectorization) while writing purely in Lua syntax?**

Phia exists to prove that the performance tax on scripting languages is not in the syntax, but in the dynamic runtime semantics. By enforcing a strict typed subset and replacing runtime guessing with compile-time mathematical proofs, the engine unlocks an entirely different performance tier.

### Missing Implementations

#### Core Language
* **Functions**: Function definitions and invocations are not yet implemented.
* **Global Variables**: Global scope is missing; all variables must currently be declared as `local`.
* **Control Flow**: `for` loops are not yet supported.
* **Standard Library**: Built-in standard library functions are missing.
* **The `nil` Keyword** — Explicit assignment (e.g., `local x = nil`) is not yet supported.

#### Tables & Data Structures
* **Hash Maps & Sparse Arrays**: Tables are backed strictly by dense, 0-indexed `Vec`s. Sparse inserts cause Out-Of-Memory (OOM) errors, and negative keys trigger runtime panics.
* **Dot Notation & String Keys**: Table property access (e.g., `t.field`) and string-based keys are unsupported.
* **Boolean Storage**: Booleans cannot currently be stored as table values.

#### Types & Operators
* **Numeric Coercion**: Mixed integer/float arithmetic and implicit string-to-number conversions are strict build errors.
* **Integer Division**: The `/` operator performs truncating division on integers, whereas standard Lua always yields a float.
* **String Ordering**: Relational operators (`<`, `>`, etc.) cannot be used to compare strings.

#### Memory & Performance
* **String Optimization**: String concatenation (`..`) and table storage currently generate heavy heap traffic via `format!()` and `.clone()`.
* **Dynamic Memory Management**: Lacks a garbage collector, reference counting, and runtime type tags. Memory is entirely resolved at build time, meaning tables are statically allocated in a single arena and never freed.

### Supported Features

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

* **Absence & Safety** — Absent table keys safely default to the element kind's zero value (`0`, `0.0`, `""`). Dereferencing a null table handle triggers a checked runtime error (`Runtime Error: table is nil`), matching standard Lua's `attempt to index a nil value`.

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

-- absent keys read as the element kind's zero: 0 and ""
print("exit", acc[999], names[42])
print("tables", acc, wave, names, grid, row)
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
# Touch the file first to update the timestamp
touch showcase.lua && PHIA_SOURCE=showcase.lua cargo build --release


# 3. DEBUG & IR DUMPS (Options: mid | final | all)

# Writes IR dumps and the probe_map.txt sidecar directly beside the 
# generated baked_native.rs file in the target/release/build/... folder.
touch main.lua && PHIA_DEBUG_DUMP=final PHIA_SOURCE=main.lua cargo build --release
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
-- ghost_register.lua
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
STATS fast_sets=0;fast_gets=0;dyn_sets=0;dyn_gets=1;hoists=0;hoist_ctx=
TIME 14.891µs
```

*(Here, `ghost_val` was optimized away, the allocator gave its physical slot to `shadow`, and the `print` probe blindly read the recycled memory containing `42`.)*
