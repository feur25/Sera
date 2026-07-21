# Dendrogram

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

`sp.dendrogram(title, labels, parents, *, variant="vertical", **kwargs) -> Chart`

Aliases: `sp.dendrogram`, `sp.dendro`, `sp.tree`, `sp.tree_diagram`, `sp.hierarchy`, `sp.hierarchical`

## Description

Dendrograms display hierarchical tree structures using right-angle elbow connectors (vertical/horizontal) or smooth bezier curves (elegant) or a radial circular layout. Parent–child relationships are defined by the `parents` list.

## Variants

<div data-sp-registry-table="variants" data-family="dendrogram"></div>

Unknown variant strings fall back to the registered default. Variant keys may be prefixed with `en_`, `fr_`, `en-` or `fr-`.

## Data

`labels` (`list[str]`) — Node names. `parents` (`list[str]`) — Parent name per node (`""` = root). `width` / `height` (`int`) — Chart dimensions.

## Parameters

<div data-sp-registry-table="options" data-family="dendrogram"></div>

## Themes

<div data-sp-registry-table="themes" data-family="dendrogram"></div>

## Returns

`Chart` — object with `.html` property and `.show()` method.

<div class="sp-panel-source">
<h2>Parameters</h2>

<div data-sp-registry-table="variants" data-family="dendrogram"></div>
</div>

<div class="sp-cls sp-open" id="dend-en">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('dend-en')" title="Collapse / expand">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('dend-en','vertical',this)"><span class="sp-cic">↓</span><span class="sp-clb">Vertical</span></button>
<button class="sp-cls-tab" onclick="spCls('dend-en','horizontal',this)"><span class="sp-cic">→</span><span class="sp-clb">Horizontal</span></button>
<button class="sp-cls-tab" onclick="spCls('dend-en','radial',this)"><span class="sp-cic">◎</span><span class="sp-clb">Radial</span></button>
<button class="sp-cls-tab" onclick="spCls('dend-en','compact',this)"><span class="sp-cic">≡</span><span class="sp-clb">Compact</span></button>
<button class="sp-cls-tab" onclick="spCls('dend-en','elegant',this)"><span class="sp-cic">∫</span><span class="sp-clb">Elegant</span></button>
<button class="sp-cls-tab" onclick="spCls('dend-en','triangular',this)"><span class="sp-cic">△</span><span class="sp-clb">Triangular</span></button>
</div>
<div class="sp-cls-body">
<div class="sp-variant sp-von" id="dend-en-vertical">
<p>Root at top, leaves at bottom (elbow connectors)</p>
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"vertical"</code></span><span><strong>Aliases</strong> <code>vertical / top / default / classic</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/dendrogram-vertical.html"></iframe>
</div>
<div class="sp-variant" id="dend-en-horizontal">
<p>Root at left, leaves at right</p>
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"horizontal"</code></span><span><strong>Aliases</strong> <code>horizontal / left / h</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/dendrogram-horizontal.html"></iframe>
</div>
<div class="sp-variant" id="dend-en-radial">
<p>Circular radial tree layout</p>
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"radial"</code></span><span><strong>Aliases</strong> <code>radial / circular / polar</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/dendrogram-radial.html"></iframe>
</div>
<div class="sp-variant" id="dend-en-compact">
<p>Tighter spacing, smaller font</p>
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"compact"</code></span><span><strong>Aliases</strong> <code>compact / dense / tight</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/dendrogram-compact.html"></iframe>
</div>
<div class="sp-variant" id="dend-en-elegant">
<p>Smooth cubic bezier curves</p>
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"elegant"</code></span><span><strong>Aliases</strong> <code>elegant / smooth / rounded</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/dendrogram-elegant.html"></iframe>
</div>
<div class="sp-variant" id="dend-en-triangular">
<p>Connects parent to child with a single straight diagonal line — a third connector style alongside the right-angle elbows (vertical/horizontal) and smooth beziers (elegant).</p>
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"triangular"</code></span><span><strong>Aliases</strong> <code>triangular / diagonal / straight / angular</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/dendrogram-triangular.html"></iframe>
</div>
</div>
</div>

</div>

<div class="lang-fr" style="display:none">

## Signature

`sp.dendrogram(title, labels, parents, *, variant="vertical", **kwargs) -> Chart`

