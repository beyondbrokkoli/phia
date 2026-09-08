-- nested_11_target_scalar.lua  [NEGATIVE — keeps the "target is not a table" arm pinned]
-- nested_02 used to be this arm's only witness, and it flipped to PANIC:
-- there, the target's type was an unconstrained element variable (legal
-- table). Here the target is a CONCRETE Integer — no unification can make
-- a scalar an lvalue. The arm itself must stay guarded.
-- EXPECT_BUILD_FAIL: Type Error: target is not a table
local x = 1
x[0] = 2
