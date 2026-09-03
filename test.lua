-- test.lua

local table_size = 200
local outer_loops = 5000000
local data = {}
local i = 0
while i < table_size do
    data[i] = i
    i = i + 1
end
local iter = 0
while iter < outer_loops do
    local j = 0
    while j < table_size do
        local current = data[j]
        data[j] = current + iter - j
        j = j + 1
    end
    iter = iter + 1
end
