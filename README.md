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
* **Mixed-Type Tables**: Every table is monomorphic (i64, f64, string, boolean, or table elements — never a mix).

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
-- strictly typed, no implicit conversions, local scope only.

-- [0] SCOPE: Local Only
local starting_value = 100
-- ERR: Global variables are unsupported: global_counter = 50


-- [1] STRICT TYPING: i64 and f64. No Mixing.
local int_val = 10     -- i64
local float_val = 2.5  -- f64
local pure_int = int_val * 2
local pure_float = float_val + 1.5
-- ERR: Mixed int and float: local crash = int_val + float_val


-- [2] STRINGS: String .. String ONLY
local prefix = "value:"
local suffix = " units"
local message = prefix .. suffix
-- ERR: No implicit tostring(): local crash_str = prefix .. int_val


-- [3] TABLES: Inferred on first touch. Keys MUST be 0-indexed i64.
local num_list, float_list, string_list, bool_list, grid, row_zero = {}, {}, {}, {}, {}, {}

num_list[0] = pure_int      -- Inferred i64 table
num_list[1] = 42
float_list[0] = pure_float  -- Inferred f64 table
string_list[0] = message    -- Inferred string table
bool_list[0] = int_val < 50 -- Inferred boolean table (bit-packed storage)
bool_list[1] = true
row_zero[0] = 99
row_zero[1] = 100
grid[0] = row_zero          -- Inferred table-of-tables
grid[0][2] = 102
-- ERR: String keys forbidden: grid["top"] = row_zero
-- ERR: Float keys forbidden: num_list[0.5] = 10
-- ERR: Mixed element types forbidden: bool_list[2] = 42


-- [4] LOGIC: 'not' only. No 'and'/'or'.
local is_active = not false
local num_cmp = (int_val < 20)
local str_eq = (prefix == "Value: ")
-- ERR: 'and'/'or' missing: local compound = true and false
-- ERR: Relational ops (<, >) forbidden on strings: local str_cmp = (prefix < suffix)


