-- probe_computed_limit: n = 5 + 3, invariant, literal-FED but not a LoadInt.
-- Current: tier-4 declines (tier4_07's shape). Target: const-fold -> 8 > 0
-- -> convert.
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
