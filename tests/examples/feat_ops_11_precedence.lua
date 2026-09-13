-- feat_ops_11_precedence.lua [POSITIVE — precedence and associativity
-- combinatorics over the full operator set, on runtime values].
-- * / // % bind tighter than + -; unary minus/not bind tighter than *;
-- all binary levels left-associate; parentheses override. i is
-- loop-derived (= 1 at runtime) so nothing folds.
-- EXPECT: TABLE 0 LEN 3 NZ 3 CHECKSUM 464121
-- EXPECT: NTABLES 1
local i = 0
while i < 1 do i = i + 1 end
local r2 = i + 1 * 3
local r3 = (i + 1) * 3
local r4 = 10 - i * 2 - 3
local r5 = 20 // 3 * 2
local r6 = 1 + 7 % 4
local r7 = -i * 5
local r8 = -(i + 5)
local b1 = not (i > 5)
local b4 = not not b1
local w = {}
w[0] = r2 * 100000 + r3 * 10000 + r4 * 1000 + r5 * 10 + r6
w[1] = r7 * 100 + r8
local n = 0
if b1 then n = n + 1 end
if b4 then n = n + 2 end
w[2] = n
