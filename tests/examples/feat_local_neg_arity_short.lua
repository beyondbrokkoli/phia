-- feat_local_neg_arity_short.lua [NEGATIVE — a short value list cannot
-- bind every name: Lua would pad `b` with nil, but nil does not exist
-- here, so the count mismatch is a build-time syntax error — pinned
-- divergence (the error moves from nil-land into the build)].
-- EXPECT_BUILD_FAIL: Syntax Error: 'local' binds 2 names to 1 values — counts must match
local a, b = 1
