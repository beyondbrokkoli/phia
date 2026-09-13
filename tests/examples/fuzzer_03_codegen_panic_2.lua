-- FUZZER-PIN 3/3 · seed 157 (deep variant: seed 136 = fuzzer_02) · fixed 2026-09-11 · STRUCTURED CODEGEN
--
-- SYMPTOM: BUILD fails — build.rs panics
--   "structured codegen: header N reached twice" (src/backend.rs, emit_loop).
--   Not a wrong-output bug: the emission tree-walk entered one block twice.
--
-- TRAP: the Lua source is valid; a `while` containing plain nested if/else
--   is legal everywhere. The CFG walker misgraded structure, nothing else.
--
-- ROOT CAUSE: the lowerer mints if-arm blocks (and nested ifs' joins)
--   AFTER the enclosing join, so a nested if's inner join jumps BACKWARD in
--   id space to the outer join — a "back edge" in ids, but not a loop.
--   is_loop_header's old guard rejected those by reachability ("an
--   if-join's jumper sits in a sibling arm nothing downstream reaches") —
--   sound OUTSIDE loops, but inside a while everything downstream reaches
--   everything (join → latch → back edge → whole body), so every nested-if
--   join INSIDE a loop graded as a "loop header". emit_loop then wrapped it
--   as `while` and the walk re-entered it later → panic.
--
-- FIX (grep handles): src/backend.rs — is_loop_header now applies the
--   natural-loop definition: back edge a → h requires h to DOMINATE a
--   (new dominates() helper: a dominates b IFF b is unreachable from entry
--   once a is removed — "iff" is load-bearing shorthand, keep it). A real
--   while header dominates its latch; a join never dominates its own arm's
--   nested join. Emission-side only: is_loop_header is called solely by
--   emit_seq/emit_loop (optimize() has its own scan), so the IR and all
--   lock baselines are untouched by this fix.
--
-- RE-VERIFY IF THIS BREAKS:
--   touch this file && PHIA_DEBUG_DUMP=mid PHIA_SOURCE=tests/examples/fuzzer_03_codegen_panic_2.lua cargo build --release
--   In ir_dispatched.txt find the panicked-about block N: it is an if-join
--   (ends "branch cond ? x : y", receives a goto from a LATER block that N
--   does not dominate) yet got classified as a header — dominance broke.
--
-- EXPECT: PROBE final: i_r0=19 f_r184=2.0 s_r172="echo" b_r4=true b_r126=false s_r170="betaecho" s_r171="betaechobetaecho" i_r143=9 f_r187=4.0 i_r154=-2
-- EXPECT: TABLE 0 LEN 6 NZ 1 CHECKSUM 38
-- EXPECT: TABLE 1 LEN 4 NZ 3 CHECKSUM 66
-- EXPECT: TABLE 2 LEN 1 NZ 0 CHECKSUM 0 SUM 0
-- EXPECT: TABLE 3 LEN 1 NZ 0 CHECKSUM 0 SUM 0
-- EXPECT: TABLE 4 LEN 1 NZ 0 CHECKSUM 0 SUM 0
-- EXPECT: TABLE 5 LEN 1 NZ 0 CHECKSUM 0 SUM 0
-- EXPECT: TABLE 6 LEN 1 NZ 1 CHECKSUM -7587524682717941514
-- EXPECT: fast_sets=0
-- EXPECT: fast_gets=0
-- EXPECT: dyn_sets=16
-- EXPECT: dyn_gets=1
-- EXPECT: hoists=0

local root_int = 19
local root_float = -0.36
local root_str = "beta"
local root_bool = true
local v0 = {}
v0[0] = (root_int + root_int)
local v1 = {}
v1[0] = 5
root_str = ("alpha" .. "alpha")
root_str = "echo"
local v2 = 0
while v2 < 4 do
  if (12 == -12) then
    v1[1] = (-12 // -2)
    if ("alpha" ~= root_str) then
      v0[5] = (12 // 5)
      v0[5] = (v2 - 5)
      v1[0] = (root_int % -2)
    end
    if (6 <= root_int) then
      if (root_float <= root_float) then
        local v3 = (v2 - v2)
        if (-3.34 <= 5.79) then
          if (root_str == "alpha") then
            local v4 = root_str
          end
        else
          local v5 = {}
          v5[0] = "delta"
          v2 = (4 + v3)
        end
        root_float = (root_float - -8.75)
      else
        v1[3] = (-10 // -2)
      end
      if (root_float < 6.63) then
        local v6 = {}
        v6[0] = 14
        if (root_int >= v2) then
          local v7 = {}
          v7[0] = (-15 // 4)
          root_float = (-10.1 / -2.0)
        end
        local v8 = {}
        v8[0] = "echo"
      end
    end
  else
    local v9 = (root_float - root_float)
    local v10 = {}
    v10[0] = (v9 - v9)
  end
  v2 = v2 + 1
end
local v11 = (not root_bool)
local v12 = ("beta" .. root_str)
local v13 = (v12 .. v12)
root_float = (-4.28 // -2.0)
v1[3] = (3 % 2)
local v14 = (root_int + -10)
local v15 = (root_float - root_float)
v15 = (-12.96 // 5.0)
local v16 = (root_int % -3)
v0[5] = v0[1]
v15 = (14.9 // 3.0)
v1[2] = root_int
local v17 = {}
v17[0] = (v12 .. v13)
print("final", root_int, root_float, root_str, root_bool, v11, v12, v13, v14, v15, v16)
