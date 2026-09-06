-- EXPECT: fast_sets=1
-- EXPECT: dyn_sets=0
-- EXPECT: hoists=1
-- EXPECT: hoist_ctx=0

-- 1. Initialize our table and index
local data = {}
local i = 0

-- 2. The literal bound '100' is the star of the show.
-- BEFORE GLM's fix: the '100' is loaded inside the loop header. `dyn_sets` = 1, `hoists` = 0.
-- AFTER GLM's fix: the '100' is materialized before the loop. `fast_sets` = 1, `hoists` = 1.
while i < 100 do
    data[i] = i + 5
    i = i + 1
end
