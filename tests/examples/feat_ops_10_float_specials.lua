-- feat_ops_10_float_specials.lua [POSITIVE — IEEE specials survive the
-- compiler: runtime inf and NaN from float division (floats never fold,
-- and f64 division never traps), NaN's non-reflexivity through == / ~=,
-- and their bit patterns through the CHECKSUM]. Lua matches: 1.0/0.0 is
-- inf, NaN ~= NaN is true. inf bits 9218868437227405312, canonical qNaN
-- 9221120237041090560, -inf as i64 -4503604182070496.
-- EXPECT: TABLE 0 LEN 1 NZ 1 CHECKSUM 11101
-- EXPECT: TABLE 1 LEN 3 NZ 3 CHECKSUM 9200854038717923328 SUM NaN
-- EXPECT: NTABLES 2
local a = 1.0
local b = 0.0
local inf = a / b
local nan = b / b
local n = 0
if inf > a then n = n + 1 end
if nan == nan then n = n + 10 end
if nan ~= nan then n = n + 100 end
if inf == inf then n = n + 1000 end
if inf >= inf then n = n + 10000 end
local w = {}
w[0] = n
local wf = {}
wf[0] = inf
wf[1] = nan
wf[2] = -inf
