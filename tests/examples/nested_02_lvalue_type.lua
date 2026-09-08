-- nested_02_lvalue_type.lua  [NEGATIVE — Gemini's nil-panic test actually dies here]
-- EXPECT_BUILD_FAIL: Type Error: target is not a table
local a = {}
a[0][0] = 42
