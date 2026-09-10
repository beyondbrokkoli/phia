-- string_01_scalar.lua  [POSITIVE — string scalars: literals, concat, eq]
-- Strict string typing: `..` and == / ~= between two Strings only. Lua
-- coerces numbers on concat (2 .. "x"); that is a build error here —
-- pinned divergence, same family as float_03's mixed arithmetic.
-- Chains fold LEFT (associative, unobservable — see parser note).
-- Pins embed physical register names BY DESIGN (see probe_01).
-- EXPECT: PROBE concat: s_r30="pha" s_r31="ia" s_r32="phaia"
-- EXPECT: PROBE chain: s_r32="pha-ia"
-- EXPECT: PROBE eq: b_r30=true b_r32=true
-- EXPECT: PROBE prec: b_r32=true b_r30=true
-- EXPECT: NTABLES 0
local a = "pha"
local b = "ia"
local c = a .. b
probe("concat", a, b, c)
local joined = a .. "-" .. b
probe("chain", joined)
local same = a == "pha"
local diff = a ~= b
probe("eq", same, diff)
-- .. binds tighter than == (Lua precedence)
local gate = (a .. "x") == "phax"
local neg = not (a == b)
probe("prec", gate, neg)
