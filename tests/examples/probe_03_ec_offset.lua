-- probe_03_ec_offset.lua [POSITIVE — the EC padding arithmetic, literal].
-- Stores at t[i+1] are affine with offset +1, so tier4's pre-header
-- EnsureCapacity sizes t to limit + max_off = 3 + 1 = 4 — BEFORE trip 1.
-- The probe pins that literal as a runtime invariant: len frozen at 4
-- from the first line on, values t[1..3] = 0, 10, 20 (CHECKSUM 110).
-- The probe's own t[i+1] read rides the upgraded fast path.
-- Pins embed physical register names BY DESIGN (see probe_01).
-- EXPECT: PROBE iter: i_r18=0 len_r18=4
-- EXPECT: PROBE iter: i_r18=1 len_r18=4
-- EXPECT: PROBE iter: i_r18=2 len_r18=4
-- EXPECT: TABLE 0 LEN 4 NZ 2 CHECKSUM 110
-- EXPECT: NTABLES 1
-- EXPECT: fast_sets=1
-- EXPECT: dyn_sets=0
-- EXPECT: hoists=1
local t = {}
local i = 0
while i < 3 do
    t[i + 1] = i * 10
    probe("iter", i, t)
    i = i + 1
end
