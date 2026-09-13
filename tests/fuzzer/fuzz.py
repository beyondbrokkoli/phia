#!/usr/bin/env python3
import argparse, subprocess, sys, os, time
from pathlib import Path

# Ensure tests/fuzzer is always resolvable regardless of where the script is called from
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

def print_diff(ref_vals, got_vals):
    """Prints a clear side-by-side comparison of Lua vs Phia outputs."""
    print(f"\n{C.BOLD}--- Output Mismatch Diff ---{C.RESET}")
    print(f"{'Idx':<4} | {'Lua (Expected)':<16} | {'Phia (Got)':<16} | {'Status'}")
    print("-" * 52)
    max_len = max(len(ref_vals), len(got_vals))
    for i in range(max_len):
        exp = ref_vals[i] if i < len(ref_vals) else "MISSING"
        got = got_vals[i] if i < len(got_vals) else "MISSING"

        is_match = False
        if exp == got:
            is_match = True
        else:
            try:
                if abs(float(exp) - float(got)) < 1e-9:
                    is_match = True
            except ValueError:
                pass

        status = f"{C.GREEN}✔ Match{C.RESET}" if is_match else f"{C.RED}✘ Fail{C.RESET}"
        print(f"{i:<4} | {exp:<16} | {got:<16} | {status}")

def run_seed(seed):
    generator = PhiaLuaGenerator(seed)
    src, expected_types = generator.generate() # <-- Grab types here

    os.makedirs("/tmp/adv", exist_ok=True)
    path = f"/tmp/adv/fuzz_{seed}.lua"
    with open(path, "w") as f:
        f.write(src)

    # 1. Run reference Lua
    ref = subprocess.run(["lua", path], capture_output=True, text=True)
    if ref.returncode != 0:
        return {"status": "SKIP", "src": src}

    # 2. Build Phia
    cwd = os.getcwd()
    subprocess.run(["touch", path])
    build = subprocess.run(
        ["cargo", "build", "--release"],
        capture_output=True,
        text=True,
        env={**os.environ, "PHIA_SOURCE": path},
        cwd=cwd
    )
    if build.returncode != 0:
        return {"status": "BUILD_FAIL", "msg": build.stderr[-800:], "src": src}

    # 3. Execute Phia
    got = subprocess.run([os.path.join(cwd, "target/release/phia")], capture_output=True, text=True)
    if got.returncode != 0:
        return {"status": "RUN_FAIL", "msg": got.stderr[-800:], "src": src}

    # 4. Parse & Compare Outputs (NEW LOGIC)
    ref_tok = ref.stdout.split()

    # Smart nil mapping!
    ref_vals = []
    for v, typ in zip(ref_tok[1:], expected_types):
        if v == "nil":
            if typ == 'int': ref_vals.append("0")
            elif typ == 'float': ref_vals.append("0.0")
            elif typ == 'str': ref_vals.append("")
            else: ref_vals.append("false")
        else:
            ref_vals.append(v)

    try:
        got_line = [l for l in got.stdout.splitlines() if l.startswith("PROBE final")][0]
        got_vals = [t.split("=", 1)[1].strip('"') for t in got_line.split()[2:]]
    except IndexError:
        return {"status": "PARSE_ERROR", "msg": "Could not locate 'PROBE final' in phia output.", "src": src}

    if len(ref_vals) != len(got_vals):
        return {"status": "SHAPE_FAIL", "ref": ref_vals, "got": got_vals, "src": src}

    for a, b in zip(ref_vals, got_vals):
        if a == b:
            continue
        try:
            if abs(float(a) - float(b)) < 1e-9:
                continue
        except ValueError:
            pass
        return {"status": "MISMATCH", "ref": ref_vals, "got": got_vals, "src": src}

    return {"status": "PASS"}

def main():
    parser = argparse.ArgumentParser(description="Phia Unified Differential Fuzzer")
    parser.add_argument("--seeds", "-n", type=int, default=50, help="Number of random tests to execute (default: 50)")
    parser.add_argument("--seed", "-s", type=int, default=None, help="Target a specific seed to reproduce a bug")
    parser.add_argument("--max-fails", type=int, default=3, help="Halt after this many failures (default: 3)")
    # Add this new line:
    parser.add_argument("--blacklist", "-b", type=int, nargs='+', default=[], help="List of seeds to skip manually")
    args = parser.parse_args()

    # Determine seed list
    seed_list = [args.seed] if args.seed is not None else list(range(args.seeds))

    # Filter out blacklisted seeds
    if args.blacklist:
        seed_list = [s for s in seed_list if s not in args.blacklist]

    total_runs = len(seed_list)

    print(f"\n{C.CYAN}{C.BOLD}=== Phia Full-Spectrum Differential Fuzzer ==={C.RESET}")
    print(f"Targeting {total_runs} run(s) across all language features...\n")

    fails = 0
    passed = 0
    skipped = 0
    start_time = time.time()

    for idx, seed in enumerate(seed_list):
        sys.stdout.write(f"\rTesting seed {seed} ({idx + 1}/{total_runs})...")
        sys.stdout.flush()

        res = run_seed(seed)

        if res["status"] == "SKIP":
            skipped += 1
            continue

        if res["status"] == "PASS":
            passed += 1
            continue

        # Handle Failure
        fails += 1
        sys.stdout.write("\r" + " " * 40 + "\r")
        print(f"{C.RED}❌ FAIL [Seed {seed}]{C.RESET} - Reason: {C.BOLD}{res['status']}{C.RESET}")

        if res["status"] in ["MISMATCH", "SHAPE_FAIL"]:
            print_diff(res["ref"], res["got"])
        else:
            print(f"\n{C.YELLOW}Error Details:{C.RESET}\n{res.get('msg', '')}")

        print(f"\n{C.YELLOW}Reproducer Script (/tmp/adv/fuzz_{seed}.lua):{C.RESET}\n{res['src']}")

        if fails >= args.max_fails:
            print(f"\n{C.RED}Reached failure threshold ({args.max_fails}). Halting.{C.RESET}")
            break

    elapsed = time.time() - start_time
    print("-" * 52)
    print(f"Done in {elapsed:.2f}s | {C.GREEN}{passed} Passed{C.RESET} | {C.YELLOW}{skipped} Skipped{C.RESET} | {C.RED if fails else C.GREEN}{fails} Failed{C.RESET}")

    sys.exit(1 if fails > 0 else 0)

if __name__ == "__main__":
    main()
