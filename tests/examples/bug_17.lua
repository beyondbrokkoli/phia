-- bug_17.lua — gate ✓ (literal 0 materialized) but the body has zero table ops →
-- hoists=0: the BTreeSet only grows alongside upgrades, never from a bare gate pass.
-- Zero-iteration loop: the header runs once, φi resolves to the pre-loop 5, w[0] = 5.
-- The lone post-loop write is the dyn_set.
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
