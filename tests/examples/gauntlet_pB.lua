-- EXPECT: TABLE 0 LEN 299 NZ 299 CHECKSUM 8955050
local pb_t = {}
local pb_n = 300
local pb_i = 0
while pb_i < pb_n do
    local pb_j = 0
    while pb_j < pb_i do
        pb_t[pb_j] = pb_j + 1
        pb_j = pb_j + 1
    end
    pb_i = pb_i + 1
end
