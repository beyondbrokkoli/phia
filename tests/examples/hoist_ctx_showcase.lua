-- hoist_ctx_showcase.lua — the hoist_ctx collective sentinel.
-- One file, seven sections; every EC/HR placement pinned by one string.
--
-- Derivation (block ids in creation order):
--   S1 flat loop        HR in b0  (entry, depth 0)                      → "0"
--   S2 nested           INNER loop hoists; HR in b5 (outer body, d1)    → "1"
--   S3 triple nest      innermost hoists; HR in b14 (middle body, d2)   → "2"
--   S4 two tables/loop  both HRs in b12 (S3's outer EXIT = S4 pre, d0)  → "0","0"
--   S5 one table ×2     HRs in b21 and b24 (both depth 0)               → "0","0"
--   S6 tail write       outer pass region-scans it (Patch D); HR in b27 → "0"
--   S7 region poison    NO hoist — nested a7[999] poisons safe a7[i7]
--                       (Patch A); contributes the 2 dyn_sets instead
--   ctx iterates blocks by ID: b0, b5, b12, b14, b21, b24, b27
--     → "0,1,0,0,2,0,0,0"
--
-- THE SUBTLETY: S4's pair PRECEDES S3's depth-2 entry, despite S4's code
-- coming AFTER S3 in the source — b12 was created as S3's outer exit
-- before S3's middle/inner blocks (b13..b18) existed. hoist_ctx pins
-- block CREATION order, and this file wouldn't have it any other way.
--
-- Failure signatures this file diagnoses on its own:
--   Patch D regressed → S6 goes dyn: fast_sets 8→7, dyn_sets 2→3, hoists
--                       8→7, ctx loses its final "0"
--   Patch A regressed → S7 hoists again: fast_sets 8→9, dyn_sets 2→1,
--                       hoists 8→9, ctx gains a trailing "0" (HR in b30)
--   Depth accounting  → any digit shifts (the "2" becomes 1 or 3)
--   Block order bug   → the sequence permutes (S4 pair leaves positions 3,4)
--
-- EXPECT: fast_sets=8
-- EXPECT: fast_gets=0
-- EXPECT: dyn_sets=2
-- EXPECT: dyn_gets=0
-- EXPECT: hoists=8
-- EXPECT: hoist_ctx=0,1,0,0,2,0,0,0

-- S1: flat loop — hoist at depth 0
local t0 = {}
local i = 0
while i < 4 do
    t0[i] = i
    i = i + 1
end

-- S2: nested loop — the INNER loop hoists (key = inner phi), HR at depth 1
local t1 = {}
local a = 0
while a < 3 do
    local b = 0
    while b < 3 do
        t1[b] = b
        b = b + 1
    end
    a = a + 1
end

-- S3: triple nest — only the innermost hoists, HR at depth 2 (pK's shape)
local t2 = {}
local x = 0
while x < 2 do
    local y = 0
    while y < 2 do
        local z = 0
        while z < 2 do
            t2[z] = z
            z = z + 1
        end
        y = y + 1
    end
    x = x + 1
end

-- S4: two tables, one loop — two HRs in the SAME pre-header block
local u = {}
local v = {}
local k = 0
while k < 3 do
    u[k] = k
    v[k] = k
    k = k + 1
end

-- S5: one table, two sequential loops — re-hoist, HRs in two blocks
local w = {}
local p = 0
while p < 3 do
    w[p] = p
    p = p + 1
end
local q = 0
while q < 3 do
    w[q] = q
    q = q + 1
end

-- S6: tail write after a nested loop — upgraded only by the OUTER pass's
-- region scan (Patch D). Pre-D this was dyn: the direct-body scan never
-- saw it (region_tail_write.lua's shape, folded in here).
local s6 = {}
local m = 0
while m < 3 do
    local n = 0
    while n < 2 do
        n = n + 1
    end
    s6[m] = m
    m = m + 1
end

-- S7: region poison — the nested unsafe write must kill the outer hoist
-- (Patch A). Without the region-wide poison scan, a7[i7] would hoist and
-- a7[999]'s resize would dangle the cached pointer (region_table_resize's
-- shape, folded in here).
local a7 = {}
local i7 = 0
while i7 < 4 do
    a7[i7] = 1
    local j7 = 0
    while j7 < 2 do
        a7[999] = j7
        j7 = j7 + 1
    end
    i7 = i7 + 1
end
