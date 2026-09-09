#!/usr/bin/env lua
-- tests/run_boss.lua — THE BOSS: complete invariant analysis in one run.
--
-- One file, one source of truth for every test listing. Four disciplines:
--   POSITIVE  build ok -> run ok -> every EXPECT pin exact
--             (TABLE = whole line, NTABLES = counted, k=v = stats, else substring)
--   NEGATIVE  build MUST fail -> every EXPECT_BUILD_FAIL message present
--   PANIC     build ok -> run MUST fail -> the single EXPECT_PANIC pin must
--             EQUAL the program's 'Runtime Error: ...' stderr line exactly,
--             and the panic must originate inside baked code (the panic's
--             location line must name baked_native.rs — inlining-proof)
--   LOCK      byte-diff of every generated baked_native.rs against tests/lock/
--
-- The boss NEVER writes to tests/lock/ — re-issuing a baseline belongs to
-- milestone_lockdown.lua alone. Scratch output goes to tests/diff/ only.
--
-- Listing hygiene (the possibility matrix, enforced up front):
--   * a name duplicated in one list, or present in two lists      -> FAIL
--   * a listed file whose magic comments contradict its list      -> FAIL
--   * example files owned by no list                              -> notice
--   * positive tests without a lock baseline                      -> notice
--
-- Exit code 1 on any failure. Notices never fail the run.

dofile("tests/helpers.lua")

local LOCK_DIR = "tests/lock"
local DIFF_DIR = "tests/diff"

-- THE LISTINGS — single source of truth for the whole suite.
local POSITIVE = {
    "bug_01.lua",
    "bug_02.lua",
    "bug_03.lua",
    "bug_04.lua",
    "bug_06.lua",
    "bug_07a.lua",
    "bug_07b.lua",
    "bug_08.lua",
    "bug_09.lua",
    "bug_10b.lua",
    "bug_11.lua",
    "bug_12.lua",
    "bug_13.lua",
    "optfast.lua",
    "bug_16a.lua",
    "bug_16b.lua",
    "bug_17.lua",
    "bug_18.lua",
    "gauntlet_pA.lua",
    "gauntlet_pB.lua",
    "gauntlet_pC.lua",
    "gauntlet_pD.lua",
    "gauntlet_pE.lua",
    "gauntlet_pF.lua",
    "gauntlet_pG.lua",
    "gauntlet_pH.lua",
    "gauntlet_pI.lua",
    "gauntlet_pJ.lua",
    "gauntlet_pK.lua",
    "gauntlet_pL.lua",
    "gauntlet_pM.lua",
    "opt_literal_bound.lua",
    "region_table_resize.lua",
    "hoist_ctx_showcase.lua",
    "tier2_reg_limit.lua",
    "firewall_abort_all.lua",
    "firewall_global_offset.lua",
    "firewall_neg_offset.lua",
    "gauntlet_main.lua",
    "nested_01_success.lua",
    "nested_05_dyn_loop.lua",
    "nested_06_read_before_write.lua",
    "nested_07_read_then_table_store.lua",
    "nested_08_fresh_store.lua",
    "nested_10_alias_shared.lua",
    "nested_15_read_then_fresh.lua",
    "float_01_store_read.lua",
    "float_02_loop_gate.lua",
    "float_06_lazy_read.lua",
    "float_09_fast_rw.lua",
    "float_10_handle_mode.lua",
    "tier4_01_flat_plus_nested.lua",
    "tier4_02_nested_read.lua",
    "tier4_03_rebind_decline.lua",
    "tier4_04_zero_iter.lua",
    "tier4_06_float_child.lua",
    "tier4_07_reg_limit_decline.lua",
    "tier4_08_zero_trip_init.lua",
    "tier4_09_multihop.lua",
    "tier4_10_matrix.lua",
    "tier4_11_matrix_float.lua",
    "tier4_12_unprovable_limit.lua",
}
local NEGATIVE = {
    "bug_10a.lua",
    "bug_10c.lua",
    "nested_03_type_error.lua",
    "nested_04_alias_div.lua",
    "nested_11_target_scalar.lua",
    "nested_12_index_non_table.lua",
    "nested_13_recursive_type.lua",
    "nested_14_bool_element.lua",
    "float_03_mixed_arith.lua",
    "float_04_mixed_store.lua",
    "float_05_float_index.lua",
    "float_07_while_cond.lua",
    "float_08_assign_mismatch.lua",
    "float_13_deferred_coercion.lua",
}
local PANIC = {
    "bug_05a.lua",
    "bug_05b.lua",
    "firewall_neg_read_panic.lua",
    "firewall_neg_init_panic.lua",
    "float_11_neg_index_panic.lua",
    "float_12_neg_init_fast_panic.lua",
    "nested_02_lvalue_type.lua",
    "nested_09_nil_panic.lua",
    "tier4_05_nil_child_panic.lua",
}

