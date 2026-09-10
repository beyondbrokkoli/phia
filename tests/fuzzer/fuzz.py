#!/usr/bin/env python3
import argparse, random, subprocess, sys, os, time
from pathlib import Path

# Ensure tests/fuzzer is always on the module search path
sys.path.insert(0, str(Path(__file__).resolve().parent))
from generator import PhiaLuaGenerator

# --- Terminal Colors ---
class C:
    RED = '\033[91m'
    GREEN = '\033[92m'
    YELLOW = '\033[93m'
    CYAN = '\033[96m'
    RESET = '\033[0m'
    BOLD = '\033[1m'

def generate_phia_code(seed):
    generator = PhiaLuaGenerator(seed)
    return generator.generate()

# --- Runner & Verifier ---
def print_diff(ref_vals, got_vals):
    """Prints a beautiful side-by-side comparison of outputs."""
    print(f"\n{C.BOLD}--- Value Mismatch Diff ---{C.RESET}")
    print(f"{'Idx':<4} | {'Lua (Expected)':<15} | {'Phia (Got)':<15} | {'Status'}")
    print("-" * 50)
    for i in range(max(len(ref_vals), len(got_vals))):
        exp = ref_vals[i] if i < len(ref_vals) else "MISSING"
        got = got_vals[i] if i < len(got_vals) else "MISSING"

        is_match = False
        if exp == got:
            is_match = True
        else:
            try:
                if abs(float(exp) - float(got)) < 1e-9: is_match = True
            except ValueError: pass

        status = f"{C.GREEN}✔ Match{C.RESET}" if is_match else f"{C.RED}✘ Fail{C.RESET}"
        print(f"{i:<4} | {exp:<15} | {got:<15} | {status}")

def run_seed(seed, mode):
    src = generate_phia_code(seed)

    os.makedirs("/tmp/adv", exist_ok=True)
    path = f"/tmp/adv/fuzz_{mode}_{seed}.lua"
    with open(path, "w") as f:
        f.write(src)

    # Run Lua
    ref = subprocess.run(["lua", path], capture_output=True, text=True)
    if ref.returncode != 0:
        # The original script silently skipped seeds where Lua crashed on nil arithmetic
        return {"status": "SKIP"}

    # Build Phia (using dynamic cwd)
    cwd = os.getcwd()
    subprocess.run(["touch", path])
    build = subprocess.run(["cargo", "build", "--release"], capture_output=True, text=True, env={**os.environ, "PHIA_SOURCE": path}, cwd=cwd)
    if build.returncode != 0:
        return {"status": "BUILD_FAIL", "msg": build.stderr[-800:], "src": src}

    # Run Phia
    got = subprocess.run([os.path.join(cwd, "target/release/phia")], capture_output=True, text=True)
    if got.returncode != 0:
        return {"status": "RUN_FAIL", "msg": got.stderr[-800:], "src": src}

    # Parse & Compare
    ref_tok = ref.stdout.split()
    ref_vals = ["0" if v == "nil" else v for v in ref_tok[1:]] # Drop 'final', map nil->0

    try:
        got_line = [l for l in got.stdout.splitlines() if l.startswith("PROBE final")][0]
        got_vals = [t.split("=", 1)[1].strip('"') for t in got_line.split()[2:]]
    except IndexError:
        return {"status": "PARSE_ERROR", "msg": "Could not find 'PROBE final' in phia output.", "src": src}

    if len(ref_vals) != len(got_vals):
        return {"status": "SHAPE_FAIL", "ref": ref_vals, "got": got_vals, "src": src}

    for a, b in zip(ref_vals, got_vals):
        if a == b: continue
        try:
            if abs(float(a) - float(b)) < 1e-9: continue
        except ValueError: pass
        return {"status": "MISMATCH", "ref": ref_vals, "got": got_vals, "src": src}

    return {"status": "PASS"}

# --- Main CLI ---
if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Phia Differential Fuzzer")
    parser.add_argument("--mode", choices=["gen", "cfg", "all"], default="all", help="Which fuzzer to run")
    parser.add_argument("--seeds", type=int, default=50, help="Number of seeds per mode")
    args = parser.parse_args()

    modes = ["gen", "cfg"] if args.mode == "all" else [args.mode]

    total_fails = 0
    for mode in modes:
        print(f"\n{C.CYAN}{C.BOLD}=== Starting Fuzzer: Mode '{mode.upper()}' ==={C.RESET}")
        fails_this_mode = 0
        start_time = time.time()

        for seed in range(args.seeds):
            sys.stdout.write(f"\rTesting seed {seed+1}/{args.seeds}...")
            sys.stdout.flush()

            res = run_seed(seed, mode)
            if res["status"] == "SKIP":
                continue  # Silently skip this seed and move to the next one

            if res["status"] != "PASS":
                sys.stdout.write("\r" + " " * 30 + "\r") # Clear line
                fails_this_mode += 1
                total_fails += 1
                print(f"{C.RED}❌ FAIL (Seed {seed}){C.RESET} - Reason: {C.BOLD}{res['status']}{C.RESET}")

                if res["status"] in ["MISMATCH", "SHAPE_FAIL"]:
                    print_diff(res["ref"], res["got"])
                else:
                    print(f"\n{C.YELLOW}Error Output:{C.RESET}\n{res['msg']}")

                print(f"\n{C.YELLOW}Failing Source Code:{C.RESET}\n{res['src']}")
                if fails_this_mode >= 3:
                    print(f"{C.RED}Too many failures in '{mode}'. Aborting this mode.{C.RESET}")
                    break

        elapsed = time.time() - start_time
        if fails_this_mode == 0:
            sys.stdout.write(f"\r{C.GREEN}✔ Mode '{mode}' completed cleanly in {elapsed:.2f}s{C.RESET}\n")

    sys.exit(1 if total_fails > 0 else 0)
