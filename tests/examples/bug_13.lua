-- EXPECT: TABLE 0 LEN 310 NZ 20 CHECKSUM 19885
-- EXPECT: fast_sets=0
-- EXPECT: fast_gets=0
-- EXPECT: dyn_sets=2
-- EXPECT: dyn_gets=1
-- EXPECT: hoists=0
local a = {}
local n = 10
local i = 0
while i < n do
    a[i] = i + 300
    local j = i
    j = a[j]
    a[j] = 1
    i = i + 1
end
