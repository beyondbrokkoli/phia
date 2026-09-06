-- EXPECT: TABLE 0 LEN 399 NZ 200 CHECKSUM 40000
-- EXPECT: fast_sets=0
local a = {}
local n = 200
local i = 0
while i < n do
    a[i] = 1
    n = n + 1
    i = i + 2
end
