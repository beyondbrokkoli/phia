-- feat_ops_neg_lex_i64_overflow.lua [NEGATIVE — an integer literal that
-- does not fit i64 is a lex error, not a wrap: i64::MIN has no literal
-- spelling (the parser has no unary minus at the literal level) and must
-- be computed, e.g. 0 - 9223372036854775807 - 1].
-- EXPECT_BUILD_FAIL: Lexer Error: Unrecognized token or invalid literal '9223372036854775808'
local x = 9223372036854775808
