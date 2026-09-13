-- float_01_store_read.lua  [POSITIVE — the float core on the dyn path]
-- A float table on the dynamic (non-hoisted) path: Table::new_float(),
-- farray storage, f_r registers. CHECKSUM hashes bit patterns (absolutely
-- deterministic); SUM is the sequential f64 fold for human readability.
-- to_bits(1.5) = 0x3FF8000000000000 = 4609434218613702656.
-- EXPECT: TABLE 0 LEN 1 NZ 1 CHECKSUM 4609434218613702656 SUM 1.5
-- EXPECT: NTABLES 1
-- EXPECT: fast_sets=0
-- EXPECT: fast_gets=0
-- EXPECT: dyn_sets=1
-- EXPECT: dyn_gets=1
-- EXPECT: hoists=0
local t = {}
t[0] = 1.5
local x = t[0]
