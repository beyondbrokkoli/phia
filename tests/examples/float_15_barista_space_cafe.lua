-- float_15_barista_space_cafe.lua — "The Barista's Breakdown"
-- A benchmark simulating a disastrously mismanaged interstellar coffee shop
-- run entirely by goblins who only know how to use floating point numbers.
-- Formerly the project-root main.lua; now a pinned float-path invariant test.
-- The program creates 7070 tables (grid rows and cabinet leaves are tables
-- too), so the pins below anchor the stats, the count, and one structural
-- witness per phase rather than all 7070 lines.

-- EXPECT: fast_sets=12
-- EXPECT: fast_gets=7
-- EXPECT: dyn_sets=23
-- EXPECT: dyn_gets=15
-- EXPECT: hoists=17
-- EXPECT: hoist_ctx=0,0,0,0,0,1,0,1,0,0,0,0,1,1,0,2,1
-- EXPECT: NTABLES 7070

-- PHASE A — The Caffeination Ramp.
-- The morning rush begins. We need to pour 150 million shots of espresso.
-- The goblins just keep adding a quarter-shot every millisecond.
-- Manager Note: EXPECT 150M cups of pure liquid anxiety.
-- EXPECT: TABLE 0 LEN 150000000 NZ 150000000 CHECKSUM -5463692897276657664 SUM 2812500054277216
local goblin_espresso = {}
local espresso_level = 0.5
local time_clock = 0
while time_clock < 150000000 do
    goblin_espresso[time_clock] = espresso_level
    espresso_level = espresso_level + 0.25
    time_clock = time_clock + 1
end

-- PHASE B — The Sugar Strata.
-- A customer asked for "a little sugar." The goblins computed 2 million + 8 million
-- and decided 10 million cubes of pure sucrose was appropriate.
-- Manager Note: EXPECT the spoon to dissolve.
-- EXPECT: TABLE 1 LEN 1 NZ 1 CHECKSUM 3
-- EXPECT: TABLE 2 LEN 10000000 NZ 10000000 CHECKSUM 5980780305148018688 SUM 17500000
local sugar_void = {}
local sugar_cube = {}
sugar_void[0] = sugar_cube
local sugar_limit = 2000000 + 8000000
local cube_idx = 0
while cube_idx < sugar_limit do
    sugar_void[0][cube_idx] = 1.75
    cube_idx = cube_idx + 1
end

-- PHASE C — The Syrup Chain.
-- Multi-hop ordering. The customer wants vanilla, inside caramel, inside hazelnut.
-- We do this 25 million times because the pump is stuck.
-- Manager Note: EXPECT a sticky floor.
-- EXPECT: TABLE 3 LEN 1 NZ 1 CHECKSUM 5
-- EXPECT: TABLE 4 LEN 1 NZ 1 CHECKSUM 6
-- EXPECT: TABLE 5 LEN 25000000 NZ 25000000 CHECKSUM -576460752303423488 SUM 3125000
local syrup_hazelnut = {}
local syrup_caramel = {}
local syrup_vanilla = {}
syrup_vanilla[0] = 0.0
syrup_caramel[0] = syrup_vanilla
syrup_hazelnut[0] = syrup_caramel
local pump_count = 0
while pump_count < 25000000 do
    syrup_hazelnut[0][0][pump_count] = 0.125
    pump_count = pump_count + 1
end

-- PHASE D — The Counter Grid.
-- Laying out 5000 rows and 5000 columns of paper cups on the counter.
-- This is roughly 25 million cups. The store is only 400 square feet.
-- Manager Note: EXPECT a spatial anomaly.
-- EXPECT: TABLE 6 LEN 5000 NZ 5000 CHECKSUM 41766685000
-- EXPECT: TABLE 7 LEN 5000 NZ 5000 CHECKSUM -4978729388058083328 SUM 16250
-- EXPECT: TABLE 5006 LEN 5000 NZ 5000 CHECKSUM -4978729388058083328 SUM 16250
local counter_cups = {}
local build_row = 0
while build_row < 5000 do
    counter_cups[build_row] = {}
    build_row = build_row + 1
