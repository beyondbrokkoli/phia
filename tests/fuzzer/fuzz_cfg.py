#!/usr/bin/env python3
"""CFG+nesting differential fuzzer: if/while loops, nested tables, GetTable results
live across blocks and inside loop bodies (tier4 fast-path + hoisting territory)."""
import random, subprocess, sys, os

def gen(seed):
    r = random.Random(seed)
    L = []
    L.append("local sink = {}")
    L.append("local t = {}")
    L.append("t[0] = 3")
    L.append("t[1] = 5")
    L.append("local inner = {}")
    L.append("inner[0] = 7")
    L.append("inner[1] = 9")
    L.append("local grid = {}")
    L.append("grid[0] = inner")
    n = 0
    def fresh():
        nonlocal n
        n += 1
        return f"x{n}"
    live = []
    # keep a couple of GetTable results alive across everything
    a = fresh(); L.append(f"local {a} = t[0]")
    b = fresh(); L.append(f"local {b} = t[1]")
    live += [a, b]
    for s in range(r.randint(2, 5)):
        k = r.randrange(4)
        if k == 0:  # if/else merge with GetTable in both arms
            c = fresh(); d = fresh()
            L.append(f"local {c} = 0")
            L.append(f"if {r.choice(live)} < 100 then")
            L.append(f"  {c} = t[0]")
            L.append("else")
            L.append(f"  {c} = t[1]")
            L.append("end")
            live.append(c)
        elif k == 1:  # while loop with affine stores (tier4 territory)
            i = fresh(); L.append(f"local {i} = 0")
            acc = fresh(); L.append(f"local {acc} = {{}}")
            L.append(f"while {i} < {r.randint(2,4)} do")
            L.append(f"  {acc}[{i}] = {r.choice(live)} + {i}")
            L.append(f"  {i} = {i} + 1")
            L.append("end")
            live.append(i)
            g = fresh(); L.append(f"local {g} = {acc}[0] + 1000")
            live.append(g)
        elif k == 2:  # nested reads
            g = fresh(); L.append(f"local {g} = grid[0][1] + {r.choice(live)}")
            live.append(g)
        else:  # GetTable live across an if boundary
            g = fresh(); L.append(f"local {g} = t[{r.randint(0,1)}]")
            L.append(f"if {g} < 4 then")
            L.append(f"  sink[{s}] = {g} + 1")
            L.append("else")
            L.append(f"  sink[{s}] = {g} - 1")
            L.append("end")
            live.append(g)
    # pinned final state through the arena
    L.append(f"sink[50] = {a}")
    L.append(f"sink[51] = {b}")
    L.append(f"sink[52] = grid[0][0]")
    L.append(f"sink[53] = {live[-1]}")
    L.append('print("final", sink[50], sink[51], sink[52], sink[53])')
    return "\n".join(L) + "\n"

def run(seed):
    path = f"/tmp/adv/cfg_{seed}.lua"
    open(path, "w").write(gen(seed))
    ref = subprocess.run(["lua", path], capture_output=True, text=True)
    if ref.returncode != 0:
        return None
    subprocess.run(["touch", path])
    build = subprocess.run(["cargo", "build", "--release"], capture_output=True, text=True,
                           env={**os.environ, "PHIA_SOURCE": path}, cwd="/home/halim/phia")
    if build.returncode != 0:
        return ("BUILD FAIL", build.stderr[-500:], open(path).read())
    got = subprocess.run(["/home/halim/phia/target/release/phia"], capture_output=True, text=True)
    if got.returncode != 0:
        return ("RUN FAIL", got.stderr[-500:], open(path).read())
    ref_vals = ref.stdout.split()[1:]
    got_line = [l for l in got.stdout.splitlines() if l.startswith("PROBE final")][0]
    got_vals = [t.split("=", 1)[1].strip('"') for t in got_line.split()[2:]]
    ref_norm = ["0" if v == "nil" else v for v in ref_vals]
    for x, y in zip(ref_norm, got_vals):
        if x == y: continue
        try:
            if float(x) == float(y): continue
        except ValueError: pass
        return ("MISMATCH", (ref_norm, got_vals), open(path).read())
    return None

fails = 0
n = int(sys.argv[1]) if len(sys.argv) > 1 else 40
for seed in range(n):
    res = run(seed)
    if res:
        fails += 1
        print(f"seed {seed}: {res[0]}\n{res[1]}\n{res[2][:800]}\n---")
        if fails > 3: break
print("all clean" if not fails else f"{fails} failures")
