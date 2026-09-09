-- feat_ops_neg_if_cond_int.lua [NEGATIVE — an if condition must be a
-- Boolean; integers are not truthy here (float_07 pins the while flavor)].
-- EXPECT_BUILD_FAIL: Type Error: 'if' condition must be a Boolean
if 5 then
end
