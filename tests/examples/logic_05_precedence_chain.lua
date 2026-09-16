-- logic_05_precedence_chain.lua  [POSITIVE — `or` binds looser than `and`,
-- both chain left: the first row DISTINGUISHES `x or (y and z)` from
-- `(x or y) and z` (x=true, y=false, z=false → correct true, wrong-assoc
-- false); then an `a and b and c` chain and the unary-binds-tightest row
-- `not true or true` → true]
-- EXPECT_PRINT: prec	true	false	true
local x = true
local y = false
local z = false
local a = true
local b = true
local c = false
print("prec", x or y and z, a and b and c, not true or true)
