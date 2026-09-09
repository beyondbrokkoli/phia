-- probe_zero_trip_init: lim is a positive literal (8) but the induction
-- var STARTS ABOVE it (100) — a zero-trip loop with a NIL child (t is
-- empty). Dyn semantics: loop never runs, no panic, t stays empty.
-- Suspected hole: tier-4's gate proves lim > 0 but not init < lim, so
-- the minted EC/HR in the pre-header may panic on a loop that never runs.
local t = {}
local i = 100
while i < 8 do
    t[0][i] = 1
    i = i + 1
end