Alias : `sp.dendrogram`, `sp.dendro`, `sp.tree`, `sp.tree_diagram`, `sp.hierarchy`, `sp.hierarchical`

## Description

Les dendrogrammes affichent des structures arborescentes hiérarchiques à l'aide de connecteurs en coude à angle droit (vertical/horizontal), de courbes de bézier lisses (elegant), ou d'une disposition radiale circulaire. Les relations parent-enfant sont définies par la liste `parents`.

## Variantes

<div data-sp-registry-table="variants" data-family="dendrogram"></div>

Une variante inconnue retombe sur la valeur par défaut enregistrée. Les clés de variantes peuvent être préfixées par `en_`, `fr_`, `en-` ou `fr-`.

## Données

`labels` (`list[str]`) — Noms des nœuds. `parents` (`list[str]`) — Nom du parent de chaque nœud (`""` = racine). `width` / `height` (`int`) — Dimensions du graphique.

## Paramètres

<div data-sp-registry-table="options" data-family="dendrogram"></div>

## Thèmes

<div data-sp-registry-table="themes" data-family="dendrogram"></div>

## Retour

`Chart` — objet avec une propriété `.html` et une méthode `.show()`.

<div class="sp-panel-source">
<h2>Paramètres</h2>

<div data-sp-registry-table="variants" data-family="dendrogram"></div>
</div>

<div class="sp-cls sp-open" id="dend-fr">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('dend-fr')" title="Réduire / déplier">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('dend-fr','vertical',this)"><span class="sp-cic">↓</span><span class="sp-clb">Vertical</span></button>
<button class="sp-cls-tab" onclick="spCls('dend-fr','horizontal',this)"><span class="sp-cic">→</span><span class="sp-clb">Horizontal</span></button>
<button class="sp-cls-tab" onclick="spCls('dend-fr','radial',this)"><span class="sp-cic">◎</span><span class="sp-clb">Radial</span></button>
<button class="sp-cls-tab" onclick="spCls('dend-fr','compact',this)"><span class="sp-cic">≡</span><span class="sp-clb">Compact</span></button>
<button class="sp-cls-tab" onclick="spCls('dend-fr','elegant',this)"><span class="sp-cic">∫</span><span class="sp-clb">Élégant</span></button>
<button class="sp-cls-tab" onclick="spCls('dend-fr','triangular',this)"><span class="sp-cic">△</span><span class="sp-clb">Triangular</span></button>
</div>
<div class="sp-cls-body">
<div class="sp-variant sp-von" id="dend-fr-vertical">
<p>Racine en haut, feuilles en bas (connecteurs en coude)</p>
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"vertical"</code></span><span><strong>Alias</strong> <code>vertical / top / default / classic</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/dendrogram-vertical.html"></iframe>
</div>
<div class="sp-variant" id="dend-fr-horizontal">
<p>Racine à gauche, feuilles à droite</p>
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"horizontal"</code></span><span><strong>Alias</strong> <code>horizontal / left / h</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/dendrogram-horizontal.html"></iframe>
</div>
<div class="sp-variant" id="dend-fr-radial">
<p>Disposition arborescente radiale circulaire</p>
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"radial"</code></span><span><strong>Alias</strong> <code>radial / circular / polar</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/dendrogram-radial.html"></iframe>
</div>
<div class="sp-variant" id="dend-fr-compact">
<p>Espacement resserré, police plus petite</p>
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"compact"</code></span><span><strong>Alias</strong> <code>compact / dense / tight</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/dendrogram-compact.html"></iframe>
</div>
<div class="sp-variant" id="dend-fr-elegant">
<p>Courbes de bézier cubiques lisses</p>
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"elegant"</code></span><span><strong>Alias</strong> <code>elegant / smooth / rounded</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/dendrogram-elegant.html"></iframe>
</div>
<div class="sp-variant" id="dend-fr-triangular">
<p>Relie parent et enfant par une seule ligne diagonale droite — un troisième style de connecteur, aux côtés des coudes à angle droit (vertical/horizontal) et des courbes de bézier lisses (elegant).</p>
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"triangular"</code></span><span><strong>Alias</strong> <code>triangular / diagonal / straight / angular</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/dendrogram-triangular.html"></iframe>
</div>
</div>
</div>

</div>
