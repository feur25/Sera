# Chord Diagram

<div class="lang-en">

<style>
.sp-panel-source{display:none!important}
.sp-cls{display:flex;gap:0;margin:1.6em 0 1.6em 36px;border-radius:14px;background:linear-gradient(180deg,#0a0f1c 0%,#060912 100%);box-shadow:0 18px 50px -12px rgba(0,0,0,.6),0 0 0 1px #1e293b inset;position:relative;overflow:visible}.sp-cls-rail{display:flex;flex-direction:column;background:linear-gradient(180deg,#0d1426,#070b18);border-right:1px solid #1e293b;padding:18px 0;min-width:18px;transition:min-width .28s cubic-bezier(.5,0,.2,1);position:relative;z-index:2;border-radius:14px 0 0 14px;overflow:visible}.sp-cls.sp-open .sp-cls-rail{min-width:170px;padding:18px 8px}.sp-cls-toggle{position:absolute;top:-14px;left:8px;padding:5px 9px;background:#1e293b;color:#a5b4fc;border:1px solid #312e81;border-radius:6px;cursor:pointer;font-size:12px;font-weight:700;transition:all .15s;line-height:1;z-index:5;box-shadow:0 4px 12px -2px rgba(0,0,0,.5)}.sp-cls-tab{position:relative;display:flex;align-items:center;gap:8px;margin:5px 0 5px -34px;padding:11px 16px 11px 14px;background:linear-gradient(90deg,#1a2540 0%,#141d33 70%,#0f172a 100%);color:#94a3b8;font-size:12px;font-weight:600;cursor:pointer;border:none;text-align:left;white-space:nowrap;border-radius:8px 0 0 8px;box-shadow:-6px 4px 14px -4px rgba(0,0,0,.55);transition:all .25s cubic-bezier(.5,0,.2,1);clip-path:polygon(0 0,calc(100% - 10px) 0,100% 50%,calc(100% - 10px) 100%,0 100%);min-height:18px}.sp-cls-tab.sp-cact{background:linear-gradient(90deg,#3730a3 0%,#1e1b4b 50%,#0f172a 100%);color:#f5f3ff;margin-left:-46px;box-shadow:-10px 8px 22px -4px rgba(99,102,241,.35),-3px 0 0 0 #818cf8 inset;font-weight:700;z-index:3}.sp-cls-tab .sp-cic{font-size:13px;flex-shrink:0;color:#a5b4fc;font-weight:900;width:16px;text-align:center}.sp-cls-tab .sp-clb{display:none}.sp-cls.sp-open .sp-cls-tab .sp-clb{display:inline}.sp-cls-body{flex:1;padding:24px 26px 22px;background:#0a0f1c;min-width:0;border-radius:0 14px 14px 0;overflow:hidden}.sp-variant{display:none}.sp-variant.sp-von{display:block}.sp-vmeta{display:flex;flex-wrap:wrap;gap:8px 18px;align-items:center;font-size:13px;color:#94a3b8;margin:6px 0 16px;padding:10px 14px;background:rgba(99,102,241,.06);border-left:3px solid #6366f1;border-radius:0 6px 6px 0}.sp-vmeta strong{color:#a5b4fc;font-weight:700;margin-right:4px;letter-spacing:.04em;text-transform:uppercase;font-size:11px}.sp-vmeta code{background:#1e293b;padding:2px 7px;border-radius:4px;color:#e2e8f0;font-size:12px}.sp-preview-frame{width:100%;height:420px;border:none;border-radius:10px;display:block;background:#0d1117;margin-top:10px}.sp-preview-label{font-size:11px;letter-spacing:.14em;font-weight:700;color:#818cf8;margin:20px 0 8px;text-transform:uppercase}
</style>
<script>
function spCls(scope,name,btn){var root=document.getElementById(scope);root.querySelectorAll('.sp-variant').forEach(function(s){s.classList.remove('sp-von')});root.querySelectorAll('.sp-cls-tab').forEach(function(b){b.classList.remove('sp-cact')});document.getElementById(scope+'-'+name).classList.add('sp-von');btn.classList.add('sp-cact')}
function spClsTog(id){document.getElementById(id).classList.toggle('sp-open')}
</script>

## Signature

`sp.chord(title, labels, matrix, *, variant="basic", **kwargs) -> Chart`

Aliases: `sp.chord`, `sp.chord_chart`, `sp.chord_diagram`

## Description

Chord diagrams show relationships between entities using arcs and ribbons around a circle. The `matrix` is an N×N flow matrix where `matrix[i][j]` is the flow from node `i` to node `j`.

## Variants

<div data-sp-registry-table="variants" data-family="chord"></div>

Unknown variant strings fall back to the registered default. Variant keys may be prefixed with `en_`, `fr_`, `en-` or `fr-`.

## Data

`labels` (`list[str]`) — Node names. `matrix` (`list[list[float]]`) — N×N flow matrix. `width` / `height` (`int`) — Chart dimensions (default 700×700).


## Parameters

<div data-sp-registry-table="options" data-family="chord"></div>

## Themes

<div data-sp-registry-table="themes" data-family="chord"></div>


## Returns

`Chart` — object with `.html` property and `.show()` method.


<div class="sp-panel-source">
<h2>Parameters</h2>

<div data-sp-registry-table="variants" data-family="chord"></div>
</div>

<div class="sp-cls sp-open" id="chord-en">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('chord-en')" title="Collapse / expand">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('chord-en','basic',this)"><span class="sp-cic">◉</span><span class="sp-clb">Basic</span></button>
<button class="sp-cls-tab" onclick="spCls('chord-en','ribbon',this)"><span class="sp-cic">▬</span><span class="sp-clb">Ribbon</span></button>
<button class="sp-cls-tab" onclick="spCls('chord-en','arc',this)"><span class="sp-cic">◌</span><span class="sp-clb">Arc</span></button>
<button class="sp-cls-tab" onclick="spCls('chord-en','mono',this)"><span class="sp-cic">○</span><span class="sp-clb">Mono</span></button>
<button class="sp-cls-tab" onclick="spCls('chord-en','directed',this)"><span class="sp-cic">➤</span><span class="sp-clb">Directed</span></button>
</div>
<div class="sp-cls-body">
<div class="sp-variant sp-von" id="chord-en-basic">
<p>Standard filled ribbons</p>
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"basic"</code></span><span><strong>Aliases</strong> <code>basic / default / classic</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/chord-basic.html"></iframe>
</div>
<div class="sp-variant" id="chord-en-ribbon">
<p>Wider ribbon links</p>
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"ribbon"</code></span><span><strong>Aliases</strong> <code>ribbon / wide</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/chord-ribbon.html"></iframe>
</div>
<div class="sp-variant" id="chord-en-arc">
<p>Arc-only (no filled ribbons)</p>
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"arc"</code></span><span><strong>Aliases</strong> <code>arc / outline</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/chord-arc.html"></iframe>
</div>
<div class="sp-variant" id="chord-en-mono">
<p>Single-color monochrome</p>
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"mono"</code></span><span><strong>Aliases</strong> <code>mono / single</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/chord-mono.html"></iframe>
</div>
<div class="sp-variant" id="chord-en-directed">
<p>Draws a small arrowhead on each ribbon pointing toward whichever side receives more — reads the matrix's row/column asymmetry directly off the diagram instead of just the ribbon's natural taper.</p>
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"directed"</code></span><span><strong>Aliases</strong> <code>directed / asymmetric / flow_direction / arrows</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/chord-directed.html"></iframe>
</div>
</div>
</div>

</div>

<div class="lang-fr" style="display:none">

## Signature

`sp.chord(title, labels, matrix, *, variant="basic", **kwargs) -> Chart`

Alias : `sp.chord`, `sp.chord_chart`, `sp.chord_diagram`

## Description

Les diagrammes en accords (chord) montrent les relations entre entités à l'aide d'arcs et de rubans autour d'un cercle. `matrix` est une matrice de flux N×N où `matrix[i][j]` est le flux du nœud `i` vers le nœud `j`.

## Variantes

<div data-sp-registry-table="variants" data-family="chord"></div>

Une variante inconnue retombe sur la valeur par défaut enregistrée. Les clés de variantes peuvent être préfixées par `en_`, `fr_`, `en-` ou `fr-`.

## Données

`labels` (`list[str]`) — Noms des nœuds. `matrix` (`list[list[float]]`) — Matrice de flux N×N. `width` / `height` (`int`) — Dimensions du graphique (défaut 700×700).


## Paramètres

<div data-sp-registry-table="options" data-family="chord"></div>

## Thèmes

<div data-sp-registry-table="themes" data-family="chord"></div>


## Retour

`Chart` — objet avec une propriété `.html` et une méthode `.show()`.


<div class="sp-panel-source">
<h2>Paramètres</h2>

<div data-sp-registry-table="variants" data-family="chord"></div>
</div>

<div class="sp-cls sp-open" id="chord-fr">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('chord-fr')" title="Réduire / déplier">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('chord-fr','basic',this)"><span class="sp-cic">◉</span><span class="sp-clb">Basique</span></button>
<button class="sp-cls-tab" onclick="spCls('chord-fr','ribbon',this)"><span class="sp-cic">▬</span><span class="sp-clb">Ruban</span></button>
<button class="sp-cls-tab" onclick="spCls('chord-fr','arc',this)"><span class="sp-cic">◌</span><span class="sp-clb">Arc</span></button>
<button class="sp-cls-tab" onclick="spCls('chord-fr','mono',this)"><span class="sp-cic">○</span><span class="sp-clb">Mono</span></button>
<button class="sp-cls-tab" onclick="spCls('chord-fr','directed',this)"><span class="sp-cic">➤</span><span class="sp-clb">Dirigé</span></button>
</div>
<div class="sp-cls-body">
<div class="sp-variant sp-von" id="chord-fr-basic">
<p>Rubans pleins standards</p>
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"basic"</code></span><span><strong>Alias</strong> <code>basic / default / classic</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/chord-basic.html"></iframe>
</div>
<div class="sp-variant" id="chord-fr-ribbon">
<p>Liens en rubans plus larges</p>
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"ribbon"</code></span><span><strong>Alias</strong> <code>ribbon / wide</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/chord-ribbon.html"></iframe>
</div>
<div class="sp-variant" id="chord-fr-arc">
<p>Arcs seuls (sans rubans pleins)</p>
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"arc"</code></span><span><strong>Alias</strong> <code>arc / outline</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/chord-arc.html"></iframe>
</div>
<div class="sp-variant" id="chord-fr-mono">
<p>Monochrome, couleur unique</p>
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"mono"</code></span><span><strong>Alias</strong> <code>mono / single</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/chord-mono.html"></iframe>
</div>
<div class="sp-variant" id="chord-fr-directed">
<p>Trace une petite flèche sur chaque ruban pointant vers le côté qui reçoit le plus — lit l'asymétrie ligne/colonne de la matrice directement sur le diagramme plutôt que par le seul évasement naturel du ruban.</p>
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"directed"</code></span><span><strong>Alias</strong> <code>directed / asymmetric / flow_direction / arrows</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/chord-directed.html"></iframe>
</div>
</div>
</div>

</div>
