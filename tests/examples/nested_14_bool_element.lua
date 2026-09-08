-- nested_14_bool_element.lua  [NEGATIVE — element domain guard]
-- Table elements can only be Integer or Table: the backend's element
-- access codegens exactly i_r (integer) and t_r (handle) targets. The old
-- checker ACCEPTED a Boolean store into an unresolved table and died later
-- in broken generated code; unification rejects it up front with a clean
-- build error.
-- EXPECT_BUILD_FAIL: Type Error: table elements cannot be Boolean
local t = {}
t[0] = 1 < 2
