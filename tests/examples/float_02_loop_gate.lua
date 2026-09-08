-- float_02_loop_gate.lua  [POSITIVE — float fast paths, store edition]
-- Born in slice 1 as the float GATE pin (fast_sets=0 while the i64-only
-- fast templates had no float arms); slice 2 removed the gate and this pin
-- flipped to the upgraded shape exactly as documented: EC + HoistRawPtr on
-- the float root, SetTableFast writing f_r through *mut f64.
-- SUM = 0 + 0.25 + ... + 1.75 = 7.
-- EXPECT: TABLE 0 LEN 8 NZ 7 CHECKSUM -4760304806130614272 SUM 7
-- EXPECT: NTABLES 1
-- EXPECT: fast_sets=1
-- EXPECT: fast_gets=0
-- EXPECT: dyn_sets=0
-- EXPECT: dyn_gets=0
-- EXPECT: hoists=1
-- EXPECT: hoist_ctx=0
local t = {}
local s = 0.0
local i = 0
while i < 8 do
    t[i] = s
    s = s + 0.25
    i = i + 1
end
