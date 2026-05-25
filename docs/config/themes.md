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
chart = sp.bar("Revenue", labels, values)

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
| `"light"` | transparent | — | Clean minimal | <span class="sp-pal"><span class="sp-sw" style="background:#6366F1"></span><span class="sp-sw" style="background:#F43F5E"></span><span class="sp-sw" style="background:#10B981"></span><span class="sp-sw" style="background:#F59E0B"></span><span class="sp-sw" style="background:#8B5CF6"></span></span> |
| `"scientific"` | <span class="sp-bg" style="background:#fafafa;border:1px solid #ccc"></span> `#fafafa` | ✓ | Matplotlib / D3 academic | <span class="sp-pal"><span class="sp-sw" style="background:#1F77B4"></span><span class="sp-sw" style="background:#FF7F0E"></span><span class="sp-sw" style="background:#2CA02C"></span><span class="sp-sw" style="background:#D62728"></span><span class="sp-sw" style="background:#9467BD"></span></span> |
| `"apple"` | <span class="sp-bg" style="background:#000000"></span> `#000000` | — | Apple dark mode (iOS/macOS) | <span class="sp-pal"><span class="sp-sw" style="background:#0A84FF"></span><span class="sp-sw" style="background:#30D158"></span><span class="sp-sw" style="background:#FF453A"></span><span class="sp-sw" style="background:#FFD60A"></span><span class="sp-sw" style="background:#BF5AF2"></span></span> |
| `"notion"` | <span class="sp-bg" style="background:#191919"></span> `#191919` | — | Notion editorial dark | <span class="sp-pal"><span class="sp-sw" style="background:#E3E3E3"></span><span class="sp-sw" style="background:#A0A0A0"></span><span class="sp-sw" style="background:#CB9D6D"></span><span class="sp-sw" style="background:#7C9E7E"></span><span class="sp-sw" style="background:#7B8FC4"></span></span> |
| `"minimal"` | transparent | — | Monochrome grayscale | <span class="sp-pal"><span class="sp-sw" style="background:#374151"></span><span class="sp-sw" style="background:#6B7280"></span><span class="sp-sw" style="background:#9CA3AF"></span><span class="sp-sw" style="background:#D1D5DB"></span><span class="sp-sw" style="background:#111827"></span></span> |
| `"neon"` | <span class="sp-bg" style="background:#0a0a0a"></span> `#0a0a0a` | — | Cyberpunk / retrowave | <span class="sp-pal"><span class="sp-sw" style="background:#00FFF0"></span><span class="sp-sw" style="background:#FF00FF"></span><span class="sp-sw" style="background:#00FF41"></span><span class="sp-sw" style="background:#FF6B00"></span><span class="sp-sw" style="background:#FFFF00"></span></span> |

---

## Full palette per theme

### `"dark"`

```python
[0x818CF8, 0xFB7185, 0x34D399, 0xFBBF24, 0xA78BFA,
 0x22D3EE, 0xF472B6, 0xA3E635, 0xF87171, 0x2DD4BF]
```

### `"light"`

```python
[0x6366F1, 0xF43F5E, 0x10B981, 0xF59E0B, 0x8B5CF6,
 0x06B6D4, 0xEC4899, 0x84CC16, 0xEF4444, 0x14B8A6]
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
[0xE3E3E3, 0xA0A0A0, 0xCB9D6D, 0x7C9E7E, 0x7B8FC4,
 0xC17B7B, 0xD4A76A, 0x8BA4B0, 0xB39DDB, 0x80CBC4]
```

### `"minimal"`

```python
[0x374151, 0x6B7280, 0x9CA3AF, 0xD1D5DB, 0x111827,
 0x4B5563, 0x1F2937, 0xE5E7EB, 0x030712, 0x6B7280]
```

### `"neon"`

```python
[0x00FFF0, 0xFF00FF, 0x00FF41, 0xFF6B00, 0xFFFF00,
 0xFF1493, 0x00BFFF, 0xFF4500, 0x7FFF00, 0xDA70D6]
```

---

## Examples

```python
import seraplot as sp

sp.theme("dark")
sp.bar("Revenue", ["Q1", "Q2", "Q3", "Q4"], [120, 145, 98, 180]).show()

sp.theme("neon")
sp.scatter("Clusters", x, y).show()

sp.theme("scientific")
sp.line("Population Growth", years, values).show()

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
graphique = sp.bar("Revenus", labels, values)

sp.reset_theme()
```

---

## Les 7 thèmes disponibles

