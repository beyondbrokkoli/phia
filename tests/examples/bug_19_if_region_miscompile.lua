-- bug_19_if_region_miscompile.lua — an `if`'s branch block has the same
-- Less shape as a while header, and loop_region() from its then-block
-- flows through the join into post-if code that ALSO executes when the
-- condition is false. The header's Less proved nothing on that path, but
-- PASS 2 upgraded t[x] anyway and EC(y) sized the table: with x=100,
-- y=5 the fast path panicked "optimizer invariant violated" (and even
-- without the panic, EC zero-fill inflated LEN past dyn semantics).
-- Fixed by the LOOP-HEADER GATE in optimize(): a Less-Branch qualifies
-- only when its body can reach the branch block again — a back edge.
-- The pin: the store stays dyn, nothing hoists, LEN is the dyn 101.
-- EXPECT: TABLE 0 LEN 101 NZ 1 CHECKSUM 707
-- EXPECT: fast_sets=0
-- EXPECT: fast_gets=0
-- EXPECT: dyn_sets=1
-- EXPECT: dyn_gets=0
-- EXPECT: hoists=0
local t = {}
local y = 5
local x = 100
if 1 < 2 then
    local warm = 0
end
if x < y then
    local d = 1
end
t[x] = 7
