-- bug_10b.lua — byte-identical shape to bug_01: literal 1000 @ b0, safe key, fast,
-- HR @ b0. Control twin of bug_10a (lexer fail): same body, different failure mode —
-- the pair pins that panic isolation doesn't leak into codegen behavior.
-- EXPECT: TABLE 0 LEN 1000 NZ 1000 CHECKSUM 3503500
-- EXPECT: fast_sets=1
-- EXPECT: fast_gets=0
-- EXPECT: dyn_sets=0
-- EXPECT: dyn_gets=0
-- EXPECT: hoists=1
local a = {}
local i = 0
while i < 1000 do
    a[i] = 7
    i = i + 1
end
