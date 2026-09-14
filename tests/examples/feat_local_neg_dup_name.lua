-- feat_local_neg_dup_name.lua [NEGATIVE — two names in one binding are
-- two declarations in the SAME scope: a duplicate is the redeclare
-- error, not a silent last-wins overwrite (feat_ops_neg_redeclare
-- pins the two-statement spelling of the same rule)].
-- EXPECT_BUILD_FAIL: Variable 'a' already declared in this scope
local a, a = 1, 2
