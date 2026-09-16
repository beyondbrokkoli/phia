-- logic_11_while_cond_not_btab.lua  [POSITIVE — `not` over a while-cond
-- chain (the Not lowers into the chain's join, branching on the negated
-- phi); a deferred bool-table read in a condition (`bt[0] and w < 2` —
-- the absent-key zero is false, this key is present); and a plain
-- single-condition while AFTER the chain loops, proving the direct-gate
-- path still emits its usual while form]
-- EXPECT: TABLE 0 LEN 1 NZ 1 CHECKSUM 1
-- EXPECT: NTABLES 1
-- EXPECT_PRINT: notchain	5
-- EXPECT_PRINT: btab	2
-- EXPECT_PRINT: plain	10
local p = true
local q = false
local m = 0
while not (p and q) do
    m = m + 1
    if m > 4 then
        q = true
    end
end
print("notchain", m)
local bt = {}
bt[0] = true
local w = 0
while bt[0] and w < 2 do
    w = w + 1
end
print("btab", w)
local z = 0
while z < 10 do
    z = z + 1
end
print("plain", z)
