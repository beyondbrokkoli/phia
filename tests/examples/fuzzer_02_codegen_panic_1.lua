-- FUZZER-PIN 2/3 · seed 136 · fixed 2026-09-11 · STRUCTURED CODEGEN
-- Full story in fuzzer_03_codegen_panic_2.lua (same fix, seed 157).
--
-- SYMPTOM: BUILD fails — build.rs panics
--   "structured codegen: header N reached twice" (src/backend.rs, emit_loop).
--   This is the DEEP-nesting variant: nested ifs inside a while, inside
--   if/else arms, three levels of join stacked in one loop body.
--
-- TRAP: the Lua source is valid — nothing is wrong with the program. The
--   CFG walk, not the program, is at fault (see fuzzer_03 for the exact
--   mechanism: an if-join misgraded as a loop header inside a cyclic
--   region).
--
-- NOTE: the probe's f_r291=-0.5700000000000001 differs from Lua's
--   -0.57000000000000006 in the last ulp — that is formatting only (Lua
--   %.14g vs Rust shortest-roundtrip), NOT a bug; the fuzzer tolerates
--   1e-9. Don't "fix" this pin to match Lua byte-for-byte.

-- EXPECT: PROBE final: i_r0=16 f_r291=-0.5700000000000001 s_r271="betabeta" b_r273=false s_r269="alphadelta" i_r26=-2 s_r268="betaalphadelta" f_r290=0.19000000000000003 i_r185=11 i_r190=1
-- EXPECT: TABLE 0 LEN 6 NZ 3 CHECKSUM 55033987446467463 SUM 14.794666666666664
-- EXPECT: TABLE 1 LEN 5 NZ 2 CHECKSUM -3
-- EXPECT: TABLE 2 LEN 5 NZ 1 CHECKSUM -90
-- EXPECT: TABLE 3 LEN 3 NZ 2 CHECKSUM 53

-- EXPECT: fast_sets=0
-- EXPECT: fast_gets=0
-- EXPECT: dyn_sets=20
-- EXPECT: dyn_gets=6
-- EXPECT: hoists=0

local root_int = 16
local root_float = -2.85
local root_str = "alpha"
local root_bool = false
local v0 = {}
v0[0] = (root_float - -18.13)
local v1 = {}
v1[0] = (root_int % 4)
root_bool = (not false)
local v2 = {}
v2[0] = (root_int % 4)
local v3 = (root_str .. "delta")
local v4 = (root_int % -3)
local v5 = {}
v5[0] = (10 // -3)
local v6 = ("beta" .. v3)
root_float = (root_float / 5.0)
local v7 = (root_float / -3.0)
local v8 = 0
while v8 < 2 do
  if (not false) then
    local v9 = v3
    if (-3.57 > 9.19) then
      if (root_int == 3) then
        if (root_int >= v8) then
          local v10 = ("beta" == "echo")
          local v11 = 0
          while v11 < 5 do
            v5[5] = (15 - 10)
            v11 = v11 + 1
          end
          local v12 = (v7 + -1.86)
        else
          local v13 = (v7 + v7)
          if (true ~= root_bool) then
            local v14 = v9
          else
            local v15 = v2[5]
            root_bool = (16.98 < v13)
            if (v15 < root_int) then
              root_bool = (v13 ~= -19.61)
              root_float = (-17.98 * v7)
              v3 = ("echo" .. v3)
            end
          end
        end
        local v16 = 9
      end
      if ("echo" == v9) then
        if (not root_bool) then
          local v17 = (-19.59 % 3.0)
        else
          local v18 = v0[0]
          local v19 = (-9.42 % 2.0)
          root_bool = (root_bool ~= root_bool)
        end
        v5[5] = v2[4]
        v2[2] = v2[4]
      end
    else
      local v20 = (1.9 > 9.67)
    end
    local v21 = (true == root_bool)
  end
  local v22 = (v7 - v7)
  v1[2] = v1[5]
  v8 = v8 + 1
end
v1[3] = (root_int % -3)
local v23 = (-9 - -20)
local v24 = (root_int % 5)
if (3.12 == root_float) then
  if (v3 == v3) then
    local v25 = (v7 + 16.98)
    v0[4] = (v25 + -3.42)
    if (v23 <= -4) then
      v2[1] = v5[0]
      v5[0] = root_int
      v1[3] = (-4 - -2)
    else
      v1[0] = (14 % 5)
    end
  end
  local v26 = {}
  v26[0] = ("beta" .. v3)
end
root_bool = ("alpha" ~= "alpha")
v2[4] = (v4 + -16)
v0[4] = (17.14 / 5.0)
local v27 = (v7 - root_float)
local v28 = 0
while v28 < 4 do
  v0[5] = (-11.74 / 3.0)
  v1[4] = v24
  v5[2] = 19
  v28 = v28 + 1
end
root_str = ("beta" .. "beta")
print("final", root_int, root_float, root_str, root_bool, v3, v4, v6, v7, v23, v24)
