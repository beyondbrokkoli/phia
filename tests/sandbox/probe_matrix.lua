-- probe_matrix: t[i][j] — the child varies per OUTER trip; the feeder t[i]
-- lives in the inner loop's pre-header with the OUTER phi as key.
-- Current: declines twice over (feeder not in region; key not const).
-- Target: the per-row matrix path — feeder is region-invariant when no
-- store hits the root anywhere in the inner region, so EC + hoist the
-- feeder's handle per outer trip (hoist_ctx=1, the phase-B/F placement)
-- and fast-store the inner write.
local t = {}
local row0 = {}
row0[0] = 0
t[0] = row0
local row1 = {}
row1[0] = 0
t[1] = row1
local i = 0
while i < 2 do
    local j = 0
    while j < 4 do
        t[i][j] = 1
        j = j + 1
    end
    i = i + 1
end
