-- probe_02_loop_phi.lua  [POSITIVE — in-loop probe: phi trajectory + arena growth]
-- Nested program (table-valued stores) => handle mode: table operands print
-- their 1-based arena handle and materialized length. The per-iteration
-- probe shows the induction variable's coalesced phi slot, t's len growth,
-- and the fresh row handles the arena mints each trip.
-- Observables pinned below: len_r17=3 on the FIRST trip is tier4's
-- pre-header EnsureCapacity sizing t to the limit before iteration 1;
-- t_r18 walking 2,3,4 is the arena's monotone growth.
-- Pins embed physical register names BY DESIGN (see probe_01).
-- EXPECT: PROBE iter: i_r17=0 t_r17=1 len_r17=3 t_r18=2 len_r18=1
-- EXPECT: PROBE iter: i_r17=1 t_r17=1 len_r17=3 t_r18=3 len_r18=1
-- EXPECT: PROBE iter: i_r17=2 t_r17=1 len_r17=3 t_r18=4 len_r18=1
-- EXPECT: PROBE exit: i_r17=3 t_r17=1 len_r17=3
-- EXPECT: NTABLES 4
-- EXPECT: TABLE 0 LEN 3 NZ 3 CHECKSUM 20
local t = {}
local i = 0
while i < 3 do
    local row = {}
    row[0] = i
    t[i] = row
    probe("iter", i, t, row)
    i = i + 1
end
probe("exit", i, t)
