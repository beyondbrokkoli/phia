#!/usr/bin/env bash

# phia — regression suite (bugs #1–#13 + anti-amputation guard)
#
# Every case compiles through the REAL pipeline (build.rs: lexer -> parser ->
# checker -> lowerer -> optimizer -> rust codegen -> rustc) in release mode
# and executes the baked binary. Four assertion layers:
#   values   — final table state (LEN / NZ / position-weighted CHECKSUM)
#   stats    — optimizer fast-path counts and hoist placement (hoist_ctx);
#              safety fixes must not kill the fast path, and the fast path
#              must not lie about safety
#   timeouts — build timeout = compiler hang is a failure; run timeout =
#              runtime hang is a failure
#   loudness — lexer errors must FAIL the build with a message

set -u
cd "$(dirname "$0")/.."

BIN=target/release/phia
OUT=target/run_out.txt; ERR=target/run_err.txt; BERR=target/build_err.txt
CASES=target/cases; mkdir -p "$CASES"

G=$'\e[32m'; R=$'\e[31m'; N=$'\e[0m'
declare -i PASS=0 FAIL=0; declare -a FAILED=()

compile()  { PHIA_SOURCE="$1" timeout 120 cargo build --release --quiet 2>"$BERR"; }
execute()  { timeout 60 "$BIN" >"$OUT" 2>"$ERR"; }
tblcount() { grep -c '^TABLE ' "$OUT"; }
stat()     { grep -m1 -oE "$1=[0-9,]+" "$OUT" | cut -d= -f2; }
line_ok()  { grep -qx "$1" "$OUT"; }

pass() { printf ' %b✓%b %s\n' "$G" "$N" "$1"; PASS+=1; }
fail() { printf ' %b✗%b %s\n' "$R" "$N" "$1"; FAIL+=1; FAILED+=("$1"); }

