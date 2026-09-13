-- float_13_deferred_coercion.lua  [NEGATIVE — coercion conflict via the deferred path]
-- The direct mixed pair (1 + 0.5) dies at the BinaryOp arm (float_03); this
-- is the sneakier route. The read of a[0] constrains nothing; the
-- ARITHMETIC USE of x binds a's element variable to Integer; only then
-- does the float store arrive — and unification reports the genuine
-- conflict. Pins that arithmetic use, not the read, is what fixes the
-- element type under lazy unification.
-- EXPECT_BUILD_FAIL: Type Error: table element type conflict (Integer vs Float)
local a = {}
local x = a[0]
local y = x + 1
a[0] = 1.5
