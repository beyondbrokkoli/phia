-- nested_02_lvalue_type.lua  [PANIC — was NEGATIVE; lazy unification killed the build error]
-- `a[0][0] = 42` on an empty table used to die at BUILD time ("target is not
-- a table"): the eager read of a[0] locked a's elements to Integer, and an
-- Integer cannot be an lvalue. Under lazy unification the read constrains
-- nothing — the target a[0] legitimately has table type, and the program is
-- well-typed. It dies at RUNTIME exactly like nested_09: a[0] is the null
-- handle, and storing through it is the nil panic. Statement order no longer
-- decides build-vs-run; the value does.
-- EXPECT_PANIC: Runtime Error: table is nil
local a = {}
a[0][0] = 42
