-- logic_01_and_truth_table.lua  [POSITIVE — the full `a and b` truth table
-- through variables, not literals, so the const fold can't eat the shape;
-- every false row flows through the short-circuit const arm, only the
-- true/true row evaluates the right operand]
-- EXPECT_PRINT: and	false	false	false	true
local a0 = false
local a1 = true
local b0 = false
local b1 = true
print("and", a0 and b0, a0 and b1, a1 and b0, a1 and b1)
