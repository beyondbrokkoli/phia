-- gauntlet_pC.lua — two sequential loops, same pc_t. L1: safe set → fast.
-- L2: safe set and safe get → both fast;
-- second HR in b3 (L1's exit = L2's pre-header).
-- EXPECT: TABLE 0 LEN 250 NZ 250 CHECKSUM 188250
-- EXPECT: fast_sets=2
-- EXPECT: fast_gets=1
-- EXPECT: dyn_sets=0
-- EXPECT: dyn_gets=0
-- EXPECT: hoists=2
local pc_t = {}
local pc_n = 250
local pc_i = 0
while pc_i < pc_n do
    pc_t[pc_i] = 5
    pc_i = pc_i + 1
end
local pc_j = 0
while pc_j < pc_n do
    pc_t[pc_j] = pc_t[pc_j] + 1
    pc_j = pc_j + 1
end
