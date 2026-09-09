-- feat_ops_08_loop_forms.lua [POSITIVE — the three loop-condition shapes
-- that decline the tier4 gate and render through the loop{} fallback].
-- `while i <= lim` (variable bound — the +1 desugar would sit inside the
-- header, so the gate declines and the condition lowers to a native Leq),
-- `while not done` (Not in the header), and a `while k >= 1` countdown
-- (native Geq in the header). The done-flag loop is the shape that found
-- the trivial_else orphan bug: the only in-loop mutation of a loop-carried
-- bool sits in a single no-else if, which coalesces the else arm's join
-- Move into a removable self-copy.
-- EXPECT: TABLE 0 LEN 2 NZ 2 CHECKSUM 9244
-- EXPECT: fast_sets=0
-- EXPECT: hoists=0
-- EXPECT: dyn_sets=2
local lim = 3
local i = 0
local n = 0
while i <= lim do
    n = n + i
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
local k = 4
local d = 0
while k >= 1 do
    d = d * 10 + k
    k = k - 1
end
local w = {}
w[0] = n * 100 + c
w[1] = d
