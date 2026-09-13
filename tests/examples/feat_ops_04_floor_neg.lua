-- feat_ops_04_floor_neg.lua [POSITIVE — Lua floor semantics for // and %,
-- plus the PINNED trunc divergence of integer /].
-- `//` floors toward -inf and `%` takes the divisor's sign on BOTH sides
-- (-7 // 2 == -4, -7 % 3 == 2, 7 // -2 == -4, 7 % -3 == -2; floats:
-- -7.5 // 2.0 == -4.0, -7.5 % 2.0 == 0.5). Integer `/` is the deliberate,
-- pinned divergence: Lua's `/` always yields a float and strict typing
-- forbids silent coercion, so int `/` is TRUNCATING division
-- (7 / -2 == -3, -7 / 2 == -3) and float `/` stays plain division.
-- All operands are loop-derived so every template executes at runtime
-- (const folding would bypass the emitted code entirely).
-- EXPECT: TABLE 0 LEN 6 NZ 6 CHECKSUM -53
-- EXPECT: TABLE 1 LEN 6 NZ 6 CHECKSUM 4581849670896058368 SUM -5
-- EXPECT: NTABLES 2
local i = 0
while i < 1 do i = i + 1 end
local a = 0 - 7 * i
local b = 2 * i
local w = {}
w[0] = a // b
w[1] = a % (3 * i)
w[2] = (7 * i) // (0 - 2 * i)
w[3] = (7 * i) % (0 - 3 * i)
w[4] = (7 * i) / (0 - 2 * i)
w[5] = a / b
local wf = {}
wf[0] = -7.5
wf[1] = 2.0
wf[2] = wf[0] // wf[1]
wf[3] = wf[0] % wf[1]
wf[4] = 7.0 // 2.0
wf[5] = 7.0 % 3.0
