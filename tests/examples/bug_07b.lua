-- bug_07b.lua — same loop, different disease: t is REASSIGNED in-loop, so the write
-- goes through φt (def = header) → S2 dominance fails, and get_table_root stops at a
-- Phi → untraceable. Loop-carried table = no hoist, by design. NTABLES 11 = one
-- pre-loop + ten in-loop allocations, all leaked into `tables` intentionally.
-- EXPECT: NTABLES 11
-- EXPECT: TABLE 0 LEN 0 NZ 0 CHECKSUM 0
-- EXPECT: TABLE 1 LEN 1 NZ 1 CHECKSUM 1
-- EXPECT: TABLE 2 LEN 2 NZ 1 CHECKSUM 2
-- EXPECT: TABLE 3 LEN 3 NZ 1 CHECKSUM 3
-- EXPECT: TABLE 4 LEN 4 NZ 1 CHECKSUM 4
-- EXPECT: TABLE 5 LEN 5 NZ 1 CHECKSUM 5
-- EXPECT: TABLE 6 LEN 6 NZ 1 CHECKSUM 6
-- EXPECT: TABLE 7 LEN 7 NZ 1 CHECKSUM 7
-- EXPECT: TABLE 8 LEN 8 NZ 1 CHECKSUM 8
-- EXPECT: TABLE 9 LEN 9 NZ 1 CHECKSUM 9
-- EXPECT: TABLE 10 LEN 10 NZ 1 CHECKSUM 10
-- EXPECT: fast_gets=0
-- EXPECT: dyn_sets=1
-- EXPECT: dyn_gets=0
-- EXPECT: hoists=0
local t = {}
local n = 10
local i = 0
while i < n do
    t = {}
    t[i] = 1
    i = i + 1
end
