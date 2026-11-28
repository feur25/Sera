import sys
import os
import json
import random
import resource
from time import perf_counter

TOOL = sys.argv[1]
CHART = sys.argv[2]
N = int(sys.argv[3])
TRIAL = int(sys.argv[4])
OUT_DIR = sys.argv[5]

random.seed(1000 * N + TRIAL)


def gen_categorical(n):
    cats = [f"cat_{i}" for i in range(n)]
    vals = [random.uniform(1, 1000) for _ in range(n)]
    return cats, vals


def gen_xy(n):
    xs = list(range(n))
    walk = 0.0
    ys = []
    for _ in range(n):
        walk += random.gauss(0, 1)
        ys.append(walk)
    return xs, ys


def gen_normal(n):
    return [random.gauss(50, 15) for _ in range(n)]


def gen_groups(n):
    k = 4
    per = max(1, n // k)
    return {f"group_{g}": [random.gauss(g * 10, 5) for _ in range(per)] for g in range(k)}


def out_path(ext):
    return os.path.join(OUT_DIR, f"{TOOL}_{CHART}_{N}_{TRIAL}.{ext}")


def run_seraplot():
    import seraplot as sp
    t_import = perf_counter()
    if CHART == "bar":
        cats, vals = gen_categorical(N)
        t_data = perf_counter()
        chart = sp.bar("Benchmark", labels=cats, values=vals)
    elif CHART == "line":
        xs, ys = gen_xy(N)
        t_data = perf_counter()
        chart = sp.line("Benchmark", labels=xs, values=ys)
    elif CHART == "scatter":
        xs, ys = gen_xy(N)
        t_data = perf_counter()
        chart = sp.scatter("Benchmark", x=xs, y=ys)
    elif CHART == "histogram":
        data = gen_normal(N)
        t_data = perf_counter()
        chart = sp.histogram("Benchmark", values=data)
    elif CHART == "pie":
        cats, vals = gen_categorical(N)
        t_data = perf_counter()
        chart = sp.pie("Benchmark", labels=cats, values=vals)
    elif CHART == "boxplot":
        groups = gen_groups(N)
        flat_labels = []
        flat_values = []
        for g, vs in groups.items():
            flat_labels.extend([g] * len(vs))
            flat_values.extend(vs)
        t_data = perf_counter()
        chart = sp.boxplot("Benchmark", labels=flat_labels, values=flat_values)
    else:
        raise ValueError(CHART)
    t_build = perf_counter()
    path = out_path("html")
    chart.save(path)
    t_render = perf_counter()
    return t_import, t_data, t_build, t_render, path


def run_matplotlib():
    import matplotlib
    matplotlib.use("Agg")
    import matplotlib.pyplot as plt
    t_import = perf_counter()
    fig, ax = plt.subplots(figsize=(9, 5))
    if CHART == "bar":
        cats, vals = gen_categorical(N)
        t_data = perf_counter()
        ax.bar(cats, vals)
    elif CHART == "line":
        xs, ys = gen_xy(N)
        t_data = perf_counter()
        ax.plot(xs, ys)
    elif CHART == "scatter":
        xs, ys = gen_xy(N)
        t_data = perf_counter()
        ax.scatter(xs, ys, s=4)
    elif CHART == "histogram":
        data = gen_normal(N)
        t_data = perf_counter()
        ax.hist(data, bins=min(50, max(5, N // 10) or 5))
    elif CHART == "pie":
        cats, vals = gen_categorical(N)
        t_data = perf_counter()
        ax.pie(vals, labels=cats if N <= 50 else None)
    elif CHART == "boxplot":
        groups = gen_groups(N)
        t_data = perf_counter()
        ax.boxplot(list(groups.values()), tick_labels=list(groups.keys()))
    else:
        raise ValueError(CHART)
    ax.set_title("Benchmark")
    t_build = perf_counter()
    path = out_path("png")
    fig.savefig(path)
    plt.close(fig)
    t_render = perf_counter()
    return t_import, t_data, t_build, t_render, path


def run_plotly():
    import plotly.graph_objects as go
    t_import = perf_counter()
    if CHART == "bar":
        cats, vals = gen_categorical(N)
        t_data = perf_counter()
        fig = go.Figure(data=[go.Bar(x=cats, y=vals)])
    elif CHART == "line":
        xs, ys = gen_xy(N)
        t_data = perf_counter()
        fig = go.Figure(data=[go.Scatter(x=xs, y=ys, mode="lines")])
    elif CHART == "scatter":
        xs, ys = gen_xy(N)
        t_data = perf_counter()
        fig = go.Figure(data=[go.Scatter(x=xs, y=ys, mode="markers", marker=dict(size=4))])
    elif CHART == "histogram":
        data = gen_normal(N)
        t_data = perf_counter()
        fig = go.Figure(data=[go.Histogram(x=data)])
    elif CHART == "pie":
        cats, vals = gen_categorical(N)
        t_data = perf_counter()
        fig = go.Figure(data=[go.Pie(labels=cats, values=vals)])
    elif CHART == "boxplot":
        groups = gen_groups(N)
        t_data = perf_counter()
        fig = go.Figure(data=[go.Box(y=v, name=k) for k, v in groups.items()])
    else:
        raise ValueError(CHART)
    fig.update_layout(title="Benchmark", template="plotly_white")
    t_build = perf_counter()
    path = out_path("html")
    fig.write_html(path, include_plotlyjs="cdn")
    t_render = perf_counter()
    return t_import, t_data, t_build, t_render, path


def main():
    t0 = perf_counter()
    result = {
        "tool": TOOL,
        "chart_type": CHART,
        "n": N,
        "trial": TRIAL,
        "success": False,
    }
    try:
        runner = {"seraplot": run_seraplot, "matplotlib": run_matplotlib, "plotly": run_plotly}[TOOL]
        t_import, t_data, t_build, t_render, path = runner()
        file_size = os.path.getsize(path)
        os.remove(path)
        peak_mem_kb = resource.getrusage(resource.RUSAGE_SELF).ru_maxrss
        result.update(
            success=True,
            import_ms=(t_import - t0) * 1000.0,
            data_gen_ms=(t_data - t_import) * 1000.0,
            build_ms=(t_build - t_data) * 1000.0,
            render_ms=(t_render - t_build) * 1000.0,
            total_ms=(t_render - t0) * 1000.0,
            file_size_bytes=file_size,
            peak_mem_mb=peak_mem_kb / 1024.0,
        )
    except Exception as e:
        result.update(error=f"{type(e).__name__}: {e}")
    print(json.dumps(result))


if __name__ == "__main__":
    main()
