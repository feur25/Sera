# Choropleth Map

<div class="lang-en">

## Signature

```python
sp.build_choropleth(
    title: str,
    labels: list[str],
    values: list[float],
    *,
    map: str = "world",
    iso_codes: list[str] | None = None,
    color_low: int = 0,
    color_high: int = 0,
    palette: list[int] | None = None,
    width: int = 1000,
    height: int = 600,
    background: str | None = None,
    hover_json: str | None = None,
    show_legend: bool = True,
    null_color: int = 0xdddddd,
) -> Chart
```

Aliases: `sp.choropleth`

---

## Description

Choropleth (filled map) — country or region polygons colored by a scalar value.

Countries without data receive the `null_color`. Provide `iso_codes` (ISO-3166 alpha-3) to match countries automatically.

`map` selects which registered region set to draw: `"world"` (every country, the default) or `"usa_states"` (all 50 US states + DC — aliases `"usa"` / `"us"` / `"us_states"`). `labels` then match that set's own codes (US state postal codes for `usa_states`) or full names. The region-set registry is open — see [Region Sets](#region-sets) below.

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `labels` | `list[str]` | required | Country|
| `values` | `list[float]` | required | Values to color by |
| `map` | `str` | `"world"` | Region set to draw — `"world"` or `"usa_states"` (aliases `usa`/`us`) |
| `iso_codes` | `list[str] \| None` | `None` | ISO-3166 alpha-3 codes |
| `color_low` | `int` | auto | Low value color |
| `color_high` | `int` | auto | High value color |
| `null_color` | `int` | `0xdddddd` | Color for countries with no data |
| `width` | `int` | `1000` | Canvas width |
| `height` | `int` | `600` | Canvas height |
| `show_legend` | `bool` | `True` | Show color scale legend |
| `hover_json` | `str \| None` | `None` | Custom hover JSON |

---

## Region Sets

<div data-sp-registry-table="regions"></div>

---

## Returns

`Chart`

---

## Examples

### Unemployment rate choropleth

<style>
.sp-tabs{border:1px solid var(--sp-border);border-radius:8px;overflow:hidden;margin:1.5em 0}
.sp-tab-btns{display:flex;background:var(--sp-surface);border-bottom:1px solid var(--sp-border)}
.sp-tb{padding:9px 22px;border:none;background:none;color:var(--sp-text-muted);cursor:pointer;font-size:13px;font-weight:600;border-bottom:2px solid transparent;transition:color .15s,border-color .15s;white-space:nowrap}
</style>
<script>
function spTab(g,id,btn){var r=document.getElementById(g);r.querySelectorAll('.sp-tc').forEach(function(e){e.classList.remove('sp-on')});r.querySelectorAll('.sp-tb').forEach(function(b){b.classList.remove('sp-act')});document.getElementById(id).classList.add('sp-on');btn.classList.add('sp-act');if(window.hljs)document.getElementById(id).querySelectorAll('code').forEach(function(c){hljs.highlightElement(c)})}
document.addEventListener('DOMContentLoaded',function(){if(window.hljs)document.querySelectorAll('.sp-tc code').forEach(function(c){hljs.highlightElement(c)})});
</script>
<div class="sp-tabs" id="choropleth">
<div class="sp-tab-btns"><button class="sp-tb sp-act" onclick="spTab('choropleth','choropleth-py',this)">Python</button><button class="sp-tb" onclick="spTab('choropleth','choropleth-js',this)">JavaScript</button><button class="sp-tb" onclick="spTab('choropleth','choropleth-ts',this)">TypeScript</button></div>
<div id="choropleth-py" class="sp-tc sp-on"><pre style="margin:0;border-radius:0"><code class="language-python">import seraplot as sp
chart = sp.build_choropleth(
    "Unemployment Rate by Country",
    labels=["FRA", "DEU", "ESP", "ITA", "PRT"],
    values=[7.1, 3.0, 11.8, 6.7, 6.2],
)</code></pre></div>
<div id="choropleth-js" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-javascript">const sp = require('seraplot');
const chart = sp.build_choropleth("Unemployment Rate by Country",
["FRA", "DEU", "ESP", "ITA", "PRT"],
{
    values: [7.1, 3.0, 11.8, 6.7, 6.2]
})</code></pre></div>
<div id="choropleth-ts" class="sp-tc"><pre style="margin:0;border-radius:0"><code class="language-typescript">import * as sp from 'seraplot';
const chart = sp.build_choropleth("Unemployment Rate by Country",
["FRA", "DEU", "ESP", "ITA", "PRT"],
{
    values: [7.1, 3.0, 11.8, 6.7, 6.2]
})</code></pre></div>
</div>

