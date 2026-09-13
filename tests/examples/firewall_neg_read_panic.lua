-- firewall_neg_read_panic.lua — runtime half of the off<0 contract: i=0 makes
-- the first a[i-1] read hit key -1 on the DYN path (the off<0 read is declined
-- precisely so this panic keeps its message). Pin the FULL dyn message — the
-- fast path's "Negative index in fast path" here would mean an unsound
-- upgrade of a negative-offset key.
-- EXPECT_PANIC: Runtime Error: Negative table index
local a = {}
local i = 0
while i < 3 do
    local prev = a[i - 1]
    a[i] = prev + 1
    i = i + 1
end
