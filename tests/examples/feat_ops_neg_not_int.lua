-- feat_ops_neg_not_int.lua [NEGATIVE — 'not' is Boolean-only: no truthiness
-- conversions in this language (Lua's not 1 == false is a coercion we
-- reject)].
-- EXPECT_BUILD_FAIL: Type Error: 'not' requires a Boolean operand
if not 1 then
end
