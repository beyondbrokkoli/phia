-- fconst_02_nonfinite_boundary.lua  [POSITIVE — the is_finite fold guard]
-- Boundary pin for the guard that is load-bearing, not paranoia: it
-- keeps cf finite-only — use-site const rendering spells cf values as
-- `{:?}` literals, and `inf`/`-inf`/`NaN` are not Rust expressions —
-- so non-finite results stay RUNTIME. The three Divs
-- below have all-const operands yet none folds — their targets mint
-- physicals (f_r) and the emitted code keeps the IEEE semantics:
-- 1.0/0.0 -> inf, -1.0/0.0 -> -inf, 0.0/0.0 -> NaN. The Neg DOES fold
-- (-1.0 rides cf), which is exactly why the guard sits on the RESULT,
-- not on operand finiteness.
-- Folding is one producer of non-finite values; overflowing literals
-- are the other — an extreme `d.d` decimal parses to inf, and the same
-- guard declines it there too, so it materializes as a runtime
-- LoadFloat emitting f64::INFINITY (floatinf_01's pin). Between the
-- two, this test plus that one are the whole boundary.
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
