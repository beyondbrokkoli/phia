-- feat_if_01_phi_merge.lua [POSITIVE — if/elseif/else with SSA join phis].
-- The then-arm's renames must not leak into the else-arm (scope snapshot),
-- and every var mutated in EITHER arm merges at the join phi. elseif chains
-- desugar to nested Ifs in the else arm. The if sits inside a while so the
-- join phi also feeds a loop-carried value.
-- EXPECT: TABLE 0 LEN 1 NZ 1 CHECKSUM 204
-- EXPECT: NTABLES 1
local s = 0
local i = 0
while i <= 10 do
    if i % 2 == 0 then
        s = s + i * 10
    elseif i == 5 then
        s = s - 100
    else
        s = s + 1
    end
    i = i + 1
end
local w = {}
w[0] = s
