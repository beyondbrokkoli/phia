-- floatinf_01_infinity_literal.lua  [POSITIVE — Lua-faithful inf literals]
-- The Lua-compatibility pin for overflowing float literals. Lua reads
-- an extreme numeral as +inf (strtod semantics: the value overflows,
-- the literal stays legal), and PHIA follows: the decimal parses to
-- inf, the const fold DECLINES it (cf is finite-only — fconst_02's
-- law), so it materializes as a runtime LoadFloat that emits
-- `f64::INFINITY`, never the bare `inf` spelling `{:?}` would render
-- (the pre-fix failure mode: `f_r1 = inf;` did not compile). The Neg
-- likewise stays runtime (`f_r2 = -f_r1` at -inf), and the comparison
-- and sum compute their IEEE results at runtime — all four print lines
-- are byte-identical to reference Lua on this exact program (the
-- overflow lives in the INTEGER part: 310 digits, ~1e309 > f64::MAX).
-- Probe pins: x rides f_r1, and sum's target coalesces onto neg's
-- freed slot (both f_r2) — allocator reuse among dead values, pinned
-- as-is.
-- RE-VERIFY IF THIS BREAKS: a failed build naming a bare `inf` means
-- the LoadFloat arm lost the named-constant rendering; an EXPECT_PRINT
-- mismatch means the literal no longer parses to inf (lexer) or the
-- fold started accepting it (probe pins would show no f_r for x).
-- EXPECT_PRINT: x	inf
-- EXPECT_PRINT: neg	-inf
-- EXPECT_PRINT: cmp	true
-- EXPECT_PRINT: sum	inf
-- EXPECT_PROBE: #0 tag="x" b0 depth0 f_r1
-- EXPECT_PROBE: #1 tag="neg" b0 depth0 f_r2
-- EXPECT_PROBE: #2 tag="cmp" b0 depth0 b_r0
-- EXPECT_PROBE: #3 tag="sum" b0 depth0 f_r2
local x = 1000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000.5
print("x", x)
print("neg", -x)
print("cmp", x > 5.0)
print("sum", x + 1.0)
