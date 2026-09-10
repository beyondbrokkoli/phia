#!/usr/bin/env python3
"""Differential fuzzer: random programs in phia's subset, compare phia vs lua 5.5.
Allowed divergences (documented): absent key -> typed zero (nil prints as 0/""/0.0),
integer `/` truncates (lua 5.5 yields float). We avoid `/` on ints to keep exact compare.
"""
import random, subprocess, sys, os

def gen(seed):
    r = random.Random(seed)
    lines = []
    lines.append("local acc = {}")
    nvars = r.randint(3, 6)
    vals = []
    # init some ints
    for i in range(nvars):
        v = r.randint(-50, 50)
        lines.append(f"local v{i} = {v}")
        vals.append(f"v{i}")
    # table seeds
    lines.append("local t = {}")
    for i in range(r.randint(1, 4)):
        lines.append(f"t[{i}] = {r.randint(-50, 50)}")
    # random straight-line ops with GetTable results kept live simultaneously
    for stmt in range(r.randint(4, 12)):
        kind = r.randrange(5)
        a, b = r.choice(vals), r.choice(vals)
        if kind == 0:
            lines.append(f"local g{stmt} = t[{r.randint(0,3)}]")
            vals.append(f"g{stmt}")
        elif kind == 1:
            lines.append(f"acc[{stmt}] = {a} + {b}")
        elif kind == 2:
            lines.append(f"acc[{stmt}] = ({a} - {b}) * 2")
        elif kind == 3:
            lines.append(f"local g{stmt} = t[0]")
            vals.append(f"g{stmt}")
            lines.append(f"acc[{stmt}] = g{stmt} + {a}")
        else:
            lines.append(f"acc[{stmt}] = {a}")
    # final pinned reads through the arena so DCE can't kill semantics
    lines.append(f"acc[90] = t[0]")
    lines.append(f"acc[91] = t[1]")
    lines.append(f"acc[92] = {vals[-1]}")
    lines.append("print(\"final\", acc[0], acc[1], acc[2], acc[3], acc[90], acc[91], acc[92])")
    return "\n".join(lines) + "\n"

def lua_val(x):
    # normalize lua nil prints as empty chunk; phia prints 0 etc. We only
    # compare token lists after mapping "" -> typed-zero per phia docs.
    return x

def run(seed):
    src = gen(seed)
    path = f"/tmp/adv/fuzz_{seed}.lua"
    open(path, "w").write(src)
    ref = subprocess.run(["lua", path], capture_output=True, text=True)
    if ref.returncode != 0:
        return None  # lua itself failed (shouldn't happen)
    b = subprocess.run(["touch", path])
    build = subprocess.run(["cargo", "build", "--release"], capture_output=True, text=True,
                           env={**os.environ, "PHIA_SOURCE": path}, cwd="/home/halim/phia")
    if build.returncode != 0:
        return ("BUILD FAIL", build.stderr[-800:], src)
    run = subprocess.run(["/home/halim/phia/target/release/phia"], capture_output=True, text=True)
    if run.returncode != 0:
        return ("RUN FAIL", run.stderr[-800:], src)
    # compare: lua "final nil 3 ..." vs phia "PROBE final: i_rN=0 ..."
    ref_tok = ref.stdout.split()
    # drop 'final'
    ref_vals = ref_tok[1:]
    got = run.stdout.splitlines()[0]
    got_vals = [t.split("=", 1)[1] for t in got.split()[2:]]
    # nil (absent) maps to phia zero
    ref_norm = []
    for v in ref_vals:
        if v == "nil":
            ref_norm.append("0")  # integer-typed slots
        else:
            # lua 5.5 prints ints as '3' and floats as '3.0'
            ref_norm.append(v)
    if len(ref_norm) != len(got_vals):
        return ("SHAPE", (ref_norm, got_vals), src)
    for a, b in zip(ref_norm, got_vals):
        if a == b:
            continue
        try:
            if abs(float(a) - float(b)) < 1e-9:
                continue
        except ValueError:
            pass
        return ("MISMATCH", (a, b, ref_norm, got_vals), src)
    return None

fails = 0
n = int(sys.argv[1]) if len(sys.argv) > 1 else 60
for seed in range(n):
    res = run(seed)
    if res:
        fails += 1
        print(f"seed {seed}: {res[0]}")
        print(res[1])
        print(res[2][:600])
        if fails > 4:
            break
print(f"done: {n - fails}/{n} clean" if fails else f"done: {n}/{n} clean")
