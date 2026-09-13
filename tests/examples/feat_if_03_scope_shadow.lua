-- feat_if_03_scope_shadow.lua [POSITIVE — arm scoping and table rebind].
-- `local s` in the then-arm and again in the else-arm shadow the outer s
-- and die at their `end`s (the lowerer's scope snapshot keeps arm renames
-- from leaking across arms); the not-taken arm leaves outer s untouched.
-- A table REBIND inside the taken arm re-targets the outer variable
-- through a table-typed join phi — stores after the if land in the new
-- table. Expected: s=1 q=2 tb[1]=10; NTABLES counts tb, nb (born in the
-- arm), w.
-- EXPECT: TABLE 0 LEN 1 NZ 1 CHECKSUM 7
-- EXPECT: TABLE 1 LEN 2 NZ 2 CHECKSUM 29
-- EXPECT: TABLE 2 LEN 3 NZ 3 CHECKSUM 35
-- EXPECT: NTABLES 3
local s = 1
local t = 5
if t > 1 then
    local s = 100
    s = s + 1
end
if t > 10 then
    s = 50
else
    local s = 200
    s = s + 1
end
local q = 0
if t > 1 then
    q = s + 1
end
local tb = {}
tb[0] = 7
if t > 1 then
    local nb = {}
    nb[0] = 9
    tb = nb
end
tb[1] = tb[0] + 1
local w = {}
w[0] = s
w[1] = q
w[2] = tb[1]
