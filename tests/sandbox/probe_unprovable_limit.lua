-- probe_unprovable_limit: n = src[0] + 1 — a table read feeds the limit,
-- so no const-fold can prove lim > 0. Must KEEP declining even after
-- computed-limit support lands (a zero-trip loop with a nil child must
-- never inherit a pre-header panic). Future sentinel for the decline arm.
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
