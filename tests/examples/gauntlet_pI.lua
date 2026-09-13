-- gauntlet_pI.lua — identifier condition: the Branch cond is a φ, not a Less → gate never opens.
-- EXPECT: TABLE 0 LEN 1 NZ 1 CHECKSUM 100
-- EXPECT: fast_sets=0
-- EXPECT: fast_gets=0
-- EXPECT: dyn_sets=1
-- EXPECT: dyn_gets=0
-- EXPECT: hoists=0
local pi_t = {}
local pi_flag = 0 < 1
local pi_k = 0
while pi_flag do
    pi_t[pi_k] = 100
    pi_flag = 0 < 0
    pi_k = pi_k + 1
end
