#!/usr/bin/env lua

dofile("tests/helpers.lua")

local pass_count = 0
local fail_count = 0
local failed_names = {}
local test_files = {
    "bug_05a.lua",
    "bug_05b.lua",
}

print("== Running " .. #test_files .. " Diagnostic (Panic) tests ==")

for _, file in ipairs(test_files) do
    local filepath = EXAMPLES_DIR .. "/" .. file
    -- Magic comment tag for this runner
    local expected_panics = parse_expects(filepath, "EXPECT_PANIC")

    if #expected_panics > 0 then
        local test_failed = false

        -- 1. Build (Must SUCCEED for a diagnostic test)
        local build_cmd = string.format("PHIA_SOURCE='%s' timeout 120 cargo build --release --quiet 2> %s", filepath, BERR)
        local build_res = os.execute(build_cmd)

        if build_res ~= 0 and build_res ~= true then
            print("\27[31m✗\27[0m " .. file .. " — expected build to succeed, but it failed: " .. read_file(BERR):match("([^\n]+)$"))
            test_failed = true
        else
            -- 2. Run (Must FAIL for a diagnostic test)
            local run_cmd = string.format("timeout 60 %s > %s 2> %s", BIN, OUT, ERR)
            local run_res = os.execute(run_cmd)

            if run_res == 0 or run_res == true then
                print("\27[31m✗\27[0m " .. file .. " — expected runtime panic, but it succeeded cleanly")
                test_failed = true
            else
                -- 3. Verify the expected panic message is in standard error
                local err_content = read_file(ERR)
                local matched = true

                for _, err_msg in ipairs(expected_panics) do
                    if not err_content:find(err_msg, 1, true) then
                        print("\27[31m✗\27[0m " .. file .. " — missing expected panic message: " .. err_msg)
                        matched = false
                    end
                end

                -- If it crashed but didn't give us the expected error, it might be a segfault.
                -- This is where we grab the GDB functionality!
                if not matched then
                    test_failed = true
                    print("      \27[33m[Diagnostic Backtrace]:\27[0m")
                    -- Execute GDB in batch mode to print the backtrace of the crash
                    local gdb_cmd = string.format("gdb -batch -ex run -ex bt %s 2>/dev/null | grep -E '^\\#' | head -15", BIN)
                    os.execute(gdb_cmd)
                end
            end
        end

        -- 4. Final Tally
        if test_failed then
            fail_count = fail_count + 1
            table.insert(failed_names, file)
        else
            print("\27[32m✓\27[0m " .. file)
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
