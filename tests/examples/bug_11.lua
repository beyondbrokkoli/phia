-- EXPECT: TABLE 0 LEN 10 NZ 10 CHECKSUM 55
-- EXPECT: fast_sets=1
-- EXPECT: hoists=1
local a = {}
local b = a
a = b
local n = 10
local i = 0
while i < n do
    a[i] = 1
    i = i + 1
end
