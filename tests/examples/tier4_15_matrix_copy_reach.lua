-- tier4_15_matrix_copy_reach.lua  [POSITIVE — transitive REACH via a
-- copied handle]. t[0] = u[0] copies a handle OUT of u's slots: by the
-- copy rule, everything REACH(u) names may now sit in t's slots — and
-- REACH(u) = {v}, so t[0] IS v at runtime. Without the fixpoint's copy
-- rule the analysis would miss that alias and hoist v for the affine
-- v[i] = 9 — then the inner loop's EC (sizing the aliased child to 4)
-- reallocs v's array underneath the hoisted pointer: the same UB class
-- tier4_14 pins with a direct alias, reached transitively here. v stays
-- dyn; the inner child still converts (empty hazard at depth 1).
-- Final: v = [3,3,3,3] (each row's 9 is stomped by the t[0][j] sweep —
-- both name the same table); u and t each hold v's handle once.
-- EXPECT: TABLE 0 LEN 4 NZ 4 CHECKSUM 30
-- EXPECT: TABLE 1 LEN 1 NZ 1 CHECKSUM 1
-- EXPECT: TABLE 2 LEN 1 NZ 1 CHECKSUM 1
-- EXPECT: NTABLES 3
-- EXPECT: fast_sets=1
-- EXPECT: fast_gets=0
-- EXPECT: dyn_sets=3
-- EXPECT: dyn_gets=2
-- EXPECT: hoists=1
-- EXPECT: hoist_ctx=1
local v = {}
local u = {}
u[0] = v
local t = {}
t[0] = u[0]
local i = 0
while i < 4 do
    v[i] = 9
    local j = 0
    while j < 4 do
        t[0][j] = 3
        j = j + 1
    end
    i = i + 1
end
