# Treemap — Hierarchical Proportional Tiles

<div class="lang-en">

<style>
.sp-preview-frame{width:100%;height:480px;border:none;border-radius:10px;display:block;background:#0d1117;margin-top:10px;box-shadow:0 8px 24px -8px rgba(0,0,0,.5)}
</style>
<script>
function spTab(g,id,btn){var r=document.getElementById(g);r.querySelectorAll('.sp-tc').forEach(function(e){e.classList.remove('sp-on')});r.querySelectorAll('.sp-tb').forEach(function(b){b.classList.remove('sp-act')});document.getElementById(id).classList.add('sp-on');btn.classList.add('sp-act');if(window.hljs)document.getElementById(id).querySelectorAll('code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})}
function spCls(scope,name,btn){var root=document.getElementById(scope);root.querySelectorAll('.sp-variant').forEach(function(s){s.classList.remove('sp-von')});root.querySelectorAll('.sp-cls-tab').forEach(function(b){b.classList.remove('sp-cact')});document.getElementById(scope+'-'+name).classList.add('sp-von');btn.classList.add('sp-cact');if(window.hljs)document.getElementById(scope+'-'+name).querySelectorAll('code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})}
function spClsTog(id){document.getElementById(id).classList.toggle('sp-open')}
document.addEventListener('DOMContentLoaded',function(){if(window.hljs)document.querySelectorAll('.sp-tc.sp-on code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})});
</script>

## Signature

`sp.treemap(title, labels, values, *, parents=None, variant="basic", palette=None, **kwargs) -> Chart`

## Description

`sp.treemap()` is the unified entry point for the entire treemap-chart family. A treemap divides a rectangle into proportional sub-rectangles whose area encodes value; when a `parents` list is given the layout becomes hierarchical (each parent gets its own block, leaves are squarified within). The `variant` keyword switches the visual style without touching the data. Treemaps are the standard for visualizing budgets, market cap, disk usage, portfolio weights, file systems and any 'whole = sum of parts' breakdown.

> **Hierarchical mode** — pass `parents` (one parent label per leaf, can be empty string `""` for a flat treemap). Internal totals are auto-computed from leaves. Sort leaves with the `sort_order` parameter (`"desc"` recommended).

## Variants

<div data-sp-registry-table="variants" data-family="treemap"></div>

## Parameters

<div data-sp-registry-table="options" data-family="treemap"></div>

## Returns

`Chart` — object with `.html` property and `.show()` method.

---

<div class="sp-cls sp-open" id="treemap-en">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('treemap-en')" title="Collapse / expand">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('treemap-en','basic',this)"><span class="sp-cic">B</span><span class="sp-clb">Basic</span></button>
<button class="sp-cls-tab" onclick="spCls('treemap-en','flat',this)"><span class="sp-cic">F</span><span class="sp-clb">Flat</span></button>
<button class="sp-cls-tab" onclick="spCls('treemap-en','outlined',this)"><span class="sp-cic">O</span><span class="sp-clb">Outlined</span></button>
<button class="sp-cls-tab" onclick="spCls('treemap-en','gapped',this)"><span class="sp-cic">P</span><span class="sp-clb">Gapped</span></button>
<button class="sp-cls-tab" onclick="spCls('treemap-en','nested',this)"><span class="sp-cic">N</span><span class="sp-clb">Nested</span></button>
<button class="sp-cls-tab" onclick="spCls('treemap-en','heat',this)"><span class="sp-cic">H</span><span class="sp-clb">Heat</span></button>
<button class="sp-cls-tab" onclick="spCls('treemap-en','mono',this)"><span class="sp-cic">M</span><span class="sp-clb">Mono</span></button>
</div>
<div class="sp-cls-body">

<div class="sp-variant sp-von" id="treemap-en-basic">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"basic"</code></span><span><strong>Aliases</strong> <code>basic / default / classic / filled</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Classic squarified treemap with rounded corners and white separators between tiles.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/treemap-basic.html"></iframe>
</div>

<div class="sp-variant" id="treemap-en-flat">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"flat"</code></span><span><strong>Aliases</strong> <code>flat / mosaic / edge / tight</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Edge-to-edge mosaic with no stroke and no rounding for a dense, magazine-style block.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/treemap-flat.html"></iframe>
</div>

<div class="sp-variant" id="treemap-en-outlined">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"outlined"</code></span><span><strong>Aliases</strong> <code>outlined / outline / stroke / wireframe</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Wireframe tiles: translucent fill with bold colored stroke and dark labels for print-ready look.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/treemap-outlined.html"></iframe>
</div>

<div class="sp-variant" id="treemap-en-gapped">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"gapped"</code></span><span><strong>Aliases</strong> <code>gapped / spaced / inset / separated</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Each tile inset with extra padding so the structure breathes; rounded corners and color fill.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/treemap-gapped.html"></iframe>
</div>

<div class="sp-variant" id="treemap-en-nested">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"nested"</code></span><span><strong>Aliases</strong> <code>nested / grouped / parents / hierarchy</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Draws parent group rectangles with header labels around their children, emphasising hierarchy.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/treemap-nested.html"></iframe>
</div>

<div class="sp-variant" id="treemap-en-heat">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"heat"</code></span><span><strong>Aliases</strong> <code>heat / heatmap / temperature / cold_warm</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Color encodes value (cool blue -> hot red) instead of identity, turning the treemap into a heatmap.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/treemap-heat.html"></iframe>
</div>

<div class="sp-variant" id="treemap-en-mono">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"mono"</code></span><span><strong>Aliases</strong> <code>mono / monochrome / single / uniform</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Single hue with opacity decreasing by rank; editorial, minimalist, perfect for slides.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/treemap-mono.html"></iframe>
</div>

</div>
</div>

</div>

<div class="lang-fr">

<h2>Signature</h2>

`sp.treemap(title, labels, values, *, parents=None, variant="basic", palette=None, **kwargs) -> Chart`

<h2>Description</h2>

`sp.treemap()` est le point d entree unifie pour toute la famille treemap. Un treemap decoupe un rectangle en sous-rectangles proportionnels dont l aire code la valeur ; lorsqu une liste `parents` est fournie le rendu devient hierarchique (chaque parent recoit son propre bloc, les feuilles y sont squarifiees). Le mot-cle `variant` change le style sans toucher aux donnees. Les treemaps sont la reference pour visualiser budgets, capitalisations boursieres, occupation disque, poids de portefeuille, systemes de fichiers et toute decomposition 'tout = somme des parties'.

> **Mode hierarchique** — passez `parents` (un libelle parent par feuille, chaine vide `""` pour un treemap plat). Les totaux internes sont auto-calcules. Triez les feuilles avec `sort_order` (`"desc"` recommande).

<h2>Variantes</h2>

<div data-sp-registry-table="variants" data-family="treemap"></div>

<h2>Paramètres</h2>

<div data-sp-registry-table="options" data-family="treemap"></div>

<h2>Retour</h2>

`Chart` — objet avec une propriete `.html` et une methode `.show()`.

---

<div class="sp-cls sp-open" id="treemap-fr">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('treemap-fr')" title="Collapse / expand">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('treemap-fr','basic',this)"><span class="sp-cic">B</span><span class="sp-clb">Basic</span></button>
<button class="sp-cls-tab" onclick="spCls('treemap-fr','flat',this)"><span class="sp-cic">F</span><span class="sp-clb">Flat</span></button>
<button class="sp-cls-tab" onclick="spCls('treemap-fr','outlined',this)"><span class="sp-cic">O</span><span class="sp-clb">Outlined</span></button>
<button class="sp-cls-tab" onclick="spCls('treemap-fr','gapped',this)"><span class="sp-cic">P</span><span class="sp-clb">Gapped</span></button>
<button class="sp-cls-tab" onclick="spCls('treemap-fr','nested',this)"><span class="sp-cic">N</span><span class="sp-clb">Nested</span></button>
<button class="sp-cls-tab" onclick="spCls('treemap-fr','heat',this)"><span class="sp-cic">H</span><span class="sp-clb">Heat</span></button>
<button class="sp-cls-tab" onclick="spCls('treemap-fr','mono',this)"><span class="sp-cic">M</span><span class="sp-clb">Mono</span></button>
</div>
<div class="sp-cls-body">

<div class="sp-variant sp-von" id="treemap-fr-basic">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"basic"</code></span><span><strong>Aliases</strong> <code>basic / default / classic / filled</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Treemap squarifie classique avec coins arrondis et separateurs blancs entre les tuiles.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/treemap-basic.html"></iframe>
</div>

<div class="sp-variant" id="treemap-fr-flat">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"flat"</code></span><span><strong>Aliases</strong> <code>flat / mosaic / edge / tight</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Mosaique bord-a-bord sans contour ni arrondi pour un bloc dense type magazine.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/treemap-flat.html"></iframe>
</div>

<div class="sp-variant" id="treemap-fr-outlined">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"outlined"</code></span><span><strong>Aliases</strong> <code>outlined / outline / stroke / wireframe</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Tuiles en fil de fer : fond translucide, contour colore epais et libelles sombres, style imprimable.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/treemap-outlined.html"></iframe>
</div>

<div class="sp-variant" id="treemap-fr-gapped">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"gapped"</code></span><span><strong>Aliases</strong> <code>gapped / spaced / inset / separated</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Chaque tuile en retrait avec marges supplementaires pour aerer la structure ; coins arrondis.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/treemap-gapped.html"></iframe>
</div>

<div class="sp-variant" id="treemap-fr-nested">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"nested"</code></span><span><strong>Aliases</strong> <code>nested / grouped / parents / hierarchy</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Dessine les rectangles parents avec libelle d en-tete autour de leurs enfants, met en avant la hierarchie.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/treemap-nested.html"></iframe>
</div>

<div class="sp-variant" id="treemap-fr-heat">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"heat"</code></span><span><strong>Aliases</strong> <code>heat / heatmap / temperature / cold_warm</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">La couleur code la valeur (bleu froid -> rouge chaud) au lieu de l identite, treemap en heatmap.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/treemap-heat.html"></iframe>
</div>

<div class="sp-variant" id="treemap-fr-mono">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"mono"</code></span><span><strong>Aliases</strong> <code>mono / monochrome / single / uniform</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Teinte unique avec opacite decroissante par rang ; minimaliste et editorial, ideal pour slides.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/treemap-mono.html"></iframe>
</div>

</div>
</div>

</div>
