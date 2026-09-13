-- feat_ops_07_neg_zero.lua [POSITIVE — unary minus is negation, not 0 - x].
-- 0.0 - x preserves +0.0's sign bit; true negation produces -0.0
-- (to_bits = 9223372036854775808) — checksum-visible. Double negation
-- restores +0.0. The runtime zero is loop-derived; float literals never
-- const-fold anyway. Integer negation mirrors Lua: wraps only on MIN.
-- NZ counts skip ±0.0 (IEEE: -0.0 != 0.0 is false).
-- EXPECT: TABLE 0 LEN 2 NZ 2 CHECKSUM -5
-- EXPECT: TABLE 1 LEN 5 NZ 2 CHECKSUM 4619567317775286272 SUM 0
-- EXPECT: NTABLES 2
local i = 0
local f = 1.0
while i < 1 do
    f = f - 1.0
    i = i + 1
end
local m = 0 - 5 * i
local w = {}
w[0] = -m
w[1] = -(-m)
local g = 0.0
local wf = {}
wf[0] = -f
wf[1] = -(-f)
wf[2] = -2.5
wf[3] = -(-2.5)
wf[4] = -g
