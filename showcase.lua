-- showcase.lua — Pascal's Triangle (Optimizer Tripwire Edition)
-- This program is deliberately written to lay UB traps.

local pascal = {}
local n = 6
local row_idx = 0

while row_idx <= n do

    local row = {}
    local col_idx = 0

    -- TRIPWIRE: Dynamic Loop Bounds & Strict SCEV
    -- What could go wrong: A reckless optimizer sees `col_idx` incrementing
    -- and blindly hoists a raw pointer to skip bounds checks. But the loop limit
    -- `row_idx` mutates in the outer loop! If the compiler guesses the pre-size
    -- wrong, the inner loop causes a buffer overflow.
    -- How Phia handles it: Phia's Scalar Evolution (SCEV) engine detects that
    -- the inner loop bound is dynamic. Because it cannot mathematically prove
    -- the absolute maximum limit, it deliberately surrenders the affine proof.
    -- It emits safe, on-the-fly `.resize()` and `.get_mut()` checks inside
    -- the inner loop. (Check the STATS: hoists=0, fast_sets=0).

    while col_idx <= row_idx do
        if col_idx == 0 then
            row[col_idx] = 1
        elseif col_idx == row_idx then
            row[col_idx] = 1
        else
            local prev_row = pascal[row_idx - 1]
            row[col_idx] = prev_row[col_idx - 1] + prev_row[col_idx]
        end
        col_idx = col_idx + 1
    end

    pascal[row_idx] = row
    row_idx = row_idx + 1
end

print("pascal_mid", pascal[3][1], pascal[6][3])

-- TRIPWIRE: Data-Dependent Indirection Demotion
local unprovable_read = pascal[3][1] -- Evaluates to 3 at runtime
local fallback_table = {}

-- What could go wrong: If a data-dependent read is used as a table key, the
-- compiler has no way to predict the required memory size.
-- How Phia handles it: It instantly recognizes the indirection (`fallback_table[x]`).
-- Because runtime memory dictates the index, it explicitly demotes this store
-- to the fully checked dynamic path. No panics, no UB—just memory-safe Rust.
fallback_table[unprovable_read] = 99

print("safe_fallback", fallback_table[3])
