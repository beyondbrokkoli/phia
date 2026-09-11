-- FUZZER-PIN 1/3 · seed 43 · fixed 2026-09-11 · REGISTER ALLOCATOR
--
-- SYMPTOM (how this file fails when the bug returns):
--   A GetTable result prints a stale constant (here: 0 instead of 26, idx 7
--   of the "final" probe). The load is missing from baked_native.rs
--   entirely; i_r87=26 below is the pin that catches it.
--
-- TRAP (the wrong diagnosis this bug invites):
--   "The loop optimizer dropped the store." It did not. The store
--   `v1[5] = 26` is emitted and lands — TABLE 1's checksum pins it. What
--   vanishes is the READ `local v6 = v1[5]`: swallowed whole, silently.
--
-- ROOT CAUSE (stale const key / physical id collision):
--   `local v12 = v8` lowered to a dead Move; propagate_constants folded it
--   into consts_i[85]=0; simplify's DCE then deleted the Move. Id 85 left
--   the IR but its consts entry survived. allocate_registers computed the
--   physical-id base from the POST-DCE max vreg (83 → base 84) and minted
--   physical 85 for the GetTable target. Physical 85 == stale const key 85
--   → emit_instr's const early-out (consts_i.contains_key(&target))
--   deleted the GetTable, and iop_str rendered the dead constant 0.
--
-- FIX (grep handles): src/backend.rs, allocate_registers:
--   `let base = max_reg.max(max_const) + 1;`
--   (max_const = max over consts_i/consts_b keys). Physical ids mint above
--   the ENTIRE id space the emission-side const maps can be queried with,
--   not just the surviving-IR namespace. Companion fix: build.rs's dump
--   arith() helper now uses is_float_reg (made pub) instead of a Float-hint
--   probe that always fell through — the dump had shown a bogus
--   `Add { target: f_r84 }` on an int Add.
--
-- RE-VERIFY IF THIS BREAKS:
--   touch this file && PHIA_DEBUG_DUMP=final PHIA_SOURCE=tests/examples/fuzzer_01_paying_rent.lua cargo build --release
--   In ir_final_cfg.txt: a const-marker `v<id>` on a NON-foldable
--   instruction (GetTable/NewTable/arith on non-consts) = this bug class —
--   the invariant "physical id never equals a vreg id (const or not)" is
--   broken again.

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
