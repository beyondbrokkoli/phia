-- REGION-POISON REPRO
-- EXPECT: fast_sets=0
-- EXPECT: fast_gets=0
-- EXPECT: dyn_sets=2
-- EXPECT: dyn_gets=0
-- EXPECT: hoists=0
local a = {}
local i = 0
while i < 10 do
    a[i] = 1          -- safe write, direct body → upgraded; EC+HR @ pre-header
    local j = 0
    while j < 2 do
        a[999] = j    -- unsafe write, NESTED body
        j = j + 1
    end
    i = i + 1
end
