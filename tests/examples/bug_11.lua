-- bug_11.lua — a = b / b = a Move ping-pong before the loop: root tracing walks
-- Table Moves, so both regs resolve to the single NewTable. gate ✓ (n @ b0), key φi
-- safe → fast, HR @ b0. Pins the fact that alias churn can't break root analysis.
-- EXPECT: TABLE 0 LEN 10 NZ 10 CHECKSUM 55
-- EXPECT: fast_sets=1
-- EXPECT: fast_gets=0
-- EXPECT: dyn_sets=0
-- EXPECT: dyn_gets=0
-- EXPECT: hoists=1
local a = {}
local b = a
a = b
local n = 10
local i = 0
while i < n do
    a[i] = 1
    i = i + 1
end
