-- feat_local_multi.lua [POSITIVE — single-line multiple local binding:
-- `local a, b, c = x, y, z`. Every RHS evaluates under the pre-statement
-- scope BEFORE any name binds — Lua's order, not sequential declarations:
-- the pair-swap `local r, s = b, a` exchanges values, and inside the
-- loop body the RHS `x` still names the OUTER x (sequential desugaring
-- would read the just-declared 0 and print y = 5). Tables born in a
-- multi-binding infer per-table element types exactly like solo decls,
-- and one statement may mix kinds — each pair types independently].
-- EXPECT_PRINT: ints	1	2	3
-- EXPECT_PRINT: floats	1.5	2.5
-- EXPECT_PRINT: strs	left	right
-- EXPECT_PRINT: bools	true	true
-- EXPECT_PRINT: swap	2	1
-- EXPECT_PRINT: mixed	7	0.25	seven
-- EXPECT_PRINT: tables	1	1.5	left
-- EXPECT_PRINT: shadow	0	15
-- EXPECT_PRINT: outer	10
local a, b, c = 1, 2, 3
print("ints", a, b, c)
local f, g = 1.5, 2.5
print("floats", f, g)
local u, v = "left", "right"
print("strs", u, v)
local p, q = true, not false
print("bools", p, q)
local r, s = b, a
print("swap", r, s)
local m, n, o = 7, 0.25, "seven"
print("mixed", m, n, o)
local ti, tf, ts = {}, {}, {}
ti[0] = a
tf[0] = f
ts[0] = u
print("tables", ti[0], tf[0], ts[0])
local x = 10
local i = 0
while i < 1 do
    local x, y = 0, x + 5
    print("shadow", x, y)
    i = i + 1
end
print("outer", x)
