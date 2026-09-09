-- feat_ops_06_bool_eq_variants.lua [POSITIVE — == / ~= on Booleans in
-- every constness combination]. Int and Bool physicals share one register
-- id range, so Eq carries its operand type; before that fix a const-bool
-- on the LEFT rendered undeclared i_r registers (build failure) and an
-- aliased physical bool compared the wrong variable. All four shapes
-- (const/physical x const/physical), both operators, inside loop-carried
-- phis. p and q both end true: p=true -> not -> false -> not -> true; q=p.
-- EXPECT: TABLE 0 LEN 1 NZ 1 CHECKSUM 1101101
-- EXPECT: NTABLES 1
local p = true
local q = false
local i = 0
while i < 2 do
    p = not p
    q = p
    i = i + 1
end
local n = 0
if true == p then n = n + 1 end
if false == q then n = n + 10 end
if p == true then n = n + 100 end
if p == q then n = n + 1000 end
if p ~= q then n = n + 10000 end
if p ~= false then n = n + 100000 end
if false ~= q then n = n + 1000000 end
if not p == q then n = n + 10000000 end
local w = {}
w[0] = n
