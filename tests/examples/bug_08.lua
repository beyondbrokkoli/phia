-- EXPECT: TABLE 0 LEN 310 NZ 20 CHECKSUM 6165
-- EXPECT: fast_sets=0
-- EXPECT: fast_gets=0
-- EXPECT: dyn_sets=2
-- EXPECT: dyn_gets=0
-- EXPECT: hoists=0
local a = {}
local i = 0
while i < 10 do
    a[i] = 1
    local k = i + 300
    a[k] = 2
    i = i + 1
end
