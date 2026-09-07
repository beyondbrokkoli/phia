-- bug_13.lua — a[φi + 300] write (offset key) → unsafe → poisons root a. The read
-- j = a[j] has a safe key (Move of φi) and the write a[j] = 1 has an untraceable key
-- (a GetTable result — the chain dies at non-Move defs), but it's moot: same poisoned
-- root → 2 dyn sets + 1 dyn get. Data-dependent keys are the general poison case.
-- EXPECT: TABLE 0 LEN 310 NZ 20 CHECKSUM 19885
-- EXPECT: fast_sets=0
-- EXPECT: fast_gets=0
-- EXPECT: dyn_sets=2
-- EXPECT: dyn_gets=1
-- EXPECT: hoists=0
local a = {}
local n = 10
local i = 0
while i < n do
    a[i] = i + 300
    local j = i
    j = a[j]
    a[j] = 1
    i = i + 1
end
