-- EXPECT: TABLE 0 LEN 31 NZ 31 CHECKSUM 2325
-- EXPECT: TABLE 1 LEN 1 NZ 1 CHECKSUM 30
local pd_t = {}
local pd_fill = 0
while pd_fill < 30 do
    pd_t[pd_fill] = 1
    pd_fill = pd_fill + 1
end
pd_t[30] = 60
local pd_lim = 60
local pd_i = 0
while pd_t[pd_i] < pd_lim do
    pd_i = pd_i + 1
end
local pd_w = {}
pd_w[0] = pd_i
