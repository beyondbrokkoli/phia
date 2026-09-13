-- pre-header:  EC(a, 10)   →  len ≥ 10
--              HR(a)       →  p_r = a.array.as_mut_ptr();  len_r = len   (10)
-- ...
-- a[999] = j →  dyn path: idx(999) ≥ len(10) → array.resize(1000)
--               └─ Vec::resize may REALLOCATE → the backing buffer moves
--                  → p_r now points at freed memory
-- next iter:   a[i] → SetTableFast writes through dangling p_r  ← UB
-- EXPECT: fast_sets=0
-- EXPECT: fast_gets=0
-- EXPECT: dyn_sets=2
-- EXPECT: dyn_gets=0
-- EXPECT: hoists=0
local a = {}
local i = 0
while i < 10 do
    a[i] = 1          -- safe key, but same ROOT as the resize below → poisoned for this loop
    local j = 0
    while j < 2 do
        a[999] = j    -- unsafe key, NESTED body: invisible pre-Patch-A, region-poison now
        j = j + 1
    end
    i = i + 1
end
