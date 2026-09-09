-- string_neg_mixed_concat.lua  [NEGATIVE — Lua's number coercion on
-- concat is refused: 2 .. "x" is a build error, the float_03 family]
-- EXPECT_BUILD_FAIL: Type Error: strings only support '..' and == / ~= between two strings
local x = "a" .. 5
