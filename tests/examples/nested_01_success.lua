-- nested_01_success.lua  [POSITIVE]
-- EXPECT: TABLE 0 LEN 1 NZ 1 CHECKSUM 2
-- EXPECT: TABLE 1 LEN 1 NZ 1 CHECKSUM 2
-- EXPECT: NTABLES 2
-- EXPECT: fast_sets=0
-- EXPECT: fast_gets=0
-- EXPECT: dyn_sets=2
-- EXPECT: dyn_gets=0
-- EXPECT: hoists=0
local a = {}
local b = {}
b[0] = 2
a[0] = b
