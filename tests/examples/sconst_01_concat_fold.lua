-- sconst_01_concat_fold.lua  [POSITIVE — consts_s, the allocation payoff]
-- Compile-time string concatenation: `"a" .. "b"` becomes a literal in
-- the const map — no format!, no .clone() chain, no s_r decls in the
-- lock. This is the consts_s value statement pinned as an absence class
-- (the dce_01 principle): the lock must show the print's operand as a
-- Debug-escaped literal and nothing else on the string side.
-- RE-VERIFY IF THIS BREAKS: a format! line or s_r decl in the lock means
-- the Concat fold (or the Move/store const arms) regressed; the value
-- pin below is the semantic net.
-- EXPECT_PRINT: sconst_01	alpha-beta-gamma
-- EXPECT: consts_s=5
local a = "alpha"
local b = "-beta"
local c = a .. b
local d = c .. "-gamma"
print("sconst_01", d)
