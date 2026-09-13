-- tier4_01_flat_plus_nested.lua  [POSITIVE — both fast paths in one loop]
-- The T1 taxonomy shape: a flat affine store AND a nested t[0][i] store
-- coexist; tier-4 upgrades the nested one, tier-2 the flat one. Before
-- tier-4 the nested store declined ITSELF only (fast_sets=1); now both
-- ride hoisted pointers — one per root.
-- EXPECT: TABLE 0 LEN 8 NZ 7 CHECKSUM 168
-- EXPECT: TABLE 1 LEN 1 NZ 1 CHECKSUM 3
-- EXPECT: TABLE 2 LEN 8 NZ 8 CHECKSUM 36
-- EXPECT: NTABLES 3
-- EXPECT: fast_sets=2
-- EXPECT: fast_gets=0
-- EXPECT: dyn_sets=3
-- EXPECT: dyn_gets=1
-- EXPECT: hoists=2
-- EXPECT: hoist_ctx=0,0
local a = {}
a[0] = 0
local t = {}
local inner = {}
inner[0] = 0
t[0] = inner
local i = 0
while i < 8 do
    a[i] = i
    t[0][i] = 1
    i = i + 1
end
