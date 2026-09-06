-- bug_16a.lua — identifier condition → gate closed.
-- EXPECT: TABLE 0 LEN 1 NZ 1 CHECKSUM 100
-- EXPECT: fast_sets=0
-- EXPECT: fast_gets=0
-- EXPECT: dyn_sets=1
-- EXPECT: dyn_gets=0
-- EXPECT: hoists=0
local t = {}
local flag = 0 < 1
local k = 0
while flag do
    t[k] = 100
    flag = 0 < 0
    k = k + 1
end
