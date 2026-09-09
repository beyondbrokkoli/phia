local big = 9007199254740992.0
local small = 9007199254740990.0
local n = 0
if big <= big then n = n + 1 end
if small <= small then n = n + 10 end
if big >= big then n = n + 100 end
if small >= small then n = n + 1000 end
if big > small then n = n + 10000 end
if small < big then n = n + 100000 end
local w = {}
w[0] = n
