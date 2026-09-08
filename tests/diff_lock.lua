#!/usr/bin/env lua
-- tests/diff_lock.lua — baseline drift gate.
--
-- Rebuilds every test named in tests/lock/manifest.lua, copies the generated
-- Rust to tests/diff/<name>.rs (scratch, safe to delete), and byte-compares
-- each against the locked baseline in tests/lock/<name>.rs. Since these
-- tests contain none of the new language features, ANY diff is a regression
-- in the integer path. Read-only on tests/lock/. Exit code 1 on any drift,
-- so it can gate scripts.

dofile("tests/helpers.lua")

local LOCK_DIR = "tests/lock"
local DIFF_DIR = "tests/diff"

local function lock_name(filename)
    return LOCK_DIR .. "/" .. filename:gsub("%.lua$", "") .. ".rs"
end
local function diff_name(filename)
    return DIFF_DIR .. "/" .. filename:gsub("%.lua$", "") .. ".rs"
end

local function find_baked()
    local p = io.popen("ls -t target/release/build/phia-*/out/baked_native.rs 2>/dev/null | head -1")
    local line = p:read("*l")
    p:close()
    if line and line ~= "" then return line end
    return nil
end

if not io.open(LOCK_DIR .. "/manifest.lua", "r") then
    print("no lock found (" .. LOCK_DIR .. "/manifest.lua missing)")
    print("run tests/milestone_lockdown.lua first")
    os.exit(1)
end
local LOCKED_TESTS = dofile(LOCK_DIR .. "/manifest.lua")

os.execute("rm -rf " .. DIFF_DIR)
os.execute("mkdir -p " .. DIFF_DIR)

local identical, changed, errored = 0, {}, {}
for _, filename in ipairs(LOCKED_TESTS) do
    local lock_f = io.open(lock_name(filename), "r")
    if not lock_f then
        print(string.format("\27[31m✗\27[0m %s — missing lock file %s", filename, lock_name(filename)))
        table.insert(errored, filename)
    else
        lock_f:close()
        local build_cmd = string.format("PHIA_SOURCE='%s' timeout 120 cargo build --release --quiet 2> %s",
            EXAMPLES_DIR .. "/" .. filename, BERR)
        local res = os.execute(build_cmd)
        if res ~= 0 and res ~= true then
            print(string.format("\27[31m✗\27[0m %s — build failed: %s", filename,
                read_file(BERR):match("([^\n]+)$") or "no error output"))
            table.insert(errored, filename)
        else
            local baked = find_baked()
            local code = baked and read_file(baked) or ""
            if not (code:find("pub fn run_baked", 1, true) and code:find("pub const STATS", 1, true)) then
                print(string.format("\27[31m✗\27[0m %s — could not capture generated code", filename))
                table.insert(errored, filename)
            else
                local f = io.open(diff_name(filename), "w")
                f:write(code)
                f:close()
                local p = io.popen(string.format("diff -u '%s' '%s'", lock_name(filename), diff_name(filename)))
                local d = p:read("*a")
                p:close()
                if d == "" then
                    identical = identical + 1
                    print(string.format("\27[32m✓\27[0m %s — identical", filename))
                else
                    print(string.format("\27[31m✗\27[0m %s — DRIFT:\n%s", filename, d))
                    table.insert(changed, filename)
                end
            end
        end
    end
end

-- stray lock files not covered by the manifest (stale baseline hygiene)
local expected = {}
for _, n in ipairs(LOCKED_TESTS) do expected[n:gsub("%.lua$", "") .. ".rs"] = true end
local p = io.popen("ls " .. LOCK_DIR .. " 2>/dev/null")
for line in p:lines() do
    if not expected[line] and line ~= "manifest.lua" then
        print(string.format("\27[33m!\27[0m stray lock file not in manifest: %s/%s", LOCK_DIR, line))
    end
end
p:close()

print(string.format("== lock diff: %d identical, %d drifted, %d errored ==",
    identical, #changed, #errored))
if #changed > 0 or #errored > 0 then os.exit(1) end
