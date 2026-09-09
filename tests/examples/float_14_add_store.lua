-- float_14_add_store.lua  [POSITIVE — storing a float arithmetic result]
-- Two latent bugs found by the Float Gauntlet's first build live in this
-- exact shape:
-- (1) the lowerer labeled every Add/Sub `Integer` while the allocator
--     derives the true pool from the operands — storing the add
--     DIRECTLY (t[0] = 0.5 + 0.25) drove the store template to the int
--     storage side and rendered an f_r register as i_r: broken compile.
--     The static ty of + and - now follows its operands (same-type
--     numeric is checker-guaranteed).
-- (2) exposed by (1)'s fix: float-element tables now allocate in their
--     own pool with a disjoint physical id range — a handle table's
--     slot can never be reused by a float table, or the per-id pointer
--     declaration would be ambiguous. t hoists *mut f64.
-- slot 0 = 0.75 + 0.125; slots 1..3 read EC zero-fill and add 0.125.
-- EXPECT: TABLE 0 LEN 4 NZ 4 CHECKSUM 9055612950735224832 SUM 1.25
-- EXPECT: NTABLES 1
-- EXPECT: fast_sets=1
-- EXPECT: fast_gets=1
-- EXPECT: dyn_sets=1
-- EXPECT: dyn_gets=0
-- EXPECT: hoists=1
local t = {}
t[0] = 0.5 + 0.25
local i = 0
while i < 4 do
    t[i] = t[i] + 0.125
    i = i + 1
end
