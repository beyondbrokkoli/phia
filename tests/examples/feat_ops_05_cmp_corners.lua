-- feat_ops_05_cmp_corners.lua [POSITIVE — <= and >= at the domain corners].
-- The old desugar (a <= b => a < b+1) wrapped at i64::MAX and rounded
-- wrong for floats from 2^53 up (b+1.0 == b); <= / >= now compare
-- directly. This pins the reflexive truth at both corners and the
-- one-below neighbors that the desugar got right, int and float.
-- lim is loop-derived so the comparisons run, not fold.
-- EXPECT: TABLE 0 LEN 1 NZ 1 CHECKSUM 11111111
-- EXPECT: NTABLES 1
local x = 0
local k = 0
while k < 1 do k = k + 1 end
local lim = 9223372036854775806 + k
local n = 0
if x <= 9223372036854775807 then n = n + 1 end
if x <= lim then n = n + 10 end
if lim >= lim then n = n + 100 end
if lim > 9223372036854775806 then n = n + 1000 end
local big = 9007199254740992.0
if big <= big then n = n + 10000 end
if big >= big then n = n + 100000 end
if 9007199254740990.0 <= 9007199254740990.0 then n = n + 1000000 end
if big > 9007199254740990.0 then n = n + 10000000 end
local w = {}
w[0] = n
