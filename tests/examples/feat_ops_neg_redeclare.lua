-- feat_ops_neg_redeclare.lua [NEGATIVE — the same name cannot be declared
-- twice in the SAME scope; shadowing in a nested scope (if arm, loop body)
-- is legal and pinned by feat_if_03_scope_shadow].
-- EXPECT_BUILD_FAIL: Variable 'x' already declared in this scope
local x = 1
local x = 2
