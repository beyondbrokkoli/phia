-- nested_07_read_before_write_fail.lua  [NEGATIVE — seeded so the MISMATCH arm fires]
-- EXPECT_BUILD_FAIL: Type Error: table element type mismatch
local a = {}
local x = a[0]
local b = {}
b[0] = 0
a[1] = b
