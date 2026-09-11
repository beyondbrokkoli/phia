-- Story of the bug
-- Actual root cause — stale const key / physical id collision:
-- local v12 = v8 lowered to a dead Move, which propagate_constants folded into consts_i[85] = 0.
-- simplify's DCE then deleted that Move — id 85 disappeared from the IR, but its const-map entry survived.
-- allocate_registers computed base from the post-DCE max vreg (83) → base 84, and minted physical 85 for the GetTable target.
-- Physical 85 == stale const key 85 → emit_instr's const early-out silently swallowed the GetTable,
-- and the probe printed the dead constant 0 instead of the register.
-- The fix (in src/backend.rs, allocate_registers): mint from max(max_reg, max consts key) + 1 — i.e.,
-- above the entire id space the emission-side const maps can ever be queried with.
-- Also fixed the dump's arith helper in build.rs, which was mislabeling
-- int Adds as f_r (is_float_reg is now pub for it — that's why the dump showed the bogus Add { target: f_r84 }).

-- EXPECT: fast_sets=0
-- EXPECT: fast_gets=0
-- EXPECT: dyn_sets=10
-- EXPECT: dyn_gets=3
-- EXPECT: hoists=0
-- the current formatting scheme makes it impossible to define empty hoist_ctx because it compares against nil
-- EXPECT: PROBE final: i_r86=0 f_r91=-8.56 s_r86="beta" b_r5=false i_r18=-36 b_r86=true i_r50=6 i_r87=26 f_r90=-12.99 i_r73=0
-- EXPECT: TABLE 0 LEN 6 NZ 3 CHECKSUM -9129967380583111719 SUM -12.120000000000001
-- EXPECT: TABLE 1 LEN 6 NZ 2 CHECKSUM 192
-- EXPECT: TABLE 2 LEN 1 NZ 1 CHECKSUM -4599869698905074565 SUM -12.99
-- EXPECT: TABLE 3 LEN 1 NZ 1 CHECKSUM -4602363567198730977 SUM -8.56
local root_int = -18
local root_float = -8.56
local root_str = "beta"
local root_bool = false
local v0 = {}
v0[0] = (-1.87 / -2.0)
local v1 = {}
v1[0] = (19 % 5)
local v2 = (root_int - 18)
local v3 = 0
while v3 < 4 do
  v0[0] = root_float
  v0[4] = (6.16 % -3.0)
  v1[5] = (8 - root_int)
  v3 = v3 + 1
end
v0[4] = (-17.19 // -3.0)
local v4 = (root_float >= -16.65)
local v5 = (-1 + 7)
root_int = v1[1]
v1[0] = (9 * 4)
local v6 = v1[5]
v4 = (root_float <= root_float)
v0[5] = root_float
local v7 = (-4.43 + root_float)
local v8 = (12 % 2)
local v9 = {}
v9[0] = v7
local v10 = v1[0]
local v11 = {}
v11[0] = root_float
local v12 = v8
print("final", root_int, root_float, root_str, root_bool, v2, v4, v5, v6, v7, v8)
