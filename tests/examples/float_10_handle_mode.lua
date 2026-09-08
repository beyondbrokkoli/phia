-- float_10_handle_mode.lua  [POSITIVE — float fast paths in HANDLE mode]
-- The matrix cell the other float sentinels miss: this program also stores
-- a table into a table, so the dual-template gate switches the whole
-- program to arena handles — and the float loop still upgrades. EC and
-- HoistRawPtr resolve the float root through tables.get_mut, the fast
-- store writes f_r through *mut f64, and the float table itself rides
-- INSIDE another table (outer[0] = handle 2). Closes: float × handle ×
-- nested-float-table, all three at once.
-- EXPECT: TABLE 0 LEN 1 NZ 1 CHECKSUM 2
-- EXPECT: TABLE 1 LEN 8 NZ 8 CHECKSUM -324259173170675712 SUM 4
-- EXPECT: NTABLES 2
-- EXPECT: fast_sets=1
-- EXPECT: fast_gets=0
-- EXPECT: dyn_sets=1
-- EXPECT: dyn_gets=1
-- EXPECT: hoists=1
-- EXPECT: hoist_ctx=0
local outer = {}
local ft = {}
local i = 0
while i < 8 do
    ft[i] = 0.5
    i = i + 1
end
outer[0] = ft
local x = outer[3]
