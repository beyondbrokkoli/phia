-- bug_08.lua — TIER 2 FLIP: both writes promote. a[φi] rides Some(0); a[k] with
-- k = φi + 300 rides Some(300); ONE EC(310) covers both key ranges ([0,9] and
-- [300,309]) with one capacity payment.
-- EXPECT: TABLE 0 LEN 310 NZ 20 CHECKSUM 6165
-- EXPECT: fast_sets=2
-- EXPECT: fast_gets=0
-- EXPECT: dyn_sets=0
-- EXPECT: dyn_gets=0
-- EXPECT: hoists=1
local a = {}
local i = 0
while i < 10 do
    a[i] = 1
    local k = i + 300
    a[k] = 2
    i = i + 1
end
