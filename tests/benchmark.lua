-- tests/benchmark.lua -- DEDICATED PERFORMANCE BENCHMARK.
-- Deliberately NOT in the run_boss listings: a ~10s program would
-- clutter every suite run. It lives here so PHIA and luajit chew on
-- the very same file. Dialect notes baked into the shape: no * % > or
-- >= operators, int and float tables are disjoint, and the 64+64 rows
-- are minted STATICALLY (distinct locals, pre-loop) because a dynamic
-- t[r] = row store taints REACH(t) and tier4_14-declines the fast-row
-- door in the loops below.
--
--   phia:   PHIA_SOURCE=tests/benchmark.lua cargo build --release && ./target/release/phia
--   luajit: luajit tests/benchmark.lua
--   hyperfine (keep the run count LOW, luajit runs can wander):
--     hyperfine -r 3 -w 1 "./target/release/phia" "luajit tests/benchmark.lua"
--
-- Phases: P1 int fast-row matrix sweep (512 KiB, in-cache, write door);
-- P2 float fast-row matrix sweep, write+read doors; P3 scalar int ALU
-- add/reduce chain (no tables); P4 constant-row float streaming
-- (main.lua phase-B shape, the bandwidth regime). The checksum tables
-- wi/wf pin one live value from every phase so nothing can be dead-coded.
--
-- Reference (one cold run each, 2026-09-09, this machine):
--   phia 10.0s   luajit 33.8s   => 3.4x overall
--   per-phase marginal cost (same constants scaled down, phia vs luajit):
--     P1 int matrix write    ~11x phia faster   (fast-row write door)
--     P2 float matrix rw     ~2.4x phia faster  (both doors)
--     P3 scalar ALU          ~0.55x phia SLOWER (no tables to optimize;
--                                               raw rustc-vs-JIT codegen)
--     P4 stream 12M->120M    ~2x phia faster    (bandwidth regime, ~13GB/s)
local ROWS = 64
local COLS = 1024
local t = {}
local irow0 = {}
irow0[0] = 0
t[0] = irow0
local irow1 = {}
irow1[0] = 0
t[1] = irow1
local irow2 = {}
irow2[0] = 0
t[2] = irow2
local irow3 = {}
irow3[0] = 0
t[3] = irow3
local irow4 = {}
irow4[0] = 0
t[4] = irow4
local irow5 = {}
irow5[0] = 0
t[5] = irow5
local irow6 = {}
irow6[0] = 0
t[6] = irow6
local irow7 = {}
irow7[0] = 0
t[7] = irow7
local irow8 = {}
irow8[0] = 0
t[8] = irow8
local irow9 = {}
irow9[0] = 0
t[9] = irow9
local irow10 = {}
irow10[0] = 0
t[10] = irow10
local irow11 = {}
irow11[0] = 0
t[11] = irow11
local irow12 = {}
irow12[0] = 0
t[12] = irow12
local irow13 = {}
irow13[0] = 0
t[13] = irow13
local irow14 = {}
irow14[0] = 0
t[14] = irow14
local irow15 = {}
irow15[0] = 0
t[15] = irow15
local irow16 = {}
irow16[0] = 0
t[16] = irow16
local irow17 = {}
irow17[0] = 0
t[17] = irow17
local irow18 = {}
irow18[0] = 0
t[18] = irow18
local irow19 = {}
irow19[0] = 0
t[19] = irow19
local irow20 = {}
irow20[0] = 0
t[20] = irow20
local irow21 = {}
irow21[0] = 0
t[21] = irow21
local irow22 = {}
irow22[0] = 0
t[22] = irow22
local irow23 = {}
irow23[0] = 0
t[23] = irow23
local irow24 = {}
irow24[0] = 0
t[24] = irow24
local irow25 = {}
irow25[0] = 0
t[25] = irow25
local irow26 = {}
irow26[0] = 0
t[26] = irow26
local irow27 = {}
irow27[0] = 0
t[27] = irow27
local irow28 = {}
irow28[0] = 0
t[28] = irow28
local irow29 = {}
irow29[0] = 0
t[29] = irow29
local irow30 = {}
irow30[0] = 0
t[30] = irow30
local irow31 = {}
irow31[0] = 0
t[31] = irow31
local irow32 = {}
irow32[0] = 0
t[32] = irow32
local irow33 = {}
irow33[0] = 0
t[33] = irow33
local irow34 = {}
irow34[0] = 0
t[34] = irow34
local irow35 = {}
irow35[0] = 0
t[35] = irow35
local irow36 = {}
irow36[0] = 0
t[36] = irow36
local irow37 = {}
irow37[0] = 0
t[37] = irow37
local irow38 = {}
irow38[0] = 0
t[38] = irow38
local irow39 = {}
irow39[0] = 0
t[39] = irow39
local irow40 = {}
irow40[0] = 0
t[40] = irow40
local irow41 = {}
irow41[0] = 0
t[41] = irow41
local irow42 = {}
irow42[0] = 0
t[42] = irow42
local irow43 = {}
irow43[0] = 0
t[43] = irow43
local irow44 = {}
irow44[0] = 0
t[44] = irow44
local irow45 = {}
irow45[0] = 0
t[45] = irow45
local irow46 = {}
irow46[0] = 0
t[46] = irow46
local irow47 = {}
irow47[0] = 0
t[47] = irow47
local irow48 = {}
irow48[0] = 0
t[48] = irow48
local irow49 = {}
irow49[0] = 0
t[49] = irow49
local irow50 = {}
irow50[0] = 0
t[50] = irow50
local irow51 = {}
irow51[0] = 0
t[51] = irow51
local irow52 = {}
irow52[0] = 0
t[52] = irow52
local irow53 = {}
irow53[0] = 0
t[53] = irow53
local irow54 = {}
irow54[0] = 0
t[54] = irow54
local irow55 = {}
irow55[0] = 0
t[55] = irow55
local irow56 = {}
irow56[0] = 0
t[56] = irow56
local irow57 = {}
irow57[0] = 0
t[57] = irow57
local irow58 = {}
irow58[0] = 0
t[58] = irow58
local irow59 = {}
irow59[0] = 0
t[59] = irow59
local irow60 = {}
irow60[0] = 0
t[60] = irow60
local irow61 = {}
irow61[0] = 0
t[61] = irow61
local irow62 = {}
irow62[0] = 0
t[62] = irow62
local irow63 = {}
irow63[0] = 0
t[63] = irow63
local ft = {}
local frow0 = {}
frow0[0] = 0.0
ft[0] = frow0
local frow1 = {}
frow1[0] = 0.0
ft[1] = frow1
local frow2 = {}
frow2[0] = 0.0
ft[2] = frow2
local frow3 = {}
frow3[0] = 0.0
ft[3] = frow3
local frow4 = {}
frow4[0] = 0.0
ft[4] = frow4
local frow5 = {}
frow5[0] = 0.0
ft[5] = frow5
local frow6 = {}
frow6[0] = 0.0
ft[6] = frow6
local frow7 = {}
frow7[0] = 0.0
ft[7] = frow7
local frow8 = {}
frow8[0] = 0.0
ft[8] = frow8
local frow9 = {}
frow9[0] = 0.0
ft[9] = frow9
local frow10 = {}
frow10[0] = 0.0
ft[10] = frow10
local frow11 = {}
frow11[0] = 0.0
ft[11] = frow11
local frow12 = {}
frow12[0] = 0.0
ft[12] = frow12
local frow13 = {}
frow13[0] = 0.0
ft[13] = frow13
local frow14 = {}
frow14[0] = 0.0
ft[14] = frow14
local frow15 = {}
frow15[0] = 0.0
ft[15] = frow15
local frow16 = {}
frow16[0] = 0.0
ft[16] = frow16
local frow17 = {}
frow17[0] = 0.0
ft[17] = frow17
local frow18 = {}
frow18[0] = 0.0
ft[18] = frow18
local frow19 = {}
frow19[0] = 0.0
ft[19] = frow19
local frow20 = {}
frow20[0] = 0.0
ft[20] = frow20
local frow21 = {}
frow21[0] = 0.0
ft[21] = frow21
local frow22 = {}
frow22[0] = 0.0
ft[22] = frow22
local frow23 = {}
frow23[0] = 0.0
ft[23] = frow23
local frow24 = {}
frow24[0] = 0.0
ft[24] = frow24
local frow25 = {}
frow25[0] = 0.0
ft[25] = frow25
local frow26 = {}
frow26[0] = 0.0
ft[26] = frow26
local frow27 = {}
frow27[0] = 0.0
ft[27] = frow27
local frow28 = {}
frow28[0] = 0.0
ft[28] = frow28
local frow29 = {}
frow29[0] = 0.0
ft[29] = frow29
local frow30 = {}
frow30[0] = 0.0
ft[30] = frow30
local frow31 = {}
frow31[0] = 0.0
ft[31] = frow31
local frow32 = {}
frow32[0] = 0.0
ft[32] = frow32
local frow33 = {}
frow33[0] = 0.0
ft[33] = frow33
local frow34 = {}
frow34[0] = 0.0
ft[34] = frow34
local frow35 = {}
frow35[0] = 0.0
ft[35] = frow35
local frow36 = {}
frow36[0] = 0.0
ft[36] = frow36
local frow37 = {}
frow37[0] = 0.0
ft[37] = frow37
local frow38 = {}
frow38[0] = 0.0
ft[38] = frow38
local frow39 = {}
frow39[0] = 0.0
ft[39] = frow39
local frow40 = {}
frow40[0] = 0.0
ft[40] = frow40
local frow41 = {}
frow41[0] = 0.0
ft[41] = frow41
local frow42 = {}
frow42[0] = 0.0
ft[42] = frow42
local frow43 = {}
frow43[0] = 0.0
ft[43] = frow43
local frow44 = {}
frow44[0] = 0.0
ft[44] = frow44
local frow45 = {}
frow45[0] = 0.0
ft[45] = frow45
local frow46 = {}
frow46[0] = 0.0
ft[46] = frow46
local frow47 = {}
frow47[0] = 0.0
ft[47] = frow47
local frow48 = {}
frow48[0] = 0.0
ft[48] = frow48
local frow49 = {}
frow49[0] = 0.0
ft[49] = frow49
local frow50 = {}
frow50[0] = 0.0
ft[50] = frow50
local frow51 = {}
frow51[0] = 0.0
ft[51] = frow51
local frow52 = {}
frow52[0] = 0.0
ft[52] = frow52
local frow53 = {}
frow53[0] = 0.0
ft[53] = frow53
local frow54 = {}
frow54[0] = 0.0
ft[54] = frow54
local frow55 = {}
frow55[0] = 0.0
ft[55] = frow55
local frow56 = {}
frow56[0] = 0.0
ft[56] = frow56
local frow57 = {}
frow57[0] = 0.0
ft[57] = frow57
local frow58 = {}
frow58[0] = 0.0
ft[58] = frow58
local frow59 = {}
frow59[0] = 0.0
ft[59] = frow59
local frow60 = {}
frow60[0] = 0.0
ft[60] = frow60
local frow61 = {}
frow61[0] = 0.0
ft[61] = frow61
local frow62 = {}
frow62[0] = 0.0
ft[62] = frow62
local frow63 = {}
frow63[0] = 0.0
ft[63] = frow63

