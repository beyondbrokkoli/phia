-- bug_17.lua — gate ✓ (literal 0 materialized) but the body has no table ops → no HR. w[0] = i after → dyn.
-- EXPECT: TABLE 0 LEN 1 NZ 1 CHECKSUM 5
-- EXPECT: fast_sets=0
-- EXPECT: fast_gets=0
-- EXPECT: dyn_sets=1
-- EXPECT: dyn_gets=0
-- EXPECT: hoists=0
local i = 5
local n = 0
while i < n do
    i = i + 1
end
local w = {}
w[0] = i
