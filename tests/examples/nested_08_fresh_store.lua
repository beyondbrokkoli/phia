-- nested_08_fresh_store.lua  [POSITIVE — was NEGATIVE (_restricted): wart 2 killed]
-- `a[0] = {}` used to be a build error: the checker demanded the stored
-- table's element type be resolved AT the store. Lazy unification ties the
-- fresh table's element variable to a's; both default to Integer here. The
-- third allocated table is the fresh one — a[0] holds its handle (3).
-- EXPECT: TABLE 0 LEN 1 NZ 1 CHECKSUM 3
-- EXPECT: TABLE 1 LEN 1 NZ 1 CHECKSUM 1
-- EXPECT: TABLE 2 LEN 0 NZ 0 CHECKSUM 0
-- EXPECT: NTABLES 3
-- EXPECT: fast_sets=0
-- EXPECT: fast_gets=0
-- EXPECT: dyn_sets=2
-- EXPECT: dyn_gets=0
-- EXPECT: hoists=0
local a = {}
local b = {}
b[0] = 1
a[0] = {}
