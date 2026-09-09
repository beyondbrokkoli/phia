-- feat_ops_panic_neg_entry_leq.lua [PANIC — a negative-entry `while i <= 3`
-- still opens the tier4 gate (entry -1 < bound 4), upgrades the store to
-- the fast path, and the first iteration trips the fast-path negative-key
-- wall. The dyn path would abort on the same iteration with the
-- "Negative table index" flavor — the program is erroneous either way;
-- this pins WHICH wall a gated loop hits].
-- EXPECT_PANIC: Runtime Error: Negative index in fast path
local t = {}
local i = -1
while i <= 3 do
    t[i] = i
    i = i + 1
end
