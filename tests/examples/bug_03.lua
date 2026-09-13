-- bug_03.lua — n is reassigned in the body, so the limit is a header φ (def block
-- == header) → gate fails before any key analysis runs. All dyn. find_mutated_vars
-- recurses into nested loops, so mutation anywhere lexically inside counts.
-- EXPECT: TABLE 0 LEN 399 NZ 200 CHECKSUM 40000
-- EXPECT: fast_sets=0
-- EXPECT: fast_gets=0
-- EXPECT: dyn_sets=1
-- EXPECT: dyn_gets=0
-- EXPECT: hoists=0
local a = {}
local n = 200
local i = 0
while i < n do
    a[i] = 1
    n = n + 1
    i = i + 2
end
