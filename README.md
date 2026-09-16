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


-- [4] LOGIC: 'not', 'and', 'or'. Strictly Boolean operands; short-circuit
-- evaluation is Lua-faithful (the guard idiom `i > 0 and t[i-1] > 0` never
-- touches t[-1] when i == 0). Precedence: or < and < comparisons.
local is_active = not false
local num_cmp = (int_val < 20)
local str_eq = (prefix == "Value: ")
local both = is_active and num_cmp
local either = num_cmp or str_eq
-- ERR: Boolean-only operands (no truthiness, no value-returning): local compound = 1 and true
-- ERR: The Lua default-value idiom is refused: local fallback = false or 5
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
    both,
    either,
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
