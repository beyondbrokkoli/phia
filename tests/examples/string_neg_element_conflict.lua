-- string_neg_element_conflict.lua  [NEGATIVE — a table's element kind is
-- monomorphic: String then Integer stores conflict]
-- EXPECT_BUILD_FAIL: Type Error: table element type conflict
local t = {}
t[0] = "a"
t[1] = 7
