-- bool_02_fast_rw.lua  [POSITIVE — bool fast paths, read+write edition]
-- Two loops over the same bool root: the store loop upgrades to
-- SetTableFast, the read loop to GetTableFast — the bool twin of
-- float_09's shape, riding a *mut bool hoisted pointer. The read value
-- feeds a boolean Eq, and the accumulated witness counts trues through
-- integer arithmetic (i % 2 == 0 is true for even indices: 0,2,4,6 → 4).
-- EXPECT: TABLE 0 LEN 8 NZ 4 CHECKSUM 16
-- EXPECT: NTABLES 1
-- EXPECT: fast_sets=1
-- EXPECT: fast_gets=1
-- EXPECT: dyn_sets=0
-- EXPECT: dyn_gets=1
-- EXPECT: hoists=2
-- EXPECT: hoist_ctx=0,0
-- EXPECT_PRINT: witness	4	true
local t = {}
local i = 0
while i < 8 do
    t[i] = i % 2 == 0
    i = i + 1
end
local witness = 0
local j = 0
while j < 8 do
    if t[j] == true then
        witness = witness + 1
    end
    j = j + 1
end
print("witness", witness, t[1] == false)
