-- fconst_01_literal_fold.lua  [POSITIVE — consts_f literal folding]
-- The consts_f absence-class pin (the dce_01 principle applied to float
-- folding): a fully-folded float chain has NO f_r decls and NO f_r
-- statements in the lock — the print's operand is the literal itself.
-- Fold walk: a, b, a*b, 0.5, and the Add all land in cf (consts_f=5);
-- the emitted program contains zero float registers.
-- RE-VERIFY IF THIS BREAKS: an f_r decl in the lock means a fold arm
-- regressed or the early-out stopped swallowing the folded defs; the
-- value pin below is the semantic net, the lock is the absence net.
-- EXPECT_PRINT: fconst_01	3.875
-- EXPECT: consts_f=5
local a = 1.5
local b = 2.25
local c = a * b + 0.5
print("fconst_01", c)
