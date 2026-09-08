-- nested_10_alias_shared.lua  [POSITIVE — pins the resolution-by-identity fix]
-- b = a copies a's type TERM, so b's store constrains a's element
-- variable and a's read sees the written value (7): one table, one
-- element variable, resolution by variable identity. The conflict arm
-- of the same rule is nested_04 — an alias cannot diverge, it can only
-- surface a genuine conflict.
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
