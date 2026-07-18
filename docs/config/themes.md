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

<style>
.sp-sw{display:inline-block;width:14px;height:14px;border-radius:3px;vertical-align:middle;margin-right:3px;border:1px solid rgba(255,255,255,.15)}
.sp-bg{display:inline-block;width:60px;height:20px;border-radius:4px;vertical-align:middle;border:1px solid rgba(255,255,255,.15)}
.sp-pal{display:flex;gap:3px;align-items:center}
</style>

| Theme | Background | Gridlines | Inspiration | Primary palette |
|---|---|:---:|---|---|
| `"dark"` | <span class="sp-bg" style="background:#0f172a"></span> `#0f172a` | ✓ | Deep space / Tailwind indigo | <span class="sp-pal"><span class="sp-sw" style="background:#818CF8"></span><span class="sp-sw" style="background:#FB7185"></span><span class="sp-sw" style="background:#34D399"></span><span class="sp-sw" style="background:#FBBF24"></span><span class="sp-sw" style="background:#A78BFA"></span></span> |
| `"light"` | transparent | — | Clean minimal | <span class="sp-pal"><span class="sp-sw" style="background:#636EFA"></span><span class="sp-sw" style="background:#EF553B"></span><span class="sp-sw" style="background:#00CC96"></span><span class="sp-sw" style="background:#AB63FA"></span><span class="sp-sw" style="background:#FFA15A"></span></span> |
| `"scientific"` | <span class="sp-bg" style="background:#fafafa;border:1px solid #ccc"></span> `#fafafa` | ✓ | Matplotlib / D3 academic | <span class="sp-pal"><span class="sp-sw" style="background:#1F77B4"></span><span class="sp-sw" style="background:#FF7F0E"></span><span class="sp-sw" style="background:#2CA02C"></span><span class="sp-sw" style="background:#D62728"></span><span class="sp-sw" style="background:#9467BD"></span></span> |
| `"apple"` | <span class="sp-bg" style="background:#000000"></span> `#000000` | — | Apple dark mode (iOS/macOS) | <span class="sp-pal"><span class="sp-sw" style="background:#0A84FF"></span><span class="sp-sw" style="background:#30D158"></span><span class="sp-sw" style="background:#FF453A"></span><span class="sp-sw" style="background:#FFD60A"></span><span class="sp-sw" style="background:#BF5AF2"></span></span> |
| `"notion"` | <span class="sp-bg" style="background:#191919"></span> `#191919` | — | Notion editorial dark | <span class="sp-pal"><span class="sp-sw" style="background:#529CCA"></span><span class="sp-sw" style="background:#D08B65"></span><span class="sp-sw" style="background:#6C9B7D"></span><span class="sp-sw" style="background:#CB7C7A"></span><span class="sp-sw" style="background:#9A6DD7"></span></span> |
| `"minimal"` | transparent | — | Monochrome grayscale | <span class="sp-pal"><span class="sp-sw" style="background:#374151"></span><span class="sp-sw" style="background:#6B7280"></span><span class="sp-sw" style="background:#9CA3AF"></span><span class="sp-sw" style="background:#D1D5DB"></span><span class="sp-sw" style="background:#111827"></span></span> |
| `"neon"` | <span class="sp-bg" style="background:#0a0a0a"></span> `#0a0a0a` | — | Cyberpunk / retrowave | <span class="sp-pal"><span class="sp-sw" style="background:#00FF87"></span><span class="sp-sw" style="background:#FF006E"></span><span class="sp-sw" style="background:#00B4D8"></span><span class="sp-sw" style="background:#FFBE0B"></span><span class="sp-sw" style="background:#E500A4"></span></span> |

---

## Full palette per theme

### `"dark"`

```python
[0x818CF8, 0xFB7185, 0x34D399, 0xFBBF24, 0xA78BFA,
 0x22D3EE, 0xF472B6, 0xA3E635, 0xF87171, 0x2DD4BF]
```

### `"light"`

```python
[0x636EFA, 0xEF553B, 0x00CC96, 0xAB63FA, 0xFFA15A,
 0x19D3F3, 0xFF6692, 0xB6E880, 0xFF97FF, 0xFECB52]
```

### `"scientific"`

```python
[0x1F77B4, 0xFF7F0E, 0x2CA02C, 0xD62728, 0x9467BD,
 0x8C564B, 0xE377C2, 0x7F7F7F, 0xBCBD22, 0x17BECF]
```

### `"apple"`

```python
[0x0A84FF, 0x30D158, 0xFF453A, 0xFFD60A, 0xBF5AF2,
 0x64D2FF, 0xFF9F0A, 0xFF375F, 0xAC8E68, 0x63E6E2]
```

### `"notion"`

```python
[0x529CCA, 0xD08B65, 0x6C9B7D, 0xCB7C7A, 0x9A6DD7,
 0x868686, 0xCCAA55, 0x75B5AA, 0xD477A8, 0x507AA6]
```

### `"minimal"`

```python
[0x374151, 0x6B7280, 0x9CA3AF, 0xD1D5DB, 0x111827,
 0x4B5563, 0x1F2937, 0xE5E7EB, 0x030712, 0x6B7280]
```

### `"neon"`

