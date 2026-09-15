-- whilepretty_01_shared_id.lua  [POSITIVE — pretty while-form on the
-- historical id-collision shape]
-- The pretty-form pin, kept on the shape that discovered it. Under the
-- OLD shared id range this program's loop counter and its header
-- Less's bool target co-numbered (i_rN and b_rN, one number, two
-- variables), and the pretty-arm guard's raw-id use count saw the
-- counter's uses as the cond's — EVERY `while i < n` shaped loop
-- degraded to the fallback `loop { b_rN = i_rN < lit; ... }` spelling.
-- The interim fix counted bool-CONTEXT uses (bool_reg_uses); since the
-- one-global-timeline mint gave every physical a unique id, the guard
-- is a plain raw use count (count_uses) and the collision shape is
-- ordinary. This lock keeps pinning the pretty spelling — and its
-- decl block (i_rN, then b_rN+1) pins the counter/cond ids NOT
-- co-numbering anymore.
-- RE-VERIFY IF THIS BREAKS: a `loop {` in this lock means the guard
-- over/under-counts a use site; the value pin below is the semantic
-- net.
-- EXPECT_PRINT: i	3
local i = 0
while i < 3 do i = i + 1 end
print("i", i)
