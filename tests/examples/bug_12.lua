-- bug_12.lua — the alias-poison showcase. b = a shares root a; b[k] with k = φi + 300
-- → unsafe write → poisons root a → a[i] (safe key, same root) stays dyn too.
-- Reassigning b = c mid-loop can't launder it: roots, not regs, hold the grudge.
-- EXPECT: NTABLES 2
-- EXPECT: TABLE 0 LEN 310 NZ 20 CHECKSUM 3110
-- EXPECT: TABLE 1 LEN 0 NZ 0 CHECKSUM 0
-- EXPECT: fast_sets=0
-- EXPECT: fast_gets=0
-- EXPECT: dyn_sets=2
-- EXPECT: dyn_gets=0
-- EXPECT: hoists=0
local a = {}
local c = {}
local n = 10
local i = 0
while i < n do
    local b = a
    local k = i + 300
    b[k] = 1
    a[i] = 1
    b = c
    i = i + 1
end
