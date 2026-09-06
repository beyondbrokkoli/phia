-- EXPECT: TABLE 0 LEN 1000 NZ 1000 CHECKSUM 3503500
-- EXPECT: fast_sets=1
-- EXPECT: dyn_sets=0
local a = {}
local i = 0
while i < 1000 do
    a[i] = 7
    i = i + 1
end
