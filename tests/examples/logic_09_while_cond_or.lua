-- logic_09_while_cond_or.lua  [POSITIVE — `while i < 2 or go`: left-true
-- short-circuits, left-false evaluates the right. go is a BOOLEAN loop
-- phi, mutated inside the body by an if — the or chain reads it fresh
-- every iteration, and the loop ends only when BOTH sides are false]
-- EXPECT_PRINT: orloop	4	false
local i = 0
local go = true
while i < 2 or go do
    i = i + 1
    if i > 3 then
        go = false
    end
end
print("orloop", i, go)
