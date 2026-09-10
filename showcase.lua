-- showcase.lua — the entire supported feature set in one program.

-- [TYPES] Integer (i64, wrapping), Float (f64), Boolean, String, Table.
-- Tables use Integer keys and infer a monomorphic element kind on first use
-- (Integer, Float, String, or nested Table).
local name = "phia" .. "/" .. "lua"       -- String (concatenation '..' chains fold left)
local version = 1                         -- Integer (i64, wrapping like Lua)
local ratio = 0.25                        -- Float (f64)
local tuned = true                        -- Boolean

-- [STATEMENTS: print] Requires a string literal tag. Acts as a debug probe
-- printing physical register states (values, handles, lengths; never addresses).
-- Deliberately named 'print' so this file runs unmodified in standard Lua.
print("scalars", name, version, ratio, tuned)

-- [OPERATORS] + - * / // % with Lua semantics.
-- Integer side: / truncates, // floors, % takes the divisor's sign.
print("int_sem", 7 / -2, 7 // -2, -7 % 3, 9 - 4, (1 + 2) * 3) -- parentheses supported
-- Float side: / is plain division, // floors, % is adjusted.
print("float_sem", 1.0 / 4.0, 0.75 // 0.5, 0.75 % 0.5, -ratio) -- unary minus supported

-- [OPERATORS] Comparisons (< > <= >= == ~=) on numbers, strings, and booleans.
print("cmp", version < 2, ratio >= 0.25, name == "phia/lua", tuned ~= false, not tuned) -- unary 'not' supported

-- [STATEMENTS] Structured control: if / elseif / else
local grade = 0
if version >= 2 then
    grade = 100
elseif version == 1 then
    grade = 50
else
    grade = 9
end

-- [TYPES & STATEMENTS] Nested tables (arena handles).
-- Table assignment supports t[i] = v and nested lvalues like t[0][j] = v.
local grid = {}
local row = {}
row[0] = 99
grid[0] = row

-- [STATEMENTS] while loop, local assignment (lexically scoped, shadowing allowed),
-- and table assignments (both affine and non-affine).
local acc = {}
local wave = {}
local names = {}
local x = 0.0
local i = 0
while i <= 7 do
    local shadow = i * 100        -- lexically scoped per-trip local (shadowing allowed)
    acc[i] = i * i                -- table assignment: Integer
    wave[i] = x                   -- table assignment: Float
    names[i] = name               -- table assignment: String
    grid[0][i % 3] = shadow       -- nested lvalue table assignment (dynamic/checked)
    print("iter", i, acc, acc[i])
    x = x + 0.25
    i = i + 1
end

-- [ABSENCE & SAFETY] Absent keys safely default to the element kind's zero value (0, 0.0, "").
print("exit", acc[999], names[42])
print("tables", acc, wave, names, grid, row)
print("final", name, grade, row[0] == 99, acc[7] == 49)

-- [SAFETY DEMO] Uncommenting the block below triggers a checked runtime error:
-- "Runtime Error: table is nil". This perfectly matches Lua's "attempt to index a nil value"
-- ergonomics while guaranteeing absolutely no Undefined Behavior (UB).
--
-- local empty_row = {}
-- local bad_access = empty_row[0][1]
