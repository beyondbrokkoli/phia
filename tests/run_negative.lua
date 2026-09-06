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
        -- Fix 1: Full build string
        local build_cmd = string.format("PHIA_SOURCE='%s' timeout 120 cargo build --release --quiet 2> %s", filepath, BERR)
        local build_res = os.execute(build_cmd)

        if build_res == 0 or build_res == true then
            print("\27[31m✗\27[0m " .. file .. " — expected build failure, but it succeeded")
            fail_count = fail_count + 1
            table.insert(failed_names, file)
        else
            local berr_content = read_file(BERR)
            local matched = true
            for _, err_msg in ipairs(expected_errors) do
                if not berr_content:find(err_msg, 1, true) then
                    print("\27[31m✗\27[0m " .. file .. " — missing expected error: " .. err_msg)
                    matched = false
                end
            end

            if matched then
                print("\27[32m✓\27[0m " .. file)
                pass_count = pass_count + 1
            else
                fail_count = fail_count + 1
                table.insert(failed_names, file)
            end
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