end
local fill_row = 0
while fill_row < 5000 do
    local fill_col = 0
    while fill_col < 5000 do
        counter_cups[fill_row][fill_col] = 0.25
        fill_col = fill_col + 1
    end
    fill_row = fill_row + 1
end

-- PHASE E — The Inventory Readback.
-- The manager forces a shift lead to count all 25 million cups of coffee
-- before the health inspector arrives.
-- Manager Note: EXPECT the shift lead to quit.
-- EXPECT: TABLE 5007 LEN 1 NZ 1 CHECKSUM 4708468897374797824 SUM 6250000
local spilled_total = 0.0
local count_row = 0
while count_row < 5000 do
    local count_col = 0
    while count_col < 5000 do
        spilled_total = spilled_total + counter_cups[count_row][count_col]
        count_col = count_col + 1
    end
    count_row = count_row + 1
end
local audit_clipboard = {}
audit_clipboard[0] = spilled_total

-- PHASE F — The Mixed Fleet.
-- 15 million regular orders and 15 million decaf orders processed simultaneously.
-- Nobody labeled the cups.
-- Manager Note: EXPECT widespread palpitations.
-- EXPECT: TABLE 5008 LEN 15000000 NZ 14999999 CHECKSUM -502776992575297152
-- EXPECT: TABLE 5009 LEN 15000000 NZ 15000000 CHECKSUM -2017612633061982208 SUM 7500000
local regular_orders = {}
local reg_idx = 0
while reg_idx < 15000000 do
    regular_orders[reg_idx] = reg_idx + reg_idx
    reg_idx = reg_idx + 1
end
local decaf_orders = {}
local decaf_idx = 0
while decaf_idx < 15000000 do
    decaf_orders[decaf_idx] = 0.5
    decaf_idx = decaf_idx + 1
end

-- PHASE G — The Stranded Barista.
-- The shift was supposed to end, but the replacement never showed up.
-- The barista is trapped serving 12 million ghostly late-night customers.
-- EXPECT: TABLE 5010 LEN 1 NZ 1 CHECKSUM 11999999
-- EXPECT: TABLE 5011 LEN 1 NZ 1 CHECKSUM 5013
-- EXPECT: TABLE 5012 LEN 12000000 NZ 12000000 CHECKSUM -4899916394579099648 SUM 9000000
local shift_end = {}
shift_end[0] = 11999999
local barista_station = {}
local milk_frother = {}
milk_frother[0] = 0.0
barista_station[0] = milk_frother
local eternal_shift = shift_end[0] + 1
local customer_idx = 0
while customer_idx < eternal_shift do
    barista_station[0][customer_idx] = 0.75
    customer_idx = customer_idx + 1
end

-- PHASE H — The Phantom Customer.
-- A customer arrived 100 years ago, but the store closes in 8 minutes.
-- The loop never runs. The zero is all that remains of their existence.
-- EXPECT: TABLE 5013 LEN 1 NZ 1 CHECKSUM 5015
-- EXPECT: TABLE 5014 LEN 1 NZ 0 CHECKSUM 0 SUM 0
local phantom_order = {}
local ghostly_cup = {}
ghostly_cup[0] = 0.0
phantom_order[0] = ghostly_cup
local time_travel_idx = 100
while time_travel_idx < 8 do
    phantom_order[0][time_travel_idx] = 0.5
    time_travel_idx = time_travel_idx + 1
end

-- PHASE I — The Rebound Mug.
-- The health inspector found a bug in the mug, so mid-pour, the barista switches
-- to a spare mug for the remaining 10 million drops.
-- EXPECT: TABLE 5015 LEN 1 NZ 1 CHECKSUM 5018
-- EXPECT: TABLE 5016 LEN 1 NZ 1 CHECKSUM 4603804719079489536 SUM 0.625
-- EXPECT: TABLE 5017 LEN 10000000 NZ 9999999 CHECKSUM 368169269537538048 SUM 6249999.375
local dirty_mug = {}
local dirty_bottom = {}
dirty_bottom[0] = 0.0
dirty_mug[0] = dirty_bottom
local clean_spare = {}
clean_spare[0] = 0.0
local drop_idx = 0
while drop_idx < 10000000 do
    dirty_mug[0][drop_idx] = 0.625
    dirty_mug[0] = clean_spare
    drop_idx = drop_idx + 1
