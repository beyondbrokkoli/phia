-- gauntlet_pK.lua — triple nesting: outer/middle body blocks hold only index inits;
-- only the innermost body (b8) has the table op → one upgrade, EC+HR @ b5 (depth 2).
-- EXPECT: TABLE 0 LEN 60 NZ 60 CHECKSUM 179950
-- EXPECT: fast_sets=1
-- EXPECT: fast_gets=0
-- EXPECT: dyn_sets=0
-- EXPECT: dyn_gets=0
-- EXPECT: hoists=1
local pk_t = {}
local pk_n = 60
local pk_i = 0
while pk_i < pk_n do
    local pk_j = 0
    while pk_j < pk_n do
        local pk_k = 0
        while pk_k < pk_n do
            pk_t[pk_k] = pk_k + pk_j
            pk_k = pk_k + 1
        end
        pk_j = pk_j + 1
    end
    pk_i = pk_i + 1
end
