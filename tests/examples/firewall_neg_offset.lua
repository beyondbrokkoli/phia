-- firewall_neg_offset.lua — FIREWALL sentinel for the off<0 non-resize proof:
-- key i-1 folds to Some(-1) — key < limit - 1 < limit <= len, in-bounds-or-
-- negative-panic, NEVER a resize — so the read stays dyn (message equivalence)
-- but must NOT poison the root: the a[i] write (Some(0)) stays fast and the
-- hoist survives. Tier 1 over-poisoned this shape (fast_sets would be 0);
-- a regression to that behavior blinks here first.
-- EXPECT: TABLE 0 LEN 10 NZ 9 CHECKSUM 330
-- EXPECT: fast_sets=1
-- EXPECT: fast_gets=0
-- EXPECT: dyn_sets=0
-- EXPECT: dyn_gets=1
-- EXPECT: hoists=1
-- EXPECT: hoist_ctx=0
local a = {}
local n = 10
local i = 1
while i < n do
    local prev = a[i - 1]
    a[i] = prev + 1
    i = i + 1
end
