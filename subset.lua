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
