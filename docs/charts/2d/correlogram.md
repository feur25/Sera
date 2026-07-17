# Correlogram

<div class="lang-en">

<style>
.sp-cls{display:flex;gap:0;margin:1.6em 0 1.6em 36px;border-radius:14px;background:linear-gradient(180deg,#0a0f1c 0%,#060912 100%);box-shadow:0 18px 50px -12px rgba(0,0,0,.6),0 0 0 1px #1e293b inset;position:relative;overflow:visible}.sp-cls-rail{display:flex;flex-direction:column;background:linear-gradient(180deg,#0d1426,#070b18);border-right:1px solid #1e293b;padding:18px 0;min-width:18px;transition:min-width .28s;position:relative;z-index:2;border-radius:14px 0 0 14px;overflow:visible}.sp-cls.sp-open .sp-cls-rail{min-width:170px;padding:18px 8px}.sp-cls-toggle{position:absolute;top:-14px;left:8px;padding:5px 9px;background:#1e293b;color:#a5b4fc;border:1px solid #312e81;border-radius:6px;cursor:pointer;font-size:12px;font-weight:700;line-height:1;z-index:5}.sp-cls-tab{position:relative;display:flex;align-items:center;gap:8px;margin:5px 0 5px -34px;padding:11px 16px 11px 14px;background:linear-gradient(90deg,#1a2540,#0f172a);color:#94a3b8;font-size:12px;font-weight:600;cursor:pointer;border:none;text-align:left;white-space:nowrap;border-radius:8px 0 0 8px;transition:all .25s;clip-path:polygon(0 0,calc(100% - 10px) 0,100% 50%,calc(100% - 10px) 100%,0 100%);min-height:18px}.sp-cls-tab.sp-cact{background:linear-gradient(90deg,#3730a3,#0f172a);color:#f5f3ff;margin-left:-46px;box-shadow:-3px 0 0 0 #818cf8 inset;font-weight:700;z-index:3}.sp-cls-tab .sp-cic{font-size:13px;flex-shrink:0;color:#a5b4fc;width:16px;text-align:center}.sp-cls-tab .sp-clb{display:none}.sp-cls.sp-open .sp-cls-tab .sp-clb{display:inline}.sp-cls-body{flex:1;padding:24px 26px 22px;background:#0a0f1c;min-width:0;border-radius:0 14px 14px 0;overflow:hidden}.sp-variant{display:none}.sp-variant.sp-von{display:block}.sp-vmeta{display:flex;flex-wrap:wrap;gap:8px 18px;align-items:center;font-size:13px;color:#94a3b8;margin:6px 0 16px;padding:10px 14px;background:rgba(99,102,241,.06);border-left:3px solid #6366f1;border-radius:0 6px 6px 0}.sp-vmeta strong{color:#a5b4fc;font-weight:700;margin-right:4px;text-transform:uppercase;font-size:11px}.sp-vmeta code{background:#1e293b;padding:2px 7px;border-radius:4px;color:#e2e8f0;font-size:12px}.sp-preview-frame{width:100%;height:400px;border:none;border-radius:10px;display:block;background:#0d1117;margin-top:10px}.sp-preview-label{font-size:11px;letter-spacing:.14em;font-weight:700;color:#818cf8;margin:20px 0 8px;text-transform:uppercase}
</style>
<script>
function spCls(scope,name,btn){var root=document.getElementById(scope);root.querySelectorAll('.sp-variant').forEach(function(s){s.classList.remove('sp-von')});root.querySelectorAll('.sp-cls-tab').forEach(function(b){b.classList.remove('sp-cact')});document.getElementById(scope+'-'+name).classList.add('sp-von');btn.classList.add('sp-cact')}
function spClsTog(id){document.getElementById(id).classList.toggle('sp-open')}
</script>

## Signature

`sp.correlogram(title, labels, matrix, *, variant="circle", **kwargs) -> Chart`

Aliases: `sp.correlogram`, `sp.corrplot`, `sp.correlation_matrix`, `sp.corr`, `sp.correlation_map`

## Description

A correlogram visualizes a correlation matrix as a grid. Each cell encodes the Pearson correlation coefficient (–1 to +1) using color (red = positive, blue = negative) and either circle area, square fill, or text. `matrix` is a nested N×N list — one inner list per row.

## Variants

| Variant | Description |
|---------|-------------|
| `"circle"` | Circles whose radius encodes |r| |
| `"heatmap"` | Filled squares (standard heatmap) |
| `"text"` | Numeric correlation values only |
| `"mixed"` | Circles + text overlay |
| `"gradient"` | Same as circle with diverging color |
| `"sorted"` | Reorders rows and columns by each variable's total absolute correlation with every other variable, so strongly related variables visually cluster instead of sitting in input order. |

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `labels` | `list[str]` | Variable names (length N) |
| `matrix` | `list[list[float]]` | N×N correlation matrix, one row per inner list |
| `variant` | `str` | Rendering style |
| `show_values` | `bool` | Overlay correlation values on cells |
| `width` / `height` | `int` | Chart dimensions |

## Returns

`Chart` — object with `.html` property and `.show()` method.

## Example

```python
import seraplot as sp
chart = sp.correlogram(
    "Variable Correlations",
    labels=["Sales", "Price", "Rating", "Stock"],
    matrix=[
        [ 1.0,  0.8, -0.3,  0.5],
        [ 0.8,  1.0,  0.1, -0.2],
        [-0.3,  0.1,  1.0,  0.7],
        [ 0.5, -0.2,  0.7,  1.0],
    ],
)
chart.show()
```

<div class="sp-cls sp-open" id="corr-en">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('corr-en')" title="Collapse / expand">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('corr-en','circle',this)"><span class="sp-cic">●</span><span class="sp-clb">Circle</span></button>
<button class="sp-cls-tab" onclick="spCls('corr-en','heatmap',this)"><span class="sp-cic">▦</span><span class="sp-clb">Heatmap</span></button>
<button class="sp-cls-tab" onclick="spCls('corr-en','text',this)"><span class="sp-cic">𝑟</span><span class="sp-clb">Text</span></button>
<button class="sp-cls-tab" onclick="spCls('corr-en','mixed',this)"><span class="sp-cic">◑</span><span class="sp-clb">Mixed</span></button>
<button class="sp-cls-tab" onclick="spCls('corr-en','gradient',this)"><span class="sp-cic">◐</span><span class="sp-clb">Gradient</span></button>
<button class="sp-cls-tab" onclick="spCls('corr-en','sorted',this)"><span class="sp-cic">⇆</span><span class="sp-clb">Sorted</span></button>
</div>
<div class="sp-cls-body">
<div class="sp-variant sp-von" id="corr-en-circle">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"circle"</code></span><span><strong>Aliases</strong> <code>circle / default / classic</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/correlogram-circle.html"></iframe>
</div>
<div class="sp-variant" id="corr-en-heatmap">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"heatmap"</code></span><span><strong>Aliases</strong> <code>heatmap / heat / square</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/correlogram-heatmap.html"></iframe>
</div>
<div class="sp-variant" id="corr-en-text">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"text"</code></span><span><strong>Aliases</strong> <code>text / number / value</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/correlogram-text.html"></iframe>
</div>
<div class="sp-variant" id="corr-en-mixed">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"mixed"</code></span><span><strong>Aliases</strong> <code>mixed / combo / both</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/correlogram-mixed.html"></iframe>
</div>
<div class="sp-variant" id="corr-en-gradient">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"gradient"</code></span><span><strong>Aliases</strong> <code>gradient / color / diverging</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/correlogram-gradient.html"></iframe>
</div>
<div class="sp-variant" id="corr-en-sorted">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"sorted"</code></span><span><strong>Aliases</strong> <code>sorted / clustered / reordered</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/correlogram-sorted.html"></iframe>
</div>
</div>
</div>

</div>

<div class="lang-fr">

## Signature

`sp.correlogram(title, labels, matrix, *, variant="circle", **kwargs) -> Chart`

Alias : `sp.correlogram`, `sp.corrplot`, `sp.correlation_matrix`, `sp.corr`, `sp.correlation_map`

## Description

Un correlogramme visualise une matrice de corrélation sous forme de grille. Chaque cellule encode le coefficient de corrélation de Pearson (–1 à +1) à l'aide de la couleur (rouge = positif, bleu = négatif) et soit l'aire d'un cercle, soit le remplissage d'un carré, soit du texte. `matrix` est une liste imbriquée N×N — une liste interne par ligne.

## Variantes

| Variante | Description |
|---------|-------------|
| `"circle"` | Cercles dont le rayon encode |r| |
| `"heatmap"` | Carrés pleins (heatmap standard) |
| `"text"` | Valeurs de corrélation numériques seules |
| `"mixed"` | Cercles + superposition de texte |
| `"gradient"` | Identique à circle avec couleur divergente |
| `"sorted"` | Réordonne lignes et colonnes selon la corrélation absolue totale de chaque variable avec toutes les autres, pour que les variables fortement liées se regroupent visuellement au lieu de rester dans l'ordre d'entrée. |

## Paramètres

| Paramètre | Type | Description |
|-----------|------|-------------|
| `labels` | `list[str]` | Noms des variables (longueur N) |
| `matrix` | `list[list[float]]` | Matrice de corrélation N×N, une ligne par liste interne |
| `variant` | `str` | Style de rendu |
| `show_values` | `bool` | Superposer les valeurs de corrélation sur les cellules |
| `width` / `height` | `int` | Dimensions du graphique |

## Retour

`Chart` — objet avec une propriété `.html` et une méthode `.show()`.

## Exemple

```python
import seraplot as sp
chart = sp.correlogram(
    "Variable Correlations",
    labels=["Sales", "Price", "Rating", "Stock"],
    matrix=[
        [ 1.0,  0.8, -0.3,  0.5],
        [ 0.8,  1.0,  0.1, -0.2],
        [-0.3,  0.1,  1.0,  0.7],
        [ 0.5, -0.2,  0.7,  1.0],
    ],
)
chart.show()
```

<div class="sp-cls sp-open" id="corr-fr">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('corr-fr')" title="Réduire / déplier">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('corr-fr','circle',this)"><span class="sp-cic">●</span><span class="sp-clb">Cercle</span></button>
<button class="sp-cls-tab" onclick="spCls('corr-fr','heatmap',this)"><span class="sp-cic">▦</span><span class="sp-clb">Heatmap</span></button>
<button class="sp-cls-tab" onclick="spCls('corr-fr','text',this)"><span class="sp-cic">𝑟</span><span class="sp-clb">Texte</span></button>
<button class="sp-cls-tab" onclick="spCls('corr-fr','mixed',this)"><span class="sp-cic">◑</span><span class="sp-clb">Mixte</span></button>
<button class="sp-cls-tab" onclick="spCls('corr-fr','gradient',this)"><span class="sp-cic">◐</span><span class="sp-clb">Dégradé</span></button>
<button class="sp-cls-tab" onclick="spCls('corr-fr','sorted',this)"><span class="sp-cic">⇆</span><span class="sp-clb">Sorted</span></button>
</div>
<div class="sp-cls-body">
<div class="sp-variant sp-von" id="corr-fr-circle">
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"circle"</code></span><span><strong>Alias</strong> <code>circle / default / classic</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/correlogram-circle.html"></iframe>
</div>
<div class="sp-variant" id="corr-fr-heatmap">
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"heatmap"</code></span><span><strong>Alias</strong> <code>heatmap / heat / square</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/correlogram-heatmap.html"></iframe>
</div>
<div class="sp-variant" id="corr-fr-text">
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"text"</code></span><span><strong>Alias</strong> <code>text / number / value</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/correlogram-text.html"></iframe>
</div>
<div class="sp-variant" id="corr-fr-mixed">
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"mixed"</code></span><span><strong>Alias</strong> <code>mixed / combo / both</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/correlogram-mixed.html"></iframe>
</div>
<div class="sp-variant" id="corr-fr-gradient">
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"gradient"</code></span><span><strong>Alias</strong> <code>gradient / color / diverging</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/correlogram-gradient.html"></iframe>
</div>
<div class="sp-variant" id="corr-fr-sorted">
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"sorted"</code></span><span><strong>Alias</strong> <code>sorted / clustered / reordered</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/correlogram-sorted.html"></iframe>
</div>
</div>
</div>

</div>
