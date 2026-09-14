-- bool_neg_element_conflict.lua [NEGATIVE — bool element monomorphism]
-- Boolean is a first-class element kind now, but a table is still
-- monomorphic: after `t[0] = true` fixes the element variable to
-- Boolean, an integer store into the same table is a type conflict —
-- the same rejection string_neg_element_conflict pins for strings.
-- EXPECT_BUILD_FAIL: Type Error: table element type conflict (Boolean vs Integer)
local t = {}
t[0] = true
t[1] = 5
