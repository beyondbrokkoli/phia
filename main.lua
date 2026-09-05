-- main.lua — "The Gauntlet"
-- Strict Phia subset: integers, tables, local, while, +, -, <.
-- 13 phases. Each pins one optimizer outcome class; each final table
-- state is exactly predictable. No multiplication — checksums are
-- derived with + and closed forms only.

-- PHASE A — The Stride (non-unit increment; EC zero-fills the odd slots)
local pa_t = {}
local pa_n = 400
local pa_i = 0
while pa_i < pa_n do
    pa_t[pa_i] = pa_i + 1
    pa_i = pa_i + 2
end

-- PHASE B — The Staircase (inner limit = outer idx; EC+HR re-hoisted per
-- outer iteration, INSIDE the outer body: hoist_ctx=1)
local pb_t = {}
local pb_n = 300
local pb_i = 0
while pb_i < pb_n do
    local pb_j = 0
    while pb_j < pb_i do
        pb_t[pb_j] = pb_j + 1
        pb_j = pb_j + 1
    end
    pb_i = pb_i + 1
end

-- PHASE C — The Polisher (sibling loops, read-modify-write, both fast;
-- same table hoisted twice, second EC a no-op)
local pc_t = {}
local pc_n = 250
local pc_i = 0
while pc_i < pc_n do
    pc_t[pc_i] = 5
    pc_i = pc_i + 1
end
local pc_j = 0
while pc_j < pc_n do
    pc_t[pc_j] = pc_t[pc_j] + 1
    pc_j = pc_j + 1
end

-- PHASE D — The Sentinel (literal-bound fill stays dynamic — the bug-#1
-- canary, alive in the showcase; then data-dependent termination: the
-- search loop runs until it reads the sentinel)
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

-- PHASE E — The Frozen Handoff (alias defined at outer level, never
-- redefined in the nested region: survives the entry-kill — the SOUND
-- half of #15; the hoist lands BEFORE the outer loop)
local pe_t = {}
local pe_n = 200
local pe_m = 3
local pe_i = 0
while pe_i < pe_n do
    local pe_d = pe_i
    local pe_j = 0
    while pe_j < pe_m do
        pe_t[pe_d] = 7
        pe_j = pe_j + 1
    end
    pe_i = pe_i + 1
end

-- PHASE F — The Carried Alias (the #15 shape itself: key defined inside
-- the nested region; the enclosing candidate abstains, the INNER one
-- converts and hoists at ctx=1 with its own limit)
local pf_t = {}
local pf_n = 150
local pf_i = 0
while pf_i < pf_n do
    local pf_d = pf_i
    while pf_d < pf_n do
        pf_t[pf_d] = pf_d + 1
        pf_d = pf_d + 1
    end
    pf_i = pf_i + 1
end

-- PHASE G — The Mirror (fast reads from a hoisted table feeding dynamic
-- writes at computed keys into a second table)
local pg_src = {}
local pg_dst = {}
local pg_n = 180
local pg_i = 0
while pg_i < pg_n do
    pg_src[pg_i] = pg_i + 1
    pg_i = pg_i + 1
end
local pg_j = 0
while pg_j < pg_n do
    local pg_rev = pg_n - 1 - pg_j
    pg_dst[pg_rev] = pg_src[pg_j]
    pg_j = pg_j + 1
end

-- PHASE H — The Abacus (fast reads accumulating into a scalar, exported
-- through a witness table)
local ph_t = {}
local ph_n = 120
local ph_i = 0
while ph_i < ph_n do
    ph_t[ph_i] = ph_i + 1
    ph_i = ph_i + 1
end
local ph_sum = 0
local ph_j = 0
while ph_j < ph_n do
    ph_sum = ph_sum + ph_t[ph_j]
    ph_j = ph_j + 1
end
local ph_w = {}
ph_w[0] = ph_sum

-- PHASE I — The One-Shot (boolean flag as the only branch mechanism; no
-- candidate forms — header scan breaks at WhileCondition; all dynamic)
local pi_t = {}
local pi_flag = 0 < 1
local pi_k = 0
while pi_flag do
    pi_t[pi_k] = 100
    pi_flag = 0 < 0
    pi_k = pi_k + 1
end

-- PHASE J — The Terraces (one table grown in two epochs from a nonzero
-- start; the second EC reallocs and the second HR re-derives; the witness
-- probes the zero-filled gap and both boundaries)
local pj_t = {}
local pj_hi1 = 200
local pj_i = 100
while pj_i < pj_hi1 do
    pj_t[pj_i] = pj_i - 100
    pj_i = pj_i + 1
end
local pj_hi2 = 350
local pj_j = 200
while pj_j < pj_hi2 do
    pj_t[pj_j] = pj_j
    pj_j = pj_j + 1
end
local pj_w = {}
pj_w[0] = pj_t[150]
pj_w[1] = pj_t[250]
pj_w[2] = pj_t[349]
pj_w[3] = pj_t[99]

-- PHASE K — The Cube (three-deep nesting; only the innermost candidate
-- converts — the hoist sits at depth 2)
local pk_t = {}
local pk_n = 60
local pk_i = 0
while pk_i < pk_n do
    local pk_j = 0
    while pk_j < pk_n do
        local pk_k = 0
        while pk_k < pk_n do
            pk_t[pk_k] = pk_k + pk_j
            pk_k = pk_k + 1
        end
        pk_j = pk_j + 1
    end
    pk_i = pk_i + 1
end

-- PHASE L — The Poisoned Chalice (three alias-keyed writes, one broken
-- alias: the dynamic access retroactively poisons the table for the whole
-- candidate — all three stay dynamic)
local pl_t = {}
local pl_n = 80
local pl_i = 0
while pl_i < pl_n do
    local pl_a = pl_i
    pl_t[pl_a] = 1
    local pl_b = pl_a
    pl_b = pl_b + 0
    pl_t[pl_b] = 2
    local pl_c = pl_i
    pl_t[pl_c] = 3
    pl_i = pl_i + 1
end

-- PHASE M — The Handoff (two registers, one object: both views fast-path
-- with separate EC+HR pairs into the same allocation)
local pm_a = {}
local pm_b = pm_a
local pm_n = 90
local pm_i = 0
while pm_i < pm_n do
    pm_a[pm_i] = pm_i + 1
    pm_b[pm_i] = pm_i + 2
    pm_i = pm_i + 1
end
