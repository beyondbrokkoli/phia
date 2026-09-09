local a = 1
local b = 10
local c = 100
local t = 3
if t > 2 then a = a + 1 else a = a + 2 end
if t > 1 then b = b + 5 end
if t > 10 then c = c + 1 else c = c + 2 end
local e = 0
if t > 2 then
    if t > 5 then
        e = 1
    else
        e = 2
    end
else
    e = 3
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
w[0] = e + s
