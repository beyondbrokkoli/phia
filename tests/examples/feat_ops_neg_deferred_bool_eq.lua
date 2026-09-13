-- feat_ops_neg_deferred_bool_eq.lua [NEGATIVE — a deferred table read
-- (unconstrained element type) cannot meet a Boolean in ==: the operand
-- rule only lets a deferred side adopt from an Integer/Float sibling.
-- Boolean elements are not codegenable, so this can never resolve true].
-- EXPECT_BUILD_FAIL: Type Error: Binary operations currently only support Integers
local t = {}
if t[0] == true then
end
