-- bool_05_deferred_eq.lua  [POSITIVE — deferred == with a Boolean]
-- Historical flip: this shape was feat_ops_neg_deferred_bool_eq, a
-- NEGATIVE pin ("a deferred read cannot meet a Boolean in =="). The
-- op-guarded Var/Boolean arms now make == / ~= a legal Boolean-valued
-- USE that binds the element variable — t becomes a bool table through
-- the comparison alone. t[0] is absent, so false == true is false and
-- the store never runs; the table stays empty (LEN 0, the pool's zero).
-- EXPECT: TABLE 0 LEN 0 NZ 0 CHECKSUM 0
-- EXPECT: NTABLES 1
-- EXPECT: fast_sets=0
-- EXPECT: fast_gets=0
-- EXPECT: dyn_sets=1
-- EXPECT: dyn_gets=2
-- EXPECT: hoists=0
-- EXPECT_PRINT: eq	false	false
local t = {}
local hit = false
if t[0] == true then
    t[0] = false
    hit = true
end
print("eq", t[0], hit)
