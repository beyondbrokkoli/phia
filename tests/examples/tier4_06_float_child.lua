-- tier4_06_float_child.lua  [POSITIVE — tier-4 composes with float fast paths]
-- A FLOAT table nested in another table: the materialized child handle
-- flows into SetTableFast with a Float ty, float_roots covers the minted
-- child register, and the loop stores f64 through a *mut f64 hoisted off
-- the arena. Two milestones shaking hands.
-- EXPECT: TABLE 0 LEN 1 NZ 1 CHECKSUM 2
-- EXPECT: TABLE 1 LEN 8 NZ 8 CHECKSUM -486388759756013568 SUM 2
-- EXPECT: NTABLES 2
-- EXPECT: fast_sets=1
-- EXPECT: fast_gets=0
-- EXPECT: dyn_sets=2
-- EXPECT: dyn_gets=1
-- EXPECT: hoists=1
-- EXPECT: hoist_ctx=0
local t = {}
local ft = {}
ft[0] = 0.5
t[0] = ft
local i = 0
while i < 8 do
    t[0][i] = 0.25
    i = i + 1
end
