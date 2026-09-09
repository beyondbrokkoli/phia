-- feat_ops_neg_mixed_cmp.lua [NEGATIVE — the mixed-operand wall applies to
-- comparisons too, not just arithmetic]. Strict no-coercion typing: an
-- Integer cannot be compared against a Float (same pinned divergence as
-- float_03, comparison flavor).
-- EXPECT_BUILD_FAIL: Type Error: Binary operations do not support mixed Integer and Float
local a = 1
local b = 1.5
if a < b then
end
