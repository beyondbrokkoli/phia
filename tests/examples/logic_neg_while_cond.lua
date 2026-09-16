-- logic_neg_while_cond.lua [NEGATIVE — the milestone's one restriction:
-- and/or in a while CONDITION desugars to a two-exit loop shape the
-- structured codegen cannot yet emit; bind the chain to a local first]
-- EXPECT_BUILD_FAIL: Lowerer: 'and'/'or' in a while condition is not yet supported — bind it to a local first (loops compile to single-exit shapes)
while true and false do
end
