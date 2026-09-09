-- gauntlet_float.lua — "The Float Gauntlet" · the tide observatory
-- Strict Phia subset: integers, floats (VALUES only), tables (nested),
-- bools, local, while, +, -, <. Floats never touch a key; every key is
-- a dense integer; every value that can be a float, is. All values are
-- dyadic rationals so every SUM below is hand-checkable in decimal.
-- Its first build caught two latent compiler bugs, now pinned here and
-- in float_14: the lowerer's Add/Sub type label, and physical-id reuse
-- across table element kinds (float tables allocate in a disjoint pool
-- since).

-- PHASE A — The Tide Ramp. Flat float fast path: tier-2 against the
-- farray, the loop-carried level is a coalesced float phi, one fast
-- store per trip through a *mut f64. 65536 samples of a rising tide.
-- Final: LEN 65536, NZ 65536, SUM 536895488 (= 65536·0.5 + 0.25·Σk).
-- EXPECT: TABLE 0 LEN 65536 NZ 65536 CHECKSUM 8174033323677450240 SUM 536895488
local fa_tide = {}
local fa_level = 0.5
local fa_hour = 0
while fa_hour < 65536 do
    fa_tide[fa_hour] = fa_level
    fa_level = fa_level + 0.25
    fa_hour = fa_hour + 1
end

-- PHASE B — The Depth Strata. The computed limit 3 + 5 const-folds to 8
-- and the const-key child converts: one minted resolution, EC to the
-- folded limit, one fast store. tier4_07's shape, float side.
-- Final: LEN 8, NZ 8, SUM 14 (8 × 1.75).
-- EXPECT: TABLE 1 LEN 1 NZ 1 CHECKSUM 3
-- EXPECT: TABLE 2 LEN 8 NZ 8 CHECKSUM -40532396646334464 SUM 14
local fb_sea = {}
local fb_stratum = {}
fb_sea[0] = fb_stratum
local fb_depth = 3 + 5
local fb_i = 0
while fb_i < fb_depth do
    fb_sea[0][fb_i] = 1.75
    fb_i = fb_i + 1
end

-- PHASE C — The Buoy Chain. Multi-hop to a FLOAT leaf: the chain
-- t[0][0] materializes root down (two minted resolutions — the
-- surviving dyn_gets), only the leaf is EC'd and hoisted (*mut f64).
-- tier4_09's shape with the float storage side.
-- Final: LEN 16, NZ 16, SUM 2 (16 × 0.125).
-- EXPECT: TABLE 3 LEN 1 NZ 1 CHECKSUM 5
-- EXPECT: TABLE 4 LEN 1 NZ 1 CHECKSUM 6
-- EXPECT: TABLE 5 LEN 16 NZ 16 CHECKSUM -2449958197289549824 SUM 2
local fc_reef = {}
local fc_mid = {}
local fc_leaf = {}
fc_leaf[0] = 0.0
fc_mid[0] = fc_leaf
fc_reef[0] = fc_mid
local fc_i = 0
while fc_i < 16 do
    fc_reef[0][0][fc_i] = 0.125
    fc_i = fc_i + 1
end

