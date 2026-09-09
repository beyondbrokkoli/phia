-- probe_matrix_float: the matrix shape with FLOAT children — stores 0.25
-- through a *mut f64 hoisted per outer trip. Composition check: tier-4 x
-- float storage kinds.
local t = {}
local row0 = {}
row0[0] = 0.5
t[0] = row0
local row1 = {}
row1[0] = 0.5
t[1] = row1
local i = 0
while i < 2 do
    local j = 0
    while j < 4 do
        t[i][j] = 0.25
        j = j + 1
    end
    i = i + 1
end
