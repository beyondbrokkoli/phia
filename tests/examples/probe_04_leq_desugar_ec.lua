-- probe_04_leq_desugar_ec.lua [POSITIVE — the <= desugar's literal +1,
-- observable as the EC]. `while i <= 7` materializes 7+1 = 8 in the
-- pre-header (the tier4 gate shape), the EC sizes t to that 8, and the
-- probe pins len=8 from trip 0: the exact arithmetic of the desugar,
-- frozen as a runtime invariant instead of a comment. Values i*i,
-- CHECKSUM 924 (feat_ops_03's corpus, now with the length observable).
-- Pins embed physical register names BY DESIGN (see probe_01).
-- EXPECT: PROBE iter: i_r17=0 len_r17=8
-- EXPECT: PROBE iter: i_r17=1 len_r17=8
-- EXPECT: PROBE iter: i_r17=2 len_r17=8
-- EXPECT: PROBE iter: i_r17=3 len_r17=8
-- EXPECT: PROBE iter: i_r17=4 len_r17=8
-- EXPECT: PROBE iter: i_r17=5 len_r17=8
-- EXPECT: PROBE iter: i_r17=6 len_r17=8
-- EXPECT: PROBE iter: i_r17=7 len_r17=8
-- EXPECT: TABLE 0 LEN 8 NZ 7 CHECKSUM 924
-- EXPECT: NTABLES 1
-- EXPECT: fast_sets=1
-- EXPECT: hoists=1
local t = {}
local i = 0
while i <= 7 do
    t[i] = i * i
    probe "iter" i, t
    i = i + 1
end
