-- nested_12_index_non_table.lua  [NEGATIVE — read-side twin of nested_11]
-- The read arm of indexing a non-table: never pinned by the original
-- nested suite (its sibling only covered the store lvalue). A concrete
-- Integer cannot be indexed, on either side of the assignment.
-- EXPECT_BUILD_FAIL: Type Error: Attempted to index a non-table
local x = 1
local y = x[0]
