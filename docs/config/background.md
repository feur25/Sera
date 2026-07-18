# Background Configuration

<div class="lang-en">

## Functions

| Function | Description |
|----------|-------------|
| `set_global_background(color)` | Sets a global background color applied to all charts created after the call. |
| `reset_global_background()` | Clears the global background, reverting to each chart's own default. |

| Parameter | Type | Description |
|-----------|------|-------------|
| `color` | `str` | CSS color string (hex `"#rrggbb"`, `"rgb(...)"`, named color) |

---

## Example

```python
import seraplot as sp

sp.set_global_background("#0f172a")

bar = sp.build_bar_chart("Revenue", labels=["A", "B"], values=[300, 200])
pie = sp.build_pie_chart("Share", labels=["A", "B"], values=[60, 40])

sp.reset_global_background()
```

`sp.set_global_background(...)` is equivalent to `sp.config(background=...)`, and
is overridden by `sp.theme(name)` if a theme is applied afterward — see
[Themes](themes.md) for the built-in background/palette/gridlines bundles.

</div>

<div class="lang-fr">

## Fonctions

| Fonction | Description |
|----------|-------------|
| `set_global_background(color)` | Définit une couleur de fond globale appliquée à tous les graphiques créés après l'appel. |
| `reset_global_background()` | Efface le fond global, revenant à la valeur par défaut de chaque graphique. |

| Paramètre | Type | Description |
|-----------|------|-------------|
| `color` | `str` | Couleur CSS (hex `"#rrggbb"`, `"rgb(...)"`, nom de couleur) |

---

## Exemple

```python
import seraplot as sp

sp.set_global_background("#0f172a")

barre = sp.build_bar_chart("Revenus", labels=["A", "B"], values=[300, 200])
camembert = sp.build_pie_chart("Parts", labels=["A", "B"], values=[60, 40])

sp.reset_global_background()
```

`sp.set_global_background(...)` équivaut à `sp.config(background=...)`, et est
écrasé par `sp.theme(name)` si un thème est appliqué ensuite — voir
[Thèmes](themes.md) pour la liste des thèmes intégrés (fond + palette +
quadrillage).

</div>
