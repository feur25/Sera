# Waterfall — Running-Total Bridge Chart

<div class="lang-en">

<style>
.sp-preview-frame{width:100%;height:520px;border:none;border-radius:10px;display:block;background:#0d1117;margin-top:10px;box-shadow:0 8px 24px -8px rgba(0,0,0,.5)}
</style>
<script>
function spTab(g,id,btn){var r=document.getElementById(g);r.querySelectorAll('.sp-tc').forEach(function(e){e.classList.remove('sp-on')});r.querySelectorAll('.sp-tb').forEach(function(b){b.classList.remove('sp-act')});document.getElementById(id).classList.add('sp-on');btn.classList.add('sp-act');if(window.hljs)document.getElementById(id).querySelectorAll('code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})}
function spCls(scope,name,btn){var root=document.getElementById(scope);root.querySelectorAll('.sp-variant').forEach(function(s){s.classList.remove('sp-von')});root.querySelectorAll('.sp-cls-tab').forEach(function(b){b.classList.remove('sp-cact')});document.getElementById(scope+'-'+name).classList.add('sp-von');btn.classList.add('sp-cact');if(window.hljs)document.getElementById(scope+'-'+name).querySelectorAll('code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})}
function spClsTog(id){document.getElementById(id).classList.toggle('sp-open')}
document.addEventListener('DOMContentLoaded',function(){if(window.hljs)document.querySelectorAll('.sp-tc.sp-on code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})});
</script>

## Signature

`sp.waterfall(title, labels, values, *, variant="basic", show_text=True, **kwargs) -> Chart`

## Description

`sp.waterfall()` renders the entire waterfall-chart family: a sequence of bars where each step adds (positive) or subtracts (negative) from a running total. The `variant` keyword selects the geometry without touching any other parameter. Waterfalls are the standard for P&L bridges, variance analysis, cohort decomposition, fee/tax breakdowns and any "from A to B, what changed?" narrative.

> **Totals** — set a value to `0` and use a label containing `total`, `net`, `final`, `gross` or `ebitda` to mark a subtotal bar; it is rendered with the totals color and anchored on the running sum.

## Variants

<div data-sp-registry-table="variants" data-family="waterfall"></div>

## Parameters

<div data-sp-registry-table="options" data-family="waterfall"></div>

## Returns

`Chart` — object with `.html` property and `.show()` method.

---

<div class="sp-cls sp-open" id="waterfall-en">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('waterfall-en')" title="Collapse / expand">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('waterfall-en','basic',this)"><span class="sp-cic">B</span><span class="sp-clb">Basic</span></button>
<button class="sp-cls-tab" onclick="spCls('waterfall-en','stepped',this)"><span class="sp-cic">S</span><span class="sp-clb">Stepped</span></button>
<button class="sp-cls-tab" onclick="spCls('waterfall-en','lollipop',this)"><span class="sp-cic">L</span><span class="sp-clb">Lollipop</span></button>
<button class="sp-cls-tab" onclick="spCls('waterfall-en','arrowed',this)"><span class="sp-cic">A</span><span class="sp-clb">Arrowed</span></button>
<button class="sp-cls-tab" onclick="spCls('waterfall-en','delta',this)"><span class="sp-cic">D</span><span class="sp-clb">Delta</span></button>
<button class="sp-cls-tab" onclick="spCls('waterfall-en','horizontal',this)"><span class="sp-cic">H</span><span class="sp-clb">Horizontal</span></button>
</div>
<div class="sp-cls-body">

<div class="sp-variant sp-von" id="waterfall-en-basic">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"basic"</code></span><span><strong>Aliases</strong> <code>basic / default / classic / bars</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Classic running-sum bars with dashed connectors between consecutive steps.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/waterfall-basic.html"></iframe>
</div>

<div class="sp-variant" id="waterfall-en-stepped">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"stepped"</code></span><span><strong>Aliases</strong> <code>stepped / step / staircase / stairs</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Bars touch each other forming a continuous staircase, no connectors needed.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/waterfall-stepped.html"></iframe>
</div>

<div class="sp-variant" id="waterfall-en-lollipop">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"lollipop"</code></span><span><strong>Aliases</strong> <code>lollipop / stick / popsicle / lolly</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Stick + circle marker at the end of each step. Excellent ink-to-data ratio.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/waterfall-lollipop.html"></iframe>
</div>

<div class="sp-variant" id="waterfall-en-arrowed">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"arrowed"</code></span><span><strong>Aliases</strong> <code>arrowed / arrow / directional / tipped</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Triangle on top (positives) or bottom (negatives) emphasizes direction at a glance.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/waterfall-arrowed.html"></iframe>
</div>

<div class="sp-variant" id="waterfall-en-delta">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"delta"</code></span><span><strong>Aliases</strong> <code>delta / percent / annotated / pct</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Bars + signed percentage badge (Delta vs previous running total) above each step.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/waterfall-delta.html"></iframe>
</div>

<div class="sp-variant" id="waterfall-en-horizontal">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"horizontal"</code></span><span><strong>Aliases</strong> <code>horizontal / rows / sideways / h</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Rotated 90 degrees: each step becomes a horizontal row stacking downward, anchored to the previous running total. Editorial layout for reports with long labels or vertical storytelling.</p>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/waterfall-horizontal.html"></iframe>
</div>

</div>
</div>

</div>

<div class="lang-fr">

<h2>Signature</h2>

`sp.waterfall(title, labels, values, *, variant="basic", show_text=True, **kwargs) -> Chart`

<h2>Description</h2>

`sp.waterfall()` rassemble toute la famille des graphiques waterfall : une suite de barres ou chaque etape ajoute (positif) ou retranche (negatif) au cumul courant. Le mot-cle `variant` change la geometrie sans toucher aux autres parametres. Les waterfalls sont la reference pour les ponts de P&L, l analyse d ecarts, la decomposition de cohortes, le detail des frais/taxes et tout recit du type "de A vers B, qu est-ce qui a change ?".

> **Totaux** — mettez la valeur a `0` et utilisez un libelle contenant `total`, `net`, `final`, `gross` ou `ebitda` pour marquer une barre de sous-total ; elle est rendue avec la couleur des totaux et ancree sur le cumul courant.

<h2>Variantes</h2>

<div data-sp-registry-table="variants" data-family="waterfall"></div>

<h2>Paramètres</h2>

<div data-sp-registry-table="options" data-family="waterfall"></div>

<h2>Retour</h2>

`Chart` — objet avec une propriete `.html` et une methode `.show()`.

---

<div class="sp-cls sp-open" id="waterfall-fr">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('waterfall-fr')" title="Collapse / expand">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('waterfall-fr','basic',this)"><span class="sp-cic">B</span><span class="sp-clb">Basic</span></button>
<button class="sp-cls-tab" onclick="spCls('waterfall-fr','stepped',this)"><span class="sp-cic">S</span><span class="sp-clb">Stepped</span></button>
<button class="sp-cls-tab" onclick="spCls('waterfall-fr','lollipop',this)"><span class="sp-cic">L</span><span class="sp-clb">Lollipop</span></button>
<button class="sp-cls-tab" onclick="spCls('waterfall-fr','arrowed',this)"><span class="sp-cic">A</span><span class="sp-clb">Arrowed</span></button>
<button class="sp-cls-tab" onclick="spCls('waterfall-fr','delta',this)"><span class="sp-cic">D</span><span class="sp-clb">Delta</span></button>
<button class="sp-cls-tab" onclick="spCls('waterfall-fr','horizontal',this)"><span class="sp-cic">H</span><span class="sp-clb">Horizontal</span></button>
</div>
<div class="sp-cls-body">

<div class="sp-variant sp-von" id="waterfall-fr-basic">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"basic"</code></span><span><strong>Aliases</strong> <code>basic / default / classic / bars</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Barres classiques a somme cumulee avec connecteurs pointilles entre etapes.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/waterfall-basic.html"></iframe>
</div>

<div class="sp-variant" id="waterfall-fr-stepped">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"stepped"</code></span><span><strong>Aliases</strong> <code>stepped / step / staircase / stairs</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Barres jointives formant un escalier continu, sans connecteur.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/waterfall-stepped.html"></iframe>
</div>

<div class="sp-variant" id="waterfall-fr-lollipop">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"lollipop"</code></span><span><strong>Aliases</strong> <code>lollipop / stick / popsicle / lolly</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Tige + cercle en fin de chaque etape. Excellent ratio encre/donnee.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/waterfall-lollipop.html"></iframe>
</div>

<div class="sp-variant" id="waterfall-fr-arrowed">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"arrowed"</code></span><span><strong>Aliases</strong> <code>arrowed / arrow / directional / tipped</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Triangle dessus (positifs) ou dessous (negatifs) pour souligner la direction en un coup d oeil.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/waterfall-arrowed.html"></iframe>
</div>

<div class="sp-variant" id="waterfall-fr-delta">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"delta"</code></span><span><strong>Aliases</strong> <code>delta / percent / annotated / pct</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Barres + badge de pourcentage signe (Delta vs cumul precedent) au-dessus de chaque etape.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/waterfall-delta.html"></iframe>
</div>

<div class="sp-variant" id="waterfall-fr-horizontal">

<div class="sp-vmeta"><span><strong>Variante</strong> <code>"horizontal"</code></span><span><strong>Aliases</strong> <code>horizontal / rows / sideways / h</code></span><span><strong>Retour</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Waterfall pivote a 90 degres : les etapes deviennent des lignes empilees verticalement, chacune partant du cumul precedent. Layout editorial pour rapports avec libelles longs ou storytelling vertical.</p>
<div class="sp-preview-label">Apercu</div>
<iframe class="sp-preview-frame" src="../../previews/waterfall-horizontal.html"></iframe>
</div>

</div>
</div>

</div>
