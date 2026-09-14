-- bool_03_handle_mode.lua  [POSITIVE — bool tables inside tables]
-- A bool table stored INSIDE another table forces handle mode (the
-- dual-template gate): the outer root is a handle-array table, the
-- inner table a barray arena member. Reads of the child ride 1-based
-- arena handles with checked resolution; the print of the child itself
-- pins the barray probe token through table#N(len=M).
-- EXPECT: TABLE 0 LEN 2 NZ 1 CHECKSUM 1
-- EXPECT: TABLE 1 LEN 1 NZ 1 CHECKSUM 1
-- EXPECT: NTABLES 2
-- EXPECT: fast_sets=0
-- EXPECT: fast_gets=0
-- EXPECT: dyn_sets=3
-- EXPECT: dyn_gets=3
-- EXPECT: hoists=0
-- EXPECT_PRINT: grid	true	table#1(len=2)
local flags = {}
flags[0] = true
flags[1] = false
local grid = {}
grid[0] = flags
local read_back = grid[0][0]
print("grid", read_back, grid[0])
