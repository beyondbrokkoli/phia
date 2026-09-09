-- feat_ops_02_compare_bool.lua [POSITIVE — >, >=, <=, ==, ~=, not, literals].
-- Comparisons desugar onto the Less machinery (a > b => b < a, a <= b =>
-- a < b+1, a >= b => b < a+1) so the tier4 loop gate keeps recognizing the
-- shape; == / ~= lower to Eq (+Not). true/false are Boolean locals; 'not'
-- is Boolean negation. Booleans stay out of tables (checker rejects them).
-- EXPECT: TABLE 0 LEN 2 NZ 2 CHECKSUM 30
-- EXPECT: NTABLES 1
local b = not (3 >= 4)
local c = true
local i = 0
local acc = 0
while i > 0 - 1 * 4 do
    if b then
        acc = acc + 3
    end
    if c == true then
        acc = acc + 2
    end
    if not b then
        acc = acc + 1000
    end
    i = i - 1
end
local n = 0
if not not c then
    n = 5
end
local w = {}
w[0] = acc
w[1] = n
