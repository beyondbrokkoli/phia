-- feat_local_neg_arity_long.lua [NEGATIVE — a long value list has no
-- name to bind its extra value to: Lua silently drops the `2`, this
-- subset refuses the count mismatch at build time — pinned divergence
-- (dropping a value is an implicit dead store the strict subset won't
-- hide)].
-- EXPECT_BUILD_FAIL: Syntax Error: 'local' binds 1 names to 2 values — counts must match
local a = 1, 2
