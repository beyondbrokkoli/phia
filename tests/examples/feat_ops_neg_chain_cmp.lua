-- feat_ops_neg_chain_cmp.lua [NEGATIVE — comparisons do not chain: the
-- parser takes at most one comparison operator per expression, so
-- 1 < 2 < 3 leaves a dangling '<' where a statement was expected. Lua
-- would parse (1 < 2) < 3 and fail at runtime instead — pinned
-- divergence: the error moves from runtime to build time].
-- EXPECT_BUILD_FAIL: Syntax Error: Unexpected statement starting with Some(LessThan)
local x = 1 < 2 < 3
