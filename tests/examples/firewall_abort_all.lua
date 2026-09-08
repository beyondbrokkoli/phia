-- firewall_abort_all.lua — FIREWALL sentinel for PASS 1's abort_all branch: the
-- loop mixes a hoistable root write (a[i], Some(0), pre-loop root) with a write
-- whose key is unprovable (j = a[i], a GetTable result -> None) through an
-- UNTRACEABLE table (t is loop-carried: t = a makes it a phi). None-key +
-- untraceable table => abort_all: the whole loop declines, because at runtime
-- t DOES alias a from iteration 1 on — an upgraded a[i] would ride a hoisted
-- pointer that the aliased dyn write could resize. If this ever flips to
-- fast_sets=1/hoists=1, the Patch A UB class is re-open.
-- EXPECT: NTABLES 2
-- EXPECT: TABLE 0 LEN 10 NZ 10 CHECKSUM 57
-- EXPECT: TABLE 1 LEN 2 NZ 1 CHECKSUM 4
-- EXPECT: fast_sets=0
-- EXPECT: fast_gets=0
-- EXPECT: dyn_sets=2
-- EXPECT: dyn_gets=1
-- EXPECT: hoists=0
local a = {}
local t = {}
local n = 10
local i = 0
while i < n do
    a[i] = 1
    local j = a[i]
    t[j] = 2
    t = a
    i = i + 1
end
