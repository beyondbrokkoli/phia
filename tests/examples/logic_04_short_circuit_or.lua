-- logic_04_short_circuit_or.lua  [POSITIVE — the or half of the guard
-- idiom: a left-true short-circuits the dangerous right (`i <= 0 or
-- t[i-1] > 0` with i == 0 → true, t[-1] never touched); the left-false
-- rows show the value arm really evaluates the right operand]
-- EXPECT_PRINT: or	true	true	false
local i = 0
local t = {}
local i2 = 5
local i3 = 7
print("or", i <= 0 or t[i-1] > 0, i2 > 100 or i2 == 5, i3 > 100 or i3 == 5)
