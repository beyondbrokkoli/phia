-- string_01_scalar.lua  [POSITIVE — string scalars: literals, concat, eq]
-- Strict string typing: `..` and == / ~= between two Strings only. Lua
-- coerces numbers on concat (2 .. "x"); that is a build error here —
-- pinned divergence, same family as float_03's mixed arithmetic.
-- Chains fold LEFT (associative, unobservable — see parser note).
-- EXPECT_PROBE pins embed physical register names BY DESIGN (see probe_01).
-- EXPECT_PRINT: concat	pha	ia	phaia
-- EXPECT_PRINT: chain	pha-ia
-- EXPECT_PRINT: eq	true	true
-- EXPECT_PRINT: prec	true	true
-- EXPECT_PROBE: #0 tag="concat" b0 depth0 c0 c1 c2
-- EXPECT_PROBE: #1 tag="chain" b0 depth0 c3
-- EXPECT_PROBE: #2 tag="eq" b0 depth0 c6 c8
-- EXPECT_PROBE: #3 tag="prec" b0 depth0 c10 c14
-- EXPECT: NTABLES 0
local a = "pha"
local b = "ia"
local c = a .. b
print("concat", a, b, c)
local joined = a .. "-" .. b
print("chain", joined)
local same = a == "pha"
local diff = a ~= b
print("eq", same, diff)
-- .. binds tighter than == (Lua precedence)
local gate = (a .. "x") == "phax"
local neg = not (a == b)
print("prec", gate, neg)
