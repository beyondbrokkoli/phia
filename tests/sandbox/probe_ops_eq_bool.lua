local f = true
local i = 0
while i < 3 do
    f = not f
    i = i + 1
end
local n = 0
if true == f then
    n = 1
else
    n = 2
end
if f == false then
    n = n + 10
end
local w = {}
w[0] = n