-- ---------------------------------------------------------------- reporting
local GREEN, RED, YELLOW = "\27[32m", "\27[31m", "\27[33m"
local function c(code, s) return code .. s .. "\27[0m" end

local failed, notices = {}, {}
local function report_fail(section, name, reason)
    table.insert(failed, { section = section, name = name })
    print(string.format("  %s %s — %s", c(RED, "✗"), name, reason))
end
local function report_pass(name, extra)
    print(string.format("  %s %s%s", c(GREEN, "✓"), name, extra and ("   " .. extra) or ""))
end
local function report_notice(text)
    table.insert(notices, text)
    print(string.format("  %s %s", c(YELLOW, "!"), text))
end

-- ---------------------------------------------------------------- machinery
local function build_ok(src)
    -- Bump the source mtime first: cargo's rerun triggers are mtime-based,
    -- and a same-path content swap with an older mtime would silently skip
    -- build.rs. The stamp in main.rs catches any survivor loudly; this
    -- touch makes sure the rebuild actually happens in the first place.
    os.execute("touch '" .. src .. "'")
    local cmd = string.format("PHIA_SOURCE='%s' timeout 120 cargo build --release --quiet 2> %s", src, BERR)
    local res = os.execute(cmd)
    return res == 0 or res == true
end

local function run_ok(with_backtrace)
    local bt = with_backtrace and "RUST_BACKTRACE=1 " or ""
    local cmd = string.format("%stimeout 60 %s > %s 2> %s", bt, BIN, OUT, ERR)
    local res = os.execute(cmd)
    return res == 0 or res == true
end

local function capture_baked()
    local baked = find_baked()
    if not baked then return nil end
    local code = read_file(baked)
    if code:find("pub fn run_baked", 1, true) and code:find("pub const STATS", 1, true) then
        return code
    end
    return nil
end

-- Every user-facing runtime error starts with 'Runtime Error: ' (backend.rs
-- codegen contract). The differing tails are load-bearing: they encode
-- dyn-path vs fast-path origin. Extract whole lines by that prefix.
local function runtime_error_lines(s)
    local t = {}
    for line in s:gmatch("([^\r\n]+)") do
        if line:sub(1, 15) == "Runtime Error: " then table.insert(t, line) end
    end
    return t
end

local function first_nonempty(s)
    return s:match("([^\r\n]+)") or "(empty)"
end

