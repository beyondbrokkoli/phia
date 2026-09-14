-- bool_01_store_read.lua  [POSITIVE — the bool core on the dyn path]
-- A bool table on the dynamic (non-hoisted) path: Table::new_bool(),
-- barray storage, b_r registers. Stored values are literals and
-- comparison results; the absent key yields the pool's zero (false),
-- exactly like 0, 0.0 and "". NZ counts true elements; CHECKSUM hashes
-- true as 1, position-weighted like the integer side.
-- EXPECT: TABLE 0 LEN 3 NZ 2 CHECKSUM 3
-- EXPECT: NTABLES 1
-- EXPECT: fast_sets=0
-- EXPECT: fast_gets=0
-- EXPECT: dyn_sets=3
-- EXPECT: dyn_gets=3
-- EXPECT: hoists=0
-- EXPECT_PRINT: flags	true	true	false
local t = {}
t[0] = true
t[1] = 10 < 20
t[2] = false
local a = t[0]
local b = t[1]
local missing = t[999]
print("flags", a, b, missing)
