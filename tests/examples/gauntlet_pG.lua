-- gauntlet_pG.lua — L1: fast. L2: pg_dst[pg_rev] key is computed (Sub chain) → unsafe write poisons pg_dst;
-- but pg_src[φj] is a safe-key read on a different root → still fast. HRs: pg_src @ b0 and again @ b3.
-- EXPECT: TABLE 0 LEN 180 NZ 180 CHECKSUM 1960230
-- EXPECT: TABLE 1 LEN 180 NZ 180 CHECKSUM 988260
-- EXPECT: fast_sets=1
-- EXPECT: fast_gets=1
-- EXPECT: dyn_sets=1
-- EXPECT: dyn_gets=0
-- EXPECT: hoists=2
local pg_src = {}
local pg_dst = {}
local pg_n = 180
local pg_i = 0
while pg_i < pg_n do
    pg_src[pg_i] = pg_i + 1
    pg_i = pg_i + 1
end
local pg_j = 0
while pg_j < pg_n do
    local pg_rev = pg_n - 1 - pg_j
    pg_dst[pg_rev] = pg_src[pg_j]
    pg_j = pg_j + 1
end
