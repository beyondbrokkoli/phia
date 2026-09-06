-- gauntlet_pB.lua — the headline case.
-- Outer body (b2) holds only pb_j = 0, so the outer pass sees no table ops.
-- Inner loop: limit is pb_i = the outer header's φ (def b1 < inner header b4)
-- → gate passes, and soundly: pb_i is stable for each entire inner execution
-- (bumped only after, in E2), and EC+HR land in b2, re-arming capacity every outer iteration.
-- pb_t[φj] safe → upgrade.
-- EXPECT: TABLE 0 LEN 299 NZ 299 CHECKSUM 8955050
-- EXPECT: fast_sets=1
-- EXPECT: fast_gets=0
-- EXPECT: dyn_sets=0
-- EXPECT: dyn_gets=0
-- EXPECT: hoists=1
local pb_t = {}
local pb_n = 300
local pb_i = 0
while pb_i < pb_n do
    local pb_j = 0
    while pb_j < pb_i do
        pb_t[pb_j] = pb_j + 1
        pb_j = pb_j + 1
    end
    pb_i = pb_i + 1
end
