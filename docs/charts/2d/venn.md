# Venn Diagram

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

`sp.venn(title, labels, values, *, variant="basic", **kwargs) -> Chart`

Aliases: `sp.venn`, `sp.venn_diagram`, `sp.euler`, `sp.set_diagram`, `sp.overlap`

## Description

Venn diagrams show set relationships using overlapping circles. Supply one value per set for circle sizes. The `"euler"` variant scales circle radii proportionally to the first N values.

## Variants

<div data-sp-registry-table="variants" data-family="venn"></div>

## Data

`labels` (`list[str]`) — Set names. `values` (`list[float]`) — Set sizes (first N entries used for Euler radii). `width` / `height` (`int`) — Chart dimensions.

## Parameters

<div data-sp-registry-table="options" data-family="venn"></div>

## Themes

<div data-sp-registry-table="themes" data-family="venn"></div>

## Returns

`Chart` — object with `.html` property and `.show()` method.

<div class="sp-panel-source">
<h2>Parameters</h2>

<div data-sp-registry-table="variants" data-family="venn"></div>
</div>

<div class="sp-cls sp-open" id="venn-en">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('venn-en')" title="Collapse / expand">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('venn-en','basic',this)"><span class="sp-cic">⊙</span><span class="sp-clb">Basic</span></button>
<button class="sp-cls-tab" onclick="spCls('venn-en','euler',this)"><span class="sp-cic">⊗</span><span class="sp-clb">Euler</span></button>
<button class="sp-cls-tab" onclick="spCls('venn-en','filled',this)"><span class="sp-cic">●</span><span class="sp-clb">Filled</span></button>
<button class="sp-cls-tab" onclick="spCls('venn-en','minimal',this)"><span class="sp-cic">◌</span><span class="sp-clb">Minimal</span></button>
<button class="sp-cls-tab" onclick="spCls('venn-en','exclusive',this)"><span class="sp-cic">◐</span><span class="sp-clb">Exclusive</span></button>
</div>
<div class="sp-cls-body">
<div class="sp-variant sp-von" id="venn-en-basic">
<p>Semi-transparent overlapping circles</p>
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"basic"</code></span><span><strong>Aliases</strong> <code>basic / default / classic</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/venn-basic.html"></iframe>
</div>
<div class="sp-variant" id="venn-en-euler">
<p>Proportional circle areas (Euler diagram)</p>
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"euler"</code></span><span><strong>Aliases</strong> <code>euler / proportional / area</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/venn-euler.html"></iframe>
</div>
<div class="sp-variant" id="venn-en-filled">
<p>Fully opaque circles</p>
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"filled"</code></span><span><strong>Aliases</strong> <code>filled / solid / opaque</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/venn-filled.html"></iframe>
</div>
<div class="sp-variant" id="venn-en-minimal">
<p>Forces every circle into its stroke-only outline form regardless of the configured fill opacity, for a clean contour-only read of the set overlaps.</p>
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"minimal"</code></span><span><strong>Aliases</strong> <code>minimal / outline / thin</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/venn-minimal.html"></iframe>
</div>
<div class="sp-variant" id="venn-en-exclusive">
<p>Masks each circle down to the region that belongs to it alone and renders that region at full color, while shared/overlapping areas fade into the background — makes it obvious what's unique to each set versus what's shared.</p>
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"exclusive"</code></span><span><strong>Aliases</strong> <code>exclusive / unique / distinct</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/venn-exclusive.html"></iframe>
</div>
</div>
</div>

</div>

<div class="lang-fr" style="display:none">

## Signature

`sp.venn(title, labels, values, *, variant="basic", **kwargs) -> Chart`

Alias : `sp.venn`, `sp.venn_diagram`, `sp.euler`, `sp.set_diagram`, `sp.overlap`

## Description

Les diagrammes de Venn montrent des relations entre ensembles à l'aide de cercles qui se chevauchent. Fournissez une valeur par ensemble pour la taille des cercles. La variante `"euler"` met à l'échelle les rayons des cercles proportionnellement aux N premières valeurs.

## Variantes

<div data-sp-registry-table="variants" data-family="venn"></div>

## Données

`labels` (`list[str]`) — Noms des ensembles. `values` (`list[float]`) — Tailles des ensembles (les N premières entrées pilotent les rayons Euler). `width` / `height` (`int`) — Dimensions du graphique.

## Paramètres

<div data-sp-registry-table="options" data-family="venn"></div>

## Thèmes

<div data-sp-registry-table="themes" data-family="venn"></div>

## Retour

`Chart` — objet avec une propriété `.html` et une méthode `.show()`.

<div class="sp-panel-source">
<h2>Paramètres</h2>

<div data-sp-registry-table="variants" data-family="venn"></div>
</div>

<div class="sp-cls sp-open" id="venn-fr">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('venn-fr')" title="Réduire / déplier">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('venn-fr','basic',this)"><span class="sp-cic">⊙</span><span class="sp-clb">Basique</span></button>
<button class="sp-cls-tab" onclick="spCls('venn-fr','euler',this)"><span class="sp-cic">⊗</span><span class="sp-clb">Euler</span></button>
<button class="sp-cls-tab" onclick="spCls('venn-fr','filled',this)"><span class="sp-cic">●</span><span class="sp-clb">Plein</span></button>
<button class="sp-cls-tab" onclick="spCls('venn-fr','minimal',this)"><span class="sp-cic">◌</span><span class="sp-clb">Minimal</span></button>
<button class="sp-cls-tab" onclick="spCls('venn-fr','exclusive',this)"><span class="sp-cic">◐</span><span class="sp-clb">Exclusif</span></button>
</div>
<div class="sp-cls-body">
<div class="sp-variant sp-von" id="venn-fr-basic">
<p>Cercles semi-transparents qui se chevauchent</p>
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"basic"</code></span><span><strong>Alias</strong> <code>basic / default / classic</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/venn-basic.html"></iframe>
</div>
<div class="sp-variant" id="venn-fr-euler">
<p>Aires de cercles proportionnelles (diagramme d'Euler)</p>
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"euler"</code></span><span><strong>Alias</strong> <code>euler / proportional / area</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/venn-euler.html"></iframe>
</div>
<div class="sp-variant" id="venn-fr-filled">
<p>Cercles entièrement opaques</p>
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"filled"</code></span><span><strong>Alias</strong> <code>filled / solid / opaque</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/venn-filled.html"></iframe>
</div>
<div class="sp-variant" id="venn-fr-minimal">
<p>Force chaque cercle dans sa forme en contour seul, quelle que soit l’opacité de remplissage configurée, pour une lecture épurée des recouvrements d’ensembles en simples contours.</p>
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"minimal"</code></span><span><strong>Alias</strong> <code>minimal / outline / thin</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/venn-minimal.html"></iframe>
</div>
<div class="sp-variant" id="venn-fr-exclusive">
<p>Masque chaque cercle jusqu'à la région qui lui appartient en propre et l'affiche en pleine couleur, tandis que les zones partagées/superposées s'estompent — rend évident ce qui est unique à chaque ensemble par rapport à ce qui est partagé.</p>
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"exclusive"</code></span><span><strong>Alias</strong> <code>exclusive / unique / distinct</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/venn-exclusive.html"></iframe>
</div>
</div>
</div>

</div>
