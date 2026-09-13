-- float_08_assign_mismatch.lua  [NEGATIVE — Rule #2, float edition]
-- Scalar variables are monomorphic: an Integer variable cannot be rebound
-- to a Float value (and vice versa). Table variables rebind (wart 3);
-- scalars never do.
-- EXPECT_BUILD_FAIL: Type Error (Rule #2 Violation): Cannot assign Float to variable 'x' of type Integer
local x = 1
x = 2.5
