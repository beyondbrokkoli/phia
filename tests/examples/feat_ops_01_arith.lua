-- feat_ops_01_arith.lua [POSITIVE — *, /, //, %, unary minus, parens].
-- Mul/Div/IntDiv/Mod are pool-typed like Add/Sub (int and float sides);
-- unary minus lowers to a typed 0 - x. Division/modulo by zero and
-- i64::MIN // -1 stay runtime panics — the const folder uses checked_
-- division and refuses to fold them.
-- EXPECT: TABLE 0 LEN 2 NZ 2 CHECKSUM 35
-- EXPECT: TABLE 1 LEN 1 NZ 1 CHECKSUM 4652576855864377344 SUM 1105.5
-- EXPECT: NTABLES 2
local f = 2.5
local g = f * 2.0 - -0.5
local h = g / 5.0
local m = 17 % 5
local d = 17 // 5
local w = {}
w[0] = m + d
w[1] = (4 - 1) * (2 + 3)
local wf = {}
wf[0] = h * 1000.0 + g
