import sys
import os
import json
import csv
import subprocess

TOOL = sys.argv[1]
ENV_TAG = sys.argv[2]
OUT_CSV = sys.argv[3]
TMP_DIR = sys.argv[4]

N = 8
K = 1000
TRIALS = 10
TIMEOUT_S = 180

FIELDS = [
    "tool", "env", "chart_type", "n", "k", "trial", "success", "error",
    "import_ms", "total_delivery_ms", "ms_per_chart",
    "total_output_bytes", "avg_output_bytes", "peak_mem_mb",
]


def main():
    rows = []
    for trial in range(TRIALS):
        row = {k: "" for k in FIELDS}
        row.update(tool=TOOL, env=ENV_TAG, chart_type="bar", n=N, k=K, trial=trial, success=False, error="")
        work_dir = os.path.join(TMP_DIR, f"delivery_{TOOL}_{trial}")
        try:
            proc = subprocess.run(
                [sys.executable, "-I", "/bench/delivery_worker.py", TOOL, "bar", str(N), str(K), str(trial), work_dir],
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
        print(f"[delivery {TOOL}/{ENV_TAG}] {trial+1}/{TRIALS}", flush=True)

    with open(OUT_CSV, "w", newline="") as f:
        w = csv.DictWriter(f, fieldnames=FIELDS)
        w.writeheader()
        for r in rows:
            w.writerow({k: r.get(k, "") for k in FIELDS})

    print(f"[delivery {TOOL}/{ENV_TAG}] done, {len(rows)} rows -> {OUT_CSV}")


if __name__ == "__main__":
    main()
