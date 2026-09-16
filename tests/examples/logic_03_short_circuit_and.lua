-- logic_03_short_circuit_and.lua  [POSITIVE — THE semantic pin of the
-- milestone: a left operand that decides the result also decides SAFETY.
-- `i > 0 and t[i-1] > 0` with i == 0 prints false and never evaluates
-- t[-1] (eager evaluation would die with "Runtime Error: Negative table
-- index"); the div-guard row is the same story for `z ~= 0 and 10 // z > 0`
-- with z == 0. The guard2 row proves the right side still evaluates —
-- and can panic — when the left cannot decide]
-- EXPECT_PRINT: guard	false	false
-- EXPECT_PRINT: guard2	true
local i = 0
local t = {}
local z = 0
print("guard", i > 0 and t[i-1] > 0, z ~= 0 and 10 // z > 0)
local j = 1
t[0] = 5
print("guard2", j > 0 and t[j-1] > 0)
