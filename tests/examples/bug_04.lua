-- EXPECT: TABLE 0 LEN 1000 NZ 999 CHECKSUM 333333000
-- EXPECT: fast_sets=0
-- EXPECT: hoists=0
local a = {}
local b = a
local n = 1000
local i = 0
while i < 10 do
    local j = 0
    while j < n do
        a[j] = j
        b[i] = i
        j = j + 1
    end
    i = i + 1
end
