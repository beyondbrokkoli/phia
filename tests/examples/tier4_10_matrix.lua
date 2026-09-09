-- tier4_10_matrix.lua  [POSITIVE — the per-row matrix path]
-- t[i][j]: the child varies per OUTER trip, so the feeder t[i] can never
-- be materialized above the outer loop — but it is INVARIANT for the
-- INNER loop: its key is the enclosing phi (SSA-stable, defined outside
-- the inner region and header) and the root t receives no store inside
-- the inner region (a t[k]=v there would rebind the slot between trips,
-- region_stored_roots declines that shape). Tier-4 mints ONE resolution
-- t[i] in the inner pre-header — which lives INSIDE the outer loop, so
-- the child re-resolves once per outer trip — then EC + HoistRawPtr land
-- right after it (hoist_ctx=1, the phase-B/F placement): the pointer
-- cache is per-ROW, re-armed every outer trip. The in-loop feeder dies;
-- the store is SetTableFast. The sound first landing of the matrix
-- family — a uniform EC above the outer loop (true rectangularity) needs
-- an equal-child-lengths proof and stays future work.
-- EXPECT: TABLE 0 LEN 2 NZ 2 CHECKSUM 8
-- EXPECT: TABLE 1 LEN 4 NZ 4 CHECKSUM 10
-- EXPECT: TABLE 2 LEN 4 NZ 4 CHECKSUM 10
-- EXPECT: NTABLES 3
-- EXPECT: fast_sets=1
-- EXPECT: fast_gets=0
-- EXPECT: dyn_sets=4
-- EXPECT: dyn_gets=1
-- EXPECT: hoists=1
-- EXPECT: hoist_ctx=1
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
