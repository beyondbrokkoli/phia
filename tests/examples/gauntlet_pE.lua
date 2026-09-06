-- gauntlet_pE.lua — gate ✓ (pe_m @ b0) but: key pe_d (copy of the outer φpe_i) is not the inner induction var
-- → unsafe write → poisons pe_t → everything dyn, no hoist. Note this is the safety system working, not a missed win:
-- pe_d ranges to 199 against an inner limit of 3 — upgrading would trip SetTableFast's optimizer invariant violated panic at runtime.
-- EXPECT: TABLE 0 LEN 200 NZ 200 CHECKSUM 140700
-- EXPECT: fast_sets=0
-- EXPECT: fast_gets=0
-- EXPECT: dyn_sets=1
-- EXPECT: dyn_gets=0
-- EXPECT: hoists=0
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
