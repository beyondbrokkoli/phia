-- EXPECT_PRINT: concat	|_|
-- EXPECT_PROBE: #0 tag="concat" b0 depth0 s_r6
local line = "_"
print("concat", "|" .. line .. "|")