-- [5] DIVISION: Integer '/' truncates. Float '/' behaves normally.
local trunc_div = 7 / -2    -- i64: Yields -3
local floor_div = 7 // -2   -- i64: Yields -4
local mod_op = -7 % 3       -- i64: Yields 2 (Takes divisor's sign)
local float_div = 1.0 / 4.0 -- f64: Yields 0.25


-- [6] CONTROL FLOW & DEFAULTS: 'while' loops, 'if' blocks. No 'nil'.
local iterator = 0
while iterator <= 2 do
    if iterator == 1 then
        -- Standard conditionals apply
    end
    -- local empty = nil -- ERR: explicit 'nil' is unsupported
    iterator = iterator + 1
end

-- Absent keys safely yield the type's zero value
local missing_num = num_list[999]     -- Yields 0
local missing_float = float_list[999] -- Yields 0.0
local missing_str = string_list[999]  -- Yields ""
local missing_bool = bool_list[999]   -- Yields false


-- [7] COMPREHENSIVE OUTPUT
print(
    pure_float,
    int_val,
    trunc_div,
    is_active,
    prefix .. suffix,
    grid[0][2],
    missing_num,
    missing_str,
    bool_list[1],
    missing_bool
)
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

### Benchmark: `race`

The `race` benchmark is a deterministic, compute-intensive program designed to compare the native code generated by Phia against an equivalent C++ implementation. It serves as a stress test for Loop-Invariant Code Motion (LICM), x86-64 register allocation, and memory aliasing under extreme register pressure.

* **Scale:** `N = 10,000,000` elements across `100` rounds, totaling **1 billion hot-loop iterations**.
* **Computation:** Iterates over three 64-bit integer arrays (`x`, `y`, `z`), computing a 3-way multiply-accumulate recurrence modulo `1,000,000,007`.
* **Fair Modulo:** Phia’s modulo operator (`%`) guarantees the result takes the divisor's sign (matching standard Lua's mathematical definition). Conversely, standard C++ and x86 hardware use a truncated remainder (taking the dividend's sign). A naive C++ benchmark would execute fewer instructions by relying on the native hardware truncation, unfairly dodging register pressure. To level the field, the C++ baseline explicitly implements Phia's modulo rules via `lua_mod()`. This forces both compilers to emit extra Euclidean adjustments, deliberately causing general-purpose register (GPR) starvation and forcing both LLVM and GCC to manage heavy stack spilling.
* **Verification:** A complete pass over the arrays produces a final modulo-reduced `witness` value, preventing Dead Code Elimination (DCE) and guaranteeing deterministic correctness.


```lua
-- PHIA_SOURCE=tests/benchmark/race.lua cargo rustc --release -- -C target-cpu=native --emit asm -C llvm-args=-x86-asm-syntax=intel

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
// g++ -O3 -march=native -DNDEBUG -g1 -masm=intel -fverbose-asm -fno-exceptions -fno-rtti -save-temps=obj tests/benchmark/race.cpp -o tests/benchmark/race

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
$ time target/release/phia
witness	160355959
TABLE 0 LEN 10000000 NZ 10000000 CHECKSUM 4649550937620586046
TABLE 1 LEN 10000000 NZ 10000000 CHECKSUM 4621099214076745768
TABLE 2 LEN 10000000 NZ 10000000 CHECKSUM 4655222127977869585
STATS fast_sets=6;fast_gets=6;dyn_sets=0;dyn_gets=0;hoists=9;hoist_ctx=0,0,0,1,1,1,0,0,0
TIME 4.958683467s

real	0m4,982s
user	0m4,964s
sys	0m0,011s
$ time tests/benchmark/race
witness 160355959

real	0m5,435s
user	0m5,410s
sys	0m0,017s
$ hyperfine --warmup 1 'target/release/phia>/dev/null' 'tests/benchmark/race>/dev/null'
Benchmark 1: target/release/phia>/dev/null
  Time (mean ± σ):      5.108 s ±  0.129 s    [User: 5.088 s, System: 0.009 s]
  Range (min … max):    4.991 s …  5.300 s    10 runs

Benchmark 2: tests/benchmark/race>/dev/null
  Time (mean ± σ):      5.808 s ±  0.135 s    [User: 5.782 s, System: 0.011 s]
  Range (min … max):    5.627 s …  5.967 s    10 runs

Summary
  target/release/phia>/dev/null ran
    1.14 ± 0.04 times faster than tests/benchmark/race>/dev/null
```

### About Memory

* **Build-Time Memory Resolution:** Every memory decision—including data types, array storage models, and register assignments—is strictly calculated during compilation. The final program completely skips the overhead of figuring out data types or managing memory allocation on the fly.
* **Scalar Variables & Register Packing:** Standard variables (scalars) compile directly into Rust local variables. The compiler analyzes exactly when variables are created and last used, packing them efficiently into a limited number of physical hardware registers.
* **Zero-Cost Dead Code:** If a scalar variable — integer, boolean, float, or string — is assigned but never read, the compiler completely eliminates it: it never becomes a Rust variable and costs zero runtime memory. Integer and boolean values are additionally constant-folded into their uses at compile time; dead float loads and string concatenations are removed outright, along with their heap allocations.
* **Efficient Loop Variables (Phi Nodes):** For variables that update across loop iterations, the compiler assigns the same physical hardware register to the input and the output. This ensures the data is already in the correct slot for the next iteration without moving it around.
* **Strict Type Boundaries:** Every value's type is known at compile time, so each value lives in a type-specific pool of Rust variables (`i_r*`, `b_r*`, `f_r*`, `s_r*`, `t_r*`), with floats occupying a register range fully disjoint from the integer range. This prevents the generated Rust code from accidentally mixing up memory addresses (like treating a float pointer as an integer pointer), which guarantees memory safety.
* **Direct String Allocation:** Strings completely rely on Rust's standard heap allocator. There is no background system trying to save space by reusing identical text (no interning or pooling). Operations like string concatenation create entirely new memory allocations, meaning heavy text processing will generate significant heap traffic.
* **Permanent Table Storage (Arena):** Tables live for the entire program in an arena (a vector of individually boxed tables). Once a table is created, its core structure never moves and is never deleted. Because its memory address is permanent, the transpiler can safely hand out direct raw pointers for instant access.
* **Dynamic Internal Arrays:** While a table's core address never changes, the internal arrays holding its actual data are allowed to dynamically resize via the global memory allocator as elements are added.
* **Aggressive Loop Optimization:** To make loops fast, the compiler requests all necessary memory before the loop begins. It also calculates a table's raw memory address once and stores it in a register (hoisting), so the loop doesn't have to look up the table's location on every single iteration.
* **Direct Pointer Arithmetic:** Inside loops, table accesses skip standard lookups entirely. The code uses direct math on the raw memory pointers to jump straight to the data, provided the compiler's safety checks (Tier-4 analysis) confirm this shortcut won't cause errors.
* **Predictable Runtime Performance:** Because memory and types are entirely resolved at compile time, the final execution has no unpredictable background tasks. There are no garbage collection pauses, no reference counting, and no runtime type checking. Basic arithmetic and comparisons map to single CPU instructions (the floor-division and modulo sign corrections expand into short branch-free instruction sequences, which LLVM frequently strength-reduces into multiply-shift code containing no division instruction at all), and memory access is instant and direct.