| Thème | Fond | Quadrillage | Inspiration | Palette principale |
|---|---|:---:|---|---|
| `"dark"` | <span class="sp-bg" style="background:#0f172a"></span> `#0f172a` | ✓ | Espace profond / indigo Tailwind | <span class="sp-pal"><span class="sp-sw" style="background:#818CF8"></span><span class="sp-sw" style="background:#FB7185"></span><span class="sp-sw" style="background:#34D399"></span><span class="sp-sw" style="background:#FBBF24"></span><span class="sp-sw" style="background:#A78BFA"></span></span> |
| `"light"` | transparent | — | Épuré minimal | <span class="sp-pal"><span class="sp-sw" style="background:#6366F1"></span><span class="sp-sw" style="background:#F43F5E"></span><span class="sp-sw" style="background:#10B981"></span><span class="sp-sw" style="background:#F59E0B"></span><span class="sp-sw" style="background:#8B5CF6"></span></span> |
| `"scientific"` | <span class="sp-bg" style="background:#fafafa;border:1px solid #ccc"></span> `#fafafa` | ✓ | Matplotlib / D3 académique | <span class="sp-pal"><span class="sp-sw" style="background:#1F77B4"></span><span class="sp-sw" style="background:#FF7F0E"></span><span class="sp-sw" style="background:#2CA02C"></span><span class="sp-sw" style="background:#D62728"></span><span class="sp-sw" style="background:#9467BD"></span></span> |
| `"apple"` | <span class="sp-bg" style="background:#000000"></span> `#000000` | — | Mode sombre Apple (iOS/macOS) | <span class="sp-pal"><span class="sp-sw" style="background:#0A84FF"></span><span class="sp-sw" style="background:#30D158"></span><span class="sp-sw" style="background:#FF453A"></span><span class="sp-sw" style="background:#FFD60A"></span><span class="sp-sw" style="background:#BF5AF2"></span></span> |
| `"notion"` | <span class="sp-bg" style="background:#191919"></span> `#191919` | — | Notion éditorial sombre | <span class="sp-pal"><span class="sp-sw" style="background:#E3E3E3"></span><span class="sp-sw" style="background:#A0A0A0"></span><span class="sp-sw" style="background:#CB9D6D"></span><span class="sp-sw" style="background:#7C9E7E"></span><span class="sp-sw" style="background:#7B8FC4"></span></span> |
| `"minimal"` | transparent | — | Nuances de gris monochrome | <span class="sp-pal"><span class="sp-sw" style="background:#374151"></span><span class="sp-sw" style="background:#6B7280"></span><span class="sp-sw" style="background:#9CA3AF"></span><span class="sp-sw" style="background:#D1D5DB"></span><span class="sp-sw" style="background:#111827"></span></span> |
| `"neon"` | <span class="sp-bg" style="background:#0a0a0a"></span> `#0a0a0a` | — | Cyberpunk / retrowave | <span class="sp-pal"><span class="sp-sw" style="background:#00FFF0"></span><span class="sp-sw" style="background:#FF00FF"></span><span class="sp-sw" style="background:#00FF41"></span><span class="sp-sw" style="background:#FF6B00"></span><span class="sp-sw" style="background:#FFFF00"></span></span> |

---

## Palettes complètes

### `"dark"`

```python
[0x818CF8, 0xFB7185, 0x34D399, 0xFBBF24, 0xA78BFA,
 0x22D3EE, 0xF472B6, 0xA3E635, 0xF87171, 0x2DD4BF]
```

### `"light"`

```python
[0x6366F1, 0xF43F5E, 0x10B981, 0xF59E0B, 0x8B5CF6,
 0x06B6D4, 0xEC4899, 0x84CC16, 0xEF4444, 0x14B8A6]
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
[0xE3E3E3, 0xA0A0A0, 0xCB9D6D, 0x7C9E7E, 0x7B8FC4,
 0xC17B7B, 0xD4A76A, 0x8BA4B0, 0xB39DDB, 0x80CBC4]
```

### `"minimal"`

```python
[0x374151, 0x6B7280, 0x9CA3AF, 0xD1D5DB, 0x111827,
 0x4B5563, 0x1F2937, 0xE5E7EB, 0x030712, 0x6B7280]
```

### `"neon"`

```python
[0x00FFF0, 0xFF00FF, 0x00FF41, 0xFF6B00, 0xFFFF00,
 0xFF1493, 0x00BFFF, 0xFF4500, 0x7FFF00, 0xDA70D6]
```

---

## Exemples

```python
import seraplot as sp

sp.theme("dark")
sp.bar("Revenus", ["T1", "T2", "T3", "T4"], [120, 145, 98, 180]).show()

sp.theme("neon")
sp.scatter("Clusters", x, y).show()

sp.theme("scientific")
sp.line("Croissance démographique", annees, valeurs).show()

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
