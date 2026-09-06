-- EXPECT: TABLE 0 LEN 2 NZ 2 CHECKSUM 65
--
-- Bug 18: Coalesced phis must read each other's PRE-update values.
-- (y's new value is computed from x's old value, and vice versa)

local a = {}
local x = 1
local y = 2
local i = 0
while i < 4 do
    local t = y
    y = x + 10
    x = t
    i = i + 1
end
a[0] = x
a[1] = y
