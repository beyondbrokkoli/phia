-- EXPECT: TABLE 0 LEN 100000 NZ 99999 CHECKSUM 333333333300000
-- EXPECT: fast_sets=1
-- EXPECT: hoists=1
-- EXPECT: hoist_ctx=0
local a = {}
local n = 100000
local i = 0
while i < n do
    a[i] = i
    i = i + 1
end
