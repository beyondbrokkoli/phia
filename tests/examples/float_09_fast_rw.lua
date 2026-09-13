-- float_09_fast_rw.lua  [POSITIVE — float fast paths, read+write edition]
-- Two loops over the same float root: the store loop upgrades to
-- SetTableFast, the read loop to GetTableFast — the float twin of the
-- gauntlet's integer shape. The read feeds a float accumulation chain
-- (s = s + t[j] stays in f_r; rustc keeps it scalar because IEEE addition
-- is not associative — faithful left-to-right Lua semantics).
-- SUM = 8 * 0.5 = 4.
-- EXPECT: TABLE 0 LEN 8 NZ 8 CHECKSUM -324259173170675712 SUM 4
-- EXPECT: NTABLES 1
-- EXPECT: fast_sets=1
-- EXPECT: fast_gets=1
-- EXPECT: dyn_sets=0
-- EXPECT: dyn_gets=0
-- EXPECT: hoists=2
-- EXPECT: hoist_ctx=0,0
local t = {}
local i = 0
while i < 8 do
    t[i] = 0.5
    i = i + 1
end
local s = 0.0
local j = 0
while j < 8 do
    s = s + t[j]
    j = j + 1
end
