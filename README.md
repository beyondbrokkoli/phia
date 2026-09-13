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

### Quickstart Guide
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
