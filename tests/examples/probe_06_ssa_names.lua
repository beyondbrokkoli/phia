-- probe_06_ssa_names.lua [POSITIVE — SSA versioning as observable
-- register NAMES]. The same variable x probed at three program points
-- prints three DIFFERENT physical names (before: the initial def; join:
-- the if-join phi's slot; after: the loop-phi's coalesced slot) with the
-- values 1 -> 2 -> 202 flowing through them. i's loop coalesced slot
-- reads 2 after the loop. The name changes ARE the pins: an allocator
-- or phi-coalescing change breaks this test like a lock.
-- Pins embed physical register names BY DESIGN (see probe_01).
-- EXPECT: PROBE before: i_r0=1 b_r1=true
-- EXPECT: PROBE join: i_r30=2 b_r1=true
-- EXPECT: PROBE after: i_r31=202 i_r30=2
-- EXPECT: NTABLES 1
local x = 1
local flag = true
probe("before", x, flag)
if flag then
    x = x + 1
else
    x = x + 10
end
probe("join", x, flag)
local i = 0
while i < 2 do
    x = x + 100
    i = i + 1
end
probe("after", x, i)
local w = {}
w[0] = x
