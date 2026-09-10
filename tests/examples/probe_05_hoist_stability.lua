-- probe_05_hoist_stability.lua [POSITIVE — the fast-path invariant:
-- a hoisted table's len NEVER changes mid-loop, a dyn sibling's does].
-- a's stores are affine (key i) -> upgrade + EC to 4 + hoist: len_r
-- frozen at 4 every trip, and the probe's a[i] read rides the hoisted
-- pointer (fast_gets=1). b's stores have NON-affine keys (i * 50 ->
-- key_offset follows only Add/Sub chains) -> b is clobbered, stays dyn,
-- and its len grows 1, 51, 101, 151 across the same trips. The probe
-- pins the contrast line by line.
-- Pins embed physical register names BY DESIGN (see probe_01).
-- EXPECT: PROBE iter: i_r24=0 len_r24=4 i_r25=100 len_r25=1
-- EXPECT: PROBE iter: i_r24=1 len_r24=4 i_r25=101 len_r25=51
-- EXPECT: PROBE iter: i_r24=2 len_r24=4 i_r25=102 len_r25=101
-- EXPECT: PROBE iter: i_r24=3 len_r24=4 i_r25=103 len_r25=151
-- EXPECT: TABLE 0 LEN 4 NZ 4 CHECKSUM 1020
-- EXPECT: TABLE 1 LEN 151 NZ 4 CHECKSUM 304
-- EXPECT: NTABLES 2
-- EXPECT: fast_sets=1
-- EXPECT: fast_gets=1
-- EXPECT: hoists=1
local a = {}
local b = {}
local i = 0
while i < 4 do
    a[i] = i + 100
    b[i * 50] = 1
    probe("iter", i, a, a[i], b)
    i = i + 1
end
