-- EXPECT: TABLE 0 LEN 0 NZ 0 CHECKSUM 0
-- EXPECT: TABLE 1 LEN 1 NZ 1 CHECKSUM 5
-- EXPECT: TABLE 2 LEN 2 NZ 1 CHECKSUM 22
-- EXPECT: TABLE 3 LEN 3 NZ 1 CHECKSUM 21
-- EXPECT: TABLE 4 LEN 1 NZ 1 CHECKSUM 7
local t = {}
local n = 3
local i = 0
while i < n do
    local x = t[0]
    t = {}
    t[i] = x + i + 5
    i = i + 1
end
local w = {}
w[0] = t[2]
