-- bug_12.lua — TIER 2 + 2b FLIP: b[k] (Some(300)) rides the EC(310) contract,
-- and 2b's ROOT-based S2 lets the in-body alias upgrade: post-C the fast op
-- targets the root reg, so the operand Move's def position is irrelevant.
-- b = c mid-loop still can't launder anything — roots, not regs, hold the
-- contract, and the unpoisoned root is provably pre-loop.
-- EXPECT: NTABLES 2
-- EXPECT: TABLE 0 LEN 310 NZ 20 CHECKSUM 3110
-- EXPECT: TABLE 1 LEN 0 NZ 0 CHECKSUM 0
-- EXPECT: fast_sets=2
-- EXPECT: fast_gets=0
-- EXPECT: dyn_sets=0
-- EXPECT: dyn_gets=0
-- EXPECT: hoists=1
local a = {}
local c = {}
local n = 10
local i = 0
while i < n do
    local b = a
    local k = i + 300
    b[k] = 1
    a[i] = 1
    b = c
    i = i + 1
end
