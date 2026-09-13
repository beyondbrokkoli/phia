-- float_03_mixed_arith.lua  [NEGATIVE — strict numeric typing, no coercion]
-- Lua would evaluate 1 + 0.5 as 1.5 (numbers are numbers); this language
-- keeps Integer and Float strictly apart — a mixed arithmetic pair is a
-- build error. Deliberate, pinned divergence: silent int->float promotion
-- would betray the monomorphic element contracts the backend relies on.
-- EXPECT_BUILD_FAIL: Type Error: Binary operations do not support mixed Integer and Float
local x = 1 + 0.5
