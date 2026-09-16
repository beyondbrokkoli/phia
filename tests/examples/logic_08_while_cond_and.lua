-- logic_08_while_cond_and.lua  [POSITIVE — THE guard idiom at the loop
-- gate: `while i > 0 and t[i-1] > 0 do` walks i down to 0 and the FINAL
-- gate evaluation short-circuits (t[-1] never read; eager evaluation
-- would die with "Runtime Error: Negative table index"). The k-loop
-- starts at zero: the very FIRST evaluation short-circuits — zero
-- iterations, no panic. The condition chain lives inside the loop and
-- re-evaluates every iteration]
-- EXPECT: TABLE 0 LEN 3 NZ 3 CHECKSUM 6
-- EXPECT: NTABLES 1
-- EXPECT: dyn_sets=3
-- EXPECT: dyn_gets=2
-- EXPECT_PRINT: walk	0
-- EXPECT_PRINT: zero	0
local t = {}
t[0] = 1
t[1] = 1
t[2] = 1
local i = 3
while i > 0 and t[i-1] > 0 do
    i = i - 1
end
print("walk", i)
local k = 0
while k > 0 and t[k-1] > 0 do
    k = k - 1
end
print("zero", k)
