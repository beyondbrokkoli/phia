-- tier4_03_rebind_decline.lua  [POSITIVE — pins the conservative rebind rule]
-- The loop stores through t (t[0] = spare), so t's root is in
-- region_stored_roots: the child handle t[0] may be rebound mid-loop and
-- MUST NOT be materialized. Whole loop stays dyn — correctness over
-- speed. The runtime shuffle is the payoff witness: iteration 0 writes
-- inner (t[0] was inner), iterations 1..7 re-read the rebound slot and
-- write spare; the final t[0] is spare's handle (3).
-- EXPECT: TABLE 0 LEN 1 NZ 1 CHECKSUM 3
-- EXPECT: TABLE 1 LEN 1 NZ 1 CHECKSUM 1
-- EXPECT: TABLE 2 LEN 8 NZ 7 CHECKSUM 35
-- EXPECT: NTABLES 3
-- EXPECT: fast_sets=0
-- EXPECT: fast_gets=0
-- EXPECT: dyn_sets=5
-- EXPECT: dyn_gets=1
-- EXPECT: hoists=0
local t = {}
local inner = {}
inner[0] = 0
t[0] = inner
local spare = {}
spare[0] = 0
local i = 0
while i < 8 do
    t[0][i] = 1
    t[0] = spare
    i = i + 1
end
