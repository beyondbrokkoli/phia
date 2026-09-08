#!/usr/bin/env lua
-- tests/milestone_lockdown.lua — MANUAL-ONLY milestone lockdown.
--
-- Run this ONLY after a milestone is fully verified: positive suite green,
-- negative suite green, invariants audited. For every test in LOCKED_TESTS
-- it rebuilds the project with that test as PHIA_SOURCE and copies the
-- generated Rust (the post-rustfmt baked_native.rs, INCLUDING the STATS
-- const) into tests/lock/<name>.rs. That directory is the frozen baseline
-- diff_lock.lua verifies against.
--
-- This script OVERWRITES tests/lock/* — git history is the audit trail.
-- Future milestones: adjust LOCKED_TESTS below, rerun (with `force` to skip
-- the confirmation prompt). It refuses to write the manifest unless every
-- single build succeeded, so a half-executed lockdown leaves no usable lock.

dofile("tests/helpers.lua")

-- THE MILESTONE-1 BASELINE. Mirrors tests/run_positive.lua's list at
-- lockdown time (35 tests, tier2_reg_limit.lua included). Keep in sync by
-- hand when reissuing a lockdown — diff_lock.lua does NOT read this table;
-- it reads the derived tests/lock/manifest.lua this script writes.
local LOCKED_TESTS = {
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
}

local LOCK_DIR = "tests/lock"

local ARGS = {...}
if ARGS[1] ~= "force" then
    print("This OVERWRITES everything in " .. LOCK_DIR .. "/ with the current codegen.")
    print("Type LOCK to proceed (or rerun with `force`):")
    if io.read("*l") ~= "LOCK" then print("aborted — nothing written."); os.exit(1) end
end

os.execute("mkdir -p " .. LOCK_DIR)

-- ADVISORY RECORD (not enforced by anything): lockdown moment, toolchain
-- version, and each test's TIME line — the perf history of every baseline,
-- captured where the baseline is born. Lives as comments in manifest.lua so
-- tests/lock/ gains no file the boss would flag as stray.
local advisory = {}
local function note(s) table.insert(advisory, s) end
note("locked: " .. os.date("%Y-%m-%d %H:%M:%S"))
do
    local p = io.popen("rustc --version 2>/dev/null")
    local v = p:read("*l") or "rustc?"
    p:close()
    note(v)
end

local failed = {}
for _, filename in ipairs(LOCKED_TESTS) do
    local src = EXAMPLES_DIR .. "/" .. filename
    os.execute("touch '" .. src .. "'") -- mtime defense, same as the boss
    local build_cmd = string.format("PHIA_SOURCE='%s' timeout 120 cargo build --release --quiet 2> %s", src, BERR)
    local res = os.execute(build_cmd)
    if res ~= 0 and res ~= true then
        print(string.format("\27[31m✗\27[0m %s — build failed: %s", filename,
            read_file(BERR):match("([^\n]+)$") or "no error output"))
        table.insert(failed, filename)
    else
        local baked = find_baked()
        local code = baked and read_file(baked) or ""
        -- sanity: this must be OUR generated artifact, not a stale leftover
        if not (code:find("pub fn run_baked", 1, true) and code:find("pub const STATS", 1, true)) then
            print(string.format("\27[31m✗\27[0m %s — could not capture generated code (baked=%s)",
                filename, tostring(baked)))
            table.insert(failed, filename)
        else
            local dest = LOCK_DIR .. "/" .. filename:gsub("%.lua$", "") .. ".rs"
            local f = io.open(dest, "w")
            f:write(code)
            f:close()
            -- advisory perf record: the TIME of the exact binary whose
            -- codegen was just frozen
            local run = io.popen(string.format("timeout 60 %s 2>/dev/null", BIN))
            local out = run:read("*a")
            run:close()
            note(string.format("TIME %s — %s", out:match("TIME ([^\r\n]+)") or "?", filename))
            print(string.format("\27[32m✓\27[0m locked %s   (from %s)", dest, baked))
        end
    end
end

if #failed > 0 then
    print(string.format("\27[31mLOCKDOWN INCOMPLETE\27[0m — %d/%d failed: %s",
        #failed, #LOCKED_TESTS, table.concat(failed, ", ")))
    print("Some lock files were already overwritten. Fix and rerun — the")
    print("manifest was NOT written, so diff_lock.lua will not use this state.")
    os.exit(1)
end

-- The manifest is the single source of truth for diff_lock.lua: exactly what
-- this lockdown actually verified, derived — never hand-edited.
local m = io.open(LOCK_DIR .. "/manifest.lua", "w")
m:write("-- generated by tests/milestone_lockdown.lua — do not edit by hand\n")
for _, line in ipairs(advisory) do
    m:write("-- " .. line .. "\n")
end
m:write("return {\n")
for _, n in ipairs(LOCKED_TESTS) do
    m:write(string.format('    "%s",\n', n))
end
m:write("}\n")
m:close()

print(string.format("== milestone locked: %d/%d tests -> %s/ ==\n", #LOCKED_TESTS, #LOCKED_TESTS, LOCK_DIR))
