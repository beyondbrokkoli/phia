-- nested_05_dyn_loop.lua  [POSITIVE — was the decline pin; TIER-4 flipped it]
-- The nested write t[0][i] = 1 now traces through the const-key feeder:
-- the child is materialized + EC + hoisted in the pre-header, the store
-- is SetTableFast, the in-loop feeder dies. dyn_gets=1 is the minted
-- resolution. TABLE pins unchanged — only the STATS shape moved.
-- EXPECT: TABLE 0 LEN 1 NZ 1 CHECKSUM 2
-- EXPECT: TABLE 1 LEN 10 NZ 10 CHECKSUM 55
-- EXPECT: NTABLES 2
-- EXPECT: fast_sets=1
-- EXPECT: fast_gets=0
-- EXPECT: dyn_sets=2
-- EXPECT: dyn_gets=1
-- EXPECT: hoists=1
local t = {}
local inner = {}
inner[0] = 0
t[0] = inner
local i = 0
while i < 10 do
    t[0][i] = 1
    i = i + 1
end
