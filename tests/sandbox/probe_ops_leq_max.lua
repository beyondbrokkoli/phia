local x = 0
local n = 0
if x <= 9223372036854775807 then
    n = 1
else
    n = 2
end
local m = 0
local lim = 9223372036854775807
if x <= lim then
    m = 1
else
    m = 2
end
local w = {}
w[0] = n * 10 + m
