-- bug_18.lua — gate ✓, no table ops in body; a[0], a[1] after → dyn ×2.
-- EXPECT: TABLE 0 LEN 2 NZ 2 CHECKSUM 65
-- EXPECT: fast_sets=0
-- EXPECT: fast_gets=0
-- EXPECT: dyn_sets=2
-- EXPECT: dyn_gets=0
-- EXPECT: hoists=0
local a = {}
local x = 1
local y = 2
local i = 0
while i < 4 do
    local t = y
    y = x + 10
    x = t
    i = i + 1
end
a[0] = x
a[1] = y
