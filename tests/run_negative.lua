#!/usr/bin/env lua

dofile("tests/helpers.lua")

local pass_count = 0
local fail_count = 0
local failed_names = {}
local test_files = {
    "bug_10a.lua",
    "bug_10c.lua",
}

print("== Running " .. #test_files .. " Negative Build tests ==")

for _, file in ipairs(test_files) do
    local filepath = EXAMPLES_DIR .. "/" .. file
    local expected_errors = parse_expects(filepath, "EXPECT_BUILD_FAIL")

    if #expected_errors > 0 then
        local test_failed = false
        local first_err_line = ""

        -- 1. Build (Must FAIL, adding RUST_BACKTRACE=1 to capture the native stack from build.rs)
        local build_cmd = string.format("RUST_BACKTRACE=1 PHIA_SOURCE='%s' timeout 120 cargo build --release --quiet 2> %s", filepath, BERR)
        local build_res = os.execute(build_cmd)

        if build_res == 0 or build_res == true then
            print("\27[31m✗\27[0m " .. file .. " — expected build failure, but it succeeded cleanly")
            test_failed = true
        else
            -- 2. Verify Build Error
            local berr_content = read_file(BERR)
            -- Grab the FIRST line of the file to show what actually triggered the failure
            first_err_line = berr_content:match("([^\r\n]+)") or "(empty stderr)"
            local matched = true

            for _, err_msg in ipairs(expected_errors) do
                if not berr_content:find(err_msg, 1, true) then
                    print("\27[31m✗\27[0m " .. file .. " — missing expected error: " .. err_msg)
                    matched = false
                end
            end

            if not matched then
                test_failed = true
                print("      \27[33m[Actual Stderr]:\27[0m " .. first_err_line)
                print("      \27[31m[Diagnostic Backtrace]:\27[0m")
                -- Extract the native Rust backtrace from the build error logs
                os.execute(string.format("grep -E '^[ ]+[0-9]+:' %s | head -15", BERR))
            end
        end

        -- 3. Final Tally (Only ONE print per test)
        if test_failed then
            fail_count = fail_count + 1
            table.insert(failed_names, file)
        else
            print("\27[32m✓\27[0m " .. file)
            print("      \27[32m[Caught Expected Build Error]:\27[0m " .. first_err_line)
            print("      \27[32m[Rust Backtrace]:\27[0m")
            -- Extract a compact native Rust backtrace from the build error logs
            os.execute(string.format("grep -E '^[ ]+[0-9]+:' %s | head -5", BERR))
            pass_count = pass_count + 1
        end
    end
end

print(string.format("\npassed: %d  failed: %d", pass_count, fail_count))
if fail_count > 0 then
    print("\27[31mFAILED TESTS:\27[0m")
    for _, name in ipairs(failed_names) do
        print("  - " .. name)
    end
    os.exit(1)
end