-- PHASE D — The Sensor Grid. The build loop fast-stores fresh FLOAT row
-- tables into the grid's HANDLE array (the grid's own hoisted pointer is
-- *mut i64 — handle and float tables live in disjoint id ranges since
-- the pool split; this phase is why that range exists). The fill loop is
-- the matrix path: t[i] re-resolves once per outer trip, EC + hoist
-- re-arm per row. Values shown after phase L's two epochs (0.75 each).
-- Final: grid LEN 32 of handles; each row LEN 1024, NZ 1024, SUM 768.
-- EXPECT: TABLE 6 LEN 32 NZ 32 CHECKSUM 15136
-- EXPECT: TABLE 7 LEN 1024 NZ 1024 CHECKSUM -3458764513820540928 SUM 768
-- EXPECT: TABLE 8 LEN 1024 NZ 1024 CHECKSUM -3458764513820540928 SUM 768
-- EXPECT: TABLE 9 LEN 1024 NZ 1024 CHECKSUM -3458764513820540928 SUM 768
-- EXPECT: TABLE 10 LEN 1024 NZ 1024 CHECKSUM -3458764513820540928 SUM 768
-- EXPECT: TABLE 11 LEN 1024 NZ 1024 CHECKSUM -3458764513820540928 SUM 768
-- EXPECT: TABLE 12 LEN 1024 NZ 1024 CHECKSUM -3458764513820540928 SUM 768
-- EXPECT: TABLE 13 LEN 1024 NZ 1024 CHECKSUM -3458764513820540928 SUM 768
-- EXPECT: TABLE 14 LEN 1024 NZ 1024 CHECKSUM -3458764513820540928 SUM 768
-- EXPECT: TABLE 15 LEN 1024 NZ 1024 CHECKSUM -3458764513820540928 SUM 768
-- EXPECT: TABLE 16 LEN 1024 NZ 1024 CHECKSUM -3458764513820540928 SUM 768
-- EXPECT: TABLE 17 LEN 1024 NZ 1024 CHECKSUM -3458764513820540928 SUM 768
-- EXPECT: TABLE 18 LEN 1024 NZ 1024 CHECKSUM -3458764513820540928 SUM 768
-- EXPECT: TABLE 19 LEN 1024 NZ 1024 CHECKSUM -3458764513820540928 SUM 768
-- EXPECT: TABLE 20 LEN 1024 NZ 1024 CHECKSUM -3458764513820540928 SUM 768
-- EXPECT: TABLE 21 LEN 1024 NZ 1024 CHECKSUM -3458764513820540928 SUM 768
-- EXPECT: TABLE 22 LEN 1024 NZ 1024 CHECKSUM -3458764513820540928 SUM 768
-- EXPECT: TABLE 23 LEN 1024 NZ 1024 CHECKSUM -3458764513820540928 SUM 768
-- EXPECT: TABLE 24 LEN 1024 NZ 1024 CHECKSUM -3458764513820540928 SUM 768
-- EXPECT: TABLE 25 LEN 1024 NZ 1024 CHECKSUM -3458764513820540928 SUM 768
-- EXPECT: TABLE 26 LEN 1024 NZ 1024 CHECKSUM -3458764513820540928 SUM 768
-- EXPECT: TABLE 27 LEN 1024 NZ 1024 CHECKSUM -3458764513820540928 SUM 768
-- EXPECT: TABLE 28 LEN 1024 NZ 1024 CHECKSUM -3458764513820540928 SUM 768
-- EXPECT: TABLE 29 LEN 1024 NZ 1024 CHECKSUM -3458764513820540928 SUM 768
-- EXPECT: TABLE 30 LEN 1024 NZ 1024 CHECKSUM -3458764513820540928 SUM 768
-- EXPECT: TABLE 31 LEN 1024 NZ 1024 CHECKSUM -3458764513820540928 SUM 768
-- EXPECT: TABLE 32 LEN 1024 NZ 1024 CHECKSUM -3458764513820540928 SUM 768
-- EXPECT: TABLE 33 LEN 1024 NZ 1024 CHECKSUM -3458764513820540928 SUM 768
-- EXPECT: TABLE 34 LEN 1024 NZ 1024 CHECKSUM -3458764513820540928 SUM 768
-- EXPECT: TABLE 35 LEN 1024 NZ 1024 CHECKSUM -3458764513820540928 SUM 768
-- EXPECT: TABLE 36 LEN 1024 NZ 1024 CHECKSUM -3458764513820540928 SUM 768
-- EXPECT: TABLE 37 LEN 1024 NZ 1024 CHECKSUM -3458764513820540928 SUM 768
-- EXPECT: TABLE 38 LEN 1024 NZ 1024 CHECKSUM -3458764513820540928 SUM 768
local fd_grid = {}
local fd_build = 0
while fd_build < 32 do
    fd_grid[fd_build] = {}
    fd_build = fd_build + 1
end
local fd_row = 0
while fd_row < 32 do
    local fd_col = 0
    while fd_col < 1024 do
        fd_grid[fd_row][fd_col] = 0.25
        fd_col = fd_col + 1
    end
    fd_row = fd_row + 1
end

-- PHASE E — The Grid Readback. The matrix READ: t[i] resolves per outer
-- trip, the read rides the hoisted row pointer (fast_get), the
-- accumulator is a float phi carried across both loops. Post-loop
-- witness store is dyn by design. Read happens BEFORE phase L's epochs.
-- Final: report SUM 8192 (= 32768 × 0.25).
-- EXPECT: TABLE 39 LEN 1 NZ 1 CHECKSUM 4665729213955833856 SUM 8192
local fe_total = 0.0
local fe_row = 0
while fe_row < 32 do
    local fe_col = 0
    while fe_col < 1024 do
        fe_total = fe_total + fd_grid[fe_row][fe_col]
        fe_col = fe_col + 1
    end
    fe_row = fe_row + 1
