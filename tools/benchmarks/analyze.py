import os
import glob
import json
import pandas as pd
import seraplot as sp

BASE = os.path.dirname(os.path.abspath(__file__))
RESULTS = os.path.join(BASE, "results")
OUT = os.path.join(BASE, "report")
os.makedirs(OUT, exist_ok=True)

TOOLS = ["seraplot", "matplotlib", "plotly"]
ENVS = ["constrained", "standard", "generous"]

RESOURCE_TIERS = {
    "constrained": "1 CPU, 512 MB RAM",
    "standard": "2 CPU, 1.5 GB RAM",
    "generous": "3 CPU, 3 GB RAM",
}

COMPLETE_WARM_TYPES = ["line", "scatter", "histogram", "boxplot"]


def load_cold():
    frames = [pd.read_csv(f) for f in glob.glob(os.path.join(RESULTS, "*.csv")) if "warm_" not in os.path.basename(f)]
    df = pd.concat(frames, ignore_index=True)
    for c in ["import_ms", "data_gen_ms", "build_ms", "render_ms", "total_ms", "file_size_bytes", "peak_mem_mb"]:
        df[c] = pd.to_numeric(df[c], errors="coerce")
    df["lib_ms"] = df["import_ms"] + df["build_ms"] + df["render_ms"]
    df.to_csv(os.path.join(OUT, "raw_cold.csv"), index=False)
    return df


def load_warm():
    files = [f for f in glob.glob(os.path.join(RESULTS, "warm_*.csv")) if "supplementary" not in os.path.basename(f)]
    if not files:
        return None
    frames = [pd.read_csv(f) for f in files]
    df = pd.concat(frames, ignore_index=True)
    for c in ["import_ms", "first_iter_ms", "mean_warm_ms", "total_loop_ms"]:
        df[c] = pd.to_numeric(df[c], errors="coerce")
    df.to_csv(os.path.join(OUT, "raw_warm.csv"), index=False)
    return df


def load_delivery():
    files = glob.glob(os.path.join(RESULTS, "delivery_*.csv"))
    if not files:
        return None
    frames = [pd.read_csv(f) for f in files]
    df = pd.concat(frames, ignore_index=True)
    for c in ["import_ms", "total_delivery_ms", "ms_per_chart", "total_output_bytes", "avg_output_bytes", "peak_mem_mb"]:
        df[c] = pd.to_numeric(df[c], errors="coerce")
    df.to_csv(os.path.join(OUT, "raw_delivery.csv"), index=False)
    return df


def chart_delivery(delivery):
    if delivery is None:
        return None, None
    ok = delivery[delivery["success"] == True]  # noqa: E712
    agg = ok.groupby("tool")["ms_per_chart"].median()
    size_agg = ok.groupby("tool")["avg_output_bytes"].median() / 1024
    chart = sp.bar(
        "Self-contained HTML/PNG delivery cost, 1000 charts, real files written to disk (median ms/chart)",
        labels=["seraplot", "plotly", "matplotlib"],
        values=[agg.get(t, 0) for t in ["seraplot", "plotly", "matplotlib"]],
        y_label="ms/chart (log scale)",
        log_y=True,
    )
    path = os.path.join(OUT, "delivery_cost.html")
    chart.save(path)
    return path, (agg, size_agg)


def load_warm_supplementary():
    path = os.path.join(RESULTS, "warm_supplementary_bar_pie.csv")
    if not os.path.exists(path):
        return None
    df = pd.read_csv(path)
    for c in ["import_ms", "first_iter_ms", "mean_warm_ms", "total_loop_ms"]:
        df[c] = pd.to_numeric(df[c], errors="coerce")
    return df


