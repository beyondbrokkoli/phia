-- string_02_table_loop.lua  [POSITIVE — string phi + string-element table
-- under tier4]. The loop-carried s is a STRING phi: coalescing injects a
-- cloning Move in the pre-header. t's stores are affine (key i), so tier4
-- upgrades them, EC's t to the limit 4 and hoists it BEFORE trip 1 —
-- len frozen at 4 from the first probe line. The probe's own t[i] read
-- rides the upgraded fast path (string clone through the hoisted
-- pointer). Values x, xo, xoo, xooo.
-- EXPECT_PROBE pins embed physical register names BY DESIGN (see probe_01).
-- EXPECT_PRINT: iter	0	xo	table(len=4)	xo
-- EXPECT_PRINT: iter	1	xoo	table(len=4)	xoo
-- EXPECT_PRINT: iter	2	xooo	table(len=4)	xooo
-- EXPECT_PRINT: iter	3	xoooo	table(len=4)	xoooo
-- EXPECT_PRINT: exit	xoooo	table(len=4)
-- EXPECT_PROBE: #0 tag="iter" b2 depth1 i_r20 s_r21 len_r24 s_r20
-- EXPECT_PROBE: #1 tag="exit" b3 depth0 s_r21 len_r24
-- EXPECT: TABLE 0 LEN 4 NZ 4 CHECKSUM -7302824913170803880
-- EXPECT: NTABLES 1
-- EXPECT: fast_sets=1
-- EXPECT: fast_gets=1
-- EXPECT: hoists=1
local t = {}
local s = "x"
local i = 0
while i < 4 do
    s = s .. "o"
    t[i] = s
    print("iter", i, s, t, t[i])
    i = i + 1
end
print("exit", s, t)
