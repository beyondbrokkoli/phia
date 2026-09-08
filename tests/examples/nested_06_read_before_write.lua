-- nested_06_read_before_write.lua  [POSITIVE]
-- EXPECT: TABLE 0 LEN 1 NZ 1 CHECKSUM 5
-- EXPECT: NTABLES 1
-- EXPECT: fast_sets=0
-- EXPECT: fast_gets=0
-- EXPECT: dyn_sets=1
-- EXPECT: dyn_gets=1
-- EXPECT: hoists=0
local a = {}
local x = a[0]
a[0] = 5
