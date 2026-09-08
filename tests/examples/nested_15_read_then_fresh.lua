-- nested_15_read_then_fresh.lua  [POSITIVE — both warts killed in one program]
-- A read that constrains nothing, then a fresh-table store into the same
-- table: under the old eager rules the read locked a to Integer and the
-- store was a build error. Lazy unification gives a Table(Table(Integer))
-- and the program runs clean — x keeps the null handle and is never used.
-- Pins that the read contributed NO integer constraint at all.
-- EXPECT: TABLE 0 LEN 1 NZ 1 CHECKSUM 2
-- EXPECT: TABLE 1 LEN 0 NZ 0 CHECKSUM 0
-- EXPECT: NTABLES 2
-- EXPECT: fast_sets=0
-- EXPECT: fast_gets=0
-- EXPECT: dyn_sets=1
-- EXPECT: dyn_gets=1
-- EXPECT: hoists=0
local a = {}
local x = a[0]
a[0] = {}
