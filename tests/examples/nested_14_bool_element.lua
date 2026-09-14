-- nested_14_bool_element.lua  [POSITIVE — element domain flip]
-- Historical sentinel, flipped: this file once pinned the BUILD ERROR
-- "table elements cannot be Boolean" (the checker rejected Boolean
-- element bindings outright). Boolean is now the fourth codegenable
-- element kind — Table::new_bool(), barray storage (bit-packed
-- Vec<bool>), b_r registers — and this exact program is its smallest
-- proof: the store `t[0] = 1 < 2` infers a bool table, the read prints
-- true, and the dump shows the barray side.
-- EXPECT: TABLE 0 LEN 1 NZ 1 CHECKSUM 1
-- EXPECT: NTABLES 1
-- EXPECT: fast_sets=0
-- EXPECT: fast_gets=0
-- EXPECT: dyn_sets=1
-- EXPECT: dyn_gets=1
-- EXPECT: hoists=0
-- EXPECT_PRINT: bool element	true
local t = {}
t[0] = 1 < 2
local x = t[0]
print("bool element", x)
