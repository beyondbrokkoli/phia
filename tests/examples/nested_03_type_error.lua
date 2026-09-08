-- nested_03_type_error.lua  [NEGATIVE — Gemini's, correct as pinned]
-- EXPECT_BUILD_FAIL: Type Error: Cannot store unresolved table into table of Integer
local a = {}
a[0] = 42
local b = {}
a[1] = b
