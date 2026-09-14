# Phia 🌙

[![License: MIT/Apache-2.0](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)
[![Status: Experimental AOT](https://img.shields.io/badge/status-experimental-orange.svg)]()

> 🌟 **[logos](https://github.com/maciejhirsz/logos)!** 🌟
> This project would simply not be possible without the `logos` crate.

Phia is an ahead-of-time compiler for a statically typed Lua subset.

### Currently Missing Language Implementations

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
* **Logical Operators**: The `and` and `or` keywords are not yet implemented.
* **Numeric Coercion**: Mixed integer/float arithmetic and implicit string-to-number conversions are strict build errors.
* **Integer Division**: The `/` operator performs truncating division on integers, whereas standard Lua always yields a float.
* **String Ordering**: Relational operators (`<`, `>`, etc.) cannot be used to compare strings.
* **String Optimization**: String operations currently generate heavy heap traffic via `format!()` and `.clone()`.

### Subset Definition

```lua
-- subset.lua
-- STRICT LUA SUBSET SPECIFICATION
-- DO NOT deviate from these typing rules. There is NO implicit conversion.

-- 0. NO GLOBAL SCOPE (LOCAL ONLY)
-- ALL variables must be declared with the 'local' keyword. Global state does not exist.
local starting_value = 100

-- FAILURE PREVENTED: Missing 'local' keyword!
-- global_counter = 50 -- ERROR: Global variables are not supported


-- 1. STRICT TYPING & BINARY OPERATIONS
local int_val = 10       -- Integer (i64)
local float_val = 2.5    -- Float (f64)

-- SUCCESS: Matching types
local pure_int = int_val * 2
local pure_float = float_val + 1.5

-- FAILURE PREVENTED: Do not mix ints and floats!
-- local crash = int_val + float_val -- ERROR: Mixed Integer and Float


-- 2. STRING CONCATENATION
local prefix = "Value: "
local suffix = " units"

-- SUCCESS: String .. String ONLY
local message = prefix .. suffix

-- FAILURE PREVENTED: No implicit tostring()!
-- local crash_str = prefix .. int_val -- ERROR: Concatenation requires Strings


-- 3. TABLES: BIRTH, NESTING & INDEXING
-- Tables infer their element kind on FIRST USE.
-- ALL keys at ALL levels strictly require Integers (0-indexed).
local num_list = {}
local float_list = {}
local string_list = {}
local grid = {}
local row_zero = {}

-- SUCCESS: 1D Tables (First touch locks in the table's type)
num_list[0] = pure_int      -- Inferred as Integer table
num_list[1] = 42
float_list[0] = pure_float  -- Inferred as Float table
string_list[0] = message    -- Inferred as String table

-- SUCCESS: Nested Tables (Initialize inner first, all Integer keys)
row_zero[0] = 99
row_zero[1] = 100
grid[0] = row_zero          -- grid is inferred as a Table-of-Tables

-- SUCCESS: Direct nested assignment (inner table grid[0] already exists)
grid[0][2] = 102

-- FAILURE PREVENTED: No string keys, no float keys ANYWHERE!
-- num_list["first"] = 100  -- ERROR: Table index must be an Integer
-- float_list[0.5] = 100    -- ERROR: Table index must be an Integer
-- grid["top"] = row_zero   -- ERROR: Table index must be an Integer
-- grid[0]["x"] = 500       -- ERROR: Table index must be an Integer


-- 4. PRINT PROBE (Debugging)
-- 'print' requires a string literal tag as the first argument.
print("state", pure_int, pure_float, num_list[1])


-- 5. COMPARISONS & LOGIC
-- 'not' is the ONLY supported logical operator.
local is_active = not false

-- FAILURE PREVENTED: 'and' and 'or' DO NOT EXIST.
-- local compound = true and false   -- ERROR: 'and'/'or' are not implemented

-- Numbers and Booleans support full comparisons (<, >, <=, >=, ==, ~=).
local num_cmp = (int_val < 20)

-- Strings ONLY support equality (==, ~=).
local str_eq = (prefix == "Value: ")

-- FAILURE PREVENTED: No relational comparisons for strings!
-- local str_cmp = (prefix < suffix) -- ERROR: Relational operators cannot compare strings


-- 6. DIVISION SEMANTICS (CRITICAL DEVIATION)
-- Unlike standard Lua (which yields a float), the '/' operator on Integers truncates.
local trunc_div = 7 / -2       -- Yields -3 (Integer)
local floor_div = 7 // -2      -- Yields -4 (Integer)
local mod_op = -7 % 3          -- Yields 2 (Takes the divisor's sign)

-- Float division behaves normally.
local float_div = 1.0 / 4.0    -- Yields 0.25 (Float)


-- 7. CONTROL FLOW & ABSENT KEYS (TYPE-SPECIFIC ZERO VALUES)
-- 'for' loops are missing; you MUST use 'while'.
-- Standard 'if / elseif / else' logic is FULLY supported.
-- There is no 'nil' keyword.
local iterator = 0
while iterator <= 2 do
    if iterator == 1 then
        -- SUCCESS: Standard conditionals work exactly as expected
    end
    -- local empty = nil       -- ERROR: Explicit 'nil' assignment is unsupported
    iterator = iterator + 1
end

-- Absent keys safely default to the table's inferred type's zero value.
local missing_num = num_list[999]       -- Yields 0   (Integer)
local missing_float = float_list[999]   -- Yields 0.0 (Float)
local missing_str = string_list[999]    -- Yields ""  (String)


-- 8. ASCII ARTWORK (String Concatenation Folding)
-- Chaining string concatenations evaluates safely and pins the register for consistent spacing.
local line = "-" .. "~" .. "@"
-- PROBE art: s_r95="-~@"
print("art", line)
```

### Quickstart
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

# build.rs prevents stale binaries
# Touch the file first to update the timestamp
touch showcase.lua && PHIA_SOURCE=showcase.lua cargo build --release


# 3. DEBUG & IR DUMPS (Options: mid | final | all)

# Writes IR dumps and the probe_map.txt sidecar directly beside the
# generated baked_native.rs file in the target/release/build/... folder.
touch main.lua && PHIA_DEBUG_DUMP=final PHIA_SOURCE=main.lua cargo build --release

# 4. HAVE FUN
touch mandelbrot.lua && PHIA_SOURCE=mandelbrot.lua cargo run --release
```
### Known Quirks

#### Strict Tag Requirement for `print`

Phia strictly requires a string literal tag as the first argument in a `print` statement (e.g., `print("tag", x)`).
A plain `print(x)` is a syntax error.

**Why is the tag required?**
The compiler relies on this mandatory string to generate `probe_map.txt`.

**Why hijack `print` instead of adding a `probe()` keyword?**
Compatibility. By keeping the name `print`, you can run the exact same `.lua` script in standard Lua or LuaJIT without modification.

#### About Memory

Every memory decision is made at build time. Element kinds, storage sides (`array` / `farray` / `sarray`), and register allocation are all resolved during compilation — nothing executes per operation except the operation itself.

**Scalars compile to pre-declared Rust locals.** The register allocator uses live-interval analysis to map virtual registers to physical ones (`i_r0`, `b_r1`, `s_r2`). A dead scalar generates zero runtime overhead — its slot is simply not allocated. Loop-carried values (phi nodes) get intervals that wrap around back edges, so their physical register persists across iterations. Int, Bool, Table, and String scalars deliberately share one physical ID range, disambiguated at codegen time by prefix (`i_`, `b_`, `t_`, `s_`). Float scalars, float-element tables, and string-element tables mint from disjoint ranges to avoid pointer-type ambiguity (`*mut i64` vs `*mut f64` vs `*mut String`).

**Strings use the global allocator.** Each `LoadString` emits a `String::new()` at runtime. No interning, no compile-time string pooling. String concatenation (`..`) and table storage currently generate heavy heap traffic via `format!()` and `.clone()`.

**Tables live in a single arena** (`Vec<Box<Table>>`). A table is allocated exactly once at its literal via `tables.push(Box::new(Table::new()))`, never moves, never frees. Table references are either 1-based handles (for nested-table programs) or raw pointers (for integer-only fast paths). Inner arrays (`t.array`, `t.farray`, `t.sarray`) resize dynamically through the global allocator — the `Box<Table>` itself never moves.

**Loop optimizations are compile-time decisions.** `EnsureCapacity` pre-sizes arrays before the loop. `HoistRawPtr` materializes length and a raw pointer as physical registers (`len_r33`, `p_r33`). `SetTableFast` and `GetTableFast` skip arena lookup entirely, using `*p_r33.add(k as usize)` for direct pointer arithmetic. Tier-4 analysis proves which tables can be safely hoisted across loop iterations by tracing handle aliases and detecting storage-hazard roots.

**Zero runtime surprises.** Numeric ops are single-instruction (wrapping). Table access is either a pointer deref (fast path) or an arena lookup with bounds check (dynamic path). No GC pauses, no reference counting overhead, no type dispatch. Every operation's cost is determined at compile time by its operands' types and lifetimes.

## Benchmark: `race`

The `race` benchmark is a deterministic, compute-intensive program designed to compare the native code generated by Delaware against an equivalent hand-written C++ implementation.

The benchmark uses the following workload:

- `N = 10,000,000` elements.
- `100` rounds over all elements, for a total of **1 billion hot-loop iterations**.
- Three 64-bit integer arrays/tables (`x`, `y`, and `z`) are initialized deterministically.
- Each hot-loop iteration loads three values and computes three new values using a fixed multiply/add recurrence.
- Each recurrence performs integer modulo with `MOD = 1,000,000,007`.
- The Lua version uses Delaware's integer modulo semantics.
- The C++ version therefore does **not** use ordinary C++ remainder directly. It implements the same Lua-compatible modulo semantics explicitly through `lua_mod()`.
- The benchmark deliberately keeps several loop-invariant values inside the hot loop (`aa`, `bb`, `cc`, and `modulus`) to exercise Delaware's loop-invariant code motion / hoisting optimizations.
- After all rounds, a complete pass over the three arrays produces a final modulo-reduced `witness` value.
- The witness is printed, making the computation observable and providing a deterministic correctness check between implementations.


```lua
-- race.lua
--
-- Deterministic computational benchmark.
-- Intended for the Lua -> Rust -> LLVM compiler.
--
-- The final "witness" must match the C++ implementation.

local N = 10000000
local ROUNDS = 100

local MOD = 1000000007

-- Loop-invariant constants.
local A = 48271
local B = 69621
local C = 31337

local x = {}
local y = {}
local z = {}

local i = 0

-- Initialization
while i < N do
    x[i] = (i * 17 + 23) % MOD
    y[i] = (i * 31 + 71) % MOD
    z[i] = (i * 43 + 113) % MOD

    i = i + 1
end


local round = 0

while round < ROUNDS do

    i = 0

    while i < N do

        -- Intentionally keep invariant-looking values inside
        -- the hot loop. This is useful for testing LICM / hoisting.
        local aa = A
        local bb = B
        local cc = C
        local modulus = MOD

        local vx = x[i]
        local vy = y[i]
        local vz = z[i]

        local next_x = (vx * aa + vy * bb + vz * cc) % modulus
        local next_y = (vy * aa + vz * bb + vx * cc) % modulus
        local next_z = (vz * aa + vx * bb + vy * cc) % modulus

        x[i] = next_x
        y[i] = next_y
        z[i] = next_z

        i = i + 1
    end

    round = round + 1
end


-- Witness.
--
-- This makes the result observable and prevents the benchmark
-- from being considered dead computation.
local witness = 0

i = 0

while i < N do
    witness = (witness + x[i]) % MOD
    witness = (witness + y[i]) % MOD
    witness = (witness + z[i]) % MOD

    i = i + 1
end

print("witness", witness)
```

```c++
#include <cstdint>
#include <iostream>
#include <vector>

static volatile std::int64_t witness_sink = 0;

// Emulates Phia's Rust codegen for Lua modulo exactly
inline std::int64_t lua_mod(std::int64_t left, std::int64_t right) {
    std::int64_t rem = left % right;
    // Equivalent to: rem + i64::from(rem != 0 && (rem < 0) != (right < 0)) * right
    return rem + static_cast<std::int64_t>(rem != 0 && (rem < 0) != (right < 0)) * right;
}

int main() {
    constexpr std::int64_t N = 10000000;
    constexpr std::int64_t ROUNDS = 100;
    constexpr std::int64_t MOD = 1000000007LL;

    constexpr std::int64_t A = 48271;
    constexpr std::int64_t B = 69621;
    constexpr std::int64_t C = 31337;

    std::vector<std::int64_t> x(N);
    std::vector<std::int64_t> y(N);
    std::vector<std::int64_t> z(N);

    // Initialization
    for (std::int64_t i = 0; i < N; ++i) {
        x[i] = lua_mod(i * 17 + 23, MOD);
        y[i] = lua_mod(i * 31 + 71, MOD);
        z[i] = lua_mod(i * 43 + 113, MOD);
    }

    for (std::int64_t round = 0; round < ROUNDS; ++round) {
        for (std::int64_t i = 0; i < N; ++i) {
            const std::int64_t aa = A;
            const std::int64_t bb = B;
            const std::int64_t cc = C;
            const std::int64_t modulus = MOD;

            const std::int64_t vx = x[i];
            const std::int64_t vy = y[i];
            const std::int64_t vz = z[i];

            const std::int64_t next_x = lua_mod(vx * aa + vy * bb + vz * cc, modulus);
            const std::int64_t next_y = lua_mod(vy * aa + vz * bb + vx * cc, modulus);
            const std::int64_t next_z = lua_mod(vz * aa + vx * bb + vy * cc, modulus);

            x[i] = next_x;
            y[i] = next_y;
            z[i] = next_z;
        }
    }

    // Witness.
    std::int64_t witness = 0;
    for (std::int64_t i = 0; i < N; ++i) {
        witness = lua_mod(witness + x[i], MOD);
        witness = lua_mod(witness + y[i], MOD);
        witness = lua_mod(witness + z[i], MOD);
    }

    witness_sink = witness;
    std::cout << "witness " << witness_sink << '\n';

    return 0;
}
```

```text
$ time ./target/release/phia
PROBE witness: i_r149=160355959
TABLE 0 LEN 10000000 NZ 10000000 CHECKSUM 4649550937620586046
TABLE 1 LEN 10000000 NZ 10000000 CHECKSUM 4621099214076745768
TABLE 2 LEN 10000000 NZ 10000000 CHECKSUM 4655222127977869585
STATS fast_sets=6;fast_gets=6;dyn_sets=0;dyn_gets=0;hoists=9;hoist_ctx=0,0,0,1,1,1,0,0,0
TIME 4.954749312s

real	0m4,978s
user	0m4,962s
sys	0m0,010s
$ time ./race_cpp
witness 160355959

real	0m5,411s
user	0m5,389s
sys	0m0,013s
$ hyperfine 'target/release/phia > /dev/null' './race_cpp > /dev/null'
Benchmark 1: target/release/phia > /dev/null
  Time (mean ± σ):      5.157 s ±  0.158 s    [User: 5.136 s, System: 0.009 s]
  Range (min … max):    4.986 s …  5.455 s    10 runs

Benchmark 2: ./race_cpp > /dev/null
  Time (mean ± σ):      5.729 s ±  0.165 s    [User: 5.707 s, System: 0.008 s]
  Range (min … max):    5.559 s …  5.942 s    10 runs

Summary
  target/release/phia > /dev/null ran
    1.11 ± 0.05 times faster than ./race_cpp > /dev/null
$
```
