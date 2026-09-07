-- gauntlet_pE.lua — PATCH D FLIP: originally the showcase of a correct decline
-- (inner limit 3 vs key up to 199 — the inner pass must NEVER take this key).
-- Region-wide PASS 2 instead reaches it from the OUTER pass: pe_d = Move(φpe_i)
-- holds the outer's checked value (< pe_n = 200), it's the only SetTable in the
-- region (PASS 1: no poison), table @ b0 → upgrade + EC/HR @ b0. One key, two
-- verdicts: unsafe for the inner loop, provably safe for the outer.
-- Contrast pF, where the key IS the inner phi (reassigned in-region →
-- untraceable → outer poisons, inner upgrades — unchanged).
-- EXPECT: fast_sets=1
-- EXPECT: fast_gets=0
-- EXPECT: dyn_sets=0
-- EXPECT: dyn_gets=0
-- EXPECT: hoists=1
local pe_t = {}
local pe_n = 200
local pe_m = 3
local pe_i = 0
while pe_i < pe_n do
    local pe_d = pe_i
    local pe_j = 0
    while pe_j < pe_m do
        pe_t[pe_d] = 7
        pe_j = pe_j + 1
    end
    pe_i = pe_i + 1
end
