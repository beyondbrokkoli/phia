-- string_02_table_loop.lua  [POSITIVE — string phi + string-element table
-- under tier4]. The loop-carried s is a STRING phi: coalescing injects a
-- cloning Move in the pre-header. t's stores are affine (key i), so tier4
-- upgrades them, EC's t to the limit 4 and hoists it BEFORE trip 1 —
-- len frozen at 4 from the first probe line. The probe's own t[i] read
-- rides the upgraded fast path (string clone through the hoisted
-- pointer). Values x, xo, xoo, xooo.
-- Pins embed physical register names BY DESIGN (see probe_01).
-- EXPECT: PROBE iter: i_r22=0 s_r23="xo" len_r26=4 s_r22="xo"
-- EXPECT: PROBE iter: i_r22=1 s_r23="xoo" len_r26=4 s_r22="xoo"
-- EXPECT: PROBE iter: i_r22=2 s_r23="xooo" len_r26=4 s_r22="xooo"
-- EXPECT: PROBE iter: i_r22=3 s_r23="xoooo" len_r26=4 s_r22="xoooo"
-- EXPECT: PROBE exit: s_r23="xoooo" len_r26=4
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
    probe "iter" i, s, t, t[i]
    i = i + 1
end
probe "exit" s, t
