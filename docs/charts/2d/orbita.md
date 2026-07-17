# Orbita Chart

<div class="lang-en">

<style>
.sp-cls{display:flex;gap:0;margin:1.6em 0 1.6em 36px;border-radius:14px;background:linear-gradient(180deg,#0a0f1c 0%,#060912 100%);box-shadow:0 18px 50px -12px rgba(0,0,0,.6),0 0 0 1px #1e293b inset;position:relative;overflow:visible}.sp-cls-rail{display:flex;flex-direction:column;background:linear-gradient(180deg,#0d1426,#070b18);border-right:1px solid #1e293b;padding:18px 0;min-width:18px;transition:min-width .28s;position:relative;z-index:2;border-radius:14px 0 0 14px;overflow:visible}.sp-cls.sp-open .sp-cls-rail{min-width:170px;padding:18px 8px}.sp-cls-toggle{position:absolute;top:-14px;left:8px;padding:5px 9px;background:#1e293b;color:#a5b4fc;border:1px solid #312e81;border-radius:6px;cursor:pointer;font-size:12px;font-weight:700;line-height:1;z-index:5}.sp-cls-tab{position:relative;display:flex;align-items:center;gap:8px;margin:5px 0 5px -34px;padding:11px 16px 11px 14px;background:linear-gradient(90deg,#1a2540,#0f172a);color:#94a3b8;font-size:12px;font-weight:600;cursor:pointer;border:none;text-align:left;white-space:nowrap;border-radius:8px 0 0 8px;transition:all .25s;clip-path:polygon(0 0,calc(100% - 10px) 0,100% 50%,calc(100% - 10px) 100%,0 100%);min-height:18px}.sp-cls-tab.sp-cact{background:linear-gradient(90deg,#3730a3,#0f172a);color:#f5f3ff;margin-left:-46px;box-shadow:-3px 0 0 0 #818cf8 inset;font-weight:700;z-index:3}.sp-cls-tab .sp-cic{font-size:13px;flex-shrink:0;color:#a5b4fc;width:16px;text-align:center}.sp-cls-tab .sp-clb{display:none}.sp-cls.sp-open .sp-cls-tab .sp-clb{display:inline}.sp-cls-body{flex:1;padding:24px 26px 22px;background:#0a0f1c;min-width:0;border-radius:0 14px 14px 0;overflow:hidden}.sp-variant{display:none}.sp-variant.sp-von{display:block}.sp-vmeta{display:flex;flex-wrap:wrap;gap:8px 18px;align-items:center;font-size:13px;color:#94a3b8;margin:6px 0 16px;padding:10px 14px;background:rgba(99,102,241,.06);border-left:3px solid #6366f1;border-radius:0 6px 6px 0}.sp-vmeta strong{color:#a5b4fc;font-weight:700;margin-right:4px;text-transform:uppercase;font-size:11px}.sp-vmeta code{background:#1e293b;padding:2px 7px;border-radius:4px;color:#e2e8f0;font-size:12px}.sp-preview-frame{width:100%;height:420px;border:none;border-radius:10px;display:block;background:#0d1117;margin-top:10px}.sp-preview-label{font-size:11px;letter-spacing:.14em;font-weight:700;color:#818cf8;margin:20px 0 8px;text-transform:uppercase}
</style>
<script>
function spCls(scope,name,btn){var root=document.getElementById(scope);root.querySelectorAll('.sp-variant').forEach(function(s){s.classList.remove('sp-von')});root.querySelectorAll('.sp-cls-tab').forEach(function(b){b.classList.remove('sp-cact')});document.getElementById(scope+'-'+name).classList.add('sp-von');btn.classList.add('sp-cact')}
function spClsTog(id){document.getElementById(id).classList.toggle('sp-open')}
</script>

## Signature

`sp.orbita(title, series_names, labels, matrix, *, variant="classic", **kwargs) -> Chart`

Aliases: `sp.orbita`, `sp.orbit`, `sp.orbit_chart`, `sp.orbital`, `sp.multi_orbit`, `sp.concentric`

## Description

The Orbita chart is an original SeraPlot chart type that places multiple series on concentric ring orbits. Each series (orbit) maps categories to angular positions. The result is a planetary-system-style comparison across series and categories simultaneously, ideal for multi-period cross-category analysis.

`matrix` is a nested S×C list — one inner list per series (S = number of series, C = number of categories).

## Variants

| Variant | Description |
|---------|-------------|
| `"classic"` | Fixed-size dots on orbits |
| `"bubble"` | Dot radius proportional to value |
| `"trail"` | Closed polygon trail connecting series dots |
| `"glow"` | Gaussian blur glow effect on dots |
| `"minimal"` | Clean dots, no orbit decorations |

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `series_names` | `list[str]` | One name per orbit (e.g. years) |
| `labels` | `list[str]` | Category names (angular positions) |
| `matrix` | `list[list[float]]` | S×C value matrix, one row per series |
| `variant` | `str` | Rendering style |
| `palette` | `list[int]` | Custom colors (one per series) |
| `inner_r` | `float` | Radius of innermost orbit |
| `orbit_gap` | `float` | Spacing between orbits |
| `show_labels` | `bool` | Show category labels |
| `width` / `height` | `int` | Chart dimensions (default 580×580) |

## Returns

`Chart` — object with `.html` property and `.show()` method.

## Example

```python
import seraplot as sp
chart = sp.orbita(
    "Quarterly Performance by Year",
    series_names=["2022", "2023", "2024"],
    labels=["Q1", "Q2", "Q3", "Q4"],
    matrix=[
        [0.4, 0.7, 0.5, 0.8],
        [0.6, 0.5, 0.9, 0.6],
        [0.8, 0.7, 0.6, 0.9],
    ],
)
chart.show()
```

<div class="sp-cls sp-open" id="orb-en">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('orb-en')" title="Collapse / expand">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('orb-en','classic',this)"><span class="sp-cic">◎</span><span class="sp-clb">Classic</span></button>
<button class="sp-cls-tab" onclick="spCls('orb-en','bubble',this)"><span class="sp-cic">●</span><span class="sp-clb">Bubble</span></button>
<button class="sp-cls-tab" onclick="spCls('orb-en','trail',this)"><span class="sp-cic">∿</span><span class="sp-clb">Trail</span></button>
<button class="sp-cls-tab" onclick="spCls('orb-en','glow',this)"><span class="sp-cic">✦</span><span class="sp-clb">Glow</span></button>
<button class="sp-cls-tab" onclick="spCls('orb-en','minimal',this)"><span class="sp-cic">·</span><span class="sp-clb">Minimal</span></button>
</div>
<div class="sp-cls-body">
<div class="sp-variant sp-von" id="orb-en-classic">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"classic"</code></span><span><strong>Aliases</strong> <code>classic / default / basic</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/orbita-classic.html"></iframe>
</div>
<div class="sp-variant" id="orb-en-bubble">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"bubble"</code></span><span><strong>Aliases</strong> <code>bubble / sized / area</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/orbita-bubble.html"></iframe>
</div>
<div class="sp-variant" id="orb-en-trail">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"trail"</code></span><span><strong>Aliases</strong> <code>trail / line / connected</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/orbita-trail.html"></iframe>
</div>
<div class="sp-variant" id="orb-en-glow">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"glow"</code></span><span><strong>Aliases</strong> <code>glow / neon / light</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/orbita-glow.html"></iframe>
</div>
<div class="sp-variant" id="orb-en-minimal">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"minimal"</code></span><span><strong>Aliases</strong> <code>minimal / thin / clean</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/orbita-minimal.html"></iframe>
</div>
</div>
</div>

</div>

<div class="lang-fr">

## Signature

`sp.orbita(title, series_names, labels, matrix, *, variant="classic", **kwargs) -> Chart`

Alias : `sp.orbita`, `sp.orbit`, `sp.orbit_chart`, `sp.orbital`, `sp.multi_orbit`, `sp.concentric`

## Description

L'Orbita chart est un type de graphique original de SeraPlot qui place plusieurs séries sur des orbites en anneaux concentriques. Chaque série (orbite) associe des catégories à des positions angulaires. Le résultat est une comparaison façon système planétaire à travers séries et catégories simultanément, idéale pour l'analyse croisée multi-période/multi-catégorie.

`matrix` est une liste imbriquée S×C — une liste interne par série (S = nombre de séries, C = nombre de catégories).

## Variantes

| Variante | Description |
|---------|-------------|
| `"classic"` | Points de taille fixe sur les orbites |
| `"bubble"` | Rayon des points proportionnel à la valeur |
| `"trail"` | Traînée en polygone fermé reliant les points d'une série |
| `"glow"` | Effet de lueur (flou gaussien) sur les points |
| `"minimal"` | Points épurés, sans décoration d'orbite |

## Paramètres

| Paramètre | Type | Description |
|-----------|------|-------------|
| `series_names` | `list[str]` | Un nom par orbite (ex. années) |
| `labels` | `list[str]` | Noms des catégories (positions angulaires) |
| `matrix` | `list[list[float]]` | Matrice de valeurs S×C, une ligne par série |
| `variant` | `str` | Style de rendu |
| `palette` | `list[int]` | Couleurs personnalisées (une par série) |
| `inner_r` | `float` | Rayon de l'orbite la plus intérieure |
| `orbit_gap` | `float` | Espacement entre orbites |
| `show_labels` | `bool` | Afficher les labels de catégorie |
| `width` / `height` | `int` | Dimensions du graphique (défaut 580×580) |

## Retour

`Chart` — objet avec une propriété `.html` et une méthode `.show()`.

## Exemple

```python
import seraplot as sp
chart = sp.orbita(
    "Quarterly Performance by Year",
    series_names=["2022", "2023", "2024"],
    labels=["Q1", "Q2", "Q3", "Q4"],
    matrix=[
        [0.4, 0.7, 0.5, 0.8],
        [0.6, 0.5, 0.9, 0.6],
        [0.8, 0.7, 0.6, 0.9],
    ],
)
chart.show()
```

<div class="sp-cls sp-open" id="orb-fr">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('orb-fr')" title="Réduire / déplier">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('orb-fr','classic',this)"><span class="sp-cic">◎</span><span class="sp-clb">Classique</span></button>
<button class="sp-cls-tab" onclick="spCls('orb-fr','bubble',this)"><span class="sp-cic">●</span><span class="sp-clb">Bulle</span></button>
<button class="sp-cls-tab" onclick="spCls('orb-fr','trail',this)"><span class="sp-cic">∿</span><span class="sp-clb">Traînée</span></button>
<button class="sp-cls-tab" onclick="spCls('orb-fr','glow',this)"><span class="sp-cic">✦</span><span class="sp-clb">Lueur</span></button>
<button class="sp-cls-tab" onclick="spCls('orb-fr','minimal',this)"><span class="sp-cic">·</span><span class="sp-clb">Minimal</span></button>
</div>
<div class="sp-cls-body">
<div class="sp-variant sp-von" id="orb-fr-classic">
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"classic"</code></span><span><strong>Alias</strong> <code>classic / default / basic</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/orbita-classic.html"></iframe>
</div>
<div class="sp-variant" id="orb-fr-bubble">
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"bubble"</code></span><span><strong>Alias</strong> <code>bubble / sized / area</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/orbita-bubble.html"></iframe>
</div>
<div class="sp-variant" id="orb-fr-trail">
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"trail"</code></span><span><strong>Alias</strong> <code>trail / line / connected</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/orbita-trail.html"></iframe>
</div>
<div class="sp-variant" id="orb-fr-glow">
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"glow"</code></span><span><strong>Alias</strong> <code>glow / neon / light</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/orbita-glow.html"></iframe>
</div>
<div class="sp-variant" id="orb-fr-minimal">
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"minimal"</code></span><span><strong>Alias</strong> <code>minimal / thin / clean</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/orbita-minimal.html"></iframe>
</div>
</div>
</div>

</div>