```python
[0x00FF87, 0xFF006E, 0x00B4D8, 0xFFBE0B, 0xE500A4,
 0x8338EC, 0x3A86FF, 0xFB5607, 0xFF006E, 0x06D6A0]
```

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

| Thème | Fond | Quadrillage | Inspiration | Palette principale |
|---|---|:---:|---|---|
| `"dark"` | <span class="sp-bg" style="background:#0f172a"></span> `#0f172a` | ✓ | Espace profond / indigo Tailwind | <span class="sp-pal"><span class="sp-sw" style="background:#818CF8"></span><span class="sp-sw" style="background:#FB7185"></span><span class="sp-sw" style="background:#34D399"></span><span class="sp-sw" style="background:#FBBF24"></span><span class="sp-sw" style="background:#A78BFA"></span></span> |
| `"light"` | transparent | — | Épuré minimal | <span class="sp-pal"><span class="sp-sw" style="background:#636EFA"></span><span class="sp-sw" style="background:#EF553B"></span><span class="sp-sw" style="background:#00CC96"></span><span class="sp-sw" style="background:#AB63FA"></span><span class="sp-sw" style="background:#FFA15A"></span></span> |
| `"scientific"` | <span class="sp-bg" style="background:#fafafa;border:1px solid #ccc"></span> `#fafafa` | ✓ | Matplotlib / D3 académique | <span class="sp-pal"><span class="sp-sw" style="background:#1F77B4"></span><span class="sp-sw" style="background:#FF7F0E"></span><span class="sp-sw" style="background:#2CA02C"></span><span class="sp-sw" style="background:#D62728"></span><span class="sp-sw" style="background:#9467BD"></span></span> |
| `"apple"` | <span class="sp-bg" style="background:#000000"></span> `#000000` | — | Mode sombre Apple (iOS/macOS) | <span class="sp-pal"><span class="sp-sw" style="background:#0A84FF"></span><span class="sp-sw" style="background:#30D158"></span><span class="sp-sw" style="background:#FF453A"></span><span class="sp-sw" style="background:#FFD60A"></span><span class="sp-sw" style="background:#BF5AF2"></span></span> |
| `"notion"` | <span class="sp-bg" style="background:#191919"></span> `#191919` | — | Notion éditorial sombre | <span class="sp-pal"><span class="sp-sw" style="background:#529CCA"></span><span class="sp-sw" style="background:#D08B65"></span><span class="sp-sw" style="background:#6C9B7D"></span><span class="sp-sw" style="background:#CB7C7A"></span><span class="sp-sw" style="background:#9A6DD7"></span></span> |
| `"minimal"` | transparent | — | Nuances de gris monochrome | <span class="sp-pal"><span class="sp-sw" style="background:#374151"></span><span class="sp-sw" style="background:#6B7280"></span><span class="sp-sw" style="background:#9CA3AF"></span><span class="sp-sw" style="background:#D1D5DB"></span><span class="sp-sw" style="background:#111827"></span></span> |
| `"neon"` | <span class="sp-bg" style="background:#0a0a0a"></span> `#0a0a0a` | — | Cyberpunk / retrowave | <span class="sp-pal"><span class="sp-sw" style="background:#00FF87"></span><span class="sp-sw" style="background:#FF006E"></span><span class="sp-sw" style="background:#00B4D8"></span><span class="sp-sw" style="background:#FFBE0B"></span><span class="sp-sw" style="background:#E500A4"></span></span> |

---

## Palettes complètes

### `"dark"`

```python
[0x818CF8, 0xFB7185, 0x34D399, 0xFBBF24, 0xA78BFA,
 0x22D3EE, 0xF472B6, 0xA3E635, 0xF87171, 0x2DD4BF]
```

### `"light"`

```python
[0x636EFA, 0xEF553B, 0x00CC96, 0xAB63FA, 0xFFA15A,
 0x19D3F3, 0xFF6692, 0xB6E880, 0xFF97FF, 0xFECB52]
```

### `"scientific"`

```python
[0x1F77B4, 0xFF7F0E, 0x2CA02C, 0xD62728, 0x9467BD,
 0x8C564B, 0xE377C2, 0x7F7F7F, 0xBCBD22, 0x17BECF]
```

### `"apple"`

```python
[0x0A84FF, 0x30D158, 0xFF453A, 0xFFD60A, 0xBF5AF2,
 0x64D2FF, 0xFF9F0A, 0xFF375F, 0xAC8E68, 0x63E6E2]
```

### `"notion"`

```python
[0x529CCA, 0xD08B65, 0x6C9B7D, 0xCB7C7A, 0x9A6DD7,
 0x868686, 0xCCAA55, 0x75B5AA, 0xD477A8, 0x507AA6]
```

### `"minimal"`

```python
[0x374151, 0x6B7280, 0x9CA3AF, 0xD1D5DB, 0x111827,
 0x4B5563, 0x1F2937, 0xE5E7EB, 0x030712, 0x6B7280]
```

### `"neon"`

```python
[0x00FF87, 0xFF006E, 0x00B4D8, 0xFFBE0B, 0xE500A4,
 0x8338EC, 0x3A86FF, 0xFB5607, 0xFF006E, 0x06D6A0]
```

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
