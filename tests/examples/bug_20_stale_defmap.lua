-- bug_20_stale_defmap.lua — the tier-4 orphan-feeder cleanup RETAINed
-- dead reads out of region blocks mid-loop, shifting the instruction
-- indices of everything after the removal point; def_map (built once,
-- raw (block, index) pairs) then misread the inner loop's limit/entry
-- defs sharing the feeder's block — const_eval indexed out of bounds
-- and the build died in optimize(). Fixed by DEFERRING the deletion to
-- one program-wide retain after the header loop: final IR is identical,
-- def_map stays stable for the whole loop. The pin doubles as a
-- tier-4-positive: the outer pass still mints the child handle
-- (row[i] rides p_r through the hoist), so the fix cannot quietly
-- over-correct into declining the nested conversion.
-- EXPECT: TABLE 0 LEN 1 NZ 1 CHECKSUM 2
-- EXPECT: TABLE 1 LEN 10 NZ 10 CHECKSUM 385
-- EXPECT: fast_sets=1
-- EXPECT: fast_gets=0
-- EXPECT: dyn_sets=1
-- EXPECT: dyn_gets=1
-- EXPECT: hoists=1
-- EXPECT: hoist_ctx=0
local m = {}
m[0] = {}
local i = 0
while i < 10 do
    local row = m[0]
    local n = 5
    local j = 0
    while j < n do
        row[i] = 7
        j = j + 1
    end
    i = i + 1
end
