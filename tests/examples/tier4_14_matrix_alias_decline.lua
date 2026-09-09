-- tier4_14_matrix_alias_decline.lua  [POSITIVE — the REACH hazard gate].
-- u's handle sits in t's slot 1 (a rooted table-val store, so the REACH
-- closure sees it: REACH(t) = {r0, u}). The outer loop wants two
-- conversions but may have one: u[i] = 7 is affine and rooted (a
-- textbook tier-2 candidate), while the inner store t[0][j] = 3 is a
-- non-affine SCALAR child-store at the OUTER level whose mutation
-- target is exactly REACH(t) — u included. Hoisting u mid-loop would
-- ride a pointer the aliased store can resize under (and the inner
-- pass's EC, sizing the aliased child, can realloc it after the hoist —
-- the UB class this gate exists for). PASS 2 declines u; it stays dyn.
-- The INNER loop still converts its own child (hazard is empty there:
-- the same store's key is the INNER phi, affine) — decline-by-level,
-- not decline-everything: the old behavior aborted the whole region.
-- EXPECT: TABLE 0 LEN 4 NZ 4 CHECKSUM 70
-- EXPECT: TABLE 1 LEN 4 NZ 4 CHECKSUM 30
-- EXPECT: TABLE 2 LEN 2 NZ 2 CHECKSUM 4
-- EXPECT: NTABLES 3
-- EXPECT: fast_sets=1
-- EXPECT: fast_gets=0
-- EXPECT: dyn_sets=3
-- EXPECT: dyn_gets=1
-- EXPECT: hoists=1
-- EXPECT: hoist_ctx=1
local u = {}
local r0 = {}
local t = {}
t[0] = r0
t[1] = u
local i = 0
while i < 4 do
    u[i] = 7
    local j = 0
    while j < 4 do
        t[0][j] = 3
        j = j + 1
    end
    i = i + 1
end
