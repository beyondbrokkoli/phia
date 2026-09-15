-- opt_hoist_01_phi_carried_literal.lua  [POSITIVE — hoist vs carried values]
-- The loop-invariant load hoist's exclusion pin (fuzzer seeds 10/139/
-- 192): a Load whose target is a phi back-edge arg must NEVER hoist.
-- `x = 11` on a loop-mutated variable makes that LoadInt the phi's
-- back-edge arg; de_ssa's coalescer renames the phi onto it and
-- injects an initializing Move at pre-header END. A hoisted load
-- would sit before that Move — the init value would win every
-- iteration (x printed 16, root_float kept 18.38 in the fuzzer
-- reproducers) — or execute on zero-iteration paths where the body
-- never ran. WHETHER a carried definition executed is semantic even
-- for a pure load; only the non-carried literals hoist.
-- RE-VERIFY IF THIS BREAKS: x printing 16 or z printing 6.34 means
-- the carried-value exclusion regressed (the emitted code would show
-- the literal loads moved above their loops).
-- EXPECT_PRINT: x	11
-- EXPECT_PRINT: z	18.38
-- EXPECT_PRINT: s	kept
-- EXPECT_PRINT: r	taken
-- EXPECT_PROBE: #0 tag="x" b3 depth0 i_r0
-- EXPECT_PROBE: #1 tag="z" b6 depth0 f_r12
-- EXPECT_PROBE: #2 tag="s" b9 depth0 s_r10
-- EXPECT_PROBE: #3 tag="r" b12 depth0 s_r10
local x = 16
local i = 0
while i < 4 do
  x = 11
  i = i + 1
end
print("x", x)
local y = 0
local z = 18.38
while y < 0 do
  z = 6.34
  y = y + 1
end
print("z", z)
local s = "init"
local k = 0
while k < 2 do
  s = "kept"
  k = k + 1
end
print("s", s)
-- the if-join flavor (seed 192's exact hazard): the assignment's
-- LoadString is an IF-JOIN phi arg whose phi rides the while phi's
-- back edge — hoisting it let the coalescer + slot reuse delete the
-- per-iteration edge-Move, so r kept its init value.
local r = "init"
local j = 0
while j < 3 do
  if j >= 0 then
    r = "taken"
  end
  j = j + 1
end
print("r", r)
