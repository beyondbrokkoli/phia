-- bug_05a.lua — panic test, stats not compared. For the record: the loop itself is
-- fully optimizable (fast_sets=1, hoists=1 if pinned); the panic comes from the
-- post-loop a[k] with k = 0 - 1, caught by the dynamic path's k < 0 check.
-- The fast path plays no role in the failure.
-- EXPECT_PANIC: Runtime Error: Negative table index
local a = {}
local i = 0
while i < 10 do
    a[i] = i
    i = i + 1
end
local k = 0 - 1
a[k] = 5