def cell_ratios(ok, metric):
    cell = ok.groupby(["tool", "chart_type", "n", "env"])[metric].median().unstack("tool")
    cell = cell.dropna(subset=["seraplot"]).reset_index()
    cell["mpl_ratio"] = cell["matplotlib"] / cell["seraplot"]
    cell["ply_ratio"] = cell["plotly"] / cell["seraplot"]
    rows = []
    for ct, grp in cell.groupby("chart_type"):
        m = grp["mpl_ratio"].dropna()
        p = grp["ply_ratio"].dropna()
        rows.append({
            "chart_type": ct,
            "mpl_median": m.median(), "mpl_min": m.min(), "mpl_max": m.max(),
            "ply_median": p.median(), "ply_min": p.min(), "ply_max": p.max(),
        })
    return pd.DataFrame(rows).set_index("chart_type")


def chart_cold_multiplier(cold):
    ok = cold[cold["success"] == True]  # noqa: E712
    dist = cell_ratios(ok, "total_ms")
    chart_types = dist.index.tolist()
    chart = sp.bar(
        "Cold single-call speed, by chart type (per-cell ratio, median of medians across size/environment combinations, full pipeline incl. data generation)",
        labels=chart_types,
        series=[dist["mpl_median"].tolist(), dist["ply_median"].tolist()],
        series_names=["matplotlib", "plotly"],
        y_label="x slower than SeraPlot (full pipeline)",
        variant="grouped",
    )
    path = os.path.join(OUT, "cold_multiplier.html")
    chart.save(path)
    return path, dist


def chart_env_robustness(cold):
    ok = cold[cold["success"] == True]  # noqa: E712
    agg = ok.groupby(["tool", "env"])["total_ms"].median().unstack("tool")
    agg = agg.reindex(ENVS)
    chart = sp.bar(
        "Median cold render time by resource tier (robustness across environments)",
        labels=ENVS,
        series=[agg["seraplot"].tolist(), agg["plotly"].tolist(), agg["matplotlib"].tolist()],
        series_names=["seraplot", "plotly", "matplotlib"],
        y_label="milliseconds (log scale)",
        variant="grouped",
        log_y=True,
    )
    path = os.path.join(OUT, "env_robustness.html")
    chart.save(path)
    return path, agg


def chart_filesize_1m(cold):
    ok = cold[(cold["success"] == True) & (cold["n"] == 1000000)]  # noqa: E712
    agg = ok.groupby(["tool", "chart_type"])["file_size_bytes"].mean() / (1024 * 1024)
    agg = agg.unstack("chart_type")
    chart_types = [c for c in agg.columns if c in ("line", "scatter", "histogram", "boxplot")]
    chart = sp.bar(
        "Output file size at n=1,000,000 (MB) -- SeraPlot vs Plotly, self-contained HTML only",
        labels=chart_types,
        series=[[agg.loc["seraplot", c] for c in chart_types], [agg.loc["plotly", c] for c in chart_types]],
        series_names=["seraplot", "plotly"],
        y_label="MB",
        variant="grouped",
    )
    path = os.path.join(OUT, "filesize_1m.html")
    chart.save(path)
    note = f"Matplotlib produces a rasterized PNG (not comparable HTML): {agg.loc['matplotlib'].mean()*1024:.0f} KB average, but it is not an interactive/self-contained web chart."
    return path, note


def chart_failure_rate(cold):
    cells = [("bar", 100000), ("pie", 100000)]
    rows = []
    for ct, n in cells:
        for t in TOOLS:
            sub = cold[(cold.chart_type == ct) & (cold.n == n) & (cold.tool == t)]
            rate = 100.0 * (sub["success"] == True).sum() / len(sub) if len(sub) else 0  # noqa: E712
            rows.append((ct, t, rate))
    df = pd.DataFrame(rows, columns=["chart_type", "tool", "rate"])
    piv = df.pivot(index="chart_type", columns="tool", values="rate")
    chart = sp.bar(
        "Success rate at n=100,000 categories (45s timeout cap, 15 trials/cell, all 3 environments combined)",
        labels=piv.index.tolist(),
        series=[piv["seraplot"].tolist(), piv["plotly"].tolist(), piv["matplotlib"].tolist()],
        series_names=["seraplot", "plotly", "matplotlib"],
        y_label="% trials completed",
        variant="grouped",
    )
    path = os.path.join(OUT, "failure_rate.html")
    chart.save(path)
    return path, piv


