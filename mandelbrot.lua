-- COMPUTATIONAL MANDELBROT v2

-- 80 x 40 ASCII Mandelbrot renderer
-- Designed for the restricted phia/lua subset.
-- No:
-- * functions
-- * for loops
-- * globals
-- * standard library
-- * mixed integer/float arithmetic
-- * sparse tables
-- Approximately 153600 complex iterations in the worst case.

local title = "ASCII MANDELBROT"
print("title", title)

-- Mandelbrot viewport

local max_iter = 48

local xmin = -2.40
local xmax = 1.20
local ymin = -1.80
local ymax = 1.80

local dx = 0.045
local dy = 0.09

-- Scan the image from top to bottom.
-- cy is FLOAT.
-- row is INTEGER.

local cy = ymin
local row = 0

while row <= 39 do

local line = ""

local cx = xmin
local col = 0

-- Scan from left to right.

while col <= 79 do

    -- z = 0 + c

    local zx = 0.0
    local zy = 0.0

    local iteration = 0
    local escaped = false
    local escape_iteration = 0

    -- z(n+1) = z(n)^2 + c
    --
    -- Real:
    --     x' = x*x - y*y + cx
    --
    -- Imaginary:
    --     y' = 2*x*y + cy

    while iteration < max_iter do

        local zx2 = zx * zx
        local zy2 = zy * zy

        local magnitude2 = zx2 + zy2

        if magnitude2 > 4.0 then

            -- Preserve the iteration at which the point
            -- escaped. Do NOT overwrite it with max_iter.
            escaped = true
            escape_iteration = iteration
            iteration = max_iter

        else

            local cross = zx * zy

            local next_zx = zx2 - zy2 + cx
            local next_zy = cross + cross + cy

            zx = next_zx
            zy = next_zy

            iteration = iteration + 1
        end
    end

    -- Convert escape speed into ASCII density.
    --
    -- Fast escape:
    --     . , - ~
    --
    -- Slow escape:
    --     : ; = + *
    --
    -- Very slow:
    --     # %
    --
    -- Interior:
    --     @
    -- ----------------------------------------------------

    local ch = " "

    if escaped == false then

        -- Never escaped: this pixel belongs to the
        -- Mandelbrot set itself.
        ch = "@"

    elseif escape_iteration < 2 then

        ch = "."

    elseif escape_iteration < 4 then

        ch = ","

    elseif escape_iteration < 6 then

        ch = "-"

    elseif escape_iteration < 9 then

        ch = "~"

    elseif escape_iteration < 12 then

        ch = ":"

    elseif escape_iteration < 16 then

        ch = ";"

    elseif escape_iteration < 20 then

        ch = "="

    elseif escape_iteration < 24 then

        ch = "+"

    elseif escape_iteration < 28 then

        ch = "*"

    elseif escape_iteration < 34 then

        ch = "#"

    elseif escape_iteration < 40 then

        ch = "%"

    else

        ch = "&"
    end

    line = line .. ch

    cx = cx + dx
    col = col + 1
end

print("art", line)

cy = cy + dy
row = row + 1


end

print("done", "mandelbrot calculation complete")
