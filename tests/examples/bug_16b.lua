-- bug_16b.lua — gate ✓ but t is reassigned in-loop: both loop ops use φt (def = header ≥ header) → dyn.
-- Tail w[0] = t[2] → dyn set + dyn get.
-- EXPECT: TABLE 0 LEN 0 NZ 0 CHECKSUM 0
-- EXPECT: TABLE 1 LEN 1 NZ 1 CHECKSUM 5
-- EXPECT: TABLE 2 LEN 2 NZ 1 CHECKSUM 22
-- EXPECT: TABLE 3 LEN 3 NZ 1 CHECKSUM 21
-- EXPECT: TABLE 4 LEN 1 NZ 1 CHECKSUM 7
-- EXPECT: fast_sets=0
-- EXPECT: fast_gets=0
-- EXPECT: dyn_sets=2
-- EXPECT: dyn_gets=2
-- EXPECT: hoists=0
local t = {}
local n = 3
local i = 0
while i < n do
    local x = t[0]
    t = {}
    t[i] = x + i + 5
    i = i + 1
end
local w = {}
w[0] = t[2]
