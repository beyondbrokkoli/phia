-- tier4_05_nil_child_panic.lua  [PANIC — the lim > 0 contract, panic side]
-- Literal limit 8 > 0 with an empty t: tier-4 materializes the child,
-- EC fires under its lim > 0 guard, and the null handle is the same nil
-- panic the dyn path would raise on first iteration — hoisted to the
-- pre-header exactly like tier-2's EC has always done for roots.
-- EXPECT_PANIC: Runtime Error: table is nil
local t = {}
local i = 0
while i < 8 do
    t[0][i] = 1
    i = i + 1
end
