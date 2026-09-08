-- EXPECT_PANIC: Runtime Error: Negative table index
local a = {}
local k = 0 - 3
local x = a[k]
