-- probe_02_loop_phi.lua  [POSITIVE — in-loop probe: phi trajectory + arena growth]
-- Nested program (table-valued stores) => handle mode: table operands print
-- their 1-based arena handle and materialized length (table#H(len=N)). The
-- per-iteration print shows t's len growth and the fresh row handles the
-- arena mints each trip; the induction variable's coalesced phi slot is
-- pinned by EXPECT_PROBE (probe_map names it i_r17).
-- Observables pinned below: table#1(len=3) on the FIRST trip is tier4's
-- pre-header EnsureCapacity sizing t to the limit before iteration 1;
-- table#2..#4 walking is the arena's monotone growth.
-- EXPECT_PROBE pins embed physical register names BY DESIGN (see probe_01).
-- EXPECT_PRINT: iter	0	table#1(len=3)	table#2(len=1)
-- EXPECT_PRINT: iter	1	table#1(len=3)	table#3(len=1)
-- EXPECT_PRINT: iter	2	table#1(len=3)	table#4(len=1)
-- EXPECT_PRINT: exit	3	table#1(len=3)
-- EXPECT_PROBE: #0 tag="iter" b2 depth1 i_r15 t_r15 len_r15 t_r16 len_r16
-- EXPECT_PROBE: #1 tag="exit" b3 depth0 i_r15 t_r15 len_r15
-- EXPECT: NTABLES 4
-- EXPECT: TABLE 0 LEN 3 NZ 3 CHECKSUM 20
local t = {}
local i = 0
while i < 3 do
    local row = {}
    row[0] = i
    t[i] = row
    print("iter", i, t, row)
    i = i + 1
end
print("exit", i, t)
