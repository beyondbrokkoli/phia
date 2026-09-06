-- EXPECT: TABLE 0 LEN 400 NZ 200 CHECKSUM 10666600
local pa_t = {}
local pa_n = 400
local pa_i = 0
while pa_i < pa_n do
    pa_t[pa_i] = pa_i + 1
    pa_i = pa_i + 2
end
