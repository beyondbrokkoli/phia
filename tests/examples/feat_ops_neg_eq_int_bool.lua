-- feat_ops_neg_eq_int_bool.lua [NEGATIVE — == only relates same-type
-- operands: Integer vs Boolean is a build error, unlike Lua where
-- 1 == true is simply false]. Strict typing divergence, pinned.
-- EXPECT_BUILD_FAIL: Type Error: Binary operations currently only support Integers
if 1 == true then
end
