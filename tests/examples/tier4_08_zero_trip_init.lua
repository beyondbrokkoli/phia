-- tier4_08_zero_trip_init.lua  [POSITIVE — the >=1-trip contract, entry side]
-- lim is a positive literal (8) but the induction var ENTERS above it
-- (i = 100): a zero-trip loop with a nil child (t is empty). Dyn runs
-- silent — no iteration, no resolution, no panic — so tier-4 must
-- decline: proving lim > 0 is only half the contract; the phi's ENTRY
-- value must also fold below the limit before the pre-header EC/Hoist
-- nil-panic is faithful. Found as a live bug by the extension probes:
-- before the entry gate, this exact program panicked "table is nil" in
-- the pre-header of a loop that never ran. (tier4_04/05 pin the lim
-- side: literal 0 declines silently, positive literal with a nil child
-- panics faithfully; tier4_12 pins the unprovable-limit decline.)
-- EXPECT: TABLE 0 LEN 0 NZ 0 CHECKSUM 0
-- EXPECT: NTABLES 1
-- EXPECT: fast_sets=0
-- EXPECT: fast_gets=0
-- EXPECT: dyn_sets=1
-- EXPECT: dyn_gets=1
-- EXPECT: hoists=0
local t = {}
local i = 100
while i < 8 do
    t[0][i] = 1
    i = i + 1
end
