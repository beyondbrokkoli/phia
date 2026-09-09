-- feat_ops_neg_cmp_tables.lua [NEGATIVE — relational comparison of two
-- tables is a build error (== on tables is out too: only numeric and
-- Boolean equality exist)].
-- EXPECT_BUILD_FAIL: Type Error: Binary operations currently only support Integers
local a = {}
local b = {}
if a < b then
end
