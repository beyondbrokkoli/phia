-- float_06_lazy_read.lua  [POSITIVE — lazy unification across kinds]
-- The read of an unconstrained table fixes nothing; the later float store
-- binds the element to Float. Under the old eager checker the read would
-- have locked Integer and the store would be a build error — this is
-- nested_07's story retold for floats, and it runs clean: x reads the null
-- handle's 0.0 and is never used.
-- EXPECT: TABLE 0 LEN 1 NZ 1 CHECKSUM 4609434218613702656 SUM 1.5
-- EXPECT: NTABLES 1
-- EXPECT: fast_sets=0
-- EXPECT: fast_gets=0
-- EXPECT: dyn_sets=1
-- EXPECT: dyn_gets=1
-- EXPECT: hoists=0
local t = {}
local x = t[0]
t[0] = 1.5