local function cap(s, n)
    local lines = {}
    for line in s:gmatch("[^\r\n]+") do table.insert(lines, line) end
    if #lines <= n then return s end
    return table.concat(lines, "\n", 1, n)
        .. string.format("\n       ... (%d more diff lines — full diff: tests/diff/ vs tests/lock/)", #lines - n)
end

-- ---------------------------------------------------------------- lock setup
local lock_manifest = nil
if io.open(LOCK_DIR .. "/manifest.lua", "r") then
    lock_manifest = dofile(LOCK_DIR .. "/manifest.lua")
end
os.execute("rm -rf " .. DIFF_DIR)
os.execute("mkdir -p " .. DIFF_DIR)

local lock_identical, lock_drifted, lock_errored = 0, 0, 0

local function lock_path(name) return LOCK_DIR .. "/" .. name:gsub("%.lua$", "") .. ".rs" end
local function diff_path(name) return DIFF_DIR .. "/" .. name:gsub("%.lua$", "") .. ".rs" end

-- Must be called immediately after a successful build of THIS test (the
-- capture is freshest-artifact based). Returns a display tag, or nil.
local function diff_against_lock(name)
    if not lock_manifest then return nil end
    local lf = io.open(lock_path(name), "r")
    if not lf then
        report_notice(name .. " — no lock baseline (new test? relock at the next milestone)")
        return "(no lock)"
    end
    lf:close()
    local code = capture_baked()
    if not code then
        lock_errored = lock_errored + 1
        report_fail("lock", name, "could not capture generated code")
        return nil
    end
    local f = io.open(diff_path(name), "w")
    f:write(code)
    f:close()
    local p = io.popen(string.format("diff -u '%s' '%s'", lock_path(name), diff_path(name)))
    local d = p:read("*a")
    p:close()
    if d == "" then
        lock_identical = lock_identical + 1
        return "lock ✓"
    end
    lock_drifted = lock_drifted + 1
    report_fail("lock", name, "BYTE DRIFT vs tests/lock/:\n" .. cap(d, 60))
    return "lock ✗"
end

-- ---------------------------------------------------------------- hygiene
print("== LISTING HYGIENE ==")
local owner = {}
local hygiene_bad = 0
for _, l in ipairs({ { label = "positive", files = POSITIVE },
                     { label = "negative", files = NEGATIVE },
                     { label = "panic",    files = PANIC } }) do
    local seen = {}
    for _, n in ipairs(l.files) do
        if seen[n] then
            report_fail("hygiene", n, "duplicated inside the " .. l.label .. " list")
            hygiene_bad = hygiene_bad + 1
        elseif owner[n] then
            report_fail("hygiene", n, "listed in both '" .. owner[n] .. "' and '" .. l.label .. "'")
            hygiene_bad = hygiene_bad + 1
        else
            owner[n] = l.label
            seen[n] = true
        end
    end
end
local p = io.popen("ls " .. EXAMPLES_DIR .. " 2>/dev/null")
for line in p:lines() do
    if line:sub(-4) == ".lua" and not owner[line] then
        report_notice("orphan test file (owned by no list): " .. EXAMPLES_DIR .. "/" .. line)
    end
end
p:close()
if hygiene_bad == 0 then print("  listings consistent") end

-- ---------------------------------------------------------------- positive
print("\n== POSITIVE (build, run, pins) ==")
local pos_passed = 0
for _, name in ipairs(POSITIVE) do
    local src = EXAMPLES_DIR .. "/" .. name
    local expects = parse_expects(src, "EXPECT")
    if #expects == 0 then
        report_fail("positive", name, "listed positive but has no '-- EXPECT:' pins")
    elseif parse_expects(src, "EXPECT_PANIC")[1] or parse_expects(src, "EXPECT_BUILD_FAIL")[1] then
        report_fail("positive", name, "carries EXPECT_PANIC/EXPECT_BUILD_FAIL pins — move it to the right list")
    elseif not build_ok(src) then
        report_fail("positive", name, "build failed: " .. first_nonempty(read_file(BERR)))
    else
        local lock_tag = diff_against_lock(name)
        if not run_ok(false) then
            local err = read_file(ERR)
            report_fail("positive", name, "run failed: " .. (runtime_error_lines(err)[1] or first_nonempty(err)))
        else
            local out = read_file(OUT)
            local bad
            for _, exp in ipairs(expects) do
                if exp:match("^TABLE ") then
                    if not has_exact_line(out, exp) then bad = "want line: " .. exp break end
                elseif exp:match("^NTABLES") then
                    local want = tonumber(exp:match("%d+"))
                    local got = 0
                    for line in out:gmatch("([^\r\n]+)") do
                        if line:match("^TABLE ") then got = got + 1 end
                    end
                    if got ~= want then bad = "want NTABLES " .. want .. ", got " .. got break end
                else
                    local k, v = exp:match("^([^=]+)=(.*)$")
                    if k and v then
                        local got = out:match(k .. "=([^;\r\n]+)")
                        if got ~= v then bad = k .. ": want " .. v .. ", got " .. tostring(got) break end
                    elseif not out:find(exp, 1, true) then
                        bad = "missing text: " .. exp break
                    end
                end
            end
            if bad then
                report_fail("positive", name, bad)
            else
                pos_passed = pos_passed + 1
                report_pass(name, lock_tag)
            end
        end
    end
end

-- ---------------------------------------------------------------- negative
print("\n== NEGATIVE (build must fail) ==")
local neg_passed = 0
for _, name in ipairs(NEGATIVE) do
    local src = EXAMPLES_DIR .. "/" .. name
    local wants = parse_expects(src, "EXPECT_BUILD_FAIL")
    if #wants == 0 then
        report_fail("negative", name, "listed negative but has no EXPECT_BUILD_FAIL pins")
    elseif parse_expects(src, "EXPECT")[1] or parse_expects(src, "EXPECT_PANIC")[1] then
        report_fail("negative", name, "carries EXPECT/EXPECT_PANIC pins — classification conflict")
    else
        os.execute("touch '" .. src .. "'") -- same mtime defense as build_ok
        local cmd = string.format("RUST_BACKTRACE=1 PHIA_SOURCE='%s' timeout 120 cargo build --release --quiet 2> %s", src, BERR)
        local res = os.execute(cmd)
        if res == 0 or res == true then
            report_fail("negative", name, "expected build failure, but it SUCCEEDED — the guard is gone")
        else
            local berr = read_file(BERR)
            local bad
            for _, msg in ipairs(wants) do
                if not berr:find(msg, 1, true) then bad = "missing expected error: " .. msg break end
            end
            if bad then
                report_fail("negative", name, bad)
            else
                neg_passed = neg_passed + 1
                report_pass(name, "[" .. first_nonempty(berr) .. "]")
            end
        end
    end
end

-- ---------------------------------------------------------------- panic
print("\n== PANIC (exact runtime error) ==")
local pan_passed = 0
for _, name in ipairs(PANIC) do
    local src = EXAMPLES_DIR .. "/" .. name
    local pins = parse_expects(src, "EXPECT_PANIC")
    if #pins == 0 then
        report_fail("panic", name, "listed panic but has no EXPECT_PANIC pin")
    elseif #pins > 1 then
        report_fail("panic", name, "a run can panic at most once — pin exactly one Runtime Error line")
    elseif pins[1]:sub(1, 15) ~= "Runtime Error: " then
        report_fail("panic", name, "malformed pin — must be the FULL line starting 'Runtime Error: '")
    elseif parse_expects(src, "EXPECT")[1] or parse_expects(src, "EXPECT_BUILD_FAIL")[1] then
        report_fail("panic", name, "carries EXPECT/EXPECT_BUILD_FAIL pins — classification conflict")
    elseif not build_ok(src) then
        report_fail("panic", name, "expected build to succeed: " .. first_nonempty(read_file(BERR)))
    elseif run_ok(true) then
        report_fail("panic", name, "expected runtime panic, but it ran clean")
    else
        local err = read_file(ERR)
        local rel = runtime_error_lines(err)
        local why
        if #rel == 0 then
            if err:find("optimizer invariant violated", 1, true) then
                why = "COMPILER TRIPWIRE fired (fast-path bounds check) — compiler bug, not a program error"
            else
                why = "no 'Runtime Error: ' line in stderr — not a language-level error: " .. first_nonempty(err)
            end
        elseif rel[1] ~= pins[1] then
            why = "message mismatch:\n       want: " .. pins[1] .. "\n       got : " .. rel[1]
        elseif not err:find("baked_native.rs", 1, true) then
            why = "panic origin not in baked code (location line missing baked_native.rs)"
        end
        if why then
            report_fail("panic", name, why)
        else
            pan_passed = pan_passed + 1
            report_pass(name, "[" .. rel[1] .. "]")
        end
    end
end

-- ---------------------------------------------------------------- lock wrap
print("\n== LOCK (byte-identity vs tests/lock/) ==")
if not lock_manifest then
    report_notice("no tests/lock/manifest.lua — lock section SKIPPED (milestone_lockdown.lua establishes baselines)")
else
    local pos_set = {}
    for _, n in ipairs(POSITIVE) do pos_set[n] = true end
    -- manifest entries outside the positive list (e.g. a future main.lua lock)
    -- still get verified — with their own build, since the positive pass
    -- didn't compile them.
    for _, name in ipairs(lock_manifest) do
        if not pos_set[name] then
            if not build_ok(EXAMPLES_DIR .. "/" .. name) then
                lock_errored = lock_errored + 1
                report_fail("lock", name, "build failed: " .. first_nonempty(read_file(BERR)))
            else
                diff_against_lock(name)
            end
        end
    end
    local expected_files = { ["manifest.lua"] = true }
    for _, n in ipairs(lock_manifest) do expected_files[n:gsub("%.lua$", "") .. ".rs"] = true end
    local p = io.popen("ls " .. LOCK_DIR .. " 2>/dev/null")
    for line in p:lines() do
        if not expected_files[line] then
            report_notice("stray lock file not in manifest: " .. LOCK_DIR .. "/" .. line)
        end
    end
    p:close()
    print(string.format("  lock: %d identical / %d drifted / %d errored (manifest: %d)",
        lock_identical, lock_drifted, lock_errored, #lock_manifest))
end

-- ---------------------------------------------------------------- summary
print("\n===================== THE BOSS =====================")
print(string.format("  positive : %d/%d", pos_passed, #POSITIVE))
print(string.format("  negative : %d/%d", neg_passed, #NEGATIVE))
print(string.format("  panic    : %d/%d", pan_passed, #PANIC))
if #notices > 0 then print(c(YELLOW, string.format("  notices  : %d", #notices))) end
if #failed > 0 then
    print(c(RED, string.format("  FAILURES : %d", #failed)))
    for _, f in ipairs(failed) do print(string.format("   [%s] %s", f.section, f.name)) end
    os.exit(1)
end
print(c(GREEN, "  ALL GREEN — invariants intact."))
