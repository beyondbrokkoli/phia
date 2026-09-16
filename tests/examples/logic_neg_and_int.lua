-- logic_neg_and_int.lua [NEGATIVE — and/or take Booleans only: no
-- truthiness conversions and no value-returning, so an Integer operand
-- is a build error (Lua would accept `1 and true` as truthiness)]
-- EXPECT_BUILD_FAIL: Type Error: 'and'/'or' require Boolean operands on both sides
if 1 and true then
end
