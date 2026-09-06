-- gauntlet_pL.lua — pl_b = pl_b + 0 breaks the Move-chain (Add isn't traced) → unsafe key → poisons pl_t for the whole loop.
-- All three writes stay dyn, including the two Move-backed safe keys (pl_a, pl_c) — poisoning is loop-wide, not per-instruction.
-- EXPECT: TABLE 0 LEN 80 NZ 80 CHECKSUM 9720
-- EXPECT: fast_sets=0
-- EXPECT: fast_gets=0
-- EXPECT: dyn_sets=3
-- EXPECT: dyn_gets=0
-- EXPECT: hoists=0
local pl_t = {}
local pl_n = 80
local pl_i = 0
while pl_i < pl_n do
    local pl_a = pl_i
    pl_t[pl_a] = 1
    local pl_b = pl_a
    pl_b = pl_b + 0
    pl_t[pl_b] = 2
    local pl_c = pl_i
    pl_t[pl_c] = 3
    pl_i = pl_i + 1
end
