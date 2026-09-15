-- fconst_02_nonfinite_boundary.lua  [POSITIVE — the is_finite fold guard]
-- Boundary pin for the guard that is load-bearing, not paranoia: `{:?}`
-- renders non-finite f64s as `inf`/`-inf`/`NaN`, which are NOT valid
-- Rust literals, so non-finite results must stay RUNTIME. The three Divs
-- below have all-const operands yet none folds — their targets mint
-- physicals (f_r) and the emitted code keeps the IEEE semantics:
-- 1.0/0.0 -> inf, -1.0/0.0 -> -inf, 0.0/0.0 -> NaN. The Neg DOES fold
-- (-1.0 rides cf), which is exactly why the guard sits on the RESULT,
-- not on operand finiteness.
-- The lexer's `d.d` shape keeps literals finite, so folding itself is
-- the ONLY producer of non-finite constants — this test is the whole
-- boundary in one place.
-- EXPECT_PRINT: pinf	inf
-- EXPECT_PRINT: ninf	-inf
-- EXPECT_PRINT: nan	NaN
-- EXPECT: consts_f=7
local pinf = 1.0 / 0.0
local ninf = -1.0 / 0.0
local nan = 0.0 / 0.0
print("pinf", pinf)
print("ninf", ninf)
print("nan", nan)
