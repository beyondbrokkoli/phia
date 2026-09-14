-- probe_01_scalar.lua  [POSITIVE — the print intrinsic, straight-line slice]
-- Every operand kind in one program: int, float, bool, table (pointer mode:
-- no nested stores, so table operands print materialized length, no handle).
-- Print lines are clean Lua-style output (tab-separated values, strings raw)
-- and pinned whole-line like TABLE lines (EXPECT_PRINT). The register
-- mapping the line used to carry lives in probe_map.txt, pinned whole-line
-- by EXPECT_PROBE: the probe's subject is the vreg->physical mapping, so an
-- allocator change must break those pins just as it breaks the byte locks.
-- EXPECT_PRINT: scalars	5	2.5	true	table(len=1)
-- EXPECT_PRINT: after	7	6
-- EXPECT_PROBE: #0 tag="scalars" b0 depth0 i_r0 f_r22 b_r2 len_r20
-- EXPECT_PROBE: #1 tag="after" b0 depth0 i_r20 i_r21
-- EXPECT: NTABLES 1
-- EXPECT: TABLE 0 LEN 2 NZ 2 CHECKSUM 19
local a = 5
local f = 2.5
local flag = true
local t = {}
t[0] = 7
print("scalars", a, f, flag, t)
t[1] = a + 1
print("after", t[0], t[1])
