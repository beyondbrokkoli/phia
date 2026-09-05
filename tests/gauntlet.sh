#!/usr/bin/env bash
# =============================================================================
# tests/gauntlet.sh — bisect harness for the full workload.
#
# pass 1: each phase in isolation (values asserted — semantic ground truth)
# pass 2: cumulative prefixes (catches cross-phase interference: stale hoisted
#         pointers, cross-candidate EC/HR interactions)
# The last prefix IS the full gauntlet (== main.lua).
#
# For symbolized backtraces (the segfault's exact line in baked_native.rs!),
# add to Cargo.toml:   [profile.release]  debug = 1
# =============================================================================
set -u
cd "$(dirname "$0")/.."
BIN=target/release/phia
P=tests/phases; G=target/gauntlet
mkdir -p "$P" "$G"
OUT=$G/out.txt; ERR=$G/err.txt; BERR=$G/build_err.txt
GRN=$'\e[32m'; RED=$'\e[31m'; RST=$'\e[0m'
declare -i PASS=0 FAIL=0

# --- phase sources (the Gauntlet, split; regenerate main.lua with:
#     cat tests/phases/p*.lua > main.lua) ---
cat > "$P/pA.lua" <<'LUA'
local pa_t = {}
local pa_n = 400
local pa_i = 0
while pa_i < pa_n do
    pa_t[pa_i] = pa_i + 1
    pa_i = pa_i + 2
end
LUA
cat > "$P/pB.lua" <<'LUA'
local pb_t = {}
local pb_n = 300
local pb_i = 0
while pb_i < pb_n do
    local pb_j = 0
    while pb_j < pb_i do
        pb_t[pb_j] = pb_j + 1
        pb_j = pb_j + 1
    end
    pb_i = pb_i + 1
end
LUA
cat > "$P/pC.lua" <<'LUA'
local pc_t = {}
local pc_n = 250
local pc_i = 0
while pc_i < pc_n do
    pc_t[pc_i] = 5
    pc_i = pc_i + 1
end
local pc_j = 0
while pc_j < pc_n do
    pc_t[pc_j] = pc_t[pc_j] + 1
    pc_j = pc_j + 1
end
LUA
cat > "$P/pD.lua" <<'LUA'
local pd_t = {}
local pd_fill = 0
while pd_fill < 30 do
    pd_t[pd_fill] = 1
    pd_fill = pd_fill + 1
end
pd_t[30] = 60
local pd_lim = 60
local pd_i = 0
while pd_t[pd_i] < pd_lim do
    pd_i = pd_i + 1
end
local pd_w = {}
pd_w[0] = pd_i
LUA
cat > "$P/pE.lua" <<'LUA'
local pe_t = {}
local pe_n = 200
local pe_m = 3
local pe_i = 0
while pe_i < pe_n do
    local pe_d = pe_i
    local pe_j = 0
    while pe_j < pe_m do
        pe_t[pe_d] = 7
        pe_j = pe_j + 1
    end
    pe_i = pe_i + 1
end
LUA
cat > "$P/pF.lua" <<'LUA'
local pf_t = {}
local pf_n = 150
local pf_i = 0
while pf_i < pf_n do
    local pf_d = pf_i
    while pf_d < pf_n do
        pf_t[pf_d] = pf_d + 1
        pf_d = pf_d + 1
    end
    pf_i = pf_i + 1
end
LUA
cat > "$P/pG.lua" <<'LUA'
local pg_src = {}
local pg_dst = {}
local pg_n = 180
local pg_i = 0
while pg_i < pg_n do
    pg_src[pg_i] = pg_i + 1
    pg_i = pg_i + 1
end
local pg_j = 0
while pg_j < pg_n do
    local pg_rev = pg_n - 1 - pg_j
    pg_dst[pg_rev] = pg_src[pg_j]
    pg_j = pg_j + 1
end
LUA
cat > "$P/pH.lua" <<'LUA'
local ph_t = {}
local ph_n = 120
local ph_i = 0
while ph_i < ph_n do
    ph_t[ph_i] = ph_i + 1
    ph_i = ph_i + 1
end
local ph_sum = 0
local ph_j = 0
while ph_j < ph_n do
    ph_sum = ph_sum + ph_t[ph_j]
    ph_j = ph_j + 1
