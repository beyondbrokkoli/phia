-- nested_04_alias_div.lua  [NEGATIVE — repinned: same conflict, reached through the alias]
-- b aliases a, so b's store constrains the SAME element variable (aliases
-- share unification variables; the old write-once-by-identity rule is now
-- variable identity). The Integer/table clash fires on a's store exactly as
-- it would on b's — alias divergence cannot happen, only genuine conflicts.
-- EXPECT_BUILD_FAIL: Type Error: table element type conflict (Integer vs Table(?))
local a = {}
local b = a
b[0] = 1
a[0] = {}
