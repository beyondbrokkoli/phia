# Phia 🌙

> 🌟 **Huge Shoutout to [logos](https://github.com/maciejhirsz/logos)!** 🌟
> This project would simply not be possible without the `logos` crate.

## ✨ Supported Features

**Data Types**
- ✅ Integers
- ✅ "Tables" (keys and values must be integers)

**Control Flow**
- ✅ `while`
- ✅ `do` / `end` blocks
- ✅ Local variable scoping (`local`)

**Operators**
- ✅ Addition (`+`)
- ✅ Subtraction (`-`)
- ✅ Less Than (`<`)
- ✅ Assignment (`=`)

**Table Operations**
- ✅ Table creation (`{}`)
- ✅ Array access (`t[1]`)
- ✅ Table assignment (`t[i] = value`)

# Input
```lua
-- space_cafe_gauntlet.lua — "The Barista's Breakdown"
-- A benchmark simulating a disastrously mismanaged interstellar coffee shop
-- run entirely by goblins who only know how to use floating point numbers.

-- PHASE A — The Caffeination Ramp.
-- The morning rush begins. We need to pour 150 million shots of espresso.
-- The goblins just keep adding a quarter-shot every millisecond.
-- Manager Note: EXPECT 150M cups of pure liquid anxiety.
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
```

# Output
```rust
// target/release/build/phia-*/out/baked_native.rs

use crate::memory::Table;

#[allow(unused_variables, unused_mut, unused_assignments)]
pub fn run_baked() -> Vec<Box<Table>> {
    let mut i_r375 = 0i64;
    let mut i_r376 = 0i64;
    let mut i_r377 = 0i64;
    let mut i_r378 = 0i64;
    let mut b_r375 = false;
    let mut f_r401 = 0f64;
    let mut f_r402 = 0f64;
    let mut f_r403 = 0f64;
    let mut f_r404 = 0f64;
    let mut t_r375 = 0i64;
    let mut p_r375: *mut i64 = std::ptr::null_mut();
    let mut len_r375 = 0usize;
    let mut t_r376 = 0i64;
    let mut p_r376: *mut i64 = std::ptr::null_mut();
    let mut len_r376 = 0usize;
    let mut t_r377 = 0i64;
    let mut p_r377: *mut i64 = std::ptr::null_mut();
    let mut len_r377 = 0usize;
    let mut t_r431 = 0i64;
    let mut p_r431: *mut f64 = std::ptr::null_mut();
    let mut len_r431 = 0usize;
    let mut t_r432 = 0i64;
    let mut p_r432: *mut f64 = std::ptr::null_mut();
    let mut len_r432 = 0usize;
    let mut tables = Vec::<Box<Table>>::with_capacity(128);

    tables.push(Box::new(Table::new_float()));
    t_r431 = tables.len() as i64;
    f_r401 = 0.5;
    let lim = 150000000;
    if lim > 0 {
        if t_r431 == 0 {
            panic!("Runtime Error: table is nil");
        }
        let t = match tables.get_mut((t_r431 - 1) as usize) {
            Some(t) => &mut **t,
            None => panic!("Runtime Error: table is nil"),
        };
        if (lim as usize) > t.farray.len() {
            t.farray.resize(lim as usize, 0.0);
        }
    }
    if t_r431 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r431 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    len_r431 = t.farray.len();
    p_r431 = t.farray.as_mut_ptr();
    f_r402 = f_r401;
    i_r375 = 0;
    loop {
        b_r375 = i_r375 < 150000000;
        if b_r375 {
            let k = i_r375;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r431 {
                unsafe {
                    *p_r431.add(k as usize) = f_r402;
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            f_r401 = 0.25;
            f_r402 = f_r402 + f_r401;
            i_r375 = i_r375 + 1;
        } else {
            break;
        }
    }
    tables.push(Box::new(Table::new()));
    t_r375 = tables.len() as i64;
    tables.push(Box::new(Table::new_float()));
    t_r431 = tables.len() as i64;
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r375 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r375 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = t_r431;
    }
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r375 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get((t_r375 - 1) as usize) {
        Some(t) => &**t,
        None => panic!("Runtime Error: table is nil"),
    };
    t_r431 = if idx < t.array.len() {
        unsafe { *t.array.get_unchecked(idx) }
    } else {
        0
    };
    let lim = 10000000;
    if lim > 0 {
        if t_r431 == 0 {
            panic!("Runtime Error: table is nil");
        }
        let t = match tables.get_mut((t_r431 - 1) as usize) {
            Some(t) => &mut **t,
            None => panic!("Runtime Error: table is nil"),
        };
        if (lim as usize) > t.farray.len() {
            t.farray.resize(lim as usize, 0.0);
        }
    }
    if t_r431 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r431 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    len_r431 = t.farray.len();
    p_r431 = t.farray.as_mut_ptr();
    i_r375 = 0;
    loop {
        b_r375 = i_r375 < 10000000;
        if b_r375 {
            f_r401 = 1.75;
            let k = i_r375;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r431 {
                unsafe {
                    *p_r431.add(k as usize) = f_r401;
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            i_r375 = i_r375 + 1;
        } else {
            break;
        }
    }
    tables.push(Box::new(Table::new()));
    t_r375 = tables.len() as i64;
    tables.push(Box::new(Table::new()));
    t_r376 = tables.len() as i64;
    tables.push(Box::new(Table::new_float()));
    t_r431 = tables.len() as i64;
    f_r401 = 0.0;
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r431 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r431 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    if idx >= t.farray.len() {
        t.farray.resize(idx + 1, 0.0);
    }
    unsafe {
        *t.farray.get_unchecked_mut(idx) = f_r401;
    }
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r376 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r376 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = t_r431;
    }
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r375 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r375 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = t_r376;
    }
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r375 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get((t_r375 - 1) as usize) {
        Some(t) => &**t,
        None => panic!("Runtime Error: table is nil"),
    };
    t_r376 = if idx < t.array.len() {
        unsafe { *t.array.get_unchecked(idx) }
    } else {
        0
    };
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r376 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get((t_r376 - 1) as usize) {
        Some(t) => &**t,
        None => panic!("Runtime Error: table is nil"),
    };
    t_r431 = if idx < t.array.len() {
        unsafe { *t.array.get_unchecked(idx) }
    } else {
        0
    };
    let lim = 25000000;
    if lim > 0 {
        if t_r431 == 0 {
            panic!("Runtime Error: table is nil");
        }
        let t = match tables.get_mut((t_r431 - 1) as usize) {
            Some(t) => &mut **t,
            None => panic!("Runtime Error: table is nil"),
        };
        if (lim as usize) > t.farray.len() {
            t.farray.resize(lim as usize, 0.0);
        }
    }
    if t_r431 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r431 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    len_r431 = t.farray.len();
    p_r431 = t.farray.as_mut_ptr();
    i_r375 = 0;
    loop {
        b_r375 = i_r375 < 25000000;
        if b_r375 {
            f_r401 = 0.125;
            let k = i_r375;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r431 {
                unsafe {
                    *p_r431.add(k as usize) = f_r401;
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            i_r375 = i_r375 + 1;
        } else {
            break;
        }
    }
    tables.push(Box::new(Table::new()));
    t_r376 = tables.len() as i64;
    let lim = 5000;
    if lim > 0 {
        if t_r376 == 0 {
            panic!("Runtime Error: table is nil");
        }
        let t = match tables.get_mut((t_r376 - 1) as usize) {
            Some(t) => &mut **t,
            None => panic!("Runtime Error: table is nil"),
        };
        if (lim as usize) > t.array.len() {
            t.array.resize(lim as usize, 0);
        }
    }
    if t_r376 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r376 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    len_r376 = t.array.len();
    p_r376 = t.array.as_mut_ptr();
    i_r375 = 0;
    loop {
        b_r375 = i_r375 < 5000;
        if b_r375 {
            tables.push(Box::new(Table::new_float()));
            t_r431 = tables.len() as i64;
            let k = i_r375;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r376 {
                unsafe {
                    *p_r376.add(k as usize) = t_r431;
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            i_r375 = i_r375 + 1;
        } else {
            break;
        }
    }
    i_r375 = 0;
    loop {
        b_r375 = i_r375 < 5000;
        if b_r375 {
            let k = i_r375;
            if k < 0 {
                panic!("Runtime Error: Negative table index");
            }
            let idx = k as usize;
            if t_r376 == 0 {
                panic!("Runtime Error: table is nil");
            }
            let t = match tables.get((t_r376 - 1) as usize) {
                Some(t) => &**t,
                None => panic!("Runtime Error: table is nil"),
            };
            t_r431 = if idx < t.array.len() {
                unsafe { *t.array.get_unchecked(idx) }
            } else {
                0
            };
            let lim = 5000;
            if lim > 0 {
                if t_r431 == 0 {
                    panic!("Runtime Error: table is nil");
                }
                let t = match tables.get_mut((t_r431 - 1) as usize) {
                    Some(t) => &mut **t,
                    None => panic!("Runtime Error: table is nil"),
                };
                if (lim as usize) > t.farray.len() {
                    t.farray.resize(lim as usize, 0.0);
                }
            }
            if t_r431 == 0 {
                panic!("Runtime Error: table is nil");
            }
            let t = match tables.get_mut((t_r431 - 1) as usize) {
                Some(t) => &mut **t,
                None => panic!("Runtime Error: table is nil"),
            };
            len_r431 = t.farray.len();
            p_r431 = t.farray.as_mut_ptr();
            i_r376 = 0;
            loop {
                b_r375 = i_r376 < 5000;
                if b_r375 {
                    f_r401 = 0.25;
                    let k = i_r376;
                    if k < 0 {
                        panic!("Runtime Error: Negative index in fast path");
                    }
                    if (k as usize) < len_r431 {
                        unsafe {
                            *p_r431.add(k as usize) = f_r401;
                        }
                    } else {
                        panic!("optimizer invariant violated: fast-path bounds check failed");
                    }
                    i_r376 = i_r376 + 1;
                } else {
                    break;
                }
            }
            i_r375 = i_r375 + 1;
        } else {
            break;
        }
    }
    f_r401 = 0.0;
    let lim = 5000;
    if lim > 0 {
        if t_r376 == 0 {
            panic!("Runtime Error: table is nil");
        }
        let t = match tables.get_mut((t_r376 - 1) as usize) {
            Some(t) => &mut **t,
            None => panic!("Runtime Error: table is nil"),
        };
        if (lim as usize) > t.array.len() {
            t.array.resize(lim as usize, 0);
        }
    }
    if t_r376 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r376 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    len_r376 = t.array.len();
    p_r376 = t.array.as_mut_ptr();
    f_r402 = f_r401;
    i_r377 = 0;
    loop {
        b_r375 = i_r377 < 5000;
        if b_r375 {
            i_r376 = 0;
            loop {
                b_r375 = i_r376 < 5000;
                if b_r375 {
                    let k = i_r377;
                    if k < 0 {
                        panic!("Runtime Error: Negative index in fast path");
                    }
                    if (k as usize) < len_r376 {
                        t_r431 = unsafe { *p_r376.add(k as usize) };
                    } else {
                        panic!("optimizer invariant violated: fast-path bounds check failed");
                    }
                    let k = i_r376;
                    if k < 0 {
                        panic!("Runtime Error: Negative table index");
                    }
                    let idx = k as usize;
                    if t_r431 == 0 {
                        panic!("Runtime Error: table is nil");
                    }
                    let t = match tables.get((t_r431 - 1) as usize) {
                        Some(t) => &**t,
                        None => panic!("Runtime Error: table is nil"),
                    };
                    f_r401 = if idx < t.farray.len() {
                        unsafe { *t.farray.get_unchecked(idx) }
                    } else {
                        0.0
                    };
                    f_r402 = f_r402 + f_r401;
                    i_r376 = i_r376 + 1;
                } else {
                    break;
                }
            }
            i_r377 = i_r377 + 1;
        } else {
            break;
        }
    }
    tables.push(Box::new(Table::new_float()));
    t_r431 = tables.len() as i64;
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r431 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r431 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    if idx >= t.farray.len() {
        t.farray.resize(idx + 1, 0.0);
    }
    unsafe {
        *t.farray.get_unchecked_mut(idx) = f_r402;
    }
    tables.push(Box::new(Table::new()));
    t_r375 = tables.len() as i64;
    let lim = 15000000;
    if lim > 0 {
        if t_r375 == 0 {
            panic!("Runtime Error: table is nil");
        }
        let t = match tables.get_mut((t_r375 - 1) as usize) {
            Some(t) => &mut **t,
            None => panic!("Runtime Error: table is nil"),
        };
        if (lim as usize) > t.array.len() {
            t.array.resize(lim as usize, 0);
        }
    }
    if t_r375 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r375 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    len_r375 = t.array.len();
    p_r375 = t.array.as_mut_ptr();
    i_r375 = 0;
    loop {
        b_r375 = i_r375 < 15000000;
        if b_r375 {
            i_r376 = i_r375 + i_r375;
            let k = i_r375;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r375 {
                unsafe {
                    *p_r375.add(k as usize) = i_r376;
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            i_r375 = i_r375 + 1;
        } else {
            break;
        }
    }
    tables.push(Box::new(Table::new_float()));
    t_r431 = tables.len() as i64;
    let lim = 15000000;
    if lim > 0 {
        if t_r431 == 0 {
            panic!("Runtime Error: table is nil");
        }
        let t = match tables.get_mut((t_r431 - 1) as usize) {
            Some(t) => &mut **t,
            None => panic!("Runtime Error: table is nil"),
        };
        if (lim as usize) > t.farray.len() {
            t.farray.resize(lim as usize, 0.0);
        }
    }
    if t_r431 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r431 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    len_r431 = t.farray.len();
    p_r431 = t.farray.as_mut_ptr();
    i_r376 = 0;
    loop {
        b_r375 = i_r376 < 15000000;
        if b_r375 {
            f_r401 = 0.5;
            let k = i_r376;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r431 {
                unsafe {
                    *p_r431.add(k as usize) = f_r401;
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            i_r376 = i_r376 + 1;
        } else {
            break;
        }
    }
    tables.push(Box::new(Table::new()));
    t_r375 = tables.len() as i64;
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r375 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r375 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = 11999999;
    }
    tables.push(Box::new(Table::new()));
    t_r377 = tables.len() as i64;
    tables.push(Box::new(Table::new_float()));
    t_r431 = tables.len() as i64;
    f_r401 = 0.0;
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r431 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r431 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    if idx >= t.farray.len() {
        t.farray.resize(idx + 1, 0.0);
    }
    unsafe {
        *t.farray.get_unchecked_mut(idx) = f_r401;
    }
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r377 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r377 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = t_r431;
    }
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r375 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get((t_r375 - 1) as usize) {
        Some(t) => &**t,
        None => panic!("Runtime Error: table is nil"),
    };
    i_r376 = if idx < t.array.len() {
        unsafe { *t.array.get_unchecked(idx) }
    } else {
        0
    };
    i_r375 = i_r376 + 1;
    i_r376 = 0;
    loop {
        b_r375 = i_r376 < i_r375;
        if b_r375 {
            let k = 0;
            if k < 0 {
                panic!("Runtime Error: Negative table index");
            }
            let idx = k as usize;
            if t_r377 == 0 {
                panic!("Runtime Error: table is nil");
            }
            let t = match tables.get((t_r377 - 1) as usize) {
                Some(t) => &**t,
                None => panic!("Runtime Error: table is nil"),
            };
            t_r431 = if idx < t.array.len() {
                unsafe { *t.array.get_unchecked(idx) }
            } else {
                0
            };
            f_r401 = 0.75;
            let k = i_r376;
            if k < 0 {
                panic!("Runtime Error: Negative table index");
            }
            let idx = k as usize;
            if t_r431 == 0 {
                panic!("Runtime Error: table is nil");
            }
            let t = match tables.get_mut((t_r431 - 1) as usize) {
                Some(t) => &mut **t,
                None => panic!("Runtime Error: table is nil"),
            };
            if idx >= t.farray.len() {
                t.farray.resize(idx + 1, 0.0);
            }
            unsafe {
                *t.farray.get_unchecked_mut(idx) = f_r401;
            }
            i_r376 = i_r376 + 1;
        } else {
            break;
        }
    }
    tables.push(Box::new(Table::new()));
    t_r377 = tables.len() as i64;
    tables.push(Box::new(Table::new_float()));
    t_r431 = tables.len() as i64;
    f_r401 = 0.0;
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r431 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r431 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    if idx >= t.farray.len() {
        t.farray.resize(idx + 1, 0.0);
    }
    unsafe {
        *t.farray.get_unchecked_mut(idx) = f_r401;
    }
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r377 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r377 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = t_r431;
    }
    i_r376 = 100;
    loop {
        b_r375 = i_r376 < 8;
        if b_r375 {
            let k = 0;
            if k < 0 {
                panic!("Runtime Error: Negative table index");
            }
            let idx = k as usize;
            if t_r377 == 0 {
                panic!("Runtime Error: table is nil");
            }
            let t = match tables.get((t_r377 - 1) as usize) {
                Some(t) => &**t,
                None => panic!("Runtime Error: table is nil"),
            };
            t_r431 = if idx < t.array.len() {
                unsafe { *t.array.get_unchecked(idx) }
            } else {
                0
            };
            f_r401 = 0.5;
            let k = i_r376;
            if k < 0 {
                panic!("Runtime Error: Negative table index");
            }
            let idx = k as usize;
            if t_r431 == 0 {
                panic!("Runtime Error: table is nil");
            }
            let t = match tables.get_mut((t_r431 - 1) as usize) {
                Some(t) => &mut **t,
                None => panic!("Runtime Error: table is nil"),
            };
            if idx >= t.farray.len() {
                t.farray.resize(idx + 1, 0.0);
            }
            unsafe {
                *t.farray.get_unchecked_mut(idx) = f_r401;
            }
            i_r376 = i_r376 + 1;
        } else {
            break;
        }
    }
    tables.push(Box::new(Table::new()));
    t_r377 = tables.len() as i64;
    tables.push(Box::new(Table::new_float()));
    t_r431 = tables.len() as i64;
    f_r401 = 0.0;
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r431 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r431 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    if idx >= t.farray.len() {
        t.farray.resize(idx + 1, 0.0);
    }
    unsafe {
        *t.farray.get_unchecked_mut(idx) = f_r401;
    }
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r377 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r377 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = t_r431;
    }
    tables.push(Box::new(Table::new_float()));
    t_r431 = tables.len() as i64;
    f_r401 = 0.0;
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r431 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r431 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    if idx >= t.farray.len() {
        t.farray.resize(idx + 1, 0.0);
    }
    unsafe {
        *t.farray.get_unchecked_mut(idx) = f_r401;
    }
    i_r376 = 0;
    loop {
        b_r375 = i_r376 < 10000000;
        if b_r375 {
            let k = 0;
            if k < 0 {
                panic!("Runtime Error: Negative table index");
            }
            let idx = k as usize;
            if t_r377 == 0 {
                panic!("Runtime Error: table is nil");
            }
            let t = match tables.get((t_r377 - 1) as usize) {
                Some(t) => &**t,
                None => panic!("Runtime Error: table is nil"),
            };
            t_r432 = if idx < t.array.len() {
                unsafe { *t.array.get_unchecked(idx) }
            } else {
                0
            };
            f_r401 = 0.625;
            let k = i_r376;
            if k < 0 {
                panic!("Runtime Error: Negative table index");
            }
            let idx = k as usize;
            if t_r432 == 0 {
                panic!("Runtime Error: table is nil");
            }
            let t = match tables.get_mut((t_r432 - 1) as usize) {
                Some(t) => &mut **t,
                None => panic!("Runtime Error: table is nil"),
            };
            if idx >= t.farray.len() {
                t.farray.resize(idx + 1, 0.0);
            }
            unsafe {
                *t.farray.get_unchecked_mut(idx) = f_r401;
            }
            let k = 0;
            if k < 0 {
                panic!("Runtime Error: Negative table index");
            }
            let idx = k as usize;
            if t_r377 == 0 {
                panic!("Runtime Error: table is nil");
            }
            let t = match tables.get_mut((t_r377 - 1) as usize) {
                Some(t) => &mut **t,
                None => panic!("Runtime Error: table is nil"),
            };
            if idx >= t.array.len() {
                t.array.resize(idx + 1, 0);
            }
            unsafe {
                *t.array.get_unchecked_mut(idx) = t_r431;
            }
            i_r376 = i_r376 + 1;
        } else {
            break;
        }
    }
    tables.push(Box::new(Table::new()));
    t_r377 = tables.len() as i64;
    tables.push(Box::new(Table::new_float()));
    t_r432 = tables.len() as i64;
    f_r401 = 0.0;
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r432 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r432 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    if idx >= t.farray.len() {
        t.farray.resize(idx + 1, 0.0);
    }
    unsafe {
        *t.farray.get_unchecked_mut(idx) = f_r401;
    }
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r377 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r377 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    if idx >= t.array.len() {
        t.array.resize(idx + 1, 0);
    }
    unsafe {
        *t.array.get_unchecked_mut(idx) = t_r432;
    }
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r377 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get((t_r377 - 1) as usize) {
        Some(t) => &**t,
        None => panic!("Runtime Error: table is nil"),
    };
    t_r432 = if idx < t.array.len() {
        unsafe { *t.array.get_unchecked(idx) }
    } else {
        0
    };
    let lim = 12000000;
    if lim > 0 {
        if t_r432 == 0 {
            panic!("Runtime Error: table is nil");
        }
        let t = match tables.get_mut((t_r432 - 1) as usize) {
            Some(t) => &mut **t,
            None => panic!("Runtime Error: table is nil"),
        };
        if (lim as usize) > t.farray.len() {
            t.farray.resize(lim as usize, 0.0);
        }
    }
    if t_r432 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r432 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    len_r432 = t.farray.len();
    p_r432 = t.farray.as_mut_ptr();
    i_r376 = 0;
    loop {
        b_r375 = i_r376 < 12000000;
        if b_r375 {
            f_r401 = 0.375;
            let k = i_r376;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r432 {
                unsafe {
                    *p_r432.add(k as usize) = f_r401;
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            f_r401 = 0.875;
            let k = i_r376;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r432 {
                unsafe {
                    *p_r432.add(k as usize) = f_r401;
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            i_r376 = i_r376 + 1;
        } else {
            break;
        }
    }
    tables.push(Box::new(Table::new()));
    t_r377 = tables.len() as i64;
    let lim = 1024;
    if lim > 0 {
        if t_r377 == 0 {
            panic!("Runtime Error: table is nil");
        }
        let t = match tables.get_mut((t_r377 - 1) as usize) {
            Some(t) => &mut **t,
            None => panic!("Runtime Error: table is nil"),
        };
        if (lim as usize) > t.array.len() {
            t.array.resize(lim as usize, 0);
        }
    }
    if t_r377 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r377 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    len_r377 = t.array.len();
    p_r377 = t.array.as_mut_ptr();
    i_r376 = 0;
    loop {
        b_r375 = i_r376 < 1024;
        if b_r375 {
            tables.push(Box::new(Table::new()));
            t_r375 = tables.len() as i64;
            let k = i_r376;
            if k < 0 {
                panic!("Runtime Error: Negative index in fast path");
            }
            if (k as usize) < len_r377 {
                unsafe {
                    *p_r377.add(k as usize) = t_r375;
                }
            } else {
                panic!("optimizer invariant violated: fast-path bounds check failed");
            }
            i_r376 = i_r376 + 1;
        } else {
            break;
        }
    }
    i_r376 = 0;
    loop {
        b_r375 = i_r376 < 1024;
        if b_r375 {
            let k = i_r376;
            if k < 0 {
                panic!("Runtime Error: Negative table index");
            }
            let idx = k as usize;
            if t_r377 == 0 {
                panic!("Runtime Error: table is nil");
            }
            let t = match tables.get((t_r377 - 1) as usize) {
                Some(t) => &**t,
                None => panic!("Runtime Error: table is nil"),
            };
            t_r375 = if idx < t.array.len() {
                unsafe { *t.array.get_unchecked(idx) }
            } else {
                0
            };
            tables.push(Box::new(Table::new_float()));
            t_r432 = tables.len() as i64;
            let k = 0;
            if k < 0 {
                panic!("Runtime Error: Negative table index");
            }
            let idx = k as usize;
            if t_r375 == 0 {
                panic!("Runtime Error: table is nil");
            }
            let t = match tables.get_mut((t_r375 - 1) as usize) {
                Some(t) => &mut **t,
                None => panic!("Runtime Error: table is nil"),
            };
            if idx >= t.array.len() {
                t.array.resize(idx + 1, 0);
            }
            unsafe {
                *t.array.get_unchecked_mut(idx) = t_r432;
            }
            i_r376 = i_r376 + 1;
        } else {
            break;
        }
    }
    i_r376 = 0;
    loop {
        b_r375 = i_r376 < 1024;
        if b_r375 {
            let k = i_r376;
            if k < 0 {
                panic!("Runtime Error: Negative table index");
            }
            let idx = k as usize;
            if t_r377 == 0 {
                panic!("Runtime Error: table is nil");
            }
            let t = match tables.get((t_r377 - 1) as usize) {
                Some(t) => &**t,
                None => panic!("Runtime Error: table is nil"),
            };
            t_r375 = if idx < t.array.len() {
                unsafe { *t.array.get_unchecked(idx) }
            } else {
                0
            };
            let k = 0;
            if k < 0 {
                panic!("Runtime Error: Negative table index");
            }
            let idx = k as usize;
            if t_r375 == 0 {
                panic!("Runtime Error: table is nil");
            }
            let t = match tables.get((t_r375 - 1) as usize) {
                Some(t) => &**t,
                None => panic!("Runtime Error: table is nil"),
            };
            t_r432 = if idx < t.array.len() {
                unsafe { *t.array.get_unchecked(idx) }
            } else {
                0
            };
            let lim = 8192;
            if lim > 0 {
                if t_r432 == 0 {
                    panic!("Runtime Error: table is nil");
                }
                let t = match tables.get_mut((t_r432 - 1) as usize) {
                    Some(t) => &mut **t,
                    None => panic!("Runtime Error: table is nil"),
                };
                if (lim as usize) > t.farray.len() {
                    t.farray.resize(lim as usize, 0.0);
                }
            }
            if t_r432 == 0 {
                panic!("Runtime Error: table is nil");
            }
            let t = match tables.get_mut((t_r432 - 1) as usize) {
                Some(t) => &mut **t,
                None => panic!("Runtime Error: table is nil"),
            };
            len_r432 = t.farray.len();
            p_r432 = t.farray.as_mut_ptr();
            i_r375 = 0;
            loop {
                b_r375 = i_r375 < 8192;
                if b_r375 {
                    f_r401 = 0.0625;
                    let k = i_r375;
                    if k < 0 {
                        panic!("Runtime Error: Negative index in fast path");
                    }
                    if (k as usize) < len_r432 {
                        unsafe {
                            *p_r432.add(k as usize) = f_r401;
                        }
                    } else {
                        panic!("optimizer invariant violated: fast-path bounds check failed");
                    }
                    i_r375 = i_r375 + 1;
                } else {
                    break;
                }
            }
            i_r376 = i_r376 + 1;
        } else {
            break;
        }
    }
    i_r377 = 0;
    loop {
        b_r375 = i_r377 < 12;
        if b_r375 {
            i_r375 = 0;
            loop {
                b_r375 = i_r375 < 5000;
                if b_r375 {
                    let k = i_r375;
                    if k < 0 {
                        panic!("Runtime Error: Negative table index");
                    }
                    let idx = k as usize;
                    if t_r376 == 0 {
                        panic!("Runtime Error: table is nil");
                    }
                    let t = match tables.get((t_r376 - 1) as usize) {
                        Some(t) => &**t,
                        None => panic!("Runtime Error: table is nil"),
                    };
                    t_r432 = if idx < t.array.len() {
                        unsafe { *t.array.get_unchecked(idx) }
                    } else {
                        0
                    };
                    let lim = 5000;
                    if lim > 0 {
                        if t_r432 == 0 {
                            panic!("Runtime Error: table is nil");
                        }
                        let t = match tables.get_mut((t_r432 - 1) as usize) {
                            Some(t) => &mut **t,
                            None => panic!("Runtime Error: table is nil"),
                        };
                        if (lim as usize) > t.farray.len() {
                            t.farray.resize(lim as usize, 0.0);
                        }
                    }
                    if t_r432 == 0 {
                        panic!("Runtime Error: table is nil");
                    }
                    let t = match tables.get_mut((t_r432 - 1) as usize) {
                        Some(t) => &mut **t,
                        None => panic!("Runtime Error: table is nil"),
                    };
                    len_r432 = t.farray.len();
                    p_r432 = t.farray.as_mut_ptr();
                    i_r378 = 0;
                    loop {
                        b_r375 = i_r378 < 5000;
                        if b_r375 {
                            let k = i_r378;
                            if k < 0 {
                                panic!("Runtime Error: Negative index in fast path");
                            }
                            if (k as usize) < len_r432 {
                                f_r401 = unsafe { *p_r432.add(k as usize) };
                            } else {
                                panic!(
                                    "optimizer invariant violated: fast-path bounds check failed"
                                );
                            }
                            f_r403 = 0.25;
                            f_r404 = f_r401 + f_r403;
                            let k = i_r378;
                            if k < 0 {
                                panic!("Runtime Error: Negative index in fast path");
                            }
                            if (k as usize) < len_r432 {
                                unsafe {
                                    *p_r432.add(k as usize) = f_r404;
                                }
                            } else {
                                panic!(
                                    "optimizer invariant violated: fast-path bounds check failed"
                                );
                            }
                            i_r378 = i_r378 + 1;
                        } else {
                            break;
                        }
                    }
                    i_r375 = i_r375 + 1;
                } else {
                    break;
                }
            }
            i_r377 = i_r377 + 1;
        } else {
            break;
        }
    }
    f_r401 = 0.0;
    let lim = 5000;
    if lim > 0 {
        if t_r376 == 0 {
            panic!("Runtime Error: table is nil");
        }
        let t = match tables.get_mut((t_r376 - 1) as usize) {
            Some(t) => &mut **t,
            None => panic!("Runtime Error: table is nil"),
        };
        if (lim as usize) > t.array.len() {
            t.array.resize(lim as usize, 0);
        }
    }
    if t_r376 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r376 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    len_r376 = t.array.len();
    p_r376 = t.array.as_mut_ptr();
    f_r402 = f_r401;
    i_r376 = 0;
    loop {
        b_r375 = i_r376 < 5000;
        if b_r375 {
            i_r378 = 0;
            loop {
                b_r375 = i_r378 < 5000;
                if b_r375 {
                    let k = i_r376;
                    if k < 0 {
                        panic!("Runtime Error: Negative index in fast path");
                    }
                    if (k as usize) < len_r376 {
                        t_r431 = unsafe { *p_r376.add(k as usize) };
                    } else {
                        panic!("optimizer invariant violated: fast-path bounds check failed");
                    }
                    let k = i_r378;
                    if k < 0 {
                        panic!("Runtime Error: Negative table index");
                    }
                    let idx = k as usize;
                    if t_r431 == 0 {
                        panic!("Runtime Error: table is nil");
                    }
                    let t = match tables.get((t_r431 - 1) as usize) {
                        Some(t) => &**t,
                        None => panic!("Runtime Error: table is nil"),
                    };
                    f_r404 = if idx < t.farray.len() {
                        unsafe { *t.farray.get_unchecked(idx) }
                    } else {
                        0.0
                    };
                    f_r402 = f_r402 + f_r404;
                    i_r378 = i_r378 + 1;
                } else {
                    break;
                }
            }
            i_r376 = i_r376 + 1;
        } else {
            break;
        }
    }
    tables.push(Box::new(Table::new_float()));
    t_r432 = tables.len() as i64;
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r376 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get((t_r376 - 1) as usize) {
        Some(t) => &**t,
        None => panic!("Runtime Error: table is nil"),
    };
    t_r431 = if idx < t.array.len() {
        unsafe { *t.array.get_unchecked(idx) }
    } else {
        0
    };
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r431 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get((t_r431 - 1) as usize) {
        Some(t) => &**t,
        None => panic!("Runtime Error: table is nil"),
    };
    f_r404 = if idx < t.farray.len() {
        unsafe { *t.farray.get_unchecked(idx) }
    } else {
        0.0
    };
    let k = 0;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r432 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r432 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    if idx >= t.farray.len() {
        t.farray.resize(idx + 1, 0.0);
    }
    unsafe {
        *t.farray.get_unchecked_mut(idx) = f_r404;
    }
    let k = 4999;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r376 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get((t_r376 - 1) as usize) {
        Some(t) => &**t,
        None => panic!("Runtime Error: table is nil"),
    };
    t_r431 = if idx < t.array.len() {
        unsafe { *t.array.get_unchecked(idx) }
    } else {
        0
    };
    let k = 4999;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r431 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get((t_r431 - 1) as usize) {
        Some(t) => &**t,
        None => panic!("Runtime Error: table is nil"),
    };
    f_r404 = if idx < t.farray.len() {
        unsafe { *t.farray.get_unchecked(idx) }
    } else {
        0.0
    };
    let k = 1;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r432 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r432 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    if idx >= t.farray.len() {
        t.farray.resize(idx + 1, 0.0);
    }
    unsafe {
        *t.farray.get_unchecked_mut(idx) = f_r404;
    }
    let k = 2;
    if k < 0 {
        panic!("Runtime Error: Negative table index");
    }
    let idx = k as usize;
    if t_r432 == 0 {
        panic!("Runtime Error: table is nil");
    }
    let t = match tables.get_mut((t_r432 - 1) as usize) {
        Some(t) => &mut **t,
        None => panic!("Runtime Error: table is nil"),
    };
    if idx >= t.farray.len() {
        t.farray.resize(idx + 1, 0.0);
    }
    unsafe {
        *t.farray.get_unchecked_mut(idx) = f_r402;
    }
    return tables;
}

pub const STATS: &str = "fast_sets=12;fast_gets=3;dyn_sets=23;dyn_gets=19;hoists=13;hoist_ctx=0,0,0,0,1,0,0,0,0,0,1,0,2";
```
