-- EXPECT: TABLE 0 LEN 1000 NZ 1000 CHECKSUM 3503500
-- EXPECT: fast_sets=0
--
-- Bug 01: Literal loop bound must not be hoisted.

local a = {}
local i = 0
while i < 1000 do
    a[i] = 7
    i = i + 1
end
