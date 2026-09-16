-- logic_10_while_cond_nested.lua  [POSITIVE — chains and nesting at the
-- loop gate: `(a or b) and c` mixes both operators (the desugar nests an
-- or-chain inside an and-chain, both inside the loop region); the grid
-- loop nests a `j < 2 and i ~= 2` inner while inside a plain outer while
-- — the inner chain's zero-iteration case (i == 2) short-circuits on
-- the LEFT side]
-- EXPECT_PRINT: nested	3
-- EXPECT_PRINT: grid	4	3
local a = true
local b = false
local c = true
local n = 0
while (a or b) and c do
    n = n + 1
    if n > 2 then
        c = false
    end
end
print("nested", n)
local total = 0
local i = 0
while i < 3 do
    local j = 0
    while j < 2 and i ~= 2 do
        total = total + 1
        j = j + 1
    end
    i = i + 1
end
print("grid", total, i)
