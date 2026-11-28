import sys
import json
import random
import io
from time import perf_counter

TOOL = sys.argv[1]
CHART = sys.argv[2]
N = int(sys.argv[3])
K = int(sys.argv[4])
TRIAL = int(sys.argv[5])

random.seed(2000 * N + TRIAL)


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


def gen_normal(n):
    return [random.gauss(50, 15) for _ in range(n)]


def gen_boxplot_flat(n):
    k = 4
    per = max(1, n // k)
    labels, values = [], []
    for g in range(k):
        vs = [random.gauss(g * 10, 5) for _ in range(per)]
        labels.extend([f"group_{g}"] * len(vs))
        values.extend(vs)
    return labels, values


def make_data():
    if CHART in ("bar", "pie"):
        return gen_categorical(N)
    if CHART == "boxplot":
        return gen_boxplot_flat(N)
    if CHART == "histogram":
        return gen_normal(N), None
    return gen_xy(N)


def one_seraplot(sp, data):
    if CHART == "bar":
        cats, vals = data
        c = sp.bar("Benchmark", labels=cats, values=vals)
    elif CHART == "line":
        xs, ys = data
        c = sp.line("Benchmark", labels=xs, values=ys)
    elif CHART == "scatter":
        xs, ys = data
        c = sp.scatter("Benchmark", x=xs, y=ys)
    elif CHART == "histogram":
        vals, _ = data
        c = sp.histogram("Benchmark", values=vals)
    elif CHART == "pie":
        cats, vals = data
        c = sp.pie("Benchmark", labels=cats, values=vals)
    elif CHART == "boxplot":
        labels, vals = data
        c = sp.boxplot("Benchmark", labels=labels, values=vals)
    return c.html


def one_matplotlib(plt, data):
    fig, ax = plt.subplots(figsize=(9, 5))
    if CHART == "bar":
        cats, vals = data
        ax.bar(cats, vals)
    elif CHART == "line":
        xs, ys = data
        ax.plot(xs, ys)
    elif CHART == "scatter":
        xs, ys = data
        ax.scatter(xs, ys, s=4)
    elif CHART == "histogram":
        vals, _ = data
        ax.hist(vals, bins=min(50, max(5, N // 10) or 5))
    elif CHART == "pie":
        cats, vals = data
        ax.pie(vals, labels=cats if N <= 50 else None)
    elif CHART == "boxplot":
        labels, vals = data
        groups = {}
        for lb, v in zip(labels, vals):
            groups.setdefault(lb, []).append(v)
        ax.boxplot(list(groups.values()), tick_labels=list(groups.keys()))
    buf = io.BytesIO()
    fig.savefig(buf, format="png")
    plt.close(fig)
    return buf.tell()


def one_plotly(go, data):
    if CHART == "bar":
        cats, vals = data
        fig = go.Figure(data=[go.Bar(x=cats, y=vals)])
    elif CHART == "line":
        xs, ys = data
        fig = go.Figure(data=[go.Scatter(x=xs, y=ys, mode="lines")])
    elif CHART == "scatter":
        xs, ys = data
        fig = go.Figure(data=[go.Scatter(x=xs, y=ys, mode="markers", marker=dict(size=4))])
    elif CHART == "histogram":
        vals, _ = data
        fig = go.Figure(data=[go.Histogram(x=vals)])
    elif CHART == "pie":
        cats, vals = data
        fig = go.Figure(data=[go.Pie(labels=cats, values=vals)])
    elif CHART == "boxplot":
        labels, vals = data
        groups = {}
        for lb, v in zip(labels, vals):
            groups.setdefault(lb, []).append(v)
        fig = go.Figure(data=[go.Box(y=v, name=k) for k, v in groups.items()])
    fig.update_layout(title="Benchmark", template="plotly_white")
    return fig.to_html(include_plotlyjs="cdn")


def main():
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

        per_iter_ms = []
        for i in range(K):
            data = make_data()
            t0 = perf_counter()
            if TOOL == "seraplot":
                one_seraplot(sp, data)
            elif TOOL == "matplotlib":
                one_matplotlib(plt, data)
            elif TOOL == "plotly":
                one_plotly(go, data)
            per_iter_ms.append((perf_counter() - t0) * 1000.0)

        first_ms = per_iter_ms[0]
        rest = per_iter_ms[1:]
        mean_rest_ms = sum(rest) / len(rest) if rest else first_ms
        total_loop_ms = import_ms + sum(per_iter_ms)
        result.update(
            success=True,
            import_ms=import_ms,
            first_iter_ms=first_ms,
            mean_warm_ms=mean_rest_ms,
            total_loop_ms=total_loop_ms,
        )
    except Exception as e:
        result["error"] = f"{type(e).__name__}: {e}"
    print(json.dumps(result))


if __name__ == "__main__":
    main()
