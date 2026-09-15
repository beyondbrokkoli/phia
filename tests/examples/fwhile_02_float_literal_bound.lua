-- fwhile_02_float_literal_bound.lua  [POSITIVE — loop-invariant load hoist]
-- The float-literal-bound while: the two shapes fwhile_01 left rough,
-- now served by the lowerer's loop-invariant load hoist (every Load*
-- is pure and operand-free, so any Load inside a loop is invariant by
-- construction and relocates to the pre-header).
-- (1) `while f < 2.0` — the literal bound's LoadFloat used to sit IN
-- the header (materialize_bound is int-only; the tier-4 gate it feeds
-- never applies to floats), re-materializing every iteration. The
-- hoist lifts it out, leaving a LONE float Less in the header — which
-- the pretty arm must still decline on OPERAND KIND (fwhile_01's law:
-- floats never fold into the while-condition; the old len()==2
-- structural decline is gone, so this lock is the pin that the
-- is_float_reg/consts_f checks carry the decline alone).
-- (2) the runtime-visible case: an inf literal loaded inside the body.
-- The const fold declines it (cf is finite-only — fconst_02's law),
-- so it emits a real `f_rN = f64::INFINITY;` load — the one Load
-- whose position the emitted code can show, and the hoist moves it
-- ABOVE the loop. consts_f=5 pins that only the five finite literals
-- folded (0.0/2.0/0.5 and 0.0/1.0) — the inf literal never enters cf.
-- RE-VERIFY IF THIS BREAKS: an `f64::INFINITY` line INSIDE the `loop {`
-- body means the hoist regressed; a `while f_r`/`while i_r` spelling
-- means the pretty arm took float operands; consts_f=6 means the fold
-- started accepting inf.
-- EXPECT_PRINT: f	2.0
-- EXPECT_PRINT: g	inf
-- EXPECT: consts_f=5
-- EXPECT_PROBE: #0 tag="f" b3 depth0 f_r2
-- EXPECT_PROBE: #1 tag="g" b6 depth0 f_r3
local f = 0.0
while f < 2.0 do f = f + 0.5 end
print("f", f)
local g = 0.0
while g < 1.0 do g = g + 1000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000.5 end
print("g", g)
