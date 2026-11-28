import sys
import os
import json
import csv
import subprocess
import platform

TOOL = sys.argv[1]
ENV_TAG = sys.argv[2]
OUT_CSV = sys.argv[3]
TMP_DIR = sys.argv[4]

CATEGORICAL_TYPES = ["bar", "pie"]
CONTINUOUS_TYPES = ["line", "scatter", "histogram", "boxplot"]
CATEGORICAL_SIZES = [10, 100, 1000, 10000, 100000]
CONTINUOUS_SIZES = [10, 100, 1000, 10000, 100000, 1000000]
TRIALS_BY_SIZE = {10: 50, 100: 50, 1000: 50, 10000: 25, 100000: 15, 1000000: 8}
TIMEOUT_S = 45

PLAN = [(c, n) for c in CATEGORICAL_TYPES for n in CATEGORICAL_SIZES] + \
       [(c, n) for c in CONTINUOUS_TYPES for n in CONTINUOUS_SIZES]

FIELDS = [
    "tool", "env", "chart_type", "n", "trial", "success", "error",
    "import_ms", "data_gen_ms", "build_ms", "render_ms", "total_ms",
    "file_size_bytes", "peak_mem_mb",
]


def main():
    os.makedirs(TMP_DIR, exist_ok=True)
    rows = []
    total = sum(TRIALS_BY_SIZE[n] for _, n in PLAN)
    done = 0
    for chart, n in PLAN:
        for trial in range(TRIALS_BY_SIZE[n]):
            done += 1
            row = {k: "" for k in FIELDS}
            row.update(tool=TOOL, env=ENV_TAG, chart_type=chart, n=n, trial=trial, success=False, error="")
            try:
                proc = subprocess.run(
                    [sys.executable, "-I", "/bench/bench_worker.py", TOOL, chart, str(n), str(trial), TMP_DIR],
                    capture_output=True, text=True, timeout=TIMEOUT_S,
                )
                if proc.returncode == 0 and proc.stdout.strip():
                    parsed = json.loads(proc.stdout.strip().splitlines()[-1])
                    row.update(parsed)
                else:
                    row["error"] = f"exit={proc.returncode} stderr={proc.stderr[-300:]}"
            except subprocess.TimeoutExpired:
                row["error"] = "timeout"
            rows.append(row)
            if done % 50 == 0 or done == total:
                print(f"[{TOOL}/{ENV_TAG}] {done}/{total}", flush=True)

    with open(OUT_CSV, "w", newline="") as f:
        w = csv.DictWriter(f, fieldnames=FIELDS)
        w.writeheader()
        for r in rows:
            w.writerow({k: r.get(k, "") for k in FIELDS})

    env = {
        "tool": TOOL,
        "env_tag": ENV_TAG,
        "python_version": platform.python_version(),
        "platform": platform.platform(),
        "machine": platform.machine(),
        "cpu_count": os.cpu_count(),
    }
    try:
        import importlib
        mod = importlib.import_module(TOOL)
        env["tool_version"] = getattr(mod, "__version__", "unknown")
    except Exception as e:
        env["tool_version"] = f"error: {e}"

    with open(OUT_CSV.replace(".csv", "_env.json"), "w") as f:
        json.dump(env, f, indent=2)

    print(f"[{TOOL}/{ENV_TAG}] done, {len(rows)} rows written to {OUT_CSV}")


if __name__ == "__main__":
    main()
