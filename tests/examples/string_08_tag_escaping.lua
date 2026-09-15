-- string_08_tag_escaping.lua  [POSITIVE — print tags never sit in the
-- format template]
-- String literals are RAW (the lexer does no escape processing), so a
-- print tag can carry braces and backslashes into codegen. The tag must
-- ride as a println! ARGUMENT, {:?}-escaped exactly like LoadString:
-- embedded in the template, "C:\path" broke the cargo build (invalid
-- Rust escape \p) and "a\nb" printed a real newline instead of the
-- literal two characters. Line 3 exercises the string VALUE side: an
-- operand with backslashes prints raw via runtime Display.
-- EXPECT_PRINT: a{b}c\d	1
-- EXPECT_PRINT: C:\path	2.5
-- EXPECT_PRINT: val=C:\tmp\x	7
-- EXPECT_PROBE: #0 tag="a{b}c\d" b0 depth0 c0
-- EXPECT_PROBE: #1 tag="C:\path" b0 depth0 c1
-- EXPECT_PROBE: #2 tag="" b0 depth0 c2 c3
local n = 1
local f = 2.5
local p = "val=C:\tmp\x"
print("a{b}c\d", n)
print("C:\path", f)
print(p, 7)
