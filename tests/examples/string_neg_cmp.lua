-- string_neg_cmp.lua  [NEGATIVE — no ordering on strings: < is a build
-- error (Lua's lexicographic compare is a deferred divergence)]
-- EXPECT_BUILD_FAIL: Type Error: strings only support '..' and == / ~= between two strings
local x = "a" < "b"
