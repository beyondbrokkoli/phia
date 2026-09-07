-- bug_08.lua — a[φi] safe key, but a[k] with k = φi + 300 → offset key → unsafe
-- write → poisons root a → BOTH writes stay dyn (poison is loop-wide, not
-- per-instruction). Correct decline: k ∈ [300, 309] vs capacity 10.
-- EXPECT: TABLE 0 LEN 310 NZ 20 CHECKSUM 6165
-- EXPECT: fast_sets=0
-- EXPECT: fast_gets=0
-- EXPECT: dyn_sets=2
-- EXPECT: dyn_gets=0
-- EXPECT: hoists=0
local a = {}
local i = 0
while i < 10 do
    a[i] = 1
    local k = i + 300
    a[k] = 2
    i = i + 1
end
