-- main.lua
-- Tests scope shadowing, register recycling, and optimizer fallbacks.

local size = 50000
local data_a = {}
local data_b = {}

-- 1. Initialize data_a using the FAST PATH (Optimizer will hoist this)
local i = 0
while i < size do
    data_a[i] = i
    i = i + 1
end

-- 2. The Torture Loop
local iter = 0
while iter < 50000 do
    local idx = 0

    -- Register Exhaustion Test:
    -- This creates a massive temporary expression tree.
    local crazy_math = 0 + 1 + 2 + 3 + 4 + 5 - 15 + iter

    while idx < size do
        -- Scope Shadowing Test:
        -- 'crazy_math' exists outside, but we declare it AGAIN inside.
        local crazy_math = data_a[idx]

        -- Optimizer Bypass Test:
        -- 'offset_idx' is NOT the loop variable. The compiler cannot prove
        -- the bounds ahead of time. It MUST fall back to the dynamic resize()
        -- path instead of the C-style raw pointer path.
        local offset_idx = idx + 2

        -- Write to a new table using the un-hoistable index
        data_b[offset_idx] = crazy_math - 1 + 1

        idx = idx + 1
    end

    iter = iter + 1
end
