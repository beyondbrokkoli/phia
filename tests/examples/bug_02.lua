-- bug_02.lua — TIER 2 FLIP: the write's key j = φi + 300 folds to Some(300) —
-- a non-negative constant offset. PASS 1 no longer poisons it; the offset is
-- accounted and PAID: EC mints LoadInt(310) in the pre-header, so len covers
-- j ∈ [300, 309] and the fast store's bounds check can never trip. The old
-- decline ("offset breaks the Move-only trace") is now a bounded contract.
-- EXPECT: TABLE 0 LEN 310 NZ 10 CHECKSUM 3055
-- EXPECT: fast_sets=1
-- EXPECT: fast_gets=0
-- EXPECT: dyn_sets=0
-- EXPECT: dyn_gets=0
-- EXPECT: hoists=1
local a = {}
local i = 0
while i < 10 do
    local j = i
    j = j + 300
    a[j] = 1
    i = i + 1
end
