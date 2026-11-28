# Correlogram

<div class="lang-en">

<style>
.sp-cls-rail{display:flex;flex-direction:column;background:linear-gradient(180deg,#0d1426,#070b18);border-right:1px solid #1e293b;padding:18px 0;min-width:18px;transition:min-width .28s;position:relative;z-index:2;border-radius:14px 0 0 14px;overflow:visible}
.sp-cls-toggle{position:absolute;top:-14px;left:8px;padding:5px 9px;background:#1e293b;color:#a5b4fc;border:1px solid #312e81;border-radius:6px;cursor:pointer;font-size:12px;font-weight:700;line-height:1;z-index:5}
.sp-cls-tab{position:relative;display:flex;align-items:center;gap:8px;margin:5px 0 5px -34px;padding:11px 16px 11px 14px;background:linear-gradient(90deg,#1a2540,#0f172a);color:#94a3b8;font-size:12px;font-weight:600;cursor:pointer;border:none;text-align:left;white-space:nowrap;border-radius:8px 0 0 8px;transition:all .25s;clip-path:polygon(0 0,calc(100% - 10px) 0,100% 50%,calc(100% - 10px) 100%,0 100%);min-height:18px}
.sp-cls-tab.sp-cact{background:linear-gradient(90deg,#3730a3,#0f172a);color:#f5f3ff;margin-left:-46px;box-shadow:-3px 0 0 0 #818cf8 inset;font-weight:700;z-index:3}
.sp-cls-tab .sp-cic{font-size:13px;flex-shrink:0;color:#a5b4fc;width:16px;text-align:center}
.sp-cls-tab .sp-clb{display:none}
.sp-cls-body{flex:1;padding:24px 26px 22px;background:#0a0f1c;min-width:0;border-radius:0 14px 14px 0;overflow:hidden}
.sp-variant.sp-von{display:block}
.sp-vmeta{display:flex;flex-wrap:wrap;gap:8px 18px;align-items:center;font-size:13px;color:#94a3b8;margin:6px 0 16px;padding:10px 14px;background:rgba(99,102,241,.06);border-left:3px solid #6366f1;border-radius:0 6px 6px 0}
.sp-vmeta strong{color:#a5b4fc;font-weight:700;margin-right:4px;text-transform:uppercase;font-size:11px}
.sp-vmeta code{background:#1e293b;padding:2px 7px;border-radius:4px;color:#e2e8f0;font-size:12px}
.sp-preview-frame{width:100%;height:400px;border:none;border-radius:10px;display:block;background:#0d1117;margin-top:10px}
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

<div data-sp-registry-table="variants" data-family="correlogram"></div>

## Data

`labels` (`list[str]`) — Variable names (length N). `matrix` (`list[list[float]]`) — N×N correlation matrix, one row per inner list. `width` / `height` (`int`) — Chart dimensions.

Every variant is really just a preset of three lower-level params you can mix freely on the base circle variant instead of picking a named one: `cell_shape` (`"circle" | "square" | "ellipse" | "pie" | "number"`) controls how a single cell is drawn, `cell_shape2` sets a second shape for the lower triangle when `layout="mixed"`, and `layout` (`"full" | "upper" | "lower" | "mixed"`) controls which half of the matrix gets filled — e.g. `sp.correlogram(labels=..., matrix=..., cell_shape="ellipse", layout="upper")`.

## Parameters

<div data-sp-registry-table="options" data-family="correlogram"></div>

## Themes

<div data-sp-registry-table="themes" data-family="correlogram"></div>

## Returns

`Chart` — object with `.html` property and `.show()` method.

<div class="sp-panel-source">
<h2>Parameters</h2>

<div data-sp-registry-table="variants" data-family="correlogram"></div>
</div>

<div class="sp-cls sp-open" id="corr-en">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('corr-en')" title="Collapse / expand">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('corr-en','circle',this)"><span class="sp-cic">●</span><span class="sp-clb">Circle</span></button>
<button class="sp-cls-tab" onclick="spCls('corr-en','heatmap',this)"><span class="sp-cic">▦</span><span class="sp-clb">Heatmap</span></button>
<button class="sp-cls-tab" onclick="spCls('corr-en','text',this)"><span class="sp-cic">𝑟</span><span class="sp-clb">Text</span></button>
<button class="sp-cls-tab" onclick="spCls('corr-en','mixed',this)"><span class="sp-cic">◑</span><span class="sp-clb">Mixed</span></button>
<button class="sp-cls-tab" onclick="spCls('corr-en','ellipse',this)"><span class="sp-cic">⬭</span><span class="sp-clb">Ellipse</span></button>
<button class="sp-cls-tab" onclick="spCls('corr-en','pie_square',this)"><span class="sp-cic">◐</span><span class="sp-clb">Pie + Square</span></button>
<button class="sp-cls-tab" onclick="spCls('corr-en','circle_legend',this)"><span class="sp-cic">◔</span><span class="sp-clb">Circle + Legend</span></button>
</div>
<div class="sp-cls-body">
<div class="sp-variant sp-von" id="corr-en-circle">
<p></p>
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"circle"</code></span><span><strong>Aliases</strong> <code>circle / default / classic</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" data-src="../../previews/correlogram-circle.html"></iframe>
</div>
<div class="sp-variant" id="corr-en-heatmap">
<p>Filled squares (standard heatmap)</p>
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"heatmap"</code></span><span><strong>Aliases</strong> <code>heatmap / heat / square</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" data-src="../../previews/correlogram-heatmap.html"></iframe>
</div>
<div class="sp-variant" id="corr-en-text">
<p>Numeric correlation values only</p>
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"text"</code></span><span><strong>Aliases</strong> <code>text / number / value</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" data-src="../../previews/correlogram-text.html"></iframe>
</div>
<div class="sp-variant" id="corr-en-mixed">
<p>Circles + text overlay</p>
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"mixed"</code></span><span><strong>Aliases</strong> <code>mixed / combo / both</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" data-src="../../previews/correlogram-mixed.html"></iframe>
</div>
<div class="sp-variant" id="corr-en-ellipse">
<p>Each cell is an ellipse tilted "/" for positive correlation, "\" for negative, flattening toward a line as |r| approaches 1 and toward a circle as it approaches 0.</p>
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"ellipse"</code></span><span><strong>Aliases</strong> <code>ellipse / oval</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" data-src="../../previews/correlogram-ellipse.html"></iframe>
</div>
<div class="sp-variant" id="corr-en-pie_square">
<p>Upper triangle as pie wedges (wedge angle = |r|, color = sign), lower triangle as flat colored squares, diagonal left blank - the classic mixed correlogram layout, unsorted.</p>
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"pie_square"</code></span><span><strong>Aliases</strong> <code>pie_square / pie / mixed_pie</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" data-src="../../previews/correlogram-pie_square.html"></iframe>
</div>
<div class="sp-variant" id="corr-en-circle_legend">
<p>Upper-triangle-only circles (lower triangle and diagonal left blank) with a color/value legend bar alongside instead of numeric labels on the cells.</p>
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"circle_legend"</code></span><span><strong>Aliases</strong> <code>circle_legend / legend / scale</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" data-src="../../previews/correlogram-circle_legend.html"></iframe>
</div>
</div>
</div>

</div>

<div class="lang-fr" style="display:none">

## Signature

`sp.correlogram(title, labels, matrix, *, variant="circle", **kwargs) -> Chart`

Alias : `sp.correlogram`, `sp.corrplot`, `sp.correlation_matrix`, `sp.corr`, `sp.correlation_map`

## Description

Un correlogramme visualise une matrice de corrélation sous forme de grille. Chaque cellule encode le coefficient de corrélation de Pearson (–1 à +1) à l'aide de la couleur (rouge = positif, bleu = négatif) et soit l'aire d'un cercle, soit le remplissage d'un carré, soit du texte. `matrix` est une liste imbriquée N×N — une liste interne par ligne.

## Variantes

<div data-sp-registry-table="variants" data-family="correlogram"></div>

## Données

`labels` (`list[str]`) — Noms des variables (longueur N). `matrix` (`list[list[float]]`) — Matrice de corrélation N×N, une ligne par liste interne. `width` / `height` (`int`) — Dimensions du graphique.

Chaque variante n'est en fait qu'un préréglage de trois paramètres de plus bas niveau que tu peux combiner librement sur la variante circle de base plutôt que d'en choisir une nommée : `cell_shape` (`"circle" | "square" | "ellipse" | "pie" | "number"`) contrôle le dessin d'une cellule, `cell_shape2` définit une seconde forme pour le triangle inférieur quand `layout="mixed"`, et `layout` (`"full" | "upper" | "lower" | "mixed"`) contrôle quelle moitié de la matrice est remplie — ex. `sp.correlogram(labels=..., matrix=..., cell_shape="ellipse", layout="upper")`.

## Paramètres

<div data-sp-registry-table="options" data-family="correlogram"></div>

## Thèmes

<div data-sp-registry-table="themes" data-family="correlogram"></div>

## Retour

`Chart` — objet avec une propriété `.html` et une méthode `.show()`.

<div class="sp-panel-source">
<h2>Paramètres</h2>

<div data-sp-registry-table="variants" data-family="correlogram"></div>
</div>

<div class="sp-cls sp-open" id="corr-fr">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('corr-fr')" title="Réduire / déplier">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('corr-fr','circle',this)"><span class="sp-cic">●</span><span class="sp-clb">Cercle</span></button>
<button class="sp-cls-tab" onclick="spCls('corr-fr','heatmap',this)"><span class="sp-cic">▦</span><span class="sp-clb">Heatmap</span></button>
<button class="sp-cls-tab" onclick="spCls('corr-fr','text',this)"><span class="sp-cic">𝑟</span><span class="sp-clb">Texte</span></button>
<button class="sp-cls-tab" onclick="spCls('corr-fr','mixed',this)"><span class="sp-cic">◑</span><span class="sp-clb">Mixte</span></button>
<button class="sp-cls-tab" onclick="spCls('corr-fr','ellipse',this)"><span class="sp-cic">⬭</span><span class="sp-clb">Ellipse</span></button>
<button class="sp-cls-tab" onclick="spCls('corr-fr','pie_square',this)"><span class="sp-cic">◐</span><span class="sp-clb">Camembert + carré</span></button>
<button class="sp-cls-tab" onclick="spCls('corr-fr','circle_legend',this)"><span class="sp-cic">◔</span><span class="sp-clb">Cercle + légende</span></button>
</div>
<div class="sp-cls-body">
<div class="sp-variant sp-von" id="corr-fr-circle">
<p></p>
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"circle"</code></span><span><strong>Alias</strong> <code>circle / default / classic</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" data-src="../../previews/correlogram-circle.html"></iframe>
</div>
<div class="sp-variant" id="corr-fr-heatmap">
<p>Carrés pleins (heatmap standard)</p>
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"heatmap"</code></span><span><strong>Alias</strong> <code>heatmap / heat / square</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" data-src="../../previews/correlogram-heatmap.html"></iframe>
</div>
<div class="sp-variant" id="corr-fr-text">
<p>Valeurs de corrélation numériques seules</p>
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"text"</code></span><span><strong>Alias</strong> <code>text / number / value</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" data-src="../../previews/correlogram-text.html"></iframe>
</div>
<div class="sp-variant" id="corr-fr-mixed">
<p>Cercles + superposition de texte</p>
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"mixed"</code></span><span><strong>Alias</strong> <code>mixed / combo / both</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" data-src="../../previews/correlogram-mixed.html"></iframe>
</div>
<div class="sp-variant" id="corr-fr-ellipse">
<p>Chaque cellule est une ellipse inclinée "/" pour une corrélation positive, "\" pour une négative, s'aplatissant vers une ligne quand |r| approche 1 et vers un cercle quand |r| approche 0.</p>
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"ellipse"</code></span><span><strong>Alias</strong> <code>ellipse / oval</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" data-src="../../previews/correlogram-ellipse.html"></iframe>
</div>
<div class="sp-variant" id="corr-fr-pie_square">
<p>Triangle supérieur en camemberts (angle = |r|, couleur = signe), triangle inférieur en carrés pleins colorés, diagonale laissée vide - la disposition mixte classique, non triée.</p>
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"pie_square"</code></span><span><strong>Alias</strong> <code>pie_square / pie / mixed_pie</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" data-src="../../previews/correlogram-pie_square.html"></iframe>
</div>
<div class="sp-variant" id="corr-fr-circle_legend">
<p>Cercles uniquement dans le triangle supérieur (triangle inférieur et diagonale vides) avec une barre de légende couleur/valeur au lieu de libellés numériques sur les cellules.</p>
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"circle_legend"</code></span><span><strong>Alias</strong> <code>circle_legend / legend / scale</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" data-src="../../previews/correlogram-circle_legend.html"></iframe>
</div>
</div>
</div>

</div>
