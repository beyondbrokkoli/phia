-- probe_07_mint_child.lua [POSITIVE — the tier4 mint, observable as the
-- child's EC'd length]. The single-hop child store `t[0][i] = i * 7`
-- mints a GetTable of t[0] in the pre-header, EnsureCapacity's the CHILD
-- to the loop limit 3, and hoists it — all before trip 1. The probe's own
-- dyn read t[0] (non-affine key, no interference) shows the child handle
-- with len=3 from the FIRST line, frozen across trips. Outer t: handle 1
-- len 1; seed child: handle 2, values {0, 7, 14} CHECKSUM 56.
-- Pins embed physical register names BY DESIGN (see probe_01).
-- EXPECT: PROBE iter: i_r26=0 t_r26=1 len_r26=1 t_r28=2 len_r28=3
-- EXPECT: PROBE iter: i_r26=1 t_r26=1 len_r26=1 t_r28=2 len_r28=3
-- EXPECT: PROBE iter: i_r26=2 t_r26=1 len_r26=1 t_r28=2 len_r28=3
-- EXPECT: PROBE exit: t_r26=1 len_r26=1
-- EXPECT: TABLE 0 LEN 1 NZ 1 CHECKSUM 2
-- EXPECT: TABLE 1 LEN 3 NZ 2 CHECKSUM 56
-- EXPECT: NTABLES 2
-- EXPECT: fast_sets=1
-- EXPECT: hoists=1
local t = {}
local seed = {}
t[0] = seed
local i = 0
while i < 3 do
    t[0][i] = i * 7
    probe("iter", i, t, t[0])
    i = i + 1
end
probe("exit", t)
