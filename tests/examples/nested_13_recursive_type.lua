-- nested_13_recursive_type.lua  [NEGATIVE — the occurs check]
-- x holds a[0]'s type (a's element variable); storing a INTO x[0] demands
-- that a's elements are tables whose elements are a's own type — an
-- infinite type. The runtime arena could represent it (handles are just
-- integers), but a monomorphic static type would be infinite, so the
-- occurs check rejects at build time. This is a deliberate static-typing
-- limitation, not a soundness hole.
-- EXPECT_BUILD_FAIL: Type Error: recursive table type (a table cannot contain itself)
local a = {}
local x = a[0]
x[0] = a
