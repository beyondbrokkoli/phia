-- fwhile_01_float_ident_bound.lua  [POSITIVE — pretty-path float-Less fix]
-- The pretty while-form's regression pin. emit_loop's pretty arm folds a
-- lone header Less into `while a < b`, rendering operands with iop_str! —
-- which spells FLOAT physicals as i_r<id>, referencing undeclared
-- registers (the generated program did not compile). Identifier-bound
-- float bounds are the shape that reaches it: `while f < lim` leaves the
-- bare Less alone in the header, while literal bounds (`while f < 1.0`)
-- load inside the header and decline the pretty path for free.
-- The fix: the pretty arm declines float operands; the loop rides the
-- correct-but-ugly `loop { ... if b_r.. }` fallback instead. This lock
-- pins that fallback: while the bug lived, no lock covered the shape.
-- RE-VERIFY IF THIS BREAKS: a `while i_r` or `while f_r` spelling in the
-- lock means the pretty arm regressed — floats must never fold into the
-- while-condition.
-- EXPECT_PRINT: f	1.0
local f = 0.0
local lim = 1.0
while f < lim do f = f + 0.25 end
print("f", f)