end
local fe_report = {}
fe_report[0] = fe_total

-- PHASE F — The Mixed Fleet. INT and FLOAT storage sides in one
-- program: ff_counts rides *mut i64 (slot 0 stores 0 — NZ 47), ff_rates
-- rides *mut f64. Two fast paths, two pools, one allocator; because the
-- program nests, everything renders handle-mode.
-- Final: counts LEN 48 NZ 47; rates LEN 48 NZ 48 SUM 24.
-- EXPECT: TABLE 40 LEN 48 NZ 47 CHECKSUM 73696
-- EXPECT: TABLE 41 LEN 48 NZ 48 CHECKSUM 7854277750134145024 SUM 24
local ff_counts = {}
local ff_hour = 0
while ff_hour < 48 do
    ff_counts[ff_hour] = ff_hour + ff_hour
    ff_hour = ff_hour + 1
end
local ff_rates = {}
local ff_slot = 0
while ff_slot < 48 do
    ff_rates[ff_slot] = 0.5
    ff_slot = ff_slot + 1
end

-- PHASE G — The Stranded Sensor. n is table-fed (src[0] + 1): no folder
-- can prove the limit positive, so the whole loop stays dyn — the
-- computed-limit decline arm living inside a fast program.
-- Final: LEN 8, NZ 8, SUM 6 (8 × 0.75).
-- EXPECT: TABLE 42 LEN 1 NZ 1 CHECKSUM 7
-- EXPECT: TABLE 43 LEN 1 NZ 1 CHECKSUM 45
-- EXPECT: TABLE 44 LEN 8 NZ 8 CHECKSUM -243194379878006784 SUM 6
local fg_relief = {}
fg_relief[0] = 7
local fg_buoy = {}
local fg_inner = {}
fg_inner[0] = 0.0
fg_buoy[0] = fg_inner
local fg_n = fg_relief[0] + 1
local fg_i = 0
while fg_i < fg_n do
    fg_buoy[0][fg_i] = 0.75
    fg_i = fg_i + 1
end

-- PHASE H — The Phantom Row. The limit is positive but the induction
-- var ENTERS above it: zero trips, and the dyn path runs silent — the
-- entry-value gate keeps the mint's pre-header panic out of a loop that
-- never runs. The priming zero at slot 0 is all that remains.
-- Final: LEN 1, NZ 0, SUM 0.
-- EXPECT: TABLE 45 LEN 1 NZ 1 CHECKSUM 47
-- EXPECT: TABLE 46 LEN 1 NZ 0 CHECKSUM 0 SUM 0
local fh_ghost = {}
local fh_row = {}
fh_row[0] = 0.0
fh_ghost[0] = fh_row
local fh_i = 100
while fh_i < 8 do
    fh_ghost[0][fh_i] = 0.5
    fh_i = fh_i + 1
end

-- PHASE I — The Rebound Buoy. The loop rebinds t[0] to a spare table:
-- region_stored_roots declines everything, and the runtime shuffle is
-- the witness — iteration 0 writes the original (slot 0 only),
-- iterations 1..7 write the spare through the rebound slot.
-- Final: sea holds spare's handle after the rebind; inner LEN 1
-- SUM 0.625; spare LEN 8 NZ 7 SUM 4.375.
-- EXPECT: TABLE 47 LEN 1 NZ 1 CHECKSUM 50
-- EXPECT: TABLE 48 LEN 1 NZ 1 CHECKSUM 4603804719079489536 SUM 0.625
-- EXPECT: TABLE 49 LEN 8 NZ 7 CHECKSUM -4887531495603830784 SUM 4.375
local fi_sea = {}
local fi_inner = {}
fi_inner[0] = 0.0
fi_sea[0] = fi_inner
local fi_spare = {}
fi_spare[0] = 0.0
local fi_i = 0
while fi_i < 8 do
    fi_sea[0][fi_i] = 0.625
    fi_sea[0] = fi_spare
    fi_i = fi_i + 1
end

