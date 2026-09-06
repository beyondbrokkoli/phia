-- EXPECT: TABLE 0 LEN 1 NZ 1 CHECKSUM 100
local t = {}
local flag = 0 < 1
local k = 0
while flag do
    t[k] = 100
    flag = 0 < 0
    k = k + 1
end