-- P1: int matrix write sweep, fast-row door (write side).
local i = 0
while i < 140000 do
    local a = 0
    while a < ROWS do
        local b = 0
        while b < COLS do
            t[a][b] = i
            b = b + 1
        end
        a = a + 1
    end
    i = i + 1
end

-- P2: float matrix write+read sweep, fast-row door both ways.
local s = 0.0
local i2 = 0
while i2 < 100000 do
    local a = 0
    while a < ROWS do
        local b = 0
        while b < COLS do
            ft[a][b] = 1.75
            b = b + 1
        end
        a = a + 1
    end
    local a2 = 0
    while a2 < ROWS do
        local b2 = 0
        while b2 < COLS do
            s = s + ft[a2][b2]
            b2 = b2 + 1
        end
        a2 = a2 + 1
    end
    i2 = i2 + 1
end

-- P3: scalar int ALU churn (dependent add chain + subtract-reduce).
local x = 1
local c = 0
while c < 600000000 do
    x = x + 1234567
    while 999999 < x do
        x = x - 1000000
    end
    c = c + 1
end

-- P4: constant-row float streaming (bandwidth regime).
local lane = {}
local holder = {}
holder[0] = lane
local ci = 0
while ci < 120000000 do
    holder[0][ci] = 1.75
    ci = ci + 1
end
local s4 = 0.0
local ci2 = 0
while ci2 < 120000000 do
    s4 = s4 + holder[0][ci2]
    ci2 = ci2 + 1
end

-- Checksum tables: wi[0]=P3 x, wi[1]=P1 t[0][0]; wf[0]=P2 s, wf[1]=P4 s4.
local wi = {}
wi[0] = x
wi[1] = t[0][0]
local wf = {}
wf[0] = s
wf[1] = s4
