-- tier4_09_multihop.lua  [POSITIVE — multi-hop chains]
-- t[0][0][i]: the store's feeder is t[0][0], whose parent is the feeder
-- t[0], whose parent is the root t. Tier-4 walks the chain — every hop
-- an in-region GetTable with a const key >= 0 — and re-materializes it
-- in the pre-header, root down: two minted dyn GetTables (the surviving
-- dyn_gets=2), and only the LEAF child is EC'd + hoisted; the
-- intermediate handle is a resolution step, not a target. Both in-loop
-- feeders were singly-used and die with the rewrite. Chained mints are
-- deduped per (parent, key), so sibling ops on shared prefixes reuse
-- the upper handles.
-- EXPECT: TABLE 0 LEN 1 NZ 1 CHECKSUM 2
-- EXPECT: TABLE 1 LEN 1 NZ 1 CHECKSUM 3
-- EXPECT: TABLE 2 LEN 8 NZ 8 CHECKSUM 36
-- EXPECT: NTABLES 3
-- EXPECT: fast_sets=1
-- EXPECT: fast_gets=0
-- EXPECT: dyn_sets=3
-- EXPECT: dyn_gets=2
-- EXPECT: hoists=1
-- EXPECT: hoist_ctx=0
local t = {}
local mid = {}
local inner = {}
inner[0] = 0
mid[0] = inner
t[0] = mid
local i = 0
while i < 8 do
    t[0][0][i] = 1
    i = i + 1
end
