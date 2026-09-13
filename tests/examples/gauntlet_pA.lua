-- gauntlet_pA.lua — gate ✓ (pa_n LoadInt @ b0); pa_t[φi] safe key, table @ b0 → upgrade. HR @ b0.
-- EXPECT: TABLE 0 LEN 400 NZ 200 CHECKSUM 10666600
-- EXPECT: fast_sets=1
-- EXPECT: fast_gets=0
-- EXPECT: dyn_sets=0
-- EXPECT: dyn_gets=0
-- EXPECT: hoists=1
local pa_t = {}
local pa_n = 400
local pa_i = 0
while pa_i < pa_n do
    pa_t[pa_i] = pa_i + 1
    pa_i = pa_i + 2
end
