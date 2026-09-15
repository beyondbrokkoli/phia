-- fconst_04_floordiv_literal_cast.lua  [POSITIVE — E0689 pin, fuzzer 65]
-- Fuzzer seed 65's find: the float floor-div/mod templates spell the
-- inner division as `(x / y).floor()` — fine when operands are typed
-- registers, but consts_f can fold BOTH operands to literals, and
-- `.floor()` on an all-literal division is f32/f64-AMBIGUOUS (E0689).
-- Reaching it needs the arith to stay runtime while its operands fold:
-- a multi-def target. This shape gets one — the inner while makes the
-- if-join phi take the COALESCING path, so the injected Move makes the
-- IntDiv/Mod targets multi-def while their operands (the pre-if
-- LoadFloats) fold to literals.
-- The lock pins the fix's spelling: `((lit / lit) as f64).floor()` —
-- the cast pins the {float} placeholder to f64.
-- RE-VERIFY IF THIS BREAKS: a bare `(lit / lit).floor()` in the lock
-- (no `as f64`) means the cast regressed and this program stops
-- compiling; the value pins are the semantic net (floor semantics:
-- -8.45 // 4.0 == -3.0, and -8.45 % 5.0 takes the divisor's sign).
-- EXPECT_PRINT: f	-3.0
-- EXPECT_PRINT: g	1.5500000000000007
-- EXPECT: consts_f=6
local f = -8.45
local g = -8.45
if (not false) then
  local i = 0
  while i < 2 do i = i + 1 end
  f = f // 4.0
  g = g % 5.0
end
print("f", f)
print("g", g)
