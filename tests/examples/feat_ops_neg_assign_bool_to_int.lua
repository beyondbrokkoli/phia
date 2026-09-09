-- feat_ops_neg_assign_bool_to_int.lua [NEGATIVE — scalar variables are
-- monomorphic: a Boolean value cannot be assigned to an Integer variable
-- (Rule #2)].
-- EXPECT_BUILD_FAIL: Type Error (Rule #2 Violation): Cannot assign Boolean to variable 'x' of type Integer
local x = 1
x = true