<details open>
<summary style="cursor:pointer;font-weight:600;padding:4px 0;color:#94a3b8">&#9654;&nbsp;Live Preview</summary>

<iframe src="../../previews/choropleth.html" style="width:100%;height:520px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

</details>

### US state-level choropleth

```python
import seraplot as sp

chart = sp.build_choropleth(
    "Population by State (millions)",
    labels=["CA", "TX", "NY", "FL", "WA", "CO", "IL", "OH", "GA", "AZ", "NV", "UT", "OR", "NC", "MA"],
    values=[38.9, 30.5, 19.6, 22.6, 7.8, 5.9, 12.6, 11.8, 11.0, 7.4, 3.2, 3.4, 4.2, 10.8, 7.0],
    map="usa_states",
)
```

<details>
<summary style="cursor:pointer;font-weight:600;padding:4px 0;color:#94a3b8">&#9654;&nbsp;Live Preview</summary>

<iframe src="../../previews/choropleth-usa-states.html" style="width:100%;height:520px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

</details>

---

</div>

<div class="lang-fr">

<h2>Signature</h2>

```python
sp.build_choropleth(
    title: str,
    labels: list[str],
    values: list[float],
    *,
    map: str = "world",
    iso_codes: list[str] | None = None,
    color_low: int = 0,
    color_high: int = 0,
    palette: list[int] | None = None,
    width: int = 1000,
    height: int = 600,
    background: str | None = None,
    hover_json: str | None = None,
    show_legend: bool = True,
    null_color: int = 0xdddddd,
) -> Chart
```

Aliases: `sp.choropleth`

---

<h2>Description</h2>

Carte choro-plèthe — polygones de pays/régions colorés par une valeur scalaire. Les pays sans données reçoivent la `null_color`. Fournissez des `iso_codes` (ISO-3166 alpha-3) pour associer les pays automatiquement.

`map` sélectionne l'ensemble de régions à dessiner : `"world"` (tous les pays, par défaut) ou `"usa_states"` (les 50 états américains + DC — alias `"usa"` / `"us"` / `"us_states"`). `labels` doit alors correspondre aux codes de cet ensemble (codes postaux américains pour `usa_states`) ou aux noms complets. Le registre des ensembles de régions est ouvert — voir [Ensembles de régions](#ensembles-de-regions-1) ci-dessous.

---

<h2>Paramètres</h2>

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `title` | `str` | requis | Titre du graphique |
| `labels` | `list[str]` | requis | Pays |
| `values` | `list[float]` | requis | Valeurs pour la colorisation |
| `map` | `str` | `"world"` | Ensemble de régions à dessiner — `"world"` ou `"usa_states"` (alias `usa`/`us`) |
| `iso_codes` | `list[str] \| None` | `None` | Codes ISO-3166 alpha-3 |
| `color_low` | `int` | auto | Couleur pour les valeurs basses |
| `color_high` | `int` | auto | Couleur pour les valeurs hautes |
| `null_color` | `int` | `0xdddddd` | Couleur des pays sans données |
| `width` | `int` | `1000` | Largeur du canvas |
| `height` | `int` | `600` | Hauteur du canvas |
| `show_legend` | `bool` | `True` | Afficher l'échelle de couleur |
| `hover_json` | `str \| None` | `None` | JSON d'infobulle personnalisée |

---

<h2 id="ensembles-de-regions-1">Ensembles de régions</h2>

<div data-sp-registry-table="regions"></div>

---

<h2>Retourne</h2>

`Chart`

---

<h2>Exemples</h2>

```python
import seraplot as sp

chart = sp.build_choropleth(
    "Taux de chômage par pays",
    labels=["FRA", "DEU", "ESP", "ITA", "PRT"],
    values=[7.1, 3.0, 11.8, 6.7, 6.2],
)
```

### Choroplèthe par état américain

```python
import seraplot as sp

chart = sp.build_choropleth(
    "Population par état (millions)",
    labels=["CA", "TX", "NY", "FL", "WA", "CO", "IL", "OH", "GA", "AZ", "NV", "UT", "OR", "NC", "MA"],
    values=[38.9, 30.5, 19.6, 22.6, 7.8, 5.9, 12.6, 11.8, 11.0, 7.4, 3.2, 3.4, 4.2, 10.8, 7.0],
    map="usa_states",
)
```

<details>
<summary style="cursor:pointer;font-weight:600;padding:4px 0;color:#94a3b8">&#9654;&nbsp;Aperçu</summary>

<iframe src="../../previews/choropleth-usa-states.html" style="width:100%;height:520px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

</details>

---

</div>
