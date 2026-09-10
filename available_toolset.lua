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
