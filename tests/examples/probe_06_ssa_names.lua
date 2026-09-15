-- probe_06_ssa_names.lua [POSITIVE — SSA versioning as observable
-- register NAMES]. The same variable x printed at three program points
-- carries three DIFFERENT physical names in probe_map.txt (before: the
-- initial def; join: the if-join phi's slot; after: the loop-phi's
-- coalesced slot) with the values 1 -> 2 -> 202 flowing through them
-- (EXPECT_PRINT pins the values, EXPECT_PROBE the names). i's loop
-- coalesced slot reads 2 after the loop. The name changes ARE the pins:
-- an allocator or phi-coalescing change breaks this test like a lock.
-- EXPECT_PRINT: before	1	true
-- EXPECT_PRINT: join	2	true
-- EXPECT_PRINT: after	202	2
-- EXPECT_PROBE: #0 tag="before" b0 depth0 c0 c1
-- EXPECT_PROBE: #1 tag="join" b3 depth0 i_r29 c1
-- EXPECT_PROBE: #2 tag="after" b6 depth0 i_r30 i_r29
-- EXPECT: NTABLES 1
local x = 1
local flag = true
print("before", x, flag)
if flag then
    x = x + 1
else
    x = x + 10
end
print("join", x, flag)
local i = 0
while i < 2 do
    x = x + 100
    i = i + 1
end
print("after", x, i)
local w = {}
w[0] = x
