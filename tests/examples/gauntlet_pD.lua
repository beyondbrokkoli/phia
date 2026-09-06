-- gauntlet_pD.lua — L1 (literal 30): fast, HR @ b0. pd_t[30] = 60 sits in b3 (a pre-header — never scanned) → dyn.
-- L2's gate passes (pd_lim @ b3 < header), but its only table op is the GetTable inside the condition,
-- which lives in the header — headers are never scanned → dyn_get.
-- Body has no table ops → no HR. pd_w[0] after the loop → dyn.
-- EXPECT: TABLE 0 LEN 31 NZ 31 CHECKSUM 2325
-- EXPECT: TABLE 1 LEN 1 NZ 1 CHECKSUM 30
-- EXPECT: fast_sets=1
-- EXPECT: fast_gets=0
-- EXPECT: dyn_sets=2
-- EXPECT: dyn_gets=1
-- EXPECT: hoists=1
local pd_t = {}
local pd_fill = 0
while pd_fill < 30 do
    pd_t[pd_fill] = 1
    pd_fill = pd_fill + 1
end
pd_t[30] = 60
local pd_lim = 60
local pd_i = 0
while pd_t[pd_i] < pd_lim do
    pd_i = pd_i + 1
end
local pd_w = {}
pd_w[0] = pd_i
