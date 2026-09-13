-- nested_06_read_before_write.lua  [POSITIVE — the unused-read survivor]
-- x reads a[0] (the null handle — a is empty) and is never used again,
-- yet the read MUST survive: simplify() only deletes dead PURE defs and
-- GetTable is not pure (a negative key can panic), so its dyn_get stays
-- and the pin below counts it. This is the sentinel that forbids a
-- global "const-key reads are pure" DCE rule — tier-4's orphan-feeder
-- deletion is pass-local and singly-used-only for exactly this reason.
-- Under lazy unification the read also constrains nothing (nested_07).
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
