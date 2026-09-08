-- tier2_reg_limit.lua — TIER 2 sentinel for the REGISTER-limit EC path: the
-- inner bound is the outer phi of n (a live register, not a literal), and the
-- write key carries offset +5. PASS 1 accounts off=5 on root a; PASS 3 mints
-- LoadInt(5) + Add(phi_n, 5) in the inner pre-header and sizes EC to it — the
-- folded-literal path cannot reach this. The outer pass declines root a (the
-- write's key traces to the INNER phi: None w.r.t. the outer), so the hoist
-- belongs to the inner loop alone. k=0: EC(0+5) resizes to 5, zero iterations;
-- k=1: EC(3+5) resizes to 8, three fast writes at [5,7].
-- EXPECT: TABLE 0 LEN 8 NZ 3 CHECKSUM 21
-- EXPECT: fast_sets=1
-- EXPECT: fast_gets=0
-- EXPECT: dyn_sets=0
-- EXPECT: dyn_gets=0
-- EXPECT: hoists=1
local a = {}
local n = 0
local k = 0
while k < 2 do
    local i = 0
    while i < n do
        a[i + 5] = 1
        i = i + 1
    end
    n = n + 3
    k = k + 1
end 
