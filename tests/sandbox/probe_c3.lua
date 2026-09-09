local t = 3
local f = 0
if t == 1 then
    f = 10
elseif t == 2 then
    f = 20
elseif t == 3 then
    f = 30
else
    f = 40
end
local s = 0
local i = 0
while i < 6 do
    if i >= 3 then
        s = s + i
    end
    i = i + 1
end
local w = {}
w[0] = f + s
