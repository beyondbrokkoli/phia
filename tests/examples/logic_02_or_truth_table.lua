-- logic_02_or_truth_table.lua  [POSITIVE — the full `a or b` truth table
-- through variables: the true rows short-circuit through the const arm,
-- the false/false row is the only one that evaluates the right operand]
-- EXPECT_PRINT: or	false	true	true	true
local a0 = false
local a1 = true
local b0 = false
local b1 = true
print("or", a0 or b0, a0 or b1, a1 or b0, a1 or b1)
