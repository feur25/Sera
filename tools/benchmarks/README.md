# SeraPlot vs Plotly vs Matplotlib -- benchmark suite

Closes #17. A checked-in, Docker-isolated, reproducible benchmark comparing
SeraPlot against Plotly and Matplotlib on the same chart types, the same
data, and the same hardware constraints -- run on your own machine, not
taken on faith.

## Methodology

Two separate regimes are measured, because they give genuinely different
numbers and conflating them is misleading:

- **Cold** (`bench_worker.py` + `orchestrator.py`): one fresh Python
  subprocess per trial. Measures import + data generation + build + render,
  each timed separately. This is the realistic cost of a one-shot script,
  a CLI tool, or a serverless/subprocess-per-request workload.
- **Warm** (`warm_worker.py` + `warm_orchestrator.py`): one process builds
  the same chart `K=500` times in a loop. Import is paid once; this is the
  realistic cost of a long-running dashboard or app that builds many charts
  per process lifetime.

Both run across three Docker resource tiers to rule out "it only wins on a
beefy machine":

| Tier | Resources |
|---|---|
| `constrained` | 1 CPU, 512 MB RAM |
| `standard` | 2 CPU, 1.5 GB RAM |
| `generous` | 3 CPU, 3 GB RAM |

Chart types: `bar`, `line`, `scatter`, `histogram`, `pie`, `boxplot`.
Cold sizes go from 10 to 1,000,000 points/categories; warm sizes are
100/1,000/10,000 (chosen to stay inside a sane time budget at K=500
iterations). Every trial is seeded (`seed = 1000*n + trial` cold,
`2000*n + trial` warm) for reproducibility. A trial that exceeds its
timeout (45s cold, 90s warm) is recorded as a failure, never silently
dropped -- `results/*.csv`'s `success`/`error` columns carry every outcome.

## Running it yourself

```bash
cd tools/benchmarks
docker build --build-arg TOOL=seraplot -t bench-seraplot .
docker build --build-arg TOOL=matplotlib -t bench-matplotlib .
docker build --build-arg TOOL=plotly -t bench-plotly .

# cold sweep, one container per (tool, tier)
docker run --rm --cpus=1 --memory=512m -v "$PWD/results:/results" \
  bench-seraplot python -I orchestrator.py seraplot constrained /results/seraplot_constrained.csv /tmp/out

# warm sweep, same shape
docker run --rm --cpus=1 --memory=512m -v "$PWD/results:/results" \
  bench-seraplot python -I warm_orchestrator.py seraplot constrained /results/warm_seraplot_constrained.csv

python -I analyze.py
```

Repeat for `matplotlib`/`plotly` and the `standard`/`generous` tiers
(`--cpus=2 --memory=1536m` / `--cpus=3 --memory=3072m`). `analyze.py`
reads every CSV in `results/`, writes the merged `raw_cold.csv` /
`raw_warm.csv` into `report/`, and generates the comparison charts.

## What's in `results/`

- `{tool}_{tier}.csv` -- cold trials, one row per (chart_type, n, trial).
- `{tool}_{tier}_env.json` -- the exact environment each cold sweep ran
  in (Python version, platform, CPU count, tool version).
- `warm_{tool}_{tier}.csv` -- warm-loop trials.
- `warm_supplementary_bar_pie.csv` -- a smaller, separate K=20 run for
  `bar`/`pie` at n=1,000, needed because Matplotlib timed out on 30/30
  of the real K=500 trials for those two chart types in every tier (a
  real, reported finding, not a methodology gap -- see the failure data
  in `warm_*.csv` itself).

## What this benchmark already found

Building and running this suite surfaced two real SeraPlot bugs (now
fixed) and one real documentation bug (not yet fixed):

- #29 -- chart builders silently dropped positional arguments beyond the
  title instead of raising an error.
- #30 -- native fast-path builders (`bar`/`line`/`heatmap`/...) paid a
  ~100ms one-time numpy-import tax on the first call of a cold process,
  even when the caller never touched numpy. This alone explained most of
  the "SeraPlot seems to have gotten slower on bar/line/scatter" reports.
- #31 -- `docs/getting-started/intro-seraplot.md`'s speed table compares
  non-equivalent columns, inflating its multiplier by roughly 46x. Still
  open; will be fixed when that page is rewritten from this benchmark's
  numbers.
