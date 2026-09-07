-- bug_07a.lua — gate ✓ (literal 10), key φi safe, but the TABLE is born inside the
-- loop: NewTable's def block is the body, not < header. A table re-created every
-- iteration is loop-carried — no invariant pointer to cache, and EC in the pre-header
-- would size a table that doesn't exist yet. Even per-iteration hoisting gains nothing
-- (1 access/iteration ⇒ EC+HR cost == dynamic bounds cost). This shape's floor IS
-- dyn_sets=1; the fix would be allocation strategy, not bounds hoisting.
-- EXPECT: NTABLES 10
-- EXPECT: TABLE 0 LEN 1 NZ 1 CHECKSUM 1
-- EXPECT: TABLE 1 LEN 2 NZ 1 CHECKSUM 2
-- EXPECT: TABLE 2 LEN 3 NZ 1 CHECKSUM 3
-- EXPECT: TABLE 3 LEN 4 NZ 1 CHECKSUM 4
-- EXPECT: TABLE 4 LEN 5 NZ 1 CHECKSUM 5
-- EXPECT: TABLE 5 LEN 6 NZ 1 CHECKSUM 6
-- EXPECT: TABLE 6 LEN 7 NZ 1 CHECKSUM 7
-- EXPECT: TABLE 7 LEN 8 NZ 1 CHECKSUM 8
-- EXPECT: TABLE 8 LEN 9 NZ 1 CHECKSUM 9
-- EXPECT: TABLE 9 LEN 10 NZ 1 CHECKSUM 10
-- EXPECT: fast_sets=0
-- EXPECT: fast_gets=0
-- EXPECT: dyn_sets=1
-- EXPECT: dyn_gets=0
-- EXPECT: hoists=0
local i = 0
while i < 10 do
    local t = {}
    t[i] = 1
    i = i + 1
end
