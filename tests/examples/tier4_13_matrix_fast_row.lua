-- tier4_13_matrix_fast_row.lua  [POSITIVE — fast row resolution, the
-- outer-level matrix door]. At the OUTER loop the store t[i][j] = v is a
-- non-affine SCALAR child-store — one table hop from the root t — so it
-- no longer aborts the region (the old firewall): t's pointer pair hoists
-- above the outer loop (ctx 0), the row feeder t[i] upgrades to a fast
-- handle read, and the inner pass re-materializes the resolution IN
-- KIND — the minted row rides p_t (`row = *p_t.add(i)`, once per outer
-- trip), EC + hoist re-arm per row at depth 1. The second loop composes
-- the same door for reads: the leaf t[i2][j2] rides the per-row pointer
-- too. Same program, both doors, zero dyn left in the loop pair.
-- The REACH gate that keeps this sound is pinned by tier4_14/15.
-- EXPECT: TABLE 0 LEN 2 NZ 2 CHECKSUM 8
-- EXPECT: TABLE 1 LEN 4 NZ 4 CHECKSUM 10
-- EXPECT: TABLE 2 LEN 4 NZ 4 CHECKSUM 10
-- EXPECT: TABLE 3 LEN 1 NZ 1 CHECKSUM 8
-- EXPECT: NTABLES 4
-- EXPECT: fast_sets=1
-- EXPECT: fast_gets=3
-- EXPECT: dyn_sets=5
-- EXPECT: dyn_gets=0
-- EXPECT: hoists=4
-- EXPECT: hoist_ctx=0,1,0,1
local t = {}
local r0 = {}
r0[0] = 0
t[0] = r0
local r1 = {}
r1[0] = 0
t[1] = r1
local i = 0
while i < 2 do
    local j = 0
    while j < 4 do
        t[i][j] = 1
        j = j + 1
    end
    i = i + 1
end
local s = 0
local i2 = 0
while i2 < 2 do
    local j2 = 0
    while j2 < 4 do
        s = s + t[i2][j2]
        j2 = j2 + 1
    end
    i2 = i2 + 1
end
local w = {}
w[0] = s
