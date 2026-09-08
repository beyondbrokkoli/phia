-- nested_09_nil_panic.lua  [PANIC — the genuine nil case Gemini wanted in 02]
-- EXPECT_PANIC: Runtime Error: table is nil
local a = {}
local b = {}
b[0] = 0
a[0] = b
a[5][0] = 1
