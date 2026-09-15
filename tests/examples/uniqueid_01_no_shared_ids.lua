-- uniqueid_01_no_shared_ids.lua  [POSITIVE — one physical register, one id]
-- The uniqueness pin. Physical ids mint onto ONE global timeline —
-- eight consecutive per-pool ranges — so no two physical registers
-- ever share a number. This program keeps one LIVE value in each of
-- the four formerly co-numbered pools (int, bool, string, table; all
-- loop-carried, so none of them const-folds out of the physical
-- namespace), and the EXPECT_PROBE pins below show four pool-prefixed
-- registers with four DISTINCT ids. Under the old shared range these
-- four all minted from the same base (i_rN/b_rN/s_rN/t_rN, one
-- number, four variables) — the layout behind the Eq-ty, probe-kind
-- and bool-context-counter hazard classes.
-- RE-VERIFY IF THIS BREAKS: a repeated id across two different
-- prefixes in the probe pins (or the decl block) means the allocator
-- regressed to co-numbering pools; the EXPECT_PRINT pins are the
-- semantic net.
-- EXPECT_PRINT: mix	3	xyyy	false
-- EXPECT_PRINT: tab	table(len=3)
-- EXPECT_PROBE: #0 tag="mix" b3 depth0 i_r18 s_r22 b_r19
-- EXPECT_PROBE: #1 tag="tab" b3 depth0 len_r21
-- EXPECT: NTABLES 1
local t = {}
local i = 0
local s = "x"
local flag = true
while i < 3 do
  t[i] = i
  s = s .. "y"
  flag = not flag
  i = i + 1
end
print("mix", i, s, flag)
print("tab", t)
