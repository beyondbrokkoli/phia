#!/usr/bin/env lua

local os = require("os")
local io = require("io")

local BIN = "target/release/phia"
local OUT = "target/run_out.txt"
local ERR = "target/run_err.txt"
local BERR = "target/build_err.txt"
local EXAMPLES_DIR = "tests/examples"

local pass_count = 0
local fail_count = 0

-- 1. ADD YOUR TEST FILES HERE
local test_files = {
    "bug_01.lua",
    "bug_18.lua",
    -- "gauntlet_pA.lua",
}

local function read_file(path)
    local f = io.open(path, "r")
    if not f then return "" end
    local content = f:read("*a")
    f:close()
    return content
end

local function has_exact_line(text, target)
    for line in text:gmatch("([^\r\n]+)") do
        if line == target then return true end
    end
    return false
end

-- Extracts lines starting with "-- EXPECT: " from the source file
local function parse_expects(filepath)
    local expects = {}
    local content = read_file(filepath)
    for expected_str in content:gmatch("%-%-%s*EXPECT:%s*([^\r\n]+)") do
        table.insert(expects, expected_str)
    end
    return expects
end

local function run_test(filename)
    local filepath = EXAMPLES_DIR .. "/" .. filename
    local expects = parse_expects(filepath)

    if #expects == 0 then
        print("\27[33m?\27[0m " .. filename .. " — skipped (no '-- EXPECT:' comments found)")
        return
    end

    local build_cmd = string.format("PHIA_SOURCE='%s' timeout 120 cargo build --release --quiet 2> %s", filepath, BERR)
    local build_res = os.execute(build_cmd)

    if build_res ~= 0 and build_res ~= true then
        print("\27[31m✗\27[0m " .. filename .. " — build failed: " .. read_file(BERR):match("([^\n]+)$"))
        fail_count = fail_count + 1
        return
    end

    local run_cmd = string.format("timeout 60 %s > %s 2> %s", BIN, OUT, ERR)
    local run_res = os.execute(run_cmd)

    if run_res ~= 0 and run_res ~= true then
        print("\27[31m✗\27[0m " .. filename .. " — run failed: " .. read_file(ERR):match("([^\n]+)$"))
        fail_count = fail_count + 1
        return
    end

    local out_content = read_file(OUT)

    for _, exp in ipairs(expects) do
        if exp:match("^TABLE") then
            if not has_exact_line(out_content, exp) then
                print("\27[31m✗\27[0m " .. filename .. " — want line: " .. exp)
                fail_count = fail_count + 1
                return
            end
        else
            if not out_content:find(exp, 1, true) then
                print("\27[31m✗\27[0m " .. filename .. " — stat mismatch, expected: " .. exp)
                fail_count = fail_count + 1
                return
            end
        end
    end

    print("\27[32m✓\27[0m " .. filename)
    pass_count = pass_count + 1
end

-- EXECUTION
print("== Running " .. #test_files .. " tests ==")
for _, file in ipairs(test_files) do
    run_test(file)
end

print(string.format("\npassed: %d  failed: %d", pass_count, fail_count))
if fail_count > 0 then os.exit(1) end
