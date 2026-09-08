-- nested_10_alias_shared.lua  [POSITIVE — pins the resolution-by-identity fix]
-- EXPECT: TABLE 0 LEN 1 NZ 1 CHECKSUM 7
-- EXPECT: NTABLES 1
-- EXPECT: fast_sets=0
-- EXPECT: fast_gets=0
-- EXPECT: dyn_sets=1
-- EXPECT: dyn_gets=1
-- EXPECT: hoists=0
local a = {}
local b = a
b[0] = 7
local x = a[0]
