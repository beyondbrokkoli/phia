-- main.lua
-- Features: Strict Phia subset (integers, tables, local, while, +, -)

-- PHASE 1: The Autobahn (Massive Linear Allocation)
-- Goal: Test pure `SetTableFast` throughput and basic loop hoisting.
local p1_table = {}
local p1_size = 50000000
local p1_i = 0
while p1_i < p1_size do
    p1_table[p1_i] = p1_i
    p1_i = p1_i + 1
end


-- PHASE 2: The Alias Trap (Safeguards #12 & #13)
-- Goal: Force the optimizer to recognize aliases and safely disable
-- fast-paths mid-loop without crashing or hanging.
local p2_table = {}
local p2_alias = p2_table
local p2_size = 10000000
local p2_i = 0
while p2_i < p2_size do
    -- Fast write via original
    p2_table[p2_i] = p2_i + 100

    -- The trap: dynamic write via alias (forces clobber)
    local j = p2_i
    p2_alias[j + 1000] = 1

    p2_i = p2_i + 1
end


-- PHASE 3: The Dynamic Resize Minefield (Safeguard #8)
-- Goal: Interleave sequential writes with out-of-bounds jumps to force
-- the arena to dynamically grow, testing pointer invalidation.
local p3_table = {}
local p3_size = 20000000
local p3_i = 0
while p3_i < p3_size do
    -- Expected fast set
    p3_table[p3_i] = 7

    -- OOB jump (disables hoisting for this write, forces arena resize)
    local jump = p3_i + 30000
    p3_table[jump] = 8

    p3_i = p3_i + 1
end


-- PHASE 4: The Matrix (Safeguards #4 & #9)
-- Goal: Deep loop depth hoisting and register survival across massive iterations.
local p4_src = {}
local p4_dst = {}
local p4_size = 150000

-- Setup source table
local p4_setup = 0
while p4_setup < p4_size do
    p4_src[p4_setup] = p4_setup
    p4_setup = p4_setup + 1
end

-- The 22.5 Billion Iteration Meat Grinder
local outer = 0
while outer < p4_size do
    local inner = 0
    -- Fake math noise to ensure the register allocator doesn't just sleep
    local math_noise = outer + 1 - 1 + 5 - 5

    while inner < p4_size do
        -- Fast Get from src, Fast Set to dst (testing context depth 1)
        local val = p4_src[inner]
        local offset = inner + 2
        p4_dst[offset] = val + math_noise

        inner = inner + 1
    end
    outer = outer + 1
end
