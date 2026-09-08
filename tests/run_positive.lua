#!/usr/bin/env lua

dofile("tests/helpers.lua")

local pass_count = 0
local fail_count = 0
local failed_names = {} -- Tracks the names of the tests that failed

local test_files = {
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
    "firewall_neg_offset.lua"
}

print("== Running " .. #test_files .. " Positive Build tests ==")

for _, filename in ipairs(test_files) do
    local filepath = EXAMPLES_DIR .. "/" .. filename
    -- Notice we pass "EXPECT" as the prefix to match the updated helper
    local expects = parse_expects(filepath, "EXPECT")

    if #expects == 0 then
        print("\27[33m?\27[0m " .. filename .. " — skipped (no '-- EXPECT:' comments found)")
    else
        local test_failed = false

        -- 1. Build
        local build_cmd = string.format("PHIA_SOURCE='%s' timeout 120 cargo build --release --quiet 2> %s", filepath, BERR)
        local build_res = os.execute(build_cmd)

        if build_res ~= 0 and build_res ~= true then
            local err_line = read_file(BERR):match("([^\n]+)$") or "No error output found"
            print("\27[31m✗\27[0m " .. filename .. " — build failed: " .. err_line)
            test_failed = true
        end

        -- 2. Run
        if not test_failed then
            local run_cmd = string.format("timeout 60 %s > %s 2> %s", BIN, OUT, ERR)
            local run_res = os.execute(run_cmd)

            if run_res ~= 0 and run_res ~= true then
                local err_line = read_file(ERR):match("([^\n]+)$") or "No error output found"
                print("\27[31m✗\27[0m " .. filename .. " — run failed: " .. err_line)
                test_failed = true
            end
        end

        -- 3. Assertions
        if not test_failed then
            local out_content = read_file(OUT)

            for _, exp in ipairs(expects) do
                if exp:match("^TABLE ") then
                    if not has_exact_line(out_content, exp) then
                        print("\27[31m✗\27[0m " .. filename .. " — want line: " .. exp)
                        test_failed = true
                        break -- break out of the assertions loop
                    end
                elseif exp:match("^NTABLES") then
                    local expected_count = tonumber(exp:match("%d+"))
                    local actual_count = 0
                    for line in out_content:gmatch("([^\r\n]+)") do
                        if line:match("^TABLE ") then actual_count = actual_count + 1 end
                    end
                    if actual_count ~= expected_count then
                        print("\27[31m✗\27[0m " .. filename .. " — want NTABLES " .. tostring(expected_count) .. ", got " .. tostring(actual_count))
                        test_failed = true
                        break
                    end
                else
                    -- Check if this expectation is a stat (e.g., "fast_sets=0")
                    local k, v = exp:match("^([^=]+)=(.*)$")
                    if k and v then
                        -- Find this exact key in the output and capture its value (up to ';' or newline)
                        local actual_v = out_content:match(k .. "=([^;\r\n]+)")
                        if actual_v ~= v then
                            print(string.format("\27[31m✗\27[0m %s — %s: want %s, got %s", filename, k, v, tostring(actual_v)))
                            test_failed = true
                            break
                        end
                    else
                        -- Fallback for random expected text (strict substring)
                        if not out_content:find(exp, 1, true) then
                            print("\27[31m✗\27[0m " .. filename .. " — stat mismatch, expected: " .. exp)
                            test_failed = true
                            break
                        end
                    end
                end
            end
        end

        -- 4. Final Tally for this test
        if test_failed then
            print("      \27[33m[Diagnostic Backtrace]:\27[0m")
            os.execute(string.format("gdb -batch -ex run -ex bt %s 2>/dev/null | grep -E '^#' | head -15", BIN))
            fail_count = fail_count + 1
            table.insert(failed_names, filename)
        else
            print("\27[32m✓\27[0m " .. filename)
            pass_count = pass_count + 1
        end
    end
end

-- 5. Print Summary Stats
print(string.format("\npassed: %d  failed: %d", pass_count, fail_count))
if fail_count > 0 then
    print("\27[31mFAILED TESTS:\27[0m")
    for _, name in ipairs(failed_names) do
        print("  - " .. name)
    end
    os.exit(1)
end
