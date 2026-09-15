-- whilepretty_01_shared_id.lua  [POSITIVE — pretty while-form under the
-- shared id range]
-- The id-collision pin. Int and Bool physicals deliberately mint from
-- one shared base: this program's loop counter and its header Less's
-- bool target both take base+0 (i_rN and b_rN, one number, two
-- variables). The pretty-arm guard used to count uses by RAW id, so the
-- counter's uses inflated the cond's count and EVERY `while i < n`
-- shaped loop degraded to the fallback `loop { b_rN = i_rN < lit; ... }`
-- spelling — no lock in the corpus had ever shown a pretty
-- `while i_rN < lit {` form. The guard now counts bool-CONTEXT uses
-- only (bool_reg_uses), and this lock pins the pretty spelling on the
-- exact collision shape.
-- RE-VERIFY IF THIS BREAKS: a `loop {` in this lock means the guard
-- regressed to raw-id counting (or over/under-counts a bool reader
-- site); the value pin below is the semantic net.
-- EXPECT_PRINT: i	3
local i = 0
while i < 3 do i = i + 1 end
print("i", i)
