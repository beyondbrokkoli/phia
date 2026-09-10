-- probe_01_scalar.lua  [POSITIVE — the probe intrinsic, straight-line slice]
-- Every operand kind in one program: int, float, bool, table (pointer mode:
-- no nested stores, so table operands print materialized length, no handle).
-- PROBE lines are deterministic output and pinned exactly like TABLE lines.
-- Pins embed physical register names BY DESIGN: the probe's subject is the
-- vreg->physical mapping, so an allocator change must break these pins just
-- as it breaks the byte locks.
-- EXPECT: PROBE scalars: i_r0=5 f_r22=2.5 b_r2=true len_r20=1
-- EXPECT: PROBE after: i_r20=7 i_r21=6
-- EXPECT: NTABLES 1
-- EXPECT: TABLE 0 LEN 2 NZ 2 CHECKSUM 19
local a = 5
local f = 2.5
local flag = true
local t = {}
t[0] = 7
probe("scalars", a, f, flag, t)
t[1] = a + 1
probe("after", t[0], t[1])
