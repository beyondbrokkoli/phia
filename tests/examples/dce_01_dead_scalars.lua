-- DCE-01 · dead-scalar elimination lock · 2026-09-15
--
-- The DCE feature shipped without a deliberate pin: runtime pins can only
-- assert PRESENCE (a value that must print), never ABSENCE. This test's
-- assertion lives in its LOCK (tests/lock/dce_01_dead_scalars.rs): the
-- frozen baked_native.rs must contain NO f_r/s_r/b_r decls, no format!
-- (the dead Concat's emission), and no trace of the dead defs — only the
-- live print's code. General principle: every elimination feature gets a
-- minimal lock test like this one.
--
-- Body: one dead value per scalar kind, with the string chain built so
-- each link's only user is the NEXT link (a dead Concat) — link k dies
-- only after link k+1 was swept, the simplify() fixpoint case. The dead
-- bool rides a float Less (not const-foldable — no consts_f/consts_b path
-- can take it), so it must die through a DCE arm too.
--
-- RE-VERIFY IF THIS BREAKS: relock and diff the lock — a resurrected decl
-- (f_r/s_r/b_r) or a format! line means a DCE arm regressed; the runtime
-- pins below cannot catch that (they only see the live line).

-- EXPECT_PRINT: dce_01	40
-- EXPECT: fast_sets=0
-- EXPECT: fast_gets=0
-- EXPECT: dyn_sets=0
-- EXPECT: dyn_gets=0
-- EXPECT: hoists=0
local live = 40
local dead_int = live + 2
local dead_float = 1.5 * 2.25
local dead_bool = dead_float < 3.0
local s1 = "alpha"
local s2 = s1 .. "-mid"
local s3 = s2 .. "-end"
print("dce_01", live)
