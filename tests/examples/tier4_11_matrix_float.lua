-- tier4_11_matrix_float.lua  [POSITIVE — matrix x float composition]
-- The tier4_10 shape with FLOAT children: the minted row handle (a fast
-- *p_t.add(i) read since tier4_13, against t's *mut i64 handle-array
-- pointer hoisted at ctx 0) flows into SetTableFast with a Float ty,
-- and the per-row hoisted pointer is *mut f64 off the farray side; EC
-- zero-fills with 0.0. Four milestones composing in one loop: nested
-- tables, float values, per-row hoisting, fast row resolution. The
-- universes collide and the checksums agree.
-- EXPECT: TABLE 0 LEN 2 NZ 2 CHECKSUM 8
-- EXPECT: TABLE 1 LEN 4 NZ 4 CHECKSUM 9088264048033660928 SUM 1
-- EXPECT: TABLE 2 LEN 4 NZ 4 CHECKSUM 9088264048033660928 SUM 1
-- EXPECT: NTABLES 3
-- EXPECT: fast_sets=1
-- EXPECT: fast_gets=1
-- EXPECT: dyn_sets=4
-- EXPECT: dyn_gets=0
-- EXPECT: hoists=2
-- EXPECT: hoist_ctx=0,1
local t = {}
local row0 = {}
row0[0] = 0.5
t[0] = row0
local row1 = {}
row1[0] = 0.5
t[1] = row1
local i = 0
while i < 2 do
    local j = 0
    while j < 4 do
        t[i][j] = 0.25
        j = j + 1
    end
    i = i + 1
end
