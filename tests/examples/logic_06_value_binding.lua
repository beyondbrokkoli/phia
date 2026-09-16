-- logic_06_value_binding.lua  [POSITIVE — and/or as a VALUE: the join phi
-- feeds a local decl (`local x = p and q`), an assignment rebinds it
-- through the or desugar (the checker's Boolean×Boolean assignment arm),
-- and a nested `p and (q or r)` nests one desugar inside the other's
-- value arm]
-- EXPECT_PRINT: bind	false
-- EXPECT_PRINT: bind2	true	true
local p = true
local q = false
local r = true
local s = false
local x = p and q
print("bind", x)
x = r or s
local y = p and (q or r)
print("bind2", x, y)
