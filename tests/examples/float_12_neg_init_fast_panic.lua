-- float_12_neg_init_fast_panic.lua  [PANIC — the float twin of firewall_neg_init_panic]
-- Negative-init induction variable: tier-2 upgrades the float store (key
-- is affine in the phi with offset 0), the first fast-path trip hits
-- k = -3 < 0, and the FAST arm fires — not the dyn "Negative table
-- index" the un-upgraded loop would say. Pins that float fast paths
-- share the integer firewall's message divergence exactly.
-- EXPECT_PANIC: Runtime Error: Negative index in fast path
local t = {}
local i = 0 - 3
while i < 8 do
    t[i] = 0.5
    i = i + 1
end
