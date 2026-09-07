-- bug_04.lua — inner pass: limit n @ b0 ✓; a[φj] safe key, but b[i]'s key is the
-- OUTER φ ≠ inner idx → unsafe write. b = a shares root a via a Table Move, so one
-- poisoned chalice kills both aliases: 2 dyn sets, 0 hoists. The outer pass sees an
-- empty direct body block (just j = 0) — nothing to upgrade.
-- EXPECT: TABLE 0 LEN 1000 NZ 999 CHECKSUM 333333000
-- EXPECT: fast_sets=0
-- EXPECT: fast_gets=0
-- EXPECT: dyn_sets=2
-- EXPECT: dyn_gets=0
-- EXPECT: hoists=0
local a = {}
local b = a
local n = 1000
local i = 0
while i < 10 do
    local j = 0
    while j < n do
        a[j] = j
        b[i] = i
        j = j + 1
    end
    i = i + 1
end
