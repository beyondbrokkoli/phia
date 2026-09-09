-- tier4_12_unprovable_limit.lua  [POSITIVE — the computed-limit decline arm]
-- tier4_07 flipped: limits that CONST-FOLD positive (n = 5 + 3) now
-- convert. This is the side that stays closed — n = src[0] + 1 is a
-- table read plus an add: no folder can prove lim > 0, an unprovable
-- limit can be 0 at runtime, and the mint's pre-header EC/Hoist
-- nil-panic would be a zero-trip hazard. Tier-4 declines wholesale; the
-- whole loop stays dyn. The boundary is proof, not syntax: anything the
-- def-chain folder can evaluate (LoadInt/Move/Add/Sub over constants)
-- is admissible, everything else is not.
-- EXPECT: TABLE 0 LEN 1 NZ 1 CHECKSUM 7
-- EXPECT: TABLE 1 LEN 1 NZ 1 CHECKSUM 3
-- EXPECT: TABLE 2 LEN 8 NZ 8 CHECKSUM 36
-- EXPECT: NTABLES 3
-- EXPECT: fast_sets=0
-- EXPECT: fast_gets=0
-- EXPECT: dyn_sets=4
-- EXPECT: dyn_gets=2
-- EXPECT: hoists=0
local src = {}
src[0] = 7
local t = {}
local inner = {}
inner[0] = 0
t[0] = inner
local n = src[0] + 1
local i = 0
while i < n do
    t[0][i] = 1
    i = i + 1
end
