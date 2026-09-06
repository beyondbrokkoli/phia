-- EXPECT: TABLE 0 LEN 2000 NZ 1999 CHECKSUM 2666666000
-- EXPECT: TABLE 1 LEN 2002 NZ 1999 CHECKSUM 2670664000
-- EXPECT: fast_sets=1
-- EXPECT: fast_gets=1
-- EXPECT: dyn_sets=1
-- EXPECT: hoists=2
-- EXPECT: hoist_ctx=0,1
local size = 2000
local data_a = {}
local data_b = {}
local i = 0
while i < size do
    data_a[i] = i
    i = i + 1
end
local iter = 0
while iter < 500 do
    local idx = 0
    local crazy_math = 0 + 1 + 2 + 3 + 4 + 5 - 15 + iter
    while idx < size do
        local crazy_math = data_a[idx]
        local offset_idx = idx + 2
        data_b[offset_idx] = crazy_math - 1 + 1
        idx = idx + 1
    end
    iter = iter + 1
end
