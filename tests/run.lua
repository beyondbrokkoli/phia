#!/usr/bin/env lua

local os = require("os")
local io = require("io")

-- Paths to the compiled binary and the temporary output files.
-- Storing standard out (OUT) and standard error (ERR/BERR) in files
-- allows us to parse them for assertions after the process finishes.
local BIN = "target/release/phia"
local OUT = "target/run_out.txt"
local ERR = "target/run_err.txt"
local BERR = "target/build_err.txt"
local EXAMPLES_DIR = "tests/examples"

local pass_count = 0
local fail_count = 0

-- 1. ADD YOUR TEST FILES HERE
-- Explicit list of test files to run. Execution order follows this array.
local test_files = {
    "bug_01.lua",
    "bug_18.lua",
    -- "gauntlet_pA.lua",
}

-- Reads an entire file into a single string.
-- "*a" instructs io.read to read everything from the current position to EOF.
local function read_file(path)
    local f = io.open(path, "r")
    if not f then return "" end
    local content = f:read("*a")
    f:close()
    return content
end

-- Checks if a specific string exists exactly as a whole line in the output.
-- Equivalent to bash: grep -qx "target" text
local function has_exact_line(text, target)
    -- The pattern "([^\r\n]+)" splits the text into lines.
    -- ^ means "not". \r and \n are carriage return and newline.
    -- + means "one or more".
    -- Together: "Match sequences of characters that are not line breaks."
    for line in text:gmatch("([^\r\n]+)") do
        if line == target then return true end
    end
    return false
end

-- Parses a test file to find expected output defined in comments.
local function parse_expects(filepath)
    local expects = {}
    local content = read_file(filepath)

    -- Pattern breakdown:
    -- %-       : Matches a literal hyphen (escapes the special '-' character in Lua).
    -- %-%-     : Matches "--" (Lua comment start).
    -- %s*      : Matches zero or more whitespace characters.
    -- EXPECT:  : Matches the literal string "EXPECT:".
    -- ( ... )  : Captures the matched sequence inside the parentheses to return it.
    -- [^\r\n]+ : Matches everything up to the end of the line.
    for expected_str in content:gmatch("%-%-%s*EXPECT:%s*([^\r\n]+)") do
        table.insert(expects, expected_str)
    end
    return expects
end

local function run_test(filename)
    local filepath = EXAMPLES_DIR .. "/" .. filename
    local expects = parse_expects(filepath)

    if #expects == 0 then
        -- \27[33m is the ANSI escape code for yellow text. \27[0m resets it.
        print("\27[33m?\27[0m " .. filename .. " — skipped (no '-- EXPECT:' comments found)")
        return
    end

    -- Compiles the test file.
    -- 'timeout 120' kills the build if the rust compiler hangs.
    -- 2> redirects stderr to our BERR file.
    local build_cmd = string.format("PHIA_SOURCE='%s' timeout 120 cargo build --release --quiet 2> %s", filepath, BERR)
    local build_res = os.execute(build_cmd)

    -- Lua 5.1/5.2 os.execute returns a number, 5.3+ returns boolean true on success.
    -- Checking for both ensures cross-version compatibility.
    if build_res ~= 0 and build_res ~= true then
        -- Pattern "([^\n]+)$" matches the last line of the file (up to the end of string $).
        print("\27[31m✗\27[0m " .. filename .. " — build failed: " .. read_file(BERR):match("([^\n]+)$"))
        fail_count = fail_count + 1
        return
    end

    -- Executes the compiled binary.
    -- > redirects stdout to OUT. 2> redirects stderr to ERR.
    local run_cmd = string.format("timeout 60 %s > %s 2> %s", BIN, OUT, ERR)
    local run_res = os.execute(run_cmd)

    if run_res ~= 0 and run_res ~= true then
        print("\27[31m✗\27[0m " .. filename .. " — run failed: " .. read_file(ERR):match("([^\n]+)$"))
        fail_count = fail_count + 1
        return
    end

    local out_content = read_file(OUT)

    -- Evaluate all expected conditions parsed from the file's comments.
    for _, exp in ipairs(expects) do
        -- If the expectation string starts with "TABLE", we require an exact line match.
        -- ^TABLE matches "TABLE" only if it appears at the very beginning of the string.
        if exp:match("^TABLE") then
            if not has_exact_line(out_content, exp) then
                print("\27[31m✗\27[0m " .. filename .. " — want line: " .. exp)
                fail_count = fail_count + 1
                return
            end
        else
            -- For other checks (like "fast_sets=0"), a simple substring match is sufficient.
            -- string.find(out_content, exp, 1, true)
            -- 1: start at character 1
            -- true: disable pattern matching (treat `exp` as raw text).
            if not out_content:find(exp, 1, true) then
                print("\27[31m✗\27[0m " .. filename .. " — stat mismatch, expected: " .. exp)
                fail_count = fail_count + 1
                return
            end
        end
    end

    -- \27[32m is the ANSI escape code for green text.
    print("\27[32m✓\27[0m " .. filename)
    pass_count = pass_count + 1
end

-- EXECUTION ENTRY POINT
print("== Running " .. #test_files .. " tests ==")
for _, file in ipairs(test_files) do
    run_test(file)
end

print(string.format("\npassed: %d  failed: %d", pass_count, fail_count))
if fail_count > 0 then os.exit(1) end
