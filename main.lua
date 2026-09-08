-- main.lua — "The Gauntlet" · long-run calibration
-- Strict Phia subset: integers, tables, local, while, +, -, <.

-- PHASE A — The Stride. Non-unit IV step (+2) and the analysis doesn't
-- blink: the induction variable is still the raw phi (offset 0), the
-- bound pa_dream_limit is a pre-header LoadInt (def-block < header — loop-
-- invariant by dominance), so the gate opens on first contact. The
-- capacity contract pays once: EC(pa_dream_limit) zero-fills the whole span up
-- front, the ramp store lands only on even indices, and the odd slots
-- keep their EC zeros — half the table is written by the ABSENCE of a
-- store. One fast store per trip through the cached raw pointer; one
-- HR at depth 0. Final: LEN 8000000, NZ 4000000,
-- CHECKSUM -6900387035215758080 (position-weighted, i64-wrapped at this scale).
local pa_cozy_cabin = {}
local pa_dream_limit = 8000000
local pa_sheep_one = 0
while pa_sheep_one < pa_dream_limit do
    pa_cozy_cabin[pa_sheep_one] = pa_sheep_one + 1
    pa_sheep_one = pa_sheep_one + 2
end

-- PHASE B — The Staircase. The inner bound is the OUTER induction phi
-- (pb_sheep_one): invariant FOR THE INNER LOOP, because dominance is checked
-- per-header — defined before the inner header, stable across the whole
-- inner trip since the outer bump lands in the tail, after. The outer
-- pass abstains on its own region scan: the write pb_cozy_cabin[φ_pb_sheep_two] does not
-- trace to φ_pb_sheep_one (wrong phi) → poison → no outer contract. The inner
-- pass owns the write: its own phi, offset 0, root defined pre-loop →
-- upgrade. EC+HR land in the outer BODY (hoist_ctx=1) and re-arm every
-- outer trip — this is the file's only EC whose limit is a live
-- register (`let lim = pb_sheep_one;`), un-foldable by constant propagation, so
-- the capacity guarantee is re-derived per iteration. Sound because EC
-- is idempotent (amortized Vec growth) and nothing resizes in-region.
-- O(pb_dream_limit^2/2) ramp stores into an L2-resident table — first hand of
-- the runtime clock. Final: LEN 99999, NZ 99999, CHECKSUM 333328333350000.
local pb_cozy_cabin = {}
local pb_dream_limit = 100000
local pb_sheep_one = 0
while pb_sheep_one < pb_dream_limit do
    local pb_sheep_two = 0
    while pb_sheep_two < pb_sheep_one do
        pb_cozy_cabin[pb_sheep_two] = pb_sheep_two + 1
        pb_sheep_two = pb_sheep_two + 1
    end
    pb_sheep_one = pb_sheep_one + 1
end

-- PHASE C — The Polisher. Two sibling loops, one root, two contracts:
-- cross-loop dedup is deliberately out of scope, so each pass mints its
-- own EC+HR in its own pre-header (hoist_ctx=0,0). The second EC finds
-- len ≥ n already — a capacity no-op — but the HR re-derives p_r/len_r
-- anyway: the pointer cache is per-contract, not per-object. Loop 2 is
-- read-modify-write at the SAME safe key (φ_pc_sheep_two): the get folds to a
-- raw-pointer load (fast_get), the +1 is a scalar add, the set writes
-- back through the same cached pointer. Same table, hoisted twice, on
-- purpose. Final: LEN 2500000, NZ 2500000, CHECKSUM 18750007500000 (6s).
local pc_cozy_cabin = {}
local pc_dream_limit = 2500000
local pc_sheep_one = 0
while pc_sheep_one < pc_dream_limit do
    pc_cozy_cabin[pc_sheep_one] = 5
    pc_sheep_one = pc_sheep_one + 1
