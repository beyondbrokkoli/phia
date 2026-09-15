-- fconst_03_chain_table_checksum.lua  [POSITIVE — fold bit-exactness pin]
-- A folded float chain feeding a float TABLE: the runtime CHECKSUM is
-- computed from to_bits(), so this pin is a BIT-EXACTNESS assertion on
-- the fold — the stored literal must equal, bit for bit, what the
-- runtime templates would have computed for (0.1 + 0.2) * 3.0.
-- The evaluator folds the same left-assoc tree the emitted templates
-- spell, and `{:?}` renders the shortest round-trip literal, so the
-- folded 0.9000000000000001 lands in the farray with identical bits.
-- RE-VERIFY IF THIS BREAKS: a changed CHECKSUM means a float fold arm
-- stopped being bit-faithful to its emitted template (operand order,
-- floor-div/mod spelling, or the is_finite guard's declination set).
-- EXPECT: TABLE 0 LEN 1 NZ 1 CHECKSUM 4606281698874543310 SUM 0.9000000000000001
-- EXPECT: NTABLES 1
-- EXPECT: consts_f=5
local t = {}
local x = 0.1
local y = x + 0.2
local z = y * 3.0
t[0] = z
