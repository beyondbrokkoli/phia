-- feat_if_02_phi_variants.lua [POSITIVE — join-phi combinatorics]:
-- both-arm assignment, then-only, else-only, nested if-in-arm, a 3-deep
-- elseif chain, and a no-else if mutating a loop-carried accumulator
-- inside a while (the arm store pattern; the loop gate itself is covered
-- by feat_ops_12). Expected: a=2 b=15 c=102 e=2 f=30 s=12.
-- EXPECT: TABLE 0 LEN 6 NZ 6 CHECKSUM 568
-- EXPECT: NTABLES 1
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
w[0] = a
w[1] = b
w[2] = c
w[3] = e
w[4] = f
w[5] = s