end
local pc_sheep_two = 0
while pc_sheep_two < pc_dream_limit do
    pc_cozy_cabin[pc_sheep_two] = pc_cozy_cabin[pc_sheep_two] + 1
    pc_sheep_two = pc_sheep_two + 1
end

-- PHASE D — The Sentinel. NUMBERS FROZEN — this is the phase that bites
-- when scaled blindly. The fill converts: the literal 30000 is
-- materialized into the pre-header by the lowerer (the LoadInt the
-- bounds gate needs, def-block < header), one EC, one fast store per
-- trip. The search loop does NOT convert, twice over: (1) its only
-- table op is the GetTable living in its own HEADER, and headers are
-- excluded from every scan region by construction — they hold phis and
-- condition lowering, never body candidates; (2) its key is the loop's
-- live-out, and idx_reg is that very read's RESULT — a def-use cycle,
-- structurally untraceable. So the scan runs on the dynamic path, which
-- is exactly what you want: termination is data-dependent (stop at the
-- first slot ≥ pd_spicy_boundary; the sentinel pd_cozy_cabin[30] = 60 is planted by the
-- pre-header dyn write). RAISE pd_spicy_boundary WITHOUT MOVING THE SENTINEL and
-- the reads walk past the array returning defined zeros forever —
-- 0 < pd_spicy_boundary never fails, no panic fires, the program hangs. The
-- pre-header write and post-loop witness are dyn for free: neither
-- pre-headers nor post-loop code are ever scan candidates.
-- Final: pd_cozy_cabin LEN 30000, NZ 30000, CHECKSUM 450016829; pd_secret_diary LEN 1, NZ 1,
-- CHECKSUM 30.
local pd_cozy_cabin = {}
local pd_busy_bee = 0
while pd_busy_bee < 30000 do
    pd_cozy_cabin[pd_busy_bee] = 1
    pd_busy_bee = pd_busy_bee + 1
end
pd_cozy_cabin[30] = 60
local pd_spicy_boundary = 60
local pd_sheep_one = 0
while pd_cozy_cabin[pd_sheep_one] < pd_spicy_boundary do
    pd_sheep_one = pd_sheep_one + 1
end
local pd_secret_diary = {}
pd_secret_diary[0] = pd_sheep_one

-- PHASE E — The Frozen Handoff (the Patch-D flip, live in codegen).
-- pe_frozen_sheep = Move(φ_pe_sheep_one): SSA pins that register for the entire outer
-- iteration — the header's Less proved φ_pe_sheep_one < pe_dream_limit, and nothing in
-- the region can alter the register without minting a new vreg, which
-- would break the def-use trace and auto-decline. So the OUTER pass
-- upgrades a write that lives in the INNER body: region-wide PASS 2
-- walks into the nest, and EC(pe_dream_limit) + HR land BEFORE the outer loop
-- (hoist_ctx=0) — 2M outer iterations, 6M stores, one capacity payment.
-- The inner pass declines the very same instruction: pe_frozen_sheep does not
-- trace to φ_pe_sheep_two — and should not, since pe_frozen_sheep ranges over [0, pe_dream_limit)
-- against an inner limit of 3; converting there would arm the fast
-- path's invariant panic. One store, two verdicts, both correct: the
-- outer bound subsumes the key, the inner bound does not.
-- Final: LEN 2000000, NZ 2000000, CHECKSUM 14000007000000.
local pe_cozy_cabin = {}
local pe_dream_limit = 2000000
local pe_nap_horizon = 3
local pe_sheep_one = 0
while pe_sheep_one < pe_dream_limit do
    local pe_frozen_sheep = pe_sheep_one
    local pe_sheep_two = 0
    while pe_sheep_two < pe_nap_horizon do
        pe_cozy_cabin[pe_frozen_sheep] = 7
        pe_sheep_two = pe_sheep_two + 1
    end
    pe_sheep_one = pe_sheep_one + 1
end

