-- float_11_neg_index_panic.lua  [PANIC — float value, dyn negative key]
-- Strict typing means number coercion itself can never reach runtime
-- (mixed Integer/Float arithmetic dies at build — float_03). What CAN
-- reach runtime is a float VALUE on the dynamic guard arms; this pins
-- that float stores go through the same negative-index wall as integers.
-- EXPECT_PANIC: Runtime Error: Negative table index
local t = {}
local j = 0 - 1
t[j] = 1.5
