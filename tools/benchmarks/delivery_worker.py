import sys
import os
import json
import random
import shutil
import resource
from time import perf_counter

TOOL = sys.argv[1]
CHART = sys.argv[2]
N = int(sys.argv[3])
K = int(sys.argv[4])
TRIAL = int(sys.argv[5])
OUT_DIR = sys.argv[6]

random.seed(3000 * N + TRIAL)


def gen_categorical(n):
    return [f"cat_{i}" for i in range(n)], [random.uniform(1, 1000) for _ in range(n)]


def gen_xy(n):
    xs = list(range(n))
    walk = 0.0
    ys = []
    for _ in range(n):
        walk += random.gauss(0, 1)
        ys.append(walk)
    return xs, ys


def one_seraplot(sp, i, cats, vals):
    chart = sp.bar(f"Report #{i+1}", labels=cats, values=vals)
    path = os.path.join(OUT_DIR, f"sera_{i}.html")
    chart.save(path)
    return path


def one_matplotlib(plt, i, cats, vals):
    fig, ax = plt.subplots(figsize=(9, 5))
    ax.bar(cats, vals)
    ax.set_title(f"Report #{i+1}")
    path = os.path.join(OUT_DIR, f"mpl_{i}.png")
    fig.savefig(path)
    plt.close(fig)
    return path


def one_plotly(go, i, cats, vals):
    fig = go.Figure(data=[go.Bar(x=cats, y=vals)])
    fig.update_layout(title=f"Report #{i+1}", template="plotly_white")
    path = os.path.join(OUT_DIR, f"plotly_{i}.html")
    fig.write_html(path, include_plotlyjs=True)
    return path


def main():
    os.makedirs(OUT_DIR, exist_ok=True)
    result = {"tool": TOOL, "chart_type": CHART, "n": N, "k": K, "trial": TRIAL, "success": False}
    try:
        t0 = perf_counter()
        if TOOL == "seraplot":
            import seraplot as sp
        elif TOOL == "matplotlib":
            import matplotlib
            matplotlib.use("Agg")
            import matplotlib.pyplot as plt
        elif TOOL == "plotly":
            import plotly.graph_objects as go
        import_ms = (perf_counter() - t0) * 1000.0

        total_bytes = 0
        t0 = perf_counter()
        for i in range(K):
            cats, vals = gen_categorical(N)
            if TOOL == "seraplot":
                path = one_seraplot(sp, i, cats, vals)
            elif TOOL == "matplotlib":
                path = one_matplotlib(plt, i, cats, vals)
            elif TOOL == "plotly":
                path = one_plotly(go, i, cats, vals)
            total_bytes += os.path.getsize(path)
        total_delivery_ms = (perf_counter() - t0) * 1000.0
        peak_mem_kb = resource.getrusage(resource.RUSAGE_SELF).ru_maxrss

        result.update(
            success=True,
            import_ms=import_ms,
            total_delivery_ms=total_delivery_ms,
            ms_per_chart=total_delivery_ms / K,
            total_output_bytes=total_bytes,
            avg_output_bytes=total_bytes / K,
            peak_mem_mb=peak_mem_kb / 1024.0,
        )
    except Exception as e:
        result["error"] = f"{type(e).__name__}: {e}"
    finally:
        shutil.rmtree(OUT_DIR, ignore_errors=True)
    print(json.dumps(result))


if __name__ == "__main__":
    main()
