# Arc Diagram

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
.sp-preview-frame{width:100%;height:380px;border:none;border-radius:10px;display:block;background:#0d1117;margin-top:10px}
</style>
<script>
function spCls(scope,name,btn){var root=document.getElementById(scope);root.querySelectorAll('.sp-variant').forEach(function(s){s.classList.remove('sp-von')});root.querySelectorAll('.sp-cls-tab').forEach(function(b){b.classList.remove('sp-cact')});document.getElementById(scope+'-'+name).classList.add('sp-von');btn.classList.add('sp-cact')}
function spClsTog(id){document.getElementById(id).classList.toggle('sp-open')}
</script>

## Signature

`sp.arc_diagram(title, labels, edges_i, edges_j, edges_w, *, variant="basic", **kwargs) -> Chart`

Aliases: `sp.arc_diagram`, `sp.arc_chart`, `sp.arc_plot`, `sp.arc_graph`, `sp.linear_network`

## Description

Arc diagrams place nodes on a horizontal axis and draw quadratic bezier arcs above (and optionally below) the axis to represent connections. They are particularly effective for showing sequential or ordered relationships.

## Variants

<div data-sp-registry-table="variants" data-family="arc_diagram"></div>

Unknown variant strings fall back to the registered default. Variant keys may be prefixed with `en_`, `fr_`, `en-` or `fr-`.

## Data

`labels` (`list[str]`) — Node names. `edges_i` (`list[int]`) — Source node indices. `edges_j` (`list[int]`) — Target node indices. `edges_w` (`list[float]`) — Edge weights. `width` / `height` (`int`) — Chart dimensions.

## Parameters

<div data-sp-registry-table="options" data-family="arc_diagram"></div>

## Themes

<div data-sp-registry-table="themes" data-family="arc_diagram"></div>

## Returns

`Chart` — object with `.html` property and `.show()` method.

<div class="sp-panel-source">
<h2>Parameters</h2>

<div data-sp-registry-table="variants" data-family="arc_diagram"></div>
</div>

<div class="sp-cls sp-open" id="arc-en">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('arc-en')" title="Collapse / expand">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('arc-en','basic',this)"><span class="sp-cic">⌒</span><span class="sp-clb">Basic</span></button>
<button class="sp-cls-tab" onclick="spCls('arc-en','bilateral',this)"><span class="sp-cic">⌓</span><span class="sp-clb">Bilateral</span></button>
<button class="sp-cls-tab" onclick="spCls('arc-en','weighted',this)"><span class="sp-cic">≈</span><span class="sp-clb">Weighted</span></button>
<button class="sp-cls-tab" onclick="spCls('arc-en','minimal',this)"><span class="sp-cic">—</span><span class="sp-clb">Minimal</span></button>
<button class="sp-cls-tab" onclick="spCls('arc-en','directed',this)"><span class="sp-cic">➤</span><span class="sp-clb">Directed</span></button>
</div>
<div class="sp-cls-body">
<div class="sp-variant sp-von" id="arc-en-basic">
<p>Arcs above the axis</p>
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"basic"</code></span><span><strong>Aliases</strong> <code>basic / default / classic</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/arc_diagram-basic.html"></iframe>
</div>
<div class="sp-variant" id="arc-en-bilateral">
<p>Alternating arcs above and below</p>
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"bilateral"</code></span><span><strong>Aliases</strong> <code>bilateral / both / dual</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/arc_diagram-bilateral.html"></iframe>
</div>
<div class="sp-variant" id="arc-en-weighted">
<p>Stroke width proportional to edge weight</p>
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"weighted"</code></span><span><strong>Aliases</strong> <code>weighted / width / value</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/arc_diagram-weighted.html"></iframe>
</div>
<div class="sp-variant" id="arc-en-minimal">
<p>Thin, low-opacity arcs with small strokeless nodes — strips away the visual weight so overlapping arcs stay legible in dense diagrams.</p>
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"minimal"</code></span><span><strong>Aliases</strong> <code>minimal / thin / clean</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/arc_diagram-minimal.html"></iframe>
</div>
<div class="sp-variant" id="arc-en-directed">
<p>Draws a small arrowhead where each arc lands on its target node — turns the diagram into a proper directed graph, e.g. for dependency or citation edges where "which way" matters as much as "how much".</p>
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"directed"</code></span><span><strong>Aliases</strong> <code>directed / arrows / flow / dependency</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/arc_diagram-directed.html"></iframe>
</div>
</div>
</div>

</div>

<div class="lang-fr" style="display:none">

## Signature

`sp.arc_diagram(title, labels, edges_i, edges_j, edges_w, *, variant="basic", **kwargs) -> Chart`

Alias : `sp.arc_diagram`, `sp.arc_chart`, `sp.arc_plot`, `sp.arc_graph`, `sp.linear_network`

## Description

Les diagrammes en arcs placent les nœuds sur un axe horizontal et tracent des arcs de bézier quadratiques au-dessus (et optionnellement en-dessous) de l'axe pour représenter les connexions. Ils sont particulièrement efficaces pour montrer des relations séquentielles ou ordonnées.

## Variantes

<div data-sp-registry-table="variants" data-family="arc_diagram"></div>

Une variante inconnue retombe sur la valeur par défaut enregistrée. Les clés de variantes peuvent être préfixées par `en_`, `fr_`, `en-` ou `fr-`.

## Données

`labels` (`list[str]`) — Noms des nœuds. `edges_i` (`list[int]`) — Indices des nœuds source. `edges_j` (`list[int]`) — Indices des nœuds cible. `edges_w` (`list[float]`) — Poids des arêtes. `width` / `height` (`int`) — Dimensions du graphique.

## Paramètres

<div data-sp-registry-table="options" data-family="arc_diagram"></div>

## Thèmes

<div data-sp-registry-table="themes" data-family="arc_diagram"></div>

## Retour

`Chart` — objet avec une propriété `.html` et une méthode `.show()`.

<div class="sp-panel-source">
<h2>Paramètres</h2>

<div data-sp-registry-table="variants" data-family="arc_diagram"></div>
</div>

<div class="sp-cls sp-open" id="arc-fr">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('arc-fr')" title="Réduire / déplier">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('arc-fr','basic',this)"><span class="sp-cic">⌒</span><span class="sp-clb">Basique</span></button>
<button class="sp-cls-tab" onclick="spCls('arc-fr','bilateral',this)"><span class="sp-cic">⌓</span><span class="sp-clb">Bilatéral</span></button>
<button class="sp-cls-tab" onclick="spCls('arc-fr','weighted',this)"><span class="sp-cic">≈</span><span class="sp-clb">Pondéré</span></button>
<button class="sp-cls-tab" onclick="spCls('arc-fr','minimal',this)"><span class="sp-cic">—</span><span class="sp-clb">Minimal</span></button>
<button class="sp-cls-tab" onclick="spCls('arc-fr','directed',this)"><span class="sp-cic">➤</span><span class="sp-clb">Dirigé</span></button>
</div>
<div class="sp-cls-body">
<div class="sp-variant sp-von" id="arc-fr-basic">
<p>Arcs au-dessus de l'axe</p>
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"basic"</code></span><span><strong>Alias</strong> <code>basic / default / classic</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/arc_diagram-basic.html"></iframe>
</div>
<div class="sp-variant" id="arc-fr-bilateral">
<p>Arcs alternés au-dessus et en-dessous</p>
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"bilateral"</code></span><span><strong>Alias</strong> <code>bilateral / both / dual</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/arc_diagram-bilateral.html"></iframe>
</div>
<div class="sp-variant" id="arc-fr-weighted">
<p>Épaisseur du trait proportionnelle au poids de l'arête</p>
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"weighted"</code></span><span><strong>Alias</strong> <code>weighted / width / value</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/arc_diagram-weighted.html"></iframe>
</div>
<div class="sp-variant" id="arc-fr-minimal">
<p>Arcs fins et peu opaques, avec de petits nœuds sans contour — allège le rendu visuel pour que les arcs qui se chevauchent restent lisibles dans les diagrammes denses.</p>
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"minimal"</code></span><span><strong>Alias</strong> <code>minimal / thin / clean</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/arc_diagram-minimal.html"></iframe>
</div>
<div class="sp-variant" id="arc-fr-directed">
<p>Trace une petite flèche là où chaque arc arrive sur son nœud cible — transforme le diagramme en véritable graphe orienté, par ex. pour des arêtes de dépendance ou de citation où le sens compte autant que la quantité.</p>
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"directed"</code></span><span><strong>Alias</strong> <code>directed / arrows / flow / dependency</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/arc_diagram-directed.html"></iframe>
</div>
</div>
</div>

</div>