-- PHASE F — The Carried Alias. Identical topology to E, opposite
-- resolution — and the difference is one def-use edge. Here the key IS
-- the inner phi (pf_rebel_sheep is the inner IV, reassigned in-region), so the
-- outer pass cannot trace it: loop-carried, poisoned, abstain. The
-- inner pass converts — offset 0 against its own limit pf_dream_limit — and its
-- EC+HR land in the outer BODY (hoist_ctx=1), re-armed every outer
-- trip: idempotent, sound, cheap. E vs F is the showcase pair: same
-- block structure, same nesting depth, hoists on opposite sides of the
-- outer loop — placement decided purely by WHICH phi the key's chain
-- terminates at. A frozen copy converts at the outer level; the live
-- counter converts at the inner. O(pf_dream_limit^2/2) stores, L2-resident —
-- second hand of the clock. Final: LEN 60000, NZ 60000,
-- CHECKSUM 72001800010000.
local pf_cozy_cabin = {}
local pf_dream_limit = 60000
local pf_sheep_one = 0
while pf_sheep_one < pf_dream_limit do
    local pf_rebel_sheep = pf_sheep_one
    while pf_rebel_sheep < pf_dream_limit do
        pf_cozy_cabin[pf_rebel_sheep] = pf_rebel_sheep + 1
        pf_rebel_sheep = pf_rebel_sheep + 1
    end
    pf_sheep_one = pf_sheep_one + 1
end

-- PHASE G — The Mirror. Loop 1: textbook ramp fill, fast, HR at depth
-- 0. Loop 2 runs fast and dynamic SIDE BY SIDE, per-root: the read
-- pg_magic_hat[φ_pg_sheep_two] traces clean (fast_get, fresh HR for the src root);
-- the write pg_rabbit_hole[pg_dream_limit - 1 - φ_pg_sheep_two] has a key whose trace dies in
-- non-constant arithmetic — the subtrahend IS the phi, so no constant
-- offset exists — unsafe write, poison lands on pg_rabbit_hole's root only.
-- Alias analysis at root granularity: one poisoned chalice does not
-- spill into a sibling root. The dynamic write grows pg_rabbit_hole by
-- resize-per-store — the exact per-access capacity tax the fast
-- contract exists to abolish, kept here on purpose as the contrast
-- specimen. Final: pg_magic_hat LEN 180, NZ 180, CHECKSUM 1960230; pg_rabbit_hole
-- LEN 180, NZ 180, CHECKSUM 988260 — mirror images, slot for slot
-- (the position-weighted checksums expose the reversed order).
local pg_magic_hat = {}
local pg_rabbit_hole = {}
local pg_dream_limit = 180
local pg_sheep_one = 0
while pg_sheep_one < pg_dream_limit do
    pg_magic_hat[pg_sheep_one] = pg_sheep_one + 1
    pg_sheep_one = pg_sheep_one + 1
end
local pg_sheep_two = 0
while pg_sheep_two < pg_dream_limit do
    local pg_moonwalk_sheep = pg_dream_limit - 1 - pg_sheep_two
    pg_rabbit_hole[pg_moonwalk_sheep] = pg_magic_hat[pg_sheep_two]
    pg_sheep_two = pg_sheep_two + 1
end

