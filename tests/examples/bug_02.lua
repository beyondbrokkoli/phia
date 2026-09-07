-- bug_02.lua — gate ✓ (literal 10) but the write's key is j = φi + 300: an Add
-- breaks the Move-only trace (offset 300 ≠ 0) → unsafe write → poisons a. Correct
-- decline: j ∈ [300, 309] vs capacity 10 — the fast path's invariant panic would
-- fire where the dynamic path happily resizes.
-- EXPECT: TABLE 0 LEN 310 NZ 10 CHECKSUM 3055
-- EXPECT: fast_sets=0
-- EXPECT: fast_gets=0
-- EXPECT: dyn_sets=1
-- EXPECT: dyn_gets=0
-- EXPECT: hoists=0
local a = {}
local i = 0
while i < 10 do
    local j = i
    j = j + 300
    a[j] = 1
    i = i + 1
end
