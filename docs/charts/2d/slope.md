# Slope — Before / After Comparison Chart

<div class="lang-en">

<style>
.sp-preview-frame{width:100%;height:500px;border:none;border-radius:10px;display:block;background:#0d1117;margin-top:10px;box-shadow:0 8px 24px -8px rgba(0,0,0,.5)}
</style>
<script>
function spTab(g,id,btn){var r=document.getElementById(g);r.querySelectorAll('.sp-tc').forEach(function(e){e.classList.remove('sp-on')});r.querySelectorAll('.sp-tb').forEach(function(b){b.classList.remove('sp-act')});document.getElementById(id).classList.add('sp-on');btn.classList.add('sp-act');if(window.hljs)document.getElementById(id).querySelectorAll('code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})}
function spCls(scope,name,btn){var root=document.getElementById(scope);root.querySelectorAll('.sp-variant').forEach(function(s){s.classList.remove('sp-von')});root.querySelectorAll('.sp-cls-tab').forEach(function(b){b.classList.remove('sp-cact')});document.getElementById(scope+'-'+name).classList.add('sp-von');btn.classList.add('sp-cact');if(window.hljs)document.getElementById(scope+'-'+name).querySelectorAll('code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})}
function spClsTog(id){document.getElementById(id).classList.toggle('sp-open')}
document.addEventListener('DOMContentLoaded',function(){if(window.hljs)document.querySelectorAll('.sp-tc.sp-on code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})});
</script>

## Signature

`sp.slope(title, labels, left, right, *, variant="basic", left_label="Before", right_label="After", palette=None, show_text=True, **kwargs) -> Chart`

## Description

`sp.slope()` renders the entire slope-chart family: two parallel value axes (left / right) with one connector per row. The `variant` keyword swaps the connector style without changing any other parameter. Slope charts excel at before/after comparisons, A/B test outcomes, ranking shifts, KPI changes between periods, and any pair-wise change across many entities.
## Variants

<div data-sp-registry-table="variants" data-family="slope"></div>

## Parameters

<div data-sp-registry-table="options" data-family="slope"></div>

## Returns

`Chart` — object with `.html` property and `.show()` method.

---

## Tips

- Use `sort_order="asc"` / `"desc"` to reorder rows by `left` value before drawing.
- The `"diverging"` and `"thick"` variants encode magnitude visually — perfect for executive summaries.
- For rank shifts (positions in a league), prefer `"bumps"` rather than `"basic"`.
- Combine `palette=` with `"monochrome"` to match brand colours per category.

<div class="sp-cls sp-open" id="slope-en">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('slope-en')" title="Collapse / expand">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('slope-en','basic',this)"><span class="sp-cic">B</span><span class="sp-clb">Basic</span></button>
<button class="sp-cls-tab" onclick="spCls('slope-en','monochrome',this)"><span class="sp-cic">M</span><span class="sp-clb">Monochrome</span></button>
<button class="sp-cls-tab" onclick="spCls('slope-en','highlighted',this)"><span class="sp-cic">H</span><span class="sp-clb">Highlighted</span></button>
<button class="sp-cls-tab" onclick="spCls('slope-en','bumps',this)"><span class="sp-cic">U</span><span class="sp-clb">Bumps</span></button>
<button class="sp-cls-tab" onclick="spCls('slope-en','curved',this)"><span class="sp-cic">C</span><span class="sp-clb">Curved</span></button>
<button class="sp-cls-tab" onclick="spCls('slope-en','thick',this)"><span class="sp-cic">T</span><span class="sp-clb">Thick</span></button>
<button class="sp-cls-tab" onclick="spCls('slope-en','diverging',this)"><span class="sp-cic">D</span><span class="sp-clb">Diverging</span></button>
<button class="sp-cls-tab" onclick="spCls('slope-en','stepped',this)"><span class="sp-cic">S</span><span class="sp-clb">Stepped</span></button>
</div>
<div class="sp-cls-body">
<div class="sp-variant sp-von" id="slope-en-basic">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"basic"</code></span><span><strong>Aliases</strong> <code>basic / default / direction / classic</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Direction-coloured straight lines (green up, red down) with endpoint dots.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/slope-basic.html"></iframe>
</div>
<div class="sp-variant" id="slope-en-monochrome">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"monochrome"</code></span><span><strong>Aliases</strong> <code>monochrome / mono / uniform / single_color</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Uniform palette colour per row, no direction tint — ideal for categorical narratives.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/slope-monochrome.html"></iframe>
</div>
<div class="sp-variant" id="slope-en-highlighted">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"highlighted"</code></span><span><strong>Aliases</strong> <code>highlighted / highlight / top / movers</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Top 3 movers (largest |Δ|) drawn in vivid colour, the rest dimmed in grey.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/slope-highlighted.html"></iframe>
</div>
<div class="sp-variant" id="slope-en-bumps">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"bumps"</code></span><span><strong>Aliases</strong> <code>bumps / bumpchart / rank / ranking</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Bump chart: y axis encodes the rank (1..n) at each side instead of the raw value.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/slope-bumps.html"></iframe>
</div>
<div class="sp-variant" id="slope-en-curved">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"curved"</code></span><span><strong>Aliases</strong> <code>curved / curve / bezier / smooth</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Cubic-Bezier S-curves between the two endpoints — smoother visual flow.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/slope-curved.html"></iframe>
</div>
<div class="sp-variant" id="slope-en-thick">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"thick"</code></span><span><strong>Aliases</strong> <code>thick / magnitude / weighted / weight</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Stroke-width proportional to |right - left| — magnitude becomes the visual weight.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/slope-thick.html"></iframe>
</div>
<div class="sp-variant" id="slope-en-diverging">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"diverging"</code></span><span><strong>Aliases</strong> <code>diverging / delta / change / centered</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Centered delta bars: positive bars grow right (green), negative grow left (red).</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/slope-diverging.html"></iframe>
</div>
<div class="sp-variant" id="slope-en-stepped">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"stepped"</code></span><span><strong>Aliases</strong> <code>stepped / step / elbow / rectilinear</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">L-shape connector: horizontal then vertical then horizontal — rectilinear flow.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/slope-stepped.html"></iframe>
</div>
</div>
</div>

</div>

<div class="lang-fr">

<h2>Signature</h2>

`sp.slope(title, labels, left, right, *, variant="basic", left_label="Before", right_label="After", palette=None, show_text=True, **kwargs) -> Chart`

<h2>Description</h2>

`sp.slope()` produit toute la famille des slope charts : deux axes de valeurs parallèles (gauche / droite) avec un connecteur par ligne. Le mot-clé `variant` permute le style du connecteur sans changer aucun autre paramètre. Idéal pour comparer avant/après, résultats A/B, changements de classement, KPI entre périodes, et toute évolution par paire sur de nombreuses entités.
<h2>Variantes</h2>

<div data-sp-registry-table="variants" data-family="slope"></div>

<h2>Paramètres</h2>

<div data-sp-registry-table="options" data-family="slope"></div>

<h2>Retour</h2>

`Chart` — objet avec propriété `.html` et méthode `.show()`.

---

<h2>Astuces</h2>

- Utilisez `sort_order="asc"` / `"desc"` pour réordonner les lignes selon `left` avant le rendu.
- Les variantes `"diverging"` et `"thick"` encodent visuellement la magnitude — parfaites pour un résumé exécutif.
- Pour les changements de rang (positions dans un classement), préférez `"bumps"` à `"basic"`.
- Combinez `palette=` avec `"monochrome"` pour aligner les couleurs sur les catégories de marque.

<div class="sp-cls sp-open" id="slope-fr">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('slope-fr')" title="Collapse / expand">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('slope-fr','basic',this)"><span class="sp-cic">B</span><span class="sp-clb">Basic</span></button>
<button class="sp-cls-tab" onclick="spCls('slope-fr','monochrome',this)"><span class="sp-cic">M</span><span class="sp-clb">Monochrome</span></button>
<button class="sp-cls-tab" onclick="spCls('slope-fr','highlighted',this)"><span class="sp-cic">H</span><span class="sp-clb">Highlighted</span></button>
<button class="sp-cls-tab" onclick="spCls('slope-fr','bumps',this)"><span class="sp-cic">U</span><span class="sp-clb">Bumps</span></button>
<button class="sp-cls-tab" onclick="spCls('slope-fr','curved',this)"><span class="sp-cic">C</span><span class="sp-clb">Curved</span></button>
<button class="sp-cls-tab" onclick="spCls('slope-fr','thick',this)"><span class="sp-cic">T</span><span class="sp-clb">Thick</span></button>
<button class="sp-cls-tab" onclick="spCls('slope-fr','diverging',this)"><span class="sp-cic">D</span><span class="sp-clb">Diverging</span></button>
<button class="sp-cls-tab" onclick="spCls('slope-fr','stepped',this)"><span class="sp-cic">S</span><span class="sp-clb">Stepped</span></button>
</div>
<div class="sp-cls-body">
<div class="sp-variant sp-von" id="slope-fr-basic">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"basic"</code></span><span><strong>Aliases</strong> <code>basic / default / direction / classic</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Lignes droites colorées selon la direction (vert haut, rouge bas) avec points aux extrémités.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/slope-basic.html"></iframe>
</div>
<div class="sp-variant" id="slope-fr-monochrome">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"monochrome"</code></span><span><strong>Aliases</strong> <code>monochrome / mono / uniform / single_color</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Couleur uniforme par ligne tirée de la palette, sans coloration directionnelle.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/slope-monochrome.html"></iframe>
</div>
<div class="sp-variant" id="slope-fr-highlighted">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"highlighted"</code></span><span><strong>Aliases</strong> <code>highlighted / highlight / top / movers</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Top 3 des plus grands Δ mis en avant en couleur vive, le reste estompé en gris.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/slope-highlighted.html"></iframe>
</div>
<div class="sp-variant" id="slope-fr-bumps">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"bumps"</code></span><span><strong>Aliases</strong> <code>bumps / bumpchart / rank / ranking</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Bump chart : l’axe y encode le rang (1..n) à chaque côté plutôt que la valeur.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/slope-bumps.html"></iframe>
</div>
<div class="sp-variant" id="slope-fr-curved">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"curved"</code></span><span><strong>Aliases</strong> <code>curved / curve / bezier / smooth</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Courbes de Bézier cubiques en S entre les deux extrémités — transition douce.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/slope-curved.html"></iframe>
</div>
<div class="sp-variant" id="slope-fr-thick">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"thick"</code></span><span><strong>Aliases</strong> <code>thick / magnitude / weighted / weight</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Épaisseur du trait proportionnelle à |droite - gauche| — la magnitude devient le poids visuel.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/slope-thick.html"></iframe>
</div>
<div class="sp-variant" id="slope-fr-diverging">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"diverging"</code></span><span><strong>Aliases</strong> <code>diverging / delta / change / centered</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Barres delta centrées : positives à droite (vert), négatives à gauche (rouge).</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/slope-diverging.html"></iframe>
</div>
<div class="sp-variant" id="slope-fr-stepped">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"stepped"</code></span><span><strong>Aliases</strong> <code>stepped / step / elbow / rectilinear</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Connecteur en L : horizontal puis vertical puis horizontal — flux rectiligne.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/slope-stepped.html"></iframe>
</div>
</div>
</div>

</div>
