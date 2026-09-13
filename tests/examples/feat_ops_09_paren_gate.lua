-- feat_ops_09_paren_gate.lua [POSITIVE — parentheses are AST-transparent:
-- `while (i <= 5)` keeps the tier4 fast-path gate open exactly like the
-- bare form]. The parser unwraps (expr) before the AST exists, so the
-- literal-bound <= desugar still materializes 5+1 in the pre-header and
-- the region upgrades: fast_sets=1, hoists=1, byte-shaped like
-- feat_ops_03 with parens added.
-- EXPECT: TABLE 0 LEN 6 NZ 6 CHECKSUM 91
-- EXPECT: fast_sets=1
-- EXPECT: dyn_sets=0
-- EXPECT: hoists=1
-- EXPECT: hoist_ctx=0
local t = {}
local i = 0
while (i <= 5) do
    t[i] = i + 1
    i = i + 1
end
