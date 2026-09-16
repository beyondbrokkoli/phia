-- logic_neg_or_value_default.lua [NEGATIVE — the pinned divergence from
-- Lua: `x or default` returns the default value there; here and/or are
-- strictly Boolean × Boolean, so the idiom itself is a build error]
-- EXPECT_BUILD_FAIL: Type Error: 'and'/'or' require Boolean operands on both sides
local x = false or 5
