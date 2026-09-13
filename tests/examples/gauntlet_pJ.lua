-- gauntlet_pJ.lua — two invariant loops, safe keys → 2 fast sets, 2 HRs (b0, b3).
-- Tail block: 4 reads + 4 writes, all dyn.
-- EXPECT: TABLE 0 LEN 350 NZ 249 CHECKSUM 12453250
-- EXPECT: TABLE 1 LEN 4 NZ 3 CHECKSUM 1597
-- EXPECT: fast_sets=2
-- EXPECT: fast_gets=0
-- EXPECT: dyn_sets=4
-- EXPECT: dyn_gets=4
-- EXPECT: hoists=2
local pj_t = {}
local pj_hi1 = 200
local pj_i = 100
while pj_i < pj_hi1 do
    pj_t[pj_i] = pj_i - 100
    pj_i = pj_i + 1
end
local pj_hi2 = 350
local pj_j = 200
while pj_j < pj_hi2 do
    pj_t[pj_j] = pj_j
    pj_j = pj_j + 1
end
local pj_w = {}
pj_w[0] = pj_t[150]
pj_w[1] = pj_t[250]
pj_w[2] = pj_t[349]
pj_w[3] = pj_t[99]
