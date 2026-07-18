# Sankey Diagram

<div class="lang-en">

<style>
.sp-panel-source{display:none!important}
.sp-tabs{border:1px solid #334155;border-radius:8px;overflow:hidden;margin:1.2em 0}.sp-tab-btns{display:flex;background:#0f172a;border-bottom:1px solid #334155;flex-wrap:wrap}.sp-tb{padding:8px 14px;border:none;background:none;color:#64748b;cursor:pointer;font-size:12px;font-weight:600;border-bottom:2px solid transparent;transition:color .15s,border-color .15s;white-space:nowrap}.sp-tb:hover{color:#e2e8f0}.sp-tb.sp-act{color:#6366f1;border-bottom-color:#6366f1}.sp-tc{display:none}.sp-tc.sp-on{display:block}.sp-cls{display:flex;gap:0;margin:1.6em 0 1.6em 36px;border-radius:14px;background:linear-gradient(180deg,#0a0f1c 0%,#060912 100%);box-shadow:0 18px 50px -12px rgba(0,0,0,.6),0 0 0 1px #1e293b inset;position:relative;overflow:visible}.sp-cls-rail{display:flex;flex-direction:column;background:linear-gradient(180deg,#0d1426,#070b18);border-right:1px solid #1e293b;padding:18px 0;min-width:18px;transition:min-width .28s cubic-bezier(.5,0,.2,1);position:relative;z-index:2;border-radius:14px 0 0 14px;overflow:visible}.sp-cls.sp-open .sp-cls-rail{min-width:170px;padding:18px 8px}.sp-cls-toggle{position:absolute;top:-14px;left:8px;padding:5px 9px;background:#1e293b;color:#a5b4fc;border:1px solid #312e81;border-radius:6px;cursor:pointer;font-size:12px;font-weight:700;transition:all .15s;line-height:1;z-index:5;box-shadow:0 4px 12px -2px rgba(0,0,0,.5)}.sp-cls-toggle:hover{background:#312e81;color:#e0e7ff;transform:translateY(-1px)}.sp-cls-tab{position:relative;display:flex;align-items:center;gap:8px;margin:5px 0 5px -34px;padding:11px 16px 11px 14px;background:linear-gradient(90deg,#1a2540 0%,#141d33 70%,#0f172a 100%);color:#94a3b8;font-size:12px;font-weight:600;cursor:pointer;border:none;text-align:left;white-space:nowrap;border-radius:8px 0 0 8px;box-shadow:-6px 4px 14px -4px rgba(0,0,0,.55),inset 0 1px 0 rgba(255,255,255,.04),inset 1px 0 0 rgba(255,255,255,.05);transition:all .25s cubic-bezier(.5,0,.2,1);clip-path:polygon(0 0,calc(100% - 10px) 0,100% 50%,calc(100% - 10px) 100%,0 100%);min-height:18px}.sp-cls-tab:hover{background:linear-gradient(90deg,#23304d,#1a2540 70%,#141d33);color:#e0e7ff;margin-left:-40px;box-shadow:-8px 6px 18px -4px rgba(0,0,0,.6),inset 0 1px 0 rgba(255,255,255,.06)}.sp-cls-tab.sp-cact{background:linear-gradient(90deg,#3730a3 0%,#1e1b4b 50%,#0f172a 100%);color:#f5f3ff;margin-left:-46px;box-shadow:-10px 8px 22px -4px rgba(99,102,241,.35),-3px 0 0 0 #818cf8 inset,inset 0 1px 0 rgba(165,180,252,.2);font-weight:700;z-index:3}.sp-cls-tab .sp-cic{font-size:13px;flex-shrink:0;color:#a5b4fc;font-weight:900;letter-spacing:-1px;width:16px;text-align:center;text-shadow:0 0 6px rgba(165,180,252,.4)}.sp-cls-tab.sp-cact .sp-cic{color:#e0e7ff;text-shadow:0 0 10px rgba(165,180,252,.7)}.sp-cls-tab .sp-clb{display:none;font-weight:inherit;letter-spacing:.01em}.sp-cls.sp-open .sp-cls-tab .sp-clb{display:inline}.sp-cls-body{flex:1;padding:24px 26px 22px;background:#0a0f1c;min-width:0;position:relative;z-index:1;border-radius:0 14px 14px 0;overflow:hidden}.sp-variant{display:none}.sp-variant.sp-von{display:block;animation:spFade .25s ease}@keyframes spFade{from{opacity:0;transform:translateX(8px)}to{opacity:1;transform:translateX(0)}}.sp-variant>p:first-of-type{margin:0;padding:14px 18px 8px;background:linear-gradient(180deg,rgba(99,102,241,.08),rgba(99,102,241,.03));border:1px solid rgba(99,102,241,.18);border-bottom:none;border-radius:10px 10px 0 0;color:#e2e8f0;font-size:14px;line-height:1.55;font-weight:500}.sp-variant>p:first-of-type+pre{margin:0 0 18px;padding:14px 18px 16px;background:linear-gradient(180deg,#0d1326,#080d1a);border:1px solid rgba(99,102,241,.18);border-top:none;border-radius:0 0 10px 10px;box-shadow:0 6px 18px -8px rgba(0,0,0,.6);overflow-x:auto}.sp-variant>p:first-of-type+pre code{background:none;padding:0;font-size:12.5px;line-height:1.55;color:#cbd5e1}.sp-vmeta{display:flex;flex-wrap:wrap;gap:8px 18px;align-items:center;font-size:13px;color:#94a3b8;margin:6px 0 16px;padding:10px 14px;background:rgba(99,102,241,.06);border-left:3px solid #6366f1;border-radius:0 6px 6px 0}.sp-vmeta strong{color:#a5b4fc;font-weight:700;margin-right:4px;letter-spacing:.04em;text-transform:uppercase;font-size:11px}.sp-vmeta code{background:#1e293b;padding:2px 7px;border-radius:4px;color:#e2e8f0;font-size:12px}.sp-preview-frame{width:100%;height:380px;border:none;border-radius:10px;display:block;background:#0d1117;margin-top:10px;box-shadow:0 8px 24px -8px rgba(0,0,0,.5)}.sp-preview-label{font-size:11px;letter-spacing:.14em;font-weight:700;color:#818cf8;margin:20px 0 8px;text-transform:uppercase}
</style>
<script>
function spCls(scope,name,btn){var root=document.getElementById(scope);root.querySelectorAll('.sp-variant').forEach(function(s){s.classList.remove('sp-von')});root.querySelectorAll('.sp-cls-tab').forEach(function(b){b.classList.remove('sp-cact')});document.getElementById(scope+'-'+name).classList.add('sp-von');btn.classList.add('sp-cact')}
function spClsTog(id){document.getElementById(id).classList.toggle('sp-open')}
</script>

## Signature

`sp.sankey(title, labels, edges_i, edges_j, edges_w, *, variant="basic", **kwargs) -> Chart`

Aliases: `sp.sankey`, `sp.sankeys`, `sp.sankey_chart`, `sp.sankey_diagram`, `sp.flow_chart`

## Description

Sankey diagrams visualize flows between nodes. Node widths and link widths are proportional to flow volumes. Edges are defined by source indices (`edges_i`), target indices (`edges_j`), and weights (`edges_w`). Nodes are laid out in columns by BFS depth.

## Variants

<div data-sp-registry-table="variants" data-family="sankey"></div>

Unknown variant strings fall back to the registered default. Variant keys may be prefixed with `en_`, `fr_`, `en-` or `fr-`.

## Data

`labels` (`list[str]`) — Node names. `edges_i` (`list[int]`) — Source node indices. `edges_j` (`list[int]`) — Target node indices. `edges_w` (`list[float]`) — Flow weights. `width` / `height` (`int`) — Chart dimensions.


## Parameters

<div data-sp-registry-table="options" data-family="sankey"></div>

## Themes

<div data-sp-registry-table="themes" data-family="sankey"></div>


## Returns

`Chart` — object with `.html` property and `.show()` method.


<div class="sp-panel-source">
<h2>Parameters</h2>

<div data-sp-registry-table="variants" data-family="sankey"></div>
</div>

<div class="sp-cls sp-open" id="sankey-en">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('sankey-en')" title="Collapse / expand">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('sankey-en','basic',this)"><span class="sp-cic">⇉</span><span class="sp-clb">Basic</span></button>
<button class="sp-cls-tab" onclick="spCls('sankey-en','gapped',this)"><span class="sp-cic">⇥</span><span class="sp-clb">Gapped</span></button>
<button class="sp-cls-tab" onclick="spCls('sankey-en','ribbon',this)"><span class="sp-cic">▬</span><span class="sp-clb">Ribbon</span></button>
<button class="sp-cls-tab" onclick="spCls('sankey-en','minimal',this)"><span class="sp-cic">—</span><span class="sp-clb">Minimal</span></button>
<button class="sp-cls-tab" onclick="spCls('sankey-en','sorted',this)"><span class="sp-cic">⇅</span><span class="sp-clb">Sorted</span></button>
</div>
<div class="sp-cls-body">
<div class="sp-variant sp-von" id="sankey-en-basic">
<p>Standard bezier ribbon links</p>
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"basic"</code></span><span><strong>Aliases</strong> <code>basic / default / classic</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/sankey-basic.html"></iframe>
</div>
<div class="sp-variant" id="sankey-en-gapped">
<p>Increased node spacing</p>
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"gapped"</code></span><span><strong>Aliases</strong> <code>gapped / spaced / separated</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/sankey-gapped.html"></iframe>
</div>
<div class="sp-variant" id="sankey-en-ribbon">
<p>Wider nodes and ribbons</p>
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"ribbon"</code></span><span><strong>Aliases</strong> <code>ribbon / wide / thick</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/sankey-ribbon.html"></iframe>
</div>
<div class="sp-variant" id="sankey-en-minimal">
<p>Thin outline style</p>
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"minimal"</code></span><span><strong>Aliases</strong> <code>minimal / thin / outline</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/sankey-minimal.html"></iframe>
</div>
<div class="sp-variant" id="sankey-en-sorted">
<p>Reorders nodes within each depth column by descending total throughput, so the dominant flows cluster together instead of sitting in input order — makes it easy to spot which nodes carry the most volume.</p>
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"sorted"</code></span><span><strong>Aliases</strong> <code>sorted / reordered / by_flow / ranked</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/sankey-sorted.html"></iframe>
</div>
</div>
</div>

</div>

<div class="lang-fr" style="display:none">

## Signature

`sp.sankey(title, labels, edges_i, edges_j, edges_w, *, variant="basic", **kwargs) -> Chart`

Alias : `sp.sankey`, `sp.sankeys`, `sp.sankey_chart`, `sp.sankey_diagram`, `sp.flow_chart`

## Description

Les diagrammes de Sankey visualisent des flux entre nœuds. La largeur des nœuds et des liens est proportionnelle au volume du flux. Les arêtes sont définies par des indices source (`edges_i`), des indices cible (`edges_j`), et des poids (`edges_w`). Les nœuds sont disposés en colonnes par profondeur BFS.

## Variantes

<div data-sp-registry-table="variants" data-family="sankey"></div>

Une variante inconnue retombe sur la valeur par défaut enregistrée. Les clés de variantes peuvent être préfixées par `en_`, `fr_`, `en-` ou `fr-`.

## Données

`labels` (`list[str]`) — Noms des nœuds. `edges_i` (`list[int]`) — Indices des nœuds source. `edges_j` (`list[int]`) — Indices des nœuds cible. `edges_w` (`list[float]`) — Poids des flux. `width` / `height` (`int`) — Dimensions du graphique.


## Paramètres

<div data-sp-registry-table="options" data-family="sankey"></div>

## Thèmes

<div data-sp-registry-table="themes" data-family="sankey"></div>


## Retour

`Chart` — objet avec une propriété `.html` et une méthode `.show()`.


<div class="sp-panel-source">
<h2>Paramètres</h2>

<div data-sp-registry-table="variants" data-family="sankey"></div>
</div>

<div class="sp-cls sp-open" id="sankey-fr">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('sankey-fr')" title="Réduire / déplier">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('sankey-fr','basic',this)"><span class="sp-cic">⇉</span><span class="sp-clb">Basique</span></button>
<button class="sp-cls-tab" onclick="spCls('sankey-fr','gapped',this)"><span class="sp-cic">⇥</span><span class="sp-clb">Espacé</span></button>
<button class="sp-cls-tab" onclick="spCls('sankey-fr','ribbon',this)"><span class="sp-cic">▬</span><span class="sp-clb">Ruban</span></button>
<button class="sp-cls-tab" onclick="spCls('sankey-fr','minimal',this)"><span class="sp-cic">—</span><span class="sp-clb">Minimal</span></button>
<button class="sp-cls-tab" onclick="spCls('sankey-fr','sorted',this)"><span class="sp-cic">⇅</span><span class="sp-clb">Trié</span></button>
</div>
<div class="sp-cls-body">
<div class="sp-variant sp-von" id="sankey-fr-basic">
<p>Liens en rubans bézier standards</p>
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"basic"</code></span><span><strong>Alias</strong> <code>basic / default / classic</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/sankey-basic.html"></iframe>
</div>
<div class="sp-variant" id="sankey-fr-gapped">
<p>Espacement des nœuds augmenté</p>
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"gapped"</code></span><span><strong>Alias</strong> <code>gapped / spaced / separated</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/sankey-gapped.html"></iframe>
</div>
<div class="sp-variant" id="sankey-fr-ribbon">
<p>Nœuds et rubans plus larges</p>
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"ribbon"</code></span><span><strong>Alias</strong> <code>ribbon / wide / thick</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/sankey-ribbon.html"></iframe>
</div>
<div class="sp-variant" id="sankey-fr-minimal">
<p>Style filaire fin</p>
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"minimal"</code></span><span><strong>Alias</strong> <code>minimal / thin / outline</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/sankey-minimal.html"></iframe>
</div>
<div class="sp-variant" id="sankey-fr-sorted">
<p>Réordonne les nœuds de chaque colonne de profondeur par débit total décroissant, pour que les flux dominants se regroupent au lieu de rester dans l'ordre d'entrée — facilite le repérage des nœuds qui transportent le plus de volume.</p>
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"sorted"</code></span><span><strong>Alias</strong> <code>sorted / reordered / by_flow / ranked</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/sankey-sorted.html"></iframe>
</div>
</div>
</div>

</div>
