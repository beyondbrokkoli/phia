-- gauntlet_pM.lua — aliasing: pm_a/pm_b are distinct vregs over one table.
-- Both keys = φ → both upgrade. hoists is keyed by reg, so 2 EC + 2 HR in b0
-- (sound: EC is idempotent, both raw pointers stay valid — nothing resizes between them).
-- EXPECT: TABLE 0 LEN 90 NZ 90 CHECKSUM 251160
-- EXPECT: fast_sets=2
-- EXPECT: fast_gets=0
-- EXPECT: dyn_sets=0
-- EXPECT: dyn_gets=0
-- EXPECT: hoists=2
local pm_a = {}
local pm_b = pm_a
local pm_n = 90
local pm_i = 0
while pm_i < pm_n do
    pm_a[pm_i] = pm_i + 1
    pm_b[pm_i] = pm_i + 2
    pm_i = pm_i + 1
end
