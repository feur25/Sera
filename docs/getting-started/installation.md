# Installation

<div class="lang-en">

## Requirements

- Python **3.8+**
- pip 21+

SeraPlot ships as a compiled Rust extension (`.pyd` / `.so`) bundled in the wheel. There is **no compiler required** on the user side — the binary is pre-built for each platform.

---

## pip

```bash
pip install seraplot
```

```bash
pip install seraplot==2.3.61
```

```bash
pip install --upgrade seraplot
```

---

## uv

[uv](https://github.com/astral-sh/uv) is a fast Python package manager written in Rust.

```bash
uv add seraplot
```

---

## conda

```bash
conda install -c conda-forge seraplot
```

Or add it to your `environment.yml`:

```yaml
dependencies:
  - pip:
    - seraplot
```

---

## Why the install is this simple

SeraPlot has **zero required Python dependencies**. The Rust extension is entirely self-contained — the HTML output embeds its own JavaScript inline and does not load anything from a CDN. This means:

- charts work offline, in air-gapped environments, in emails, in PDF exports via browser print
- no version conflict with your `numpy`, `pandas`, or `scipy`
- wheels available for Windows, Linux, and macOS — no compilation needed

For comparison, `pip install plotly` downloads ~15 MB. `pip install seraplot` downloads ~2 MB.

</div>

<div class="lang-fr">

## Prérequis

- Python **3.8+**
- pip 21+

SeraPlot se distribue sous forme d'extension Rust compilée (`.pyd` / `.so`) incluse dans le wheel. **Aucun compilateur n'est requis** côté utilisateur — le binaire est pré-compilé pour chaque plateforme.

---

## pip

```bash
pip install seraplot
```

```bash
pip install seraplot==2.3.61
```

```bash
pip install --upgrade seraplot
```

---

## uv

[uv](https://github.com/astral-sh/uv) est un gestionnaire de paquets Python rapide écrit en Rust.

```bash
uv add seraplot
```

---

## conda

```bash
conda install -c conda-forge seraplot
```

Ou dans votre `environment.yml` :

```yaml
dependencies:
  - pip:
    - seraplot
```

---

## Pourquoi l'installation est aussi simple

SeraPlot n'a **aucune dépendance Python requise**. L'extension Rust est entièrement autonome — le HTML embarque son propre JavaScript sans rien charger depuis un CDN. Concrètement :

- les graphiques fonctionnent hors ligne, en environnement isolé, par e-mail, en impression PDF
- aucun conflit de version avec `numpy`, `pandas` ou `scipy`
- wheels disponibles pour Windows, Linux et macOS — aucune compilation nécessaire

Pour comparaison, `pip install plotly` télécharge ~15 Mo. `pip install seraplot` télécharge ~2 Mo.

</div>
