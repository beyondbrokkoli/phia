-- feat_ops_03_fast_leq.lua [POSITIVE — `while i <= lim` still opens the
-- fast-path gate]. a <= b desugars to a < b+1; the +1 Add is loop-invariant
-- in the pre-header, so the tier4 gate shape (idx < invariant limit) is
-- preserved and the region upgrades exactly like the plain < form.
-- EXPECT: TABLE 0 LEN 8 NZ 7 CHECKSUM 924
-- EXPECT: fast_sets=1
-- EXPECT: fast_gets=0
-- EXPECT: hoists=1
-- EXPECT: hoist_ctx=0
local t = {}
local i = 0
while i <= 7 do
    t[i] = i * i
    i = i + 1
end