end
local ph_w = {}
ph_w[0] = ph_sum
LUA
cat > "$P/pI.lua" <<'LUA'
local pi_t = {}
local pi_flag = 0 < 1
local pi_k = 0
while pi_flag do
    pi_t[pi_k] = 100
    pi_flag = 0 < 0
    pi_k = pi_k + 1
end
LUA
cat > "$P/pJ.lua" <<'LUA'
local pj_t = {}
local pj_hi1 = 200
local pj_i = 100
while pj_i < pj_hi1 do
    pj_t[pj_i] = pj_i - 100
    pj_i = pj_i + 1
end
local pj_hi2 = 350
local pj_j = 200
while pj_j < pj_hi2 do
    pj_t[pj_j] = pj_j
    pj_j = pj_j + 1
end
local pj_w = {}
pj_w[0] = pj_t[150]
pj_w[1] = pj_t[250]
pj_w[2] = pj_t[349]
pj_w[3] = pj_t[99]
LUA
cat > "$P/pK.lua" <<'LUA'
local pk_t = {}
local pk_n = 60
local pk_i = 0
while pk_i < pk_n do
    local pk_j = 0
    while pk_j < pk_n do
        local pk_k = 0
        while pk_k < pk_n do
            pk_t[pk_k] = pk_k + pk_j
            pk_k = pk_k + 1
        end
        pk_j = pk_j + 1
    end
    pk_i = pk_i + 1
end
LUA
cat > "$P/pL.lua" <<'LUA'
local pl_t = {}
local pl_n = 80
local pl_i = 0
while pl_i < pl_n do
    local pl_a = pl_i
    pl_t[pl_a] = 1
    local pl_b = pl_a
    pl_b = pl_b + 0
    pl_t[pl_b] = 2
    local pl_c = pl_i
    pl_t[pl_c] = 3
    pl_i = pl_i + 1
end
LUA
cat > "$P/pM.lua" <<'LUA'
local pm_a = {}
local pm_b = pm_a
local pm_n = 90
local pm_i = 0
while pm_i < pm_n do
    pm_a[pm_i] = pm_i + 1
    pm_b[pm_i] = pm_i + 2
    pm_i = pm_i + 1
end
LUA

# --- expected values (verified semantics; local table ids) ---
cat > "$P/expect_A.txt" <<'EOF'
TABLE 0 LEN 400 NZ 200 CHECKSUM 10666600
EOF
cat > "$P/expect_B.txt" <<'EOF'
TABLE 0 LEN 299 NZ 299 CHECKSUM 8955050
EOF
cat > "$P/expect_C.txt" <<'EOF'
TABLE 0 LEN 250 NZ 250 CHECKSUM 188250
EOF
cat > "$P/expect_D.txt" <<'EOF'
TABLE 0 LEN 31 NZ 31 CHECKSUM 2325
TABLE 1 LEN 1 NZ 1 CHECKSUM 30
EOF
cat > "$P/expect_E.txt" <<'EOF'
TABLE 0 LEN 200 NZ 200 CHECKSUM 140700
EOF
cat > "$P/expect_F.txt" <<'EOF'
TABLE 0 LEN 150 NZ 150 CHECKSUM 1136275
EOF
cat > "$P/expect_G.txt" <<'EOF'
TABLE 0 LEN 180 NZ 180 CHECKSUM 1960230
TABLE 1 LEN 180 NZ 180 CHECKSUM 988260
EOF
cat > "$P/expect_H.txt" <<'EOF'
TABLE 0 LEN 120 NZ 120 CHECKSUM 583220
TABLE 1 LEN 1 NZ 1 CHECKSUM 7260
EOF
cat > "$P/expect_I.txt" <<'EOF'
TABLE 0 LEN 1 NZ 1 CHECKSUM 100
EOF
cat > "$P/expect_J.txt" <<'EOF'
TABLE 0 LEN 350 NZ 249 CHECKSUM 12453250
TABLE 1 LEN 4 NZ 3 CHECKSUM 1597
EOF
cat > "$P/expect_K.txt" <<'EOF'
TABLE 0 LEN 60 NZ 60 CHECKSUM 179950
EOF
cat > "$P/expect_L.txt" <<'EOF'
TABLE 0 LEN 80 NZ 80 CHECKSUM 9720
EOF
cat > "$P/expect_M.txt" <<'EOF'
TABLE 0 LEN 90 NZ 90 CHECKSUM 251160
EOF

