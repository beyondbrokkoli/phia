-- EXPECT_PANIC: Negative
local a = {}
local i = 0
while i < 10 do
    a[i] = i
    i = i + 1
end
local k = 0 - 1
a[k] = 5
