-- tier4_04_zero_iter.lua  [POSITIVE — the lim > 0 contract, silent side]
-- Literal limit 0: tier-4 declines (materializing the child would put a
-- nil-panic in the pre-header of a loop that never runs). Zero trips,
-- no panic, dyn instructions counted but never executed.
-- EXPECT: TABLE 0 LEN 0 NZ 0 CHECKSUM 0
-- EXPECT: NTABLES 1
-- EXPECT: fast_sets=0
-- EXPECT: fast_gets=0
-- EXPECT: dyn_sets=1
-- EXPECT: dyn_gets=1
-- EXPECT: hoists=0
local t = {}
local i = 0
while i < 0 do
    t[0][i] = 1
    i = i + 1
end