PHASES=(A B C D E F G H I J K L M)
OFFS=(0 1 2 3 5 6 7 9 11 12 14 15 16)   # cumulative NewTables before each phase

compile() { PHIA_SOURCE="$1" timeout 120 cargo build --release --quiet 2>"$BERR"; }
runbin()  { timeout 60 "$BIN" >"$OUT" 2>"$ERR"; }

gdb_bt() {
  if command -v gdb >/dev/null 2>&1; then
    gdb -batch -ex run -ex bt "$BIN" 2>/dev/null | grep -E '^\#' | head -15
  else echo "      (gdb not found — install for a backtrace)"; fi
}

crash_report() { # label rc
  printf ' %b✗%b %s — CRASH (exit %s)\n' "$RED" "$RST" "$1" "$2"
  echo "      stderr: $(head -c 400 "$ERR" | tr '\n' ' ')"
  grep -m1 '^STATS' "$OUT" 2>/dev/null | sed 's/^/      /' || echo "      (no STATS — crashed before completion)"
  echo "      baked:  target/release/build/phia-*/out/baked_native.rs"
  echo "      backtrace (line numbers = baked_native.rs lines!):"
  gdb_bt | sed 's/^/        /'
}

run_case() { # label file expectfile (absolute table ids)
  local label=$1 file=$2 exp=$3 line rc ok=1
  if ! compile "$file"; then
    printf ' %b✗%b %s — BUILD FAILED: %s\n' "$RED" "$RST" "$label" "$(tail -n1 "$BERR")"
    FAIL+=1; return 1
  fi
  runbin; rc=$?
  if   (( rc == 124 )); then printf ' %b✗%b %s — TIMEOUT (infinite loop)\n' "$RED" "$RST" "$label"; FAIL+=1; return 1
  elif (( rc == 139 )); then crash_report "$label (SIGSEGV)" "$rc"; FAIL+=1; return 1
  elif (( rc == 134 )); then crash_report "$label (SIGABRT)" "$rc"; FAIL+=1; return 1
  elif (( rc != 0   )); then printf ' %b✗%b %s — exit %s; stderr: %s\n' "$RED" "$RST" "$label" "$rc" "$(tail -n1 "$ERR")"; FAIL+=1; return 1
  fi
  while IFS= read -r line; do
    [ -z "$line" ] && continue
    if ! grep -qx "$line" "$OUT"; then
      local id=${line%% LEN*}
      printf ' %b✗%b %s — want: %s; got: %s\n' "$RED" "$RST" "$label" "$line" \
        "$(grep -m1 "^$id " "$OUT" || echo MISSING)"
      ok=0
    fi
  done < "$exp"
  if (( ok )); then printf ' %b✓%b %s\n' "$GRN" "$RST" "$label"; PASS+=1
  else FAIL+=1; fi
}

echo "== pass 1: phases in isolation =="
for k in "${!PHASES[@]}"; do
  ph=${PHASES[$k]}
  # NOTE: phase I failing with "LEN 0" is PREDICTED — that is bug #16a
  # (resolve_phis types every phi Move as Integer; boolean phis never reach
  # b_r, so the flag loop never executes). Wrong values, not the segfault.
  run_case "phase $ph" "$P/p$ph.lua" "$P/expect_$ph.txt"
done

echo; echo "== pass 2: cumulative prefixes (last one == full gauntlet) =="
: > "$G/prefix.lua"; : > "$G/prefix_expect.txt"
for k in "${!PHASES[@]}"; do
  ph=${PHASES[$k]}; off=${OFFS[$k]}
  cat "$P/p$ph.lua" >> "$G/prefix.lua"
  awk -v off="$off" '$1=="TABLE" {$2 += off} {print}' "$P/expect_$ph.txt" >> "$G/prefix_expect.txt"
  run_case "prefix A..$ph" "$G/prefix.lua" "$G/prefix_expect.txt"
done

printf '\npassed: %d  failed: %d\n' "$PASS" "$FAIL"
(( FAIL > 0 )) && exit 1
echo "regenerate the showcase with: cat tests/phases/p*.lua > main.lua"
