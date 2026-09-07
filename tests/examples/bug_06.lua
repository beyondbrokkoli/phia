-- bug_06.lua — the loop is textbook-fast: literal 5, b[φi] safe key → fast set,
-- HR @ b0. Everything outside a loop body is never scanned: a[3] @ b0, b[10]/b[4]
-- reads @ b3, out[0..2] writes @ b3 → 3 dyn each way. Stats count the whole program.
-- EXPECT: NTABLES 3
-- EXPECT: TABLE 0 LEN 0 NZ 0 CHECKSUM 0
-- EXPECT: TABLE 1 LEN 5 NZ 5 CHECKSUM 1540
-- EXPECT: TABLE 2 LEN 3 NZ 1 CHECKSUM 312
-- EXPECT: fast_sets=1
-- EXPECT: fast_gets=0
-- EXPECT: dyn_sets=3
-- EXPECT: dyn_gets=3
-- EXPECT: hoists=1
local a = {}
local x = a[3]
local b = {}
local i = 0
while i < 5 do
    b[i] = i + 100
    i = i + 1
end
local y = b[10]
local z = b[4]
local out = {}
out[0] = x
out[1] = y
out[2] = z
