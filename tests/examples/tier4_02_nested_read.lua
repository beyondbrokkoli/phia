-- tier4_02_nested_read.lua  [POSITIVE — nested READ fast path]
-- t[0][i] as a VALUE: the inner GetTable's table operand is the feeder,
-- tier-4 rewrites it to GetTableFast against the materialized child.
-- The minted pre-header resolution is the one remaining dyn_get.
-- EXPECT: TABLE 0 LEN 1 NZ 1 CHECKSUM 2
-- EXPECT: TABLE 1 LEN 8 NZ 8 CHECKSUM 204
-- EXPECT: NTABLES 2
-- EXPECT: fast_sets=1
-- EXPECT: fast_gets=1
-- EXPECT: dyn_sets=1
-- EXPECT: dyn_gets=1
-- EXPECT: hoists=2
-- EXPECT: hoist_ctx=0,0
local t = {}
local inner = {}
local j = 0
while j < 8 do
    inner[j] = j + 1
    j = j + 1
end
t[0] = inner
local s = 0
local i = 0
while i < 8 do
    s = s + t[0][i]
    i = i + 1
end
