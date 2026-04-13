# Installation

## Requirements / Prérequis

- Python **3.8+**
- pip 21+

---

## From PyPI / Depuis PyPI

```bash
pip install seraplot
```

## Specific version / Version spécifique

```bash
pip install seraplot==2.3.47
```

## Upgrade / Mise à jour

```bash
pip install --upgrade seraplot
```

---

## Jupyter / Notebook

SeraPlot displays charts **automatically** in Jupyter without any `display()` call.

SeraPlot affiche les graphiques **automatiquement** dans Jupyter sans appel `display()`.

```python
import seraplot as sp

sp.__version__
```

---

## Optional dependencies / Dépendances optionnelles

SeraPlot has **zero required Python dependencies** beyond Python itself.

For ML comparisons in notebooks:

```bash
pip install numpy scikit-learn matplotlib
```

---

## Build from source / Compiler depuis les sources

```bash
git clone https://github.com/feur25/seraplot.git
cd seraplot
pip install maturin
maturin develop --features python
```