check() {  # name file expectation...  ("TABLE ..." | "NTABLES n" | "key=val")
  local name=$1 file=$2; shift 2
  compile "$file" || { fail "$name — build: $(tail -n1 "$BERR")"; return; }
  execute    || { fail "$name — run: $(tail -n1 "$ERR")"; return; }
  local e k v
  for e in "$@"; do
    if [[ $e == TABLE* ]]; then
      line_ok "$e" || { fail "$name — want line: $e"; return; }
    elif [[ $e == NTABLES* ]]; then
      [ "$(tblcount)" = "${e#NTABLES }" ] || { fail "$name — want ${e}, got $(tblcount)"; return; }
    else
      k=${e%%=*}; v=${e#*=}
      [ "$(stat "$k")" = "$v" ] || { fail "$name — $k: want $v, got $(stat "$k")"; return; }
    fi
  done
  pass "$name"
}

check_panic() {  # name file message
  compile "$2" || { fail "$1 — build failed"; return; }
  execute && { fail "$1 — expected panic, exited 0"; return; }
  grep -q "$3" "$ERR" && pass "$1" || fail "$1 — no '$3' on stderr"
}

check_build_fail() {  # name file message
  compile "$2" && { fail "$1 — expected build failure"; return; }
  grep -q "$3" "$BERR" && pass "$1" || fail "$1 — no '$3': $(tail -n1 "$BERR")"
}

# --- bug 01: literal loop bound must not be hoisted
cat > "$CASES/bug01.lua" <<'LUA'
local a = {}
local i = 0
while i < 1000 do
    a[i] = 7
    i = i + 1
end
LUA
check bug01 "$CASES/bug01.lua" \
  "TABLE 0 LEN 1000 NZ 1000 CHECKSUM 3503500" "fast_sets=0"

# --- bug 02: alias must die on redefinition
cat > "$CASES/bug02.lua" <<'LUA'
local a = {}
local i = 0
while i < 10 do
    local j = i
    j = j + 300
    a[j] = 1
    i = i + 1
end
LUA
check bug02 "$CASES/bug02.lua" \
  "TABLE 0 LEN 310 NZ 10 CHECKSUM 3055" "fast_sets=0"

# --- bug 03: non-invariant limit must disable hoisting
cat > "$CASES/bug03.lua" <<'LUA'
local a = {}
local n = 200
local i = 0
while i < n do
    a[i] = 1
    n = n + 1
    i = i + 2
end
LUA
check bug03 "$CASES/bug03.lua" \
  "TABLE 0 LEN 399 NZ 200 CHECKSUM 40000" "fast_sets=0"

# --- bug 04: aliased table + dynamic growth in nested loop
cat > "$CASES/bug04.lua" <<'LUA'
local a = {}
local b = a
local n = 1000
local i = 0
while i < 10 do
    local j = 0
    while j < n do
        a[j] = j
        b[i] = i
        j = j + 1
    end
    i = i + 1
end
LUA
check bug04 "$CASES/bug04.lua" \
  "TABLE 0 LEN 1000 NZ 999 CHECKSUM 333333000" "fast_sets=0" "hoists=0"

# --- bug 05: negative index must panic loudly
cat > "$CASES/bug05a.lua" <<'LUA'
local a = {}
local i = 0
while i < 10 do
    a[i] = i
    i = i + 1
end
local k = 0 - 1
a[k] = 5
LUA
check_panic bug05a_write "$CASES/bug05a.lua" "Negative"

cat > "$CASES/bug05b.lua" <<'LUA'
local a = {}
local k = 0 - 3
local x = a[k]
LUA
check_panic bug05b_read "$CASES/bug05b.lua" "Negative"

# --- bug 06: OOB reads return 0
cat > "$CASES/bug06.lua" <<'LUA'
local a = {}
local x = a[3]
local b = {}
local i = 0
while i < 5 do
    b[i] = i + 100
    i = i + 1
end
local y = b[10]
local z = b[4]
local out = {}
out[0] = x
out[1] = y
out[2] = z
LUA
check bug06 "$CASES/bug06.lua" \
  "NTABLES 3" \
  "TABLE 0 LEN 0 NZ 0 CHECKSUM 0" \
  "TABLE 1 LEN 5 NZ 5 CHECKSUM 1540" \
  "TABLE 2 LEN 3 NZ 1 CHECKSUM 312"

# --- bug 07: table created inside the loop
cat > "$CASES/bug07a.lua" <<'LUA'
local i = 0
while i < 10 do
    local t = {}
    t[i] = 1
    i = i + 1
end
LUA
exp=("NTABLES 10" "fast_sets=0")
for k in $(seq 0 9); do exp+=("TABLE $k LEN $((k+1)) NZ 1 CHECKSUM $((k+1))"); done
check bug07a_alloc_in_loop "$CASES/bug07a.lua" "${exp[@]}"

cat > "$CASES/bug07b.lua" <<'LUA'
local t = {}
local n = 10
local i = 0
while i < n do
    t = {}
    t[i] = 1
    i = i + 1
end
LUA
exp=("NTABLES 11" "TABLE 0 LEN 0 NZ 0 CHECKSUM 0")
for k in $(seq 1 10); do exp+=("TABLE $k LEN $k NZ 1 CHECKSUM $k"); done
check bug07b_rebind "$CASES/bug07b.lua" "${exp[@]}"

# --- bug 08: one table, mixed fast/dynamic writes
cat > "$CASES/bug08.lua" <<'LUA'
local a = {}
local i = 0
while i < 10 do
    a[i] = 1
    local k = i + 300
    a[k] = 2
    i = i + 1
end
LUA
check bug08 "$CASES/bug08.lua" \
  "TABLE 0 LEN 310 NZ 20 CHECKSUM 6165" "fast_sets=0"

# --- bug 09: nesting
cat > "$CASES/bug09.lua" <<'LUA'
local size = 2000
local data_a = {}
local data_b = {}
local i = 0
while i < size do
    data_a[i] = i
    i = i + 1
end
local iter = 0
while iter < 500 do
    local idx = 0
    local crazy_math = 0 + 1 + 2 + 3 + 4 + 5 - 15 + iter
    while idx < size do
        local crazy_math = data_a[idx]
        local offset_idx = idx + 2
        data_b[offset_idx] = crazy_math - 1 + 1
        idx = idx + 1
    end
    iter = iter + 1
end
LUA
check bug09_torture "$CASES/bug09.lua" \
  "TABLE 0 LEN 2000 NZ 1999 CHECKSUM 2666666000" \
  "TABLE 1 LEN 2002 NZ 1999 CHECKSUM 2670664000" \
  "fast_sets=1" "fast_gets=1" "dyn_sets=1" "hoists=2" "hoist_ctx=0,1"

# --- bug 10: lexer errors must be loud
printf 'local x = 5 $\n'                > "$CASES/bug10a.lua"
check_build_fail bug10a_unknown_char "$CASES/bug10a.lua" "Lexer Error"

sed 's/$/\r/' "$CASES/bug01.lua"        > "$CASES/bug10b.lua"
check bug10b_crlf "$CASES/bug10b.lua" \
  "TABLE 0 LEN 1000 NZ 1000 CHECKSUM 3503500"

printf 'local big = 99999999999999999999\n' > "$CASES/bug10c.lua"
check_build_fail bug10c_int_overflow "$CASES/bug10c.lua" "Lexer Error"

# --- bug 11: table cross-assignment must not hang the build
cat > "$CASES/bug11.lua" <<'LUA'
local a = {}
local b = a
a = b
local n = 10
local i = 0
while i < n do
    a[i] = 1
    i = i + 1
end
LUA
check bug11 "$CASES/bug11.lua" \
  "TABLE 0 LEN 10 NZ 10 CHECKSUM 55" "fast_sets=0" "hoists=0"

# --- bug 12: register Move-targeted twice
cat > "$CASES/bug12.lua" <<'LUA'
local a = {}
local c = {}
local n = 10
local i = 0
while i < n do
    local b = a
    local k = i + 300
    b[k] = 1
    a[i] = 1
    b = c
    i = i + 1
end
LUA
check bug12 "$CASES/bug12.lua" \
  "NTABLES 2" \
  "TABLE 0 LEN 310 NZ 20 CHECKSUM 3110" \
  "TABLE 1 LEN 0 NZ 0 CHECKSUM 0" "fast_sets=0"

# --- bug 13: scan/rewrite alias divergence
cat > "$CASES/bug13.lua" <<'LUA'
local a = {}
local n = 10
local i = 0
while i < n do
    a[i] = i + 300
    local j = i
    j = a[j]
    a[j] = 1
    i = i + 1
end
LUA
check bug13 "$CASES/bug13.lua" \
  "TABLE 0 LEN 310 NZ 20 CHECKSUM 19885" "fast_sets=0"

# --- anti-amputation: the optimizer must still FIRE
cat > "$CASES/optfast.lua" <<'LUA'
local a = {}
local n = 100000
local i = 0
while i < n do
    a[i] = i
    i = i + 1
end
LUA
check optfast "$CASES/optfast.lua" \
  "TABLE 0 LEN 100000 NZ 99999 CHECKSUM 333333333300000" \
  "fast_sets=1" "hoists=1" "hoist_ctx=0"

printf '\npassed: %d   failed: %d\n' "$PASS" "$FAIL"
if (( FAIL > 0 )); then printf 'FAILED: %s\n' "${FAILED[*]}"; exit 1; fi
echo "ALL GREEN — 13 bugs, 18 cases, full pipeline"

echo; echo "== showcase: main.lua (full torture workload) =="
compile main.lua && execute && grep -E '^(TABLE|STATS|TIME)' "$OUT"
