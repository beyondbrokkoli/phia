-- probe_fold_nonpositive: computed limits that FOLD to <= 0 must decline
-- (zero-trip contract). n = 2 - 2 folds to 0.
local t = {}
local inner = {}
inner[0] = 0
t[0] = inner
local n = 2 - 2
local i = 0
while i < n do
    t[0][i] = 1
    i = i + 1
end
