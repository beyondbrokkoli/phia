-- probe_04_leq_desugar_ec.lua [POSITIVE — the <= desugar's literal +1,
-- observable as the EC]. `while i <= 7` materializes 7+1 = 8 in the
-- pre-header (the tier4 gate shape), the EC sizes t to that 8, and the
-- probe pins len=8 from trip 0: the exact arithmetic of the desugar,
-- frozen as a runtime invariant instead of a comment. Values i*i,
-- CHECKSUM 924 (feat_ops_03's corpus, now with the length observable).
-- EXPECT_PROBE pins embed physical register names BY DESIGN (see probe_01).
-- EXPECT_PRINT: iter	0	table(len=8)
-- EXPECT_PRINT: iter	1	table(len=8)
-- EXPECT_PRINT: iter	2	table(len=8)
-- EXPECT_PRINT: iter	3	table(len=8)
-- EXPECT_PRINT: iter	4	table(len=8)
-- EXPECT_PRINT: iter	5	table(len=8)
-- EXPECT_PRINT: iter	6	table(len=8)
-- EXPECT_PRINT: iter	7	table(len=8)
-- EXPECT_PROBE: #0 tag="iter" b2 depth1 i_r17 len_r17
-- EXPECT: TABLE 0 LEN 8 NZ 7 CHECKSUM 924
-- EXPECT: NTABLES 1
-- EXPECT: fast_sets=1
-- EXPECT: hoists=1
local t = {}
local i = 0
while i <= 7 do
    t[i] = i * i
    print("iter", i, t)
    i = i + 1
end
