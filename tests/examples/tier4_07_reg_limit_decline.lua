-- tier4_07_reg_limit_decline.lua  [POSITIVE — was the literal-limit
-- decline pin; the proven-positive computed-limit extension flipped it]
-- The limit is n = 5 + 3: invariant, literal-fed, and its def is an Add —
-- once tier-4 folded limit def chains locally (propagate_constants runs
-- AFTER optimize), 8 > 0 is provable and the nested store converts: the
-- mint materializes the child once, EC sizes to the (post-fold) limit 8,
-- the store rides the hoisted pointer. The DECLINE arm did not vanish —
-- it moved to limits no folder can prove (a table-fed n: tier4_12), and
-- the >=1-trip side gained its own pin (init above limit: tier4_08).
-- Flat tier-2 never had the restriction — that divergence is now closed.
-- EXPECT: TABLE 0 LEN 1 NZ 1 CHECKSUM 2
-- EXPECT: TABLE 1 LEN 8 NZ 8 CHECKSUM 36
-- EXPECT: NTABLES 2
-- EXPECT: fast_sets=1
-- EXPECT: fast_gets=0
-- EXPECT: dyn_sets=2
-- EXPECT: dyn_gets=1
-- EXPECT: hoists=1
-- EXPECT: hoist_ctx=0
local t = {}
local inner = {}
inner[0] = 0
t[0] = inner
local n = 5 + 3
local i = 0
while i < n do
    t[0][i] = 1
    i = i + 1
end
