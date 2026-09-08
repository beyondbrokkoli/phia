-- nested_08_fresh_store_restricted.lua  [NEGATIVE — documents the `a[0] = {}` restriction]
-- EXPECT_BUILD_FAIL: Type Error: Cannot store unresolved table. Annotate by storing first.
local a = {}
local b = {}
b[0] = 1
a[0] = {}
