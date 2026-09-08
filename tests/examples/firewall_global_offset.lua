-- firewall_global_offset.lua — FIREWALL sentinel for global_max_off: the loop
-- hoists root a via a[i] (Some(0)) while t[i+3] (Some(3)) writes through an
-- UNTRACEABLE phi table that aliases a from iteration 1 on. PASS 1 bounds
-- EVERY EC in the loop by the untraceable write's offset: EC mints
-- LoadInt(5+3) and resizes a to 8 in the pre-header, so the aliased dyn
-- write t[5..7] never resizes a under the hoisted pointer. LEN 8 is the
-- payment made visible. If EC ever sizes to 5 again (global bound dropped),
-- the dyn write resizes a at i=2, the hoisted p_a/len_a go stale, and this
-- checksum is the first thing that rots.
-- EXPECT: NTABLES 2
-- EXPECT: TABLE 0 LEN 8 NZ 8 CHECKSUM 204
-- EXPECT: TABLE 1 LEN 4 NZ 1 CHECKSUM 36
-- EXPECT: fast_sets=1
-- EXPECT: fast_gets=0
-- EXPECT: dyn_sets=1
-- EXPECT: dyn_gets=0
-- EXPECT: hoists=1
-- EXPECT: hoist_ctx=0
local a = {}
local t = {}
local n = 5
local i = 0
while i < n do
    a[i] = 1
    t[i + 3] = 9
    t = a
    i = i + 1
end
