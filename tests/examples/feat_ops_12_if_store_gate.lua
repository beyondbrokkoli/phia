-- feat_ops_12_if_store.lua [POSITIVE — the interaction pin: table stores
-- INSIDE an if-arm still upgrade when the enclosing `while i <= 5` opens
-- the tier4 gate]. The region analysis follows branch edges into the arm;
-- the affine store t[i] upgrades to SetTableFast and the table hoists —
-- fast_sets=1, hoists=1, nothing dyn. LEN is the EC-sized
-- capacity (loop bound 6), not max-key+1 — key 5 exists as a zero slot. The floor-mod `i % 2` in the arm
-- runs the runtime template on every iteration (i >= 0, so the sign
-- adjustment arm never fires, but the expression is live).
-- EXPECT: TABLE 0 LEN 6 NZ 2 CHECKSUM 260
-- EXPECT: fast_sets=1
-- EXPECT: dyn_sets=0
-- EXPECT: hoists=1
-- EXPECT: hoist_ctx=0
local t = {}
local i = 0
while i <= 5 do
    if i % 2 == 0 then
        t[i] = i * 10
    end
    i = i + 1
end
