-- nested_03_type_error.lua  [NEGATIVE — repinned: lazy unification names the real conflict]
-- Storing an Integer and then a table into the same table is a genuine
-- monomorphic element conflict — unification rejects it regardless of
-- statement order. The old message ("Cannot store unresolved table into
-- table of Integer") blamed the store's unresolvedness, which is no longer
-- a crime; the conflict itself always was.
-- EXPECT_BUILD_FAIL: Type Error: table element type conflict (Integer vs Table(?))
local a = {}
a[0] = 42
local b = {}
a[1] = b
