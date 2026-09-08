-- firewall_neg_init_panic.lua — documents a LIVE message divergence: i starts
-- negative (0-3), so the upgraded a[i] (Some(0) — PASS 2 never inspects the
-- IV's init) hits k < 0 on the FAST path, where the dyn path would say
-- "Negative table index". Both panic; the messages differ. This pin freezes
-- current behavior — if it ever flips to the dyn message, someone added
-- negative-init IV detection (the init-eval slice of Tier 3).
-- EXPECT_PANIC: Runtime Error: Negative index in fast path
local a = {}
local i = 0 - 3
while i < 3 do
    a[i] = 1
    i = i + 1
end
