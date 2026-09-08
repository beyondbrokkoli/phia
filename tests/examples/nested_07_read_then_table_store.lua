-- nested_07_read_then_table_store.lua  [POSITIVE — was NEGATIVE (_fail): wart 1 killed]
-- The pure form of the read-before-write wart: `local x = a[0]` used to
-- lock a's elements to Integer forever, making the later table store a
-- build error even though x is never used as an Integer. Under lazy
-- unification the read constrains nothing; a's elements unify to tables.
-- x reads the null handle (a is empty) and is simply never used.
-- EXPECT: TABLE 0 LEN 2 NZ 1 CHECKSUM 4
-- EXPECT: TABLE 1 LEN 1 NZ 0 CHECKSUM 0
-- EXPECT: NTABLES 2
-- EXPECT: fast_sets=0
-- EXPECT: fast_gets=0
-- EXPECT: dyn_sets=2
-- EXPECT: dyn_gets=1
-- EXPECT: hoists=0
local a = {}
local x = a[0]
local b = {}
b[0] = 0
a[1] = b
