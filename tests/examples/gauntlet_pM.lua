-- gauntlet_pM.lua — PATCH C FLIP: hoists is now keyed by ROOT, not reg. pm_a/pm_b
-- are two vregs over ONE table; the EC/HR contract belongs to the table, so b0 now
-- holds ONE EnsureCapacity + ONE HoistRawPtr (root reg) and both SetTableFasts
-- point at the root. fast_sets stays 2 (both writes upgraded); hoists 2→1
-- (unique tables). Side effect: pm_b's alias Move is dead and simplify() drops it.
-- EXPECT: TABLE 0 LEN 90 NZ 90 CHECKSUM 251160
-- EXPECT: fast_sets=2
-- EXPECT: fast_gets=0
-- EXPECT: dyn_sets=0
-- EXPECT: dyn_gets=0
-- EXPECT: hoists=1
-- EXPECT: hoist_ctx=0
local pm_a = {}
local pm_b = pm_a
local pm_n = 90
local pm_i = 0
while pm_i < pm_n do
    pm_a[pm_i] = pm_i + 1
    pm_b[pm_i] = pm_i + 2
    pm_i = pm_i + 1
end
