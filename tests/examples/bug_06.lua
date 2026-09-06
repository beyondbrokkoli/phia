-- EXPECT: NTABLES 3
-- EXPECT: TABLE 0 LEN 0 NZ 0 CHECKSUM 0
-- EXPECT: TABLE 1 LEN 5 NZ 5 CHECKSUM 1540
-- EXPECT: TABLE 2 LEN 3 NZ 1 CHECKSUM 312
local a = {}
local x = a[3]
local b = {}
local i = 0
while i < 5 do
    b[i] = i + 100
    i = i + 1
end
local y = b[10]
local z = b[4]
local out = {}
out[0] = x
out[1] = y
out[2] = z
