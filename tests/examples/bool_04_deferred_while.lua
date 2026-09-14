-- bool_04_deferred_while.lua  [POSITIVE — deferred element binds Boolean]
-- The element variable's FIRST constraint is the while condition: the
-- read t[0] is still a bare Var when check_cond sees it, and binding it
-- to Boolean makes t a bool table — no prior store involved. The loop
-- then runs on table reads as conditions, and the flip store t[0] =
-- false terminates it (the store unifies Boolean with Boolean).
-- EXPECT: TABLE 0 LEN 1 NZ 0 CHECKSUM 0
-- EXPECT: NTABLES 1
-- EXPECT: fast_sets=0
-- EXPECT: fast_gets=0
-- EXPECT: dyn_sets=2
-- EXPECT: dyn_gets=2
-- EXPECT: hoists=0
-- EXPECT_PRINT: deferred	2	false
local t = {}
t[0] = true
local n = 0
while t[0] do
    n = n + 1
    if n == 2 then
        t[0] = false
    end
end
print("deferred", n, t[0])
