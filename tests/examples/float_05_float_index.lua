-- float_05_float_index.lua  [NEGATIVE — floats are values, never keys]
-- Design rule: table indexes are i64, always. key_offset's affine
-- arithmetic (and thus every fast path) is built on integer induction; a
-- float key is rejected before it can exist. Note this also diverges from
-- Lua's a[1.0] == a[1] aliasing — pinned here as a permanent restriction.
-- EXPECT_BUILD_FAIL: Type Error: Table index must be an Integer
local t = {}
t[0.5] = 1
