-- bug_10b.lua — identical shape to bug_01.
-- EXPECT: TABLE 0 LEN 1000 NZ 1000 CHECKSUM 3503500
-- EXPECT: fast_sets=1
-- EXPECT: fast_gets=0
-- EXPECT: dyn_sets=0
-- EXPECT: dyn_gets=0
-- EXPECT: hoists=1
local a = {}
local i = 0
while i < 1000 do
    a[i] = 7
    i = i + 1
end
