import sys
import os
import json
import csv
import subprocess
import platform

TOOL = sys.argv[1]
ENV_TAG = sys.argv[2]
OUT_CSV = sys.argv[3]

CHART_TYPES = ["bar", "line", "scatter", "histogram", "pie", "boxplot"]
SIZES = [100, 1000, 10000]
K = 500
TRIALS = 10
TIMEOUT_S = 90

FIELDS = [
    "tool", "env", "chart_type", "n", "k", "trial", "success", "error",
    "import_ms", "first_iter_ms", "mean_warm_ms", "total_loop_ms",
]


def main():
    rows = []
    total = len(CHART_TYPES) * len(SIZES) * TRIALS
    done = 0
    for chart in CHART_TYPES:
        for n in SIZES:
            for trial in range(TRIALS):
                done += 1
                row = {k: "" for k in FIELDS}
                row.update(tool=TOOL, env=ENV_TAG, chart_type=chart, n=n, k=K, trial=trial, success=False, error="")
                try:
                    proc = subprocess.run(
                        [sys.executable, "-I", "/bench/warm_worker.py", TOOL, chart, str(n), str(K), str(trial)],
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
                if done % 10 == 0 or done == total:
                    print(f"[warm {TOOL}/{ENV_TAG}] {done}/{total}", flush=True)

    with open(OUT_CSV, "w", newline="") as f:
        w = csv.DictWriter(f, fieldnames=FIELDS)
        w.writeheader()
        for r in rows:
            w.writerow({k: r.get(k, "") for k in FIELDS})

    print(f"[warm {TOOL}/{ENV_TAG}] done, {len(rows)} rows -> {OUT_CSV}")


if __name__ == "__main__":
    main()