def chart_warm(warm):
    if warm is None:
        return None, None
    ok = warm[warm["success"] == True]  # noqa: E712
    sub = ok[(ok["n"] == 1000) & (ok["chart_type"].isin(COMPLETE_WARM_TYPES))]
    chart_types_agg = sub.groupby(["chart_type", "tool"])["mean_warm_ms"].median().unstack("tool")
    chart_types = chart_types_agg.index.tolist()
    chart = sp.bar(
        "Warm sustained rate at n=1,000 (same process, import paid once -- median of 10 x 500-iteration trials per cell)",
        labels=chart_types,
        series=[chart_types_agg["seraplot"].tolist(), chart_types_agg["plotly"].tolist(), chart_types_agg["matplotlib"].tolist()],
        series_names=["seraplot", "plotly", "matplotlib"],
        y_label="ms/chart (log scale)",
        variant="grouped",
        log_y=True,
    )
    path = os.path.join(OUT, "warm_rate.html")
    chart.save(path)
    ratios = (chart_types_agg["matplotlib"] / chart_types_agg["seraplot"]).to_dict()
    return path, ratios


def chart_warm_dnf(supp):
    if supp is None:
        return None, None
    ok = supp[supp["success"] == True]  # noqa: E712
    agg = ok.groupby(["chart_type", "tool"])["mean_warm_ms"].median().unstack("tool")
    chart_types = agg.index.tolist()
    chart = sp.bar(
        "Categorical warm rate at n=1,000 -- supplementary K=20 run (matplotlib timed out at K=500 in the main sweep, 30/30 trials, all 3 environments)",
        labels=chart_types,
        series=[agg["seraplot"].tolist(), agg["plotly"].tolist(), agg["matplotlib"].tolist()],
        series_names=["seraplot", "plotly", "matplotlib"],
        y_label="ms/chart (log scale)",
        variant="grouped",
        log_y=True,
    )
    path = os.path.join(OUT, "warm_dnf.html")
    chart.save(path)
    ratios = (agg["matplotlib"] / agg["seraplot"]).to_dict()
    return path, ratios


def main():
    cold = load_cold()
    warm = load_warm()
    supp = load_warm_supplementary()
    delivery = load_delivery()

    p1, cold_agg = chart_cold_multiplier(cold)
    p2, env_agg = chart_env_robustness(cold)
    p3, mpl_note = chart_filesize_1m(cold)
    p4, fail_piv = chart_failure_rate(cold)
    p5, warm_ratios = chart_warm(warm)
    p6, dnf_ratios = chart_warm_dnf(supp)
    p7, delivery_stats = chart_delivery(delivery)

    versions = {}
    for f in glob.glob(os.path.join(RESULTS, "*_env.json")):
        if "warm" in os.path.basename(f):
            continue
        d = json.load(open(f))
        versions[d["tool"]] = d.get("tool_version", "unknown")

    print("cold n_trials:", len(cold), "successes:", (cold["success"] == True).sum())  # noqa: E712
    if warm is not None:
        print("warm n_trials:", len(warm), "successes:", (warm["success"] == True).sum(), "x K=500 each")  # noqa: E712
    print("versions:", versions)
    print("cold multiplier (per-cell ratio distribution, total_ms, by chart_type):\n", cold_agg)
    print("warm_ratios (matplotlib/seraplot, n=1000):", warm_ratios)
    print("dnf_ratios (matplotlib/seraplot, bar/pie supplementary):", dnf_ratios)
    print("mpl filesize note:", mpl_note)
    print("failure_rate pivot:\n", fail_piv)
    if delivery_stats is not None:
        print("delivery ms/chart by tool:\n", delivery_stats[0])
        print("delivery avg KB/chart by tool:\n", delivery_stats[1])
    print("charts written to:", OUT)


if __name__ == "__main__":
    main()
