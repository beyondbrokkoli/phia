-- float_04_mixed_store.lua  [NEGATIVE — monomorphic elements, float edition]
-- Same rule as nested_03, new kind: one table, one element type. The
-- Integer store binds the element variable; the Float store conflicts.
-- EXPECT_BUILD_FAIL: Type Error: table element type conflict (Integer vs Float)
local t = {}
t[0] = 1
t[1] = 2.5
