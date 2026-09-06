-- EXPECT: TABLE 0 LEN 1 NZ 1 CHECKSUM 100
local pi_t = {}
local pi_flag = 0 < 1
local pi_k = 0
while pi_flag do
    pi_t[pi_k] = 100
    pi_flag = 0 < 0
    pi_k = pi_k + 1
end
