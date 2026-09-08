-- tier4_07_reg_limit_decline.lua  [POSITIVE — the literal-limit gate]
-- The limit is n = 5 + 3: invariant and literal-FED, but its def is an
-- Add, not a LoadInt — tier-4 cannot prove lim > 0 at compile time and
-- declines (a zero-trip loop with a nil child must never inherit a
-- pre-header panic). Flat tier-2 has no such restriction — the divergence
-- is deliberate and pinned here. Extending to proven-positive computed
-- limits is future work.
-- EXPECT: TABLE 0 LEN 1 NZ 1 CHECKSUM 2
-- EXPECT: TABLE 1 LEN 8 NZ 8 CHECKSUM 36
-- EXPECT: NTABLES 2
-- EXPECT: fast_sets=0
-- EXPECT: fast_gets=0
-- EXPECT: dyn_sets=3
-- EXPECT: dyn_gets=1
-- EXPECT: hoists=0
local t = {}
local inner = {}
inner[0] = 0
t[0] = inner
local n = 5 + 3
local i = 0
while i < n do
    t[0][i] = 1
    i = i + 1
end
