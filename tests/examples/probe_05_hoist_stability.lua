-- probe_05_hoist_stability.lua [POSITIVE — the fast-path invariant:
-- a hoisted table's len NEVER changes mid-loop, a dyn sibling's does].
-- a's stores are affine (key i) -> upgrade + EC to 4 + hoist: len
-- frozen at 4 every trip, and the probe's a[i] read rides the hoisted
-- pointer (fast_gets=1). b's stores have NON-affine keys (i * 50 ->
-- key_offset follows only Add/Sub chains) -> b is clobbered, stays dyn,
-- and its len grows 1, 51, 101, 151 across the same trips. The probe
-- pins the contrast line by line.
-- EXPECT_PROBE pins embed physical register names BY DESIGN (see probe_01).
-- EXPECT_PRINT: iter	0	table(len=4)	100	table(len=1)
-- EXPECT_PRINT: iter	1	table(len=4)	101	table(len=51)
-- EXPECT_PRINT: iter	2	table(len=4)	102	table(len=101)
-- EXPECT_PRINT: iter	3	table(len=4)	103	table(len=151)
-- EXPECT_PROBE: #0 tag="iter" b2 depth1 i_r24 len_r24 i_r25 len_r25
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
    print("iter", i, a, a[i], b)
    i = i + 1
end
