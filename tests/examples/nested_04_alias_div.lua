-- nested_04_alias_div.lua  [NEGATIVE — repinned: the store arm fires, not the alias arm]
-- EXPECT_BUILD_FAIL: Type Error: Cannot store unresolved table into table of Integer
local a = {}
local b = a
b[0] = 1
a[0] = {}