end

-- PHASE J — The Aliased Stream.
-- Bob and Steve are both pouring into the exact same cup 12 million times
-- because communication has entirely broken down. Steve wins.
-- EXPECT: TABLE 5018 LEN 1 NZ 1 CHECKSUM 5020
-- EXPECT: TABLE 5019 LEN 12000000 NZ 12000000 CHECKSUM -1008806316530991104 SUM 10500000
local barista_bob = {}
local shared_cup = {}
shared_cup[0] = 0.0
barista_bob[0] = shared_cup
local barista_steve = barista_bob
local pour_duel = 0
while pour_duel < 12000000 do
    barista_bob[0][pour_duel] = 0.375
    barista_steve[0][pour_duel] = 0.875
    pour_duel = pour_duel + 1
end

-- PHASE K — The Two-Hop Archive.
-- The filing system for ancient recipes. It's a 3D matrix because paper was too cheap.
-- 1024 rows * 8192 columns = over 8 million dusty files.
-- EXPECT: TABLE 5020 LEN 1024 NZ 1024 CHECKSUM 2993459200
-- EXPECT: TABLE 5021 LEN 1 NZ 1 CHECKSUM 6046
-- EXPECT: TABLE 6045 LEN 8192 NZ 8192 CHECKSUM 0 SUM 512
-- EXPECT: TABLE 7068 LEN 8192 NZ 8192 CHECKSUM 0 SUM 512
local recipe_cabinet = {}
local cab_build = 0
while cab_build < 1024 do
    recipe_cabinet[cab_build] = {}
    cab_build = cab_build + 1
end
local cab_attach = 0
while cab_attach < 1024 do
    recipe_cabinet[cab_attach][0] = {}
    cab_attach = cab_attach + 1
end
local cab_row = 0
while cab_row < 1024 do
    local cab_col = 0
    while cab_col < 8192 do
        recipe_cabinet[cab_row][0][cab_col] = 0.0625
        cab_col = cab_col + 1
    end
    cab_row = cab_row + 1
end

-- PHASE L — The Grand Reconciliation.
-- The manager realizes everyone forgot to put lids on the 25 million cups in Phase D.
-- Everyone has to redo the entire grid. Not once, but 12 times because the lids keep popping off.
-- This loops 12 * 5000 * 5000 = 300,000,000 times.
-- Manager Note: EXPECT the CPU fan to sound like a jet engine.
local lid_redo = 0
while lid_redo < 12 do
    local lid_row = 0
    while lid_row < 5000 do
        local lid_col = 0
        while lid_col < 5000 do
            counter_cups[lid_row][lid_col] = counter_cups[lid_row][lid_col] + 0.25
            lid_col = lid_col + 1
        end
        lid_row = lid_row + 1
    end
    lid_redo = lid_redo + 1
end

-- The Final Audit. The manager gives up, counts the lukewarm bean juice one last time,
-- and files for interstellar bankruptcy.
-- EXPECT: TABLE 7069 LEN 3 NZ 3 CHECKSUM -8874310828782780416 SUM 81250006.5
local final_despair = 0.0
local final_row = 0
while final_row < 5000 do
    local final_col = 0
    while final_col < 5000 do
        final_despair = final_despair + counter_cups[final_row][final_col]
        final_col = final_col + 1
    end
    final_row = final_row + 1
end

local bankruptcy_filing = {}
bankruptcy_filing[0] = counter_cups[0][0]
bankruptcy_filing[1] = counter_cups[4999][4999]
bankruptcy_filing[2] = final_despair
