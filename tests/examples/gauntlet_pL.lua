-- gauntlet_pL.lua — PATCH B FLIP: originally the showcase of loop-wide poisoning
-- (pl_b = pl_b + 0 broke the Move-only trace, and ONE unsafe write killed all
-- three, including the two Move-backed safe keys). key_offset now follows
-- Add/Sub-with-a-constant: pl_b's key folds to φi + 0 → Some(0), provably equal
-- to the checked induction value. PASS 1 finds no unsafe write → no poison →
-- all three writes upgrade, EC+HR @ b0. Tier-1 deliberately accepts offset == 0
-- ONLY: bug_02/08/12 (φi + 300) and bug_09 (idx + 2) still decline — those wait
-- for tier 2 (EC(limit + offset)).
-- EXPECT: fast_sets=3
-- EXPECT: fast_gets=0
-- EXPECT: dyn_sets=0
-- EXPECT: dyn_gets=0
-- EXPECT: hoists=1
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