-- PHASE H — The Abacus. Fill loop converts (fast set, HR at 0). The
-- reduce loop's read keys off its own phi → fast_get through the cached
-- pointer; the accumulator is a loop-carried scalar (phi → coalesced to
-- one register; the add is the phase's only loop-carried dependence).
-- The witness store is post-loop → dyn, by design. Closed form
-- sum 1..n = n(n+1)/2, computed with + alone.
-- Final: ph_cozy_cabin LEN 1000000, NZ 1000000, CHECKSUM 333333833333500000;
-- ph_secret_diary LEN 1, NZ 1, CHECKSUM 500000500000.
local ph_cozy_cabin = {}
local ph_dream_limit = 1000000
local ph_sheep_one = 0
while ph_sheep_one < ph_dream_limit do
    ph_cozy_cabin[ph_sheep_one] = ph_sheep_one + 1
    ph_sheep_one = ph_sheep_one + 1
end
local ph_fluff_pile = 0
local ph_sheep_two = 0
while ph_sheep_two < ph_dream_limit do
    ph_fluff_pile = ph_fluff_pile + ph_cozy_cabin[ph_sheep_two]
    ph_sheep_two = ph_sheep_two + 1
end
local ph_secret_diary = {}
ph_secret_diary[0] = ph_fluff_pile

-- PHASE I — The One-Shot. The branch condition is a phi, not a Less:
-- def_map[cond] lands on the Phi node and the gate never opens — no
-- induction variable, no limit, no contract even formulable. Shape, not
-- conservativeness: there is nothing to prove. The loop runs once (flag
-- armed pre-loop, cleared in-body — codegen keeps the folded
-- `b = 0 < 0`). It is also the only genuine `while` in the emitted
-- file: every Less-headed loop could render in pretty form, but all
-- conditions share one physical bool register, so the uses==1
-- eligibility check fails and they render as loop{if{...}} — identical
-- semantics, identical machine-code shape, purely cosmetic demotion.
-- Final: LEN 1, NZ 1, CHECKSUM 100.
local pi_cozy_cabin = {}
local pi_is_vibing = 0 < 1
local pi_sheep_one = 0
while pi_is_vibing do
    pi_cozy_cabin[pi_sheep_one] = 100
    pi_is_vibing = 0 < 0
    pi_sheep_one = pi_sheep_one + 1
end

-- PHASE J — The Terraces. One root, two epochs, two contracts, one
-- REALLOC: epoch 2's EC(350) outgrows the amortized capacity bought by
-- EC(200), the Vec moves, and the second HR re-derives the raw pointer
-- from the fresh allocation — the pointer cache is rebuilt, never
-- assumed. The non-zero IV origin (i = 100) stresses offset-blindness:
-- the key is the raw phi over [100, 350), and EC sizes to the LIMIT,
-- not the span — the unwritten prefix [0, 100) keeps its EC zeros. The
-- four witnesses probe the gap (pj_cozy_cabin[99] = 0), both terraces, and the far
-- edge; post-loop code is never a scan candidate → 4 dyn reads, 4 dyn
-- writes, by design. Final: pj_cozy_cabin LEN 350, NZ 249, CHECKSUM 12453250;
-- pj_secret_diary LEN 4, NZ 3, CHECKSUM 1597 (50, 250, 349, 0 — the gap reads zero).
local pj_cozy_cabin = {}
local pj_nap_horizon = 200
local pj_sheep_one = 100
while pj_sheep_one < pj_nap_horizon do
    pj_cozy_cabin[pj_sheep_one] = pj_sheep_one - 100
    pj_sheep_one = pj_sheep_one + 1
end
local pj_dream_limit = 350
local pj_sheep_two = 200
while pj_sheep_two < pj_dream_limit do
    pj_cozy_cabin[pj_sheep_two] = pj_sheep_two
    pj_sheep_two = pj_sheep_two + 1
end
local pj_secret_diary = {}
pj_secret_diary[0] = pj_cozy_cabin[150]
pj_secret_diary[1] = pj_cozy_cabin[250]
pj_secret_diary[2] = pj_cozy_cabin[349]
pj_secret_diary[3] = pj_cozy_cabin[99]

-- PHASE K — The Cube. Triple nesting, single conversion, deepest hoist
-- in the file. The innermost write keys off φ_pk_sheep_three; that trace misses
-- φ_pk_sheep_two and φ_pk_sheep_one, so the region scan poisons pk_cozy_cabin for BOTH enclosing
-- passes — a poison minted at depth 2 propagates upward through every
-- enclosing candidate (their regions contain the inner body). The
-- innermost pass alone converts: EC+HR sit in the MIDDLE body
-- (hoist_ctx=2 — the deepest context in STATS), re-armed pk_dream_limit^2 =
-- 4,000,000 times to underwrite pk_dream_limit^3 = 8,000,000,000 stores — the
-- amortization ratio the whole contract exists to buy. The stored value
-- k + j is a ramp-plus-broadcast add, a vectorizable lane pattern: the
-- reason the cube finishes in seconds, not minutes.
-- Final: LEN 2000, NZ 2000, CHECKSUM 6666665000 (every slot k + 1999).
local pk_cozy_cabin = {}
local pk_dream_limit = 2000
local pk_sheep_one = 0
while pk_sheep_one < pk_dream_limit do
    local pk_sheep_two = 0
    while pk_sheep_two < pk_dream_limit do
        local pk_sheep_three = 0
        while pk_sheep_three < pk_dream_limit do
            pk_cozy_cabin[pk_sheep_three] = pk_sheep_three + pk_sheep_two
            pk_sheep_three = pk_sheep_three + 1
        end
        pk_sheep_two = pk_sheep_two + 1
    end
    pk_sheep_one = pk_sheep_one + 1
end

-- PHASE L — The Poisoned Chalice, post-Patch-B: the chalice is empty
-- and the drink is safe. All three keys trace to φ_pl_sheep_one — pl_sheep_alpha and
-- pl_sheep_gamma by Move chains, pl_sheep_beta by `+ 0`, a constant-offset fold to offset
-- 0. Note what tier-1 did NOT do: the Add still executes at runtime.
-- key_offset proves equality analytically; it does not need the add
-- folded dead — proof, not rewriting. No unsafe write in the region →
-- no poison → one EC + one HR underwrite all three stores. The file's
-- dyn_sets residue now lives in D, G, H, I, J: the chalice flipped to
-- fast and left the poison ledger entirely.
-- Final: LEN 8000, NZ 8000, CHECKSUM 96012000 (last store wins: 3).
local pl_cozy_cabin = {}
local pl_dream_limit = 8000
local pl_sheep_one = 0
while pl_sheep_one < pl_dream_limit do
    local pl_sheep_alpha = pl_sheep_one
    pl_cozy_cabin[pl_sheep_alpha] = 1
    local pl_sheep_beta = pl_sheep_alpha
    pl_sheep_beta = pl_sheep_beta + 0
    pl_cozy_cabin[pl_sheep_beta] = 2
    local pl_sheep_gamma = pl_sheep_one
    pl_cozy_cabin[pl_sheep_gamma] = 3
    pl_sheep_one = pl_sheep_one + 1
end

-- PHASE M — The Handoff, post-Patch-C. pm_twin_cabin_alpha and pm_twin_cabin_beta are two SSA
-- registers over ONE allocation — the Table Move between them is
-- root-traceable, so the alias is provably the same object, not an
-- assumed one. Both stores key off φ_pm_sheep_one and upgrade; the hoist set is
-- keyed by ROOT, so one EC + one HR cover both views (pre-C: two full
-- pairs, one per register — sound because EC is idempotent, but
-- structurally redundant). The fast stores are emitted against the root
-- register, pm_twin_cabin_beta's alias Move loses its last use, and simplify() reaps
-- it. Epilogue flourish: the physical table slot is recycled straight
-- from phase L — disjoint live ranges, 13 tables, 3 table registers,
-- zero spills.
-- Final: LEN 2000000, NZ 2000000, CHECKSUM 2666670666668000000 (pm_twin_cabin_beta's i+2
-- wins the same-slot write race).
local pm_twin_cabin_alpha = {}
local pm_twin_cabin_beta = pm_twin_cabin_alpha
local pm_dream_limit = 2000000
local pm_sheep_one = 0
while pm_sheep_one < pm_dream_limit do
    pm_twin_cabin_alpha[pm_sheep_one] = pm_sheep_one + 1
    pm_twin_cabin_beta[pm_sheep_one] = pm_sheep_one + 2
    pm_sheep_one = pm_sheep_one + 1
end
