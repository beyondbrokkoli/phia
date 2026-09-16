-- logic_07_if_cond_and_or.lua  [POSITIVE — and/or in the positions that
-- ARE supported: an if condition (`flag and 1 < 2`), and inside a while
-- BODY (`if i % 2 == 0 and flag then`), where the desugar is a plain
-- structured if in the loop region — the loop condition itself stays
-- and/or-free (that is the milestone restriction)]
-- EXPECT: TABLE 0 LEN 5 NZ 3 CHECKSUM 7
-- EXPECT: NTABLES 1
-- EXPECT: fast_sets=1
-- EXPECT: dyn_sets=2
-- EXPECT: dyn_gets=5
-- EXPECT: hoists=1
-- EXPECT_PRINT: done	1	1	0	1	0
local flag = true
local t = {}
if flag and 1 < 2 then
    t[0] = 1
else
    t[0] = 2
end
local i = 0
while i < 4 do
    if i % 2 == 0 and flag then
        t[i + 1] = 1
    end
    i = i + 1
end
print("done", t[0], t[1], t[2], t[3], t[4])
