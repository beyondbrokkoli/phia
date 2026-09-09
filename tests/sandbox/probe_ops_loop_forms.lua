local n = 0
local lim = 4
local i = 0
while i <= lim do
    n = n + 1
    i = i + 1
end
local done = false
local c = 0
while not done do
    c = c + 1
    if c >= 2 then
        done = true
    end
end
local k = 5
local d = 0
while k >= 1 do
    d = d + k
    k = k - 1
end
local w = {}
w[0] = n
w[1] = c
w[2] = d