-- PHASE J — The Aliased Stream. fj_b = fj_a is a table Move, so both
-- feeders trace to ONE root and the mint dedup gives them a single
-- shared child materialization; two fast stores, one dyn_get, one
-- hoist. 0.875 wins every slot (last store of the pair).
-- Final: LEN 8, NZ 8, SUM 7.
-- EXPECT: TABLE 50 LEN 1 NZ 1 CHECKSUM 52
-- EXPECT: TABLE 51 LEN 8 NZ 8 CHECKSUM -202661983231672320 SUM 7
local fj_a = {}
local fj_inner = {}
fj_inner[0] = 0.0
fj_a[0] = fj_inner
local fj_b = fj_a
local fj_i = 0
while fj_i < 8 do
    fj_a[0][fj_i] = 0.375
    fj_b[0][fj_i] = 0.875
    fj_i = fj_i + 1
end

-- PHASE K — The Two-Hop Archive. The matrix x multi-hop composition: a
-- MIXED chain — outer-phi key at the top hop (t[i] re-resolves per
-- outer trip), const key at the leaf hop ([0]), float leaf hoisted per
-- row. The attach loop stays dyn by shape: its key is its OWN phi, so
-- the child varies per trip. Each leaf: LEN 8, SUM 0.5 (8 × 0.0625).
-- EXPECT: TABLE 52 LEN 4 NZ 4 CHECKSUM 560
-- EXPECT: TABLE 53 LEN 1 NZ 1 CHECKSUM 58
-- EXPECT: TABLE 54 LEN 1 NZ 1 CHECKSUM 59
-- EXPECT: TABLE 55 LEN 1 NZ 1 CHECKSUM 60
-- EXPECT: TABLE 56 LEN 1 NZ 1 CHECKSUM 61
-- EXPECT: TABLE 57 LEN 8 NZ 8 CHECKSUM -810647932926689280 SUM 0.5
-- EXPECT: TABLE 58 LEN 8 NZ 8 CHECKSUM -810647932926689280 SUM 0.5
-- EXPECT: TABLE 59 LEN 8 NZ 8 CHECKSUM -810647932926689280 SUM 0.5
-- EXPECT: TABLE 60 LEN 8 NZ 8 CHECKSUM -810647932926689280 SUM 0.5
local fk_arc = {}
local fk_build = 0
while fk_build < 4 do
    fk_arc[fk_build] = {}
    fk_build = fk_build + 1
end
local fk_attach = 0
while fk_attach < 4 do
    fk_arc[fk_attach][0] = {}
    fk_attach = fk_attach + 1
end
local fk_row = 0
while fk_row < 4 do
    local fk_col = 0
    while fk_col < 8 do
        fk_arc[fk_row][0][fk_col] = 0.0625
        fk_col = fk_col + 1
    end
    fk_row = fk_row + 1
end

-- PHASE L — The Grand Reconciliation. Two read-modify-write epochs over
-- the phase-D grid at the file's deepest hoist (depth 2: the row's EC +
-- hoist re-arm in the middle body, 32 times per epoch, underwriting 64
-- fast reads and 64 fast writes each). The audit re-sums post-epochs;
-- the three witnesses are dyn by design (post-loop code is never a
-- scan candidate). Grid rows finish at 0.75 everywhere.
-- Final: probe SUM 24577.5 (= 0.75 + 0.75 + 32768 × 0.75).
-- EXPECT: TABLE 61 LEN 3 NZ 3 CHECKSUM -9061242450269437952 SUM 24577.5
-- EXPECT: NTABLES 62
-- EXPECT: fast_sets=12
-- EXPECT: fast_gets=3
-- EXPECT: dyn_sets=23
-- EXPECT: dyn_gets=19
-- EXPECT: hoists=13
-- EXPECT: hoist_ctx=0,0,0,0,1,0,0,0,0,0,1,0,2
local fl_epoch = 0
while fl_epoch < 2 do
    local fl_row = 0
    while fl_row < 32 do
        local fl_col = 0
        while fl_col < 1024 do
            fd_grid[fl_row][fl_col] = fd_grid[fl_row][fl_col] + 0.25
            fl_col = fl_col + 1
        end
        fl_row = fl_row + 1
    end
    fl_epoch = fl_epoch + 1
end
local fl_audit = 0.0
local fl_arow = 0
while fl_arow < 32 do
    local fl_acol = 0
    while fl_acol < 1024 do
        fl_audit = fl_audit + fd_grid[fl_arow][fl_acol]
        fl_acol = fl_acol + 1
    end
    fl_arow = fl_arow + 1
end
local fl_probe = {}
fl_probe[0] = fd_grid[0][0]
fl_probe[1] = fd_grid[31][1023]
fl_probe[2] = fl_audit
