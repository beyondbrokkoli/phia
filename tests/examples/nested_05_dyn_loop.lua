-- nested_05_dyn_loop.lua  [POSITIVE — seeded so it builds; pins the decline shape]
-- EXPECT: TABLE 0 LEN 1 NZ 1 CHECKSUM 2
-- EXPECT: TABLE 1 LEN 10 NZ 10 CHECKSUM 55
-- EXPECT: NTABLES 2
-- EXPECT: fast_sets=0
-- EXPECT: fast_gets=0
-- EXPECT: dyn_sets=3
-- EXPECT: dyn_gets=1
-- EXPECT: hoists=0
local t = {}
local inner = {}
inner[0] = 0
t[0] = inner
local i = 0
while i < 10 do
    t[0][i] = 1
    i = i + 1
end
