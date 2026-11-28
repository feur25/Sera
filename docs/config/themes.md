# Themes

<div class="lang-en">

## API

| Function | Description |
|---|---|
| `sp.theme(name)` | Apply a built-in theme — sets background, palette, and gridlines globally |
| `sp.reset_theme()` | Revert to defaults (no background, default palette, no gridlines) |
| `sp.themes()` | Returns a list of all available theme names |

```python
import seraplot as sp

sp.theme("dark")
chart = sp.bar("Revenue", labels=["Q1", "Q2", "Q3"], values=[120, 145, 98])

sp.reset_theme()
```

---

## All 7 themes

<div data-sp-theme-table="en">Loading theme registry…</div>

---

## Full palette per theme

<div data-sp-theme-palettes="en"></div>

---

## Examples

```python
import seraplot as sp

sp.theme("dark")
sp.bar("Revenue", labels=["Q1", "Q2", "Q3", "Q4"], values=[120, 145, 98, 180]).show()

sp.theme("neon")
sp.scatter(title="Clusters", x=[1, 2, 3, 4, 5, 6], y=[2, 5, 3, 8, 7, 9]).show()

sp.theme("scientific")
sp.line(title="Population Growth", x_labels=["2020", "2021", "2022", "2023"], values=[100, 112, 121, 135]).show()

sp.reset_theme()
```

```python
print(sp.themes())
# ['dark', 'light', 'scientific', 'apple', 'notion', 'minimal', 'neon']
```

---

## Notes

- `sp.theme()` sets the global background, palette, and gridlines. It is equivalent to calling `sp.config(background=..., palette=..., gridlines=...)` with the preset values.
- Themes persist until `sp.reset_theme()` or `sp.config()` overrides them.
- You can further override individual properties after calling a theme:

```python
sp.theme("dark")
sp.config(font_size=16, border_radius=12)
```

---

</div>

<div class="lang-fr">

## API

| Fonction | Description |
|---|---|
| `sp.theme(name)` | Applique un thème intégré — définit le fond, la palette et le quadrillage globalement |
| `sp.reset_theme()` | Revient aux valeurs par défaut (pas de fond, palette par défaut, pas de quadrillage) |
| `sp.themes()` | Retourne la liste de tous les noms de thèmes disponibles |

```python
import seraplot as sp

sp.theme("dark")
graphique = sp.bar("Revenus", labels=["T1", "T2", "T3"], values=[120, 145, 98])

sp.reset_theme()
```

---

## Les 7 thèmes disponibles

<div data-sp-theme-table="fr">Chargement du registre des thèmes…</div>

---

## Palettes complètes

<div data-sp-theme-palettes="fr"></div>

---

## Exemples

```python
import seraplot as sp

sp.theme("dark")
sp.bar("Revenus", labels=["T1", "T2", "T3", "T4"], values=[120, 145, 98, 180]).show()

sp.theme("neon")
sp.scatter(title="Clusters", x=[1, 2, 3, 4, 5, 6], y=[2, 5, 3, 8, 7, 9]).show()

sp.theme("scientific")
sp.line(title="Croissance démographique", x_labels=["2020", "2021", "2022", "2023"], values=[100, 112, 121, 135]).show()

sp.reset_theme()
```

```python
print(sp.themes())
# ['dark', 'light', 'scientific', 'apple', 'notion', 'minimal', 'neon']
```

---

## Notes

- `sp.theme()` définit le fond global, la palette et le quadrillage. C'est équivalent à `sp.config(background=..., palette=..., gridlines=...)` avec les valeurs du préréglage.
- Les thèmes persistent jusqu'à `sp.reset_theme()` ou un appel `sp.config()` qui les écrase.
- Vous pouvez continuer à surcharger des propriétés individuelles après avoir appliqué un thème :

```python
sp.theme("dark")
sp.config(font_size=16, border_radius=12)
```

---

</div>
