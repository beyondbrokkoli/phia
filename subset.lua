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
