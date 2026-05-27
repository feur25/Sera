# Slope — Before / After Comparison Chart

<div class="lang-en">

<style>
.sp-tabs{border:1px solid #334155;border-radius:8px;overflow:hidden;margin:1.2em 0}
.sp-tab-btns{display:flex;background:#0f172a;border-bottom:1px solid #334155;flex-wrap:wrap}
.sp-tb{padding:8px 14px;border:none;background:none;color:#64748b;cursor:pointer;font-size:12px;font-weight:600;border-bottom:2px solid transparent;transition:color .15s,border-color .15s;white-space:nowrap}
.sp-tb:hover{color:#e2e8f0}
.sp-tb.sp-act{color:#6366f1;border-bottom-color:#6366f1}
.sp-tc{display:none}
.sp-tc.sp-on{display:block}
.sp-cls{display:flex;gap:0;margin:1.6em 0 1.6em 36px;border-radius:14px;background:linear-gradient(180deg,#0a0f1c 0%,#060912 100%);box-shadow:0 18px 50px -12px rgba(0,0,0,.6),0 0 0 1px #1e293b inset;position:relative;overflow:visible}
.sp-cls-rail{display:flex;flex-direction:column;background:linear-gradient(180deg,#0d1426,#070b18);border-right:1px solid #1e293b;padding:18px 0;min-width:18px;transition:min-width .28s cubic-bezier(.5,0,.2,1);position:relative;z-index:2;border-radius:14px 0 0 14px;overflow:visible}
.sp-cls.sp-open .sp-cls-rail{min-width:170px;padding:18px 8px}
.sp-cls-toggle{position:absolute;top:-14px;left:8px;padding:5px 9px;background:#1e293b;color:#a5b4fc;border:1px solid #312e81;border-radius:6px;cursor:pointer;font-size:12px;font-weight:700;transition:all .15s;line-height:1;z-index:5;box-shadow:0 4px 12px -2px rgba(0,0,0,.5)}
.sp-cls-toggle:hover{background:#312e81;color:#e0e7ff;transform:translateY(-1px)}
.sp-cls-tab{position:relative;display:flex;align-items:center;gap:8px;margin:5px 0 5px -34px;padding:11px 16px 11px 14px;background:linear-gradient(90deg,#1a2540 0%,#141d33 70%,#0f172a 100%);color:#94a3b8;font-size:12px;font-weight:600;cursor:pointer;border:none;text-align:left;white-space:nowrap;border-radius:8px 0 0 8px;box-shadow:-6px 4px 14px -4px rgba(0,0,0,.55),inset 0 1px 0 rgba(255,255,255,.04),inset 1px 0 0 rgba(255,255,255,.05);transition:all .25s cubic-bezier(.5,0,.2,1);clip-path:polygon(0 0,calc(100% - 10px) 0,100% 50%,calc(100% - 10px) 100%,0 100%);min-height:18px}
.sp-cls-tab:hover{background:linear-gradient(90deg,#23304d,#1a2540 70%,#141d33);color:#e0e7ff;margin-left:-40px;box-shadow:-8px 6px 18px -4px rgba(0,0,0,.6),inset 0 1px 0 rgba(255,255,255,.06)}
.sp-cls-tab.sp-cact{background:linear-gradient(90deg,#3730a3 0%,#1e1b4b 50%,#0f172a 100%);color:#f5f3ff;margin-left:-46px;box-shadow:-10px 8px 22px -4px rgba(99,102,241,.35),-3px 0 0 0 #818cf8 inset,inset 0 1px 0 rgba(165,180,252,.2);font-weight:700;z-index:3}
.sp-cls-tab .sp-cic{font-size:13px;flex-shrink:0;color:#a5b4fc;font-weight:900;letter-spacing:-1px;width:16px;text-align:center;text-shadow:0 0 6px rgba(165,180,252,.4)}
.sp-cls-tab.sp-cact .sp-cic{color:#e0e7ff;text-shadow:0 0 10px rgba(165,180,252,.7)}
.sp-cls-tab .sp-clb{display:none;font-weight:inherit;letter-spacing:.01em}
.sp-cls.sp-open .sp-cls-tab .sp-clb{display:inline}
.sp-cls-body{flex:1;padding:24px 26px 22px;background:#0a0f1c;min-width:0;position:relative;z-index:1;border-radius:0 14px 14px 0;overflow:hidden}
.sp-variant{display:none}
.sp-variant.sp-von{display:block;animation:spFade .25s ease}
@keyframes spFade{from{opacity:0;transform:translateX(8px)}to{opacity:1;transform:translateX(0)}}
.sp-vmeta{display:flex;flex-wrap:wrap;gap:8px 18px;align-items:center;font-size:13px;color:#94a3b8;margin:6px 0 16px;padding:10px 14px;background:rgba(99,102,241,.06);border-left:3px solid #6366f1;border-radius:0 6px 6px 0}
.sp-vmeta strong{color:#a5b4fc;font-weight:700;margin-right:4px;letter-spacing:.04em;text-transform:uppercase;font-size:11px}
.sp-vmeta code{background:#1e293b;padding:2px 7px;border-radius:4px;color:#e2e8f0;font-size:12px}
.sp-preview-frame{width:100%;height:500px;border:none;border-radius:10px;display:block;background:#0d1117;margin-top:10px;box-shadow:0 8px 24px -8px rgba(0,0,0,.5)}
.sp-preview-label{font-size:11px;letter-spacing:.14em;font-weight:700;color:#818cf8;margin:20px 0 8px;text-transform:uppercase}
</style>
<script>
function spTab(g,id,btn){var r=document.getElementById(g);r.querySelectorAll('.sp-tc').forEach(function(e){e.classList.remove('sp-on')});r.querySelectorAll('.sp-tb').forEach(function(b){b.classList.remove('sp-act')});document.getElementById(id).classList.add('sp-on');btn.classList.add('sp-act');if(window.hljs)document.getElementById(id).querySelectorAll('code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})}
function spCls(scope,name,btn){var root=document.getElementById(scope);root.querySelectorAll('.sp-variant').forEach(function(s){s.classList.remove('sp-von')});root.querySelectorAll('.sp-cls-tab').forEach(function(b){b.classList.remove('sp-cact')});document.getElementById(scope+'-'+name).classList.add('sp-von');btn.classList.add('sp-cact');if(window.hljs)document.getElementById(scope+'-'+name).querySelectorAll('code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})}
function spClsTog(id){document.getElementById(id).classList.toggle('sp-open')}
document.addEventListener('DOMContentLoaded',function(){if(window.hljs)document.querySelectorAll('.sp-tc.sp-on code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})});
</script>


## Signature

`sp.slope(title, labels, left, right, *, variant="basic", left_label="Before", right_label="After", palette=None, show_text=True, **kwargs) -> Chart`

Aliases: `sp.slope`, `sp.build_slope_chart`

## Description

`sp.slope()` renders the entire slope-chart family: two parallel value axes (left / right) with one connector per row. The `variant` keyword swaps the connector style without changing any other parameter. Slope charts excel at before/after comparisons, A/B test outcomes, ranking shifts, KPI changes between periods, and any pair-wise change across many entities.

| Variant | Use case | Connector |
|---|---|---|
| `"basic"`       | Default change view                 | Straight, coloured by direction |
| `"monochrome"`  | Series identity matters             | Straight, palette colour per row |
| `"highlighted"` | Spotlight top movers                | Top 3 highlighted, rest grey |
| `"bumps"`       | Rank shifts, league tables          | Bezier on rank-y |
| `"curved"`      | Smoother flow                       | Cubic Bezier S-curve |
| `"thick"`       | Magnitude-driven                    | Width ∝ \|Δ\| |
| `"diverging"`   | Pure delta visualisation            | Centered delta bars |
| `"stepped"`     | Rectilinear / dashboard look        | Horizontal-vertical-horizontal |

## Parameters

| Parameter | Used by variants |
|-----------|------------------|
| `left_label` | diverging |
| `palette` | bumps, monochrome |
| `right_label` | diverging |
| `show_text` | basic, bumps, curved, highlighted, monochrome, stepped, thick |
| `width` | diverging |

## Returns

`Chart` — object with `.html` property and `.show()` method.

---

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

## Tips

- Use `sort_order="asc"` / `"desc"` to reorder rows by `left` value before drawing.
- The `"diverging"` and `"thick"` variants encode magnitude visually — perfect for executive summaries.
- For rank shifts (positions in a league), prefer `"bumps"` rather than `"basic"`.
- Combine `palette=` with `"monochrome"` to match brand colours per category.

</div>

<div class="lang-fr">

<h2>Signature</h2>

`sp.slope(title, labels, left, right, *, variant="basic", left_label="Before", right_label="After", palette=None, show_text=True, **kwargs) -> Chart`

Alias : `sp.slope`, `sp.build_slope_chart`

<h2>Description</h2>

`sp.slope()` produit toute la famille des slope charts : deux axes de valeurs parallèles (gauche / droite) avec un connecteur par ligne. Le mot-clé `variant` permute le style du connecteur sans changer aucun autre paramètre. Idéal pour comparer avant/après, résultats A/B, changements de classement, KPI entre périodes, et toute évolution par paire sur de nombreuses entités.

| Variante | Cas d’usage | Connecteur |
|---|---|---|
| `"basic"`       | Vue de changement par défaut         | Droite, colorée par direction |
| `"monochrome"`  | L’identité de la série compte           | Droite, couleur palette par ligne |
| `"highlighted"` | Mettre en valeur les leaders              | Top 3 en vif, reste en gris |
| `"bumps"`       | Changements de classement                  | Bézier sur l’axe des rangs |
| `"curved"`      | Flux plus doux                             | Courbe en S Bézier cubique |
| `"thick"`       | Magnitude visible                          | Épaisseur ∝ \|Δ\| |
| `"diverging"`   | Visualisation pure du delta                | Barres delta centrées |
| `"stepped"`     | Look rectiligne / dashboard                | Horizontal-vertical-horizontal |

<h2>Paramètres</h2>

| Paramètre | Utilisé par variantes |
|-----------|----------------------|
| `left_label` | diverging |
| `palette` | bumps, monochrome |
| `right_label` | diverging |
| `show_text` | basic, bumps, curved, highlighted, monochrome, stepped, thick |
| `width` | diverging |

<h2>Retour</h2>

`Chart` — objet avec propriété `.html` et méthode `.show()`.

---

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

<h2>Astuces</h2>

- Utilisez `sort_order="asc"` / `"desc"` pour réordonner les lignes selon `left` avant le rendu.
- Les variantes `"diverging"` et `"thick"` encodent visuellement la magnitude — parfaites pour un résumé exécutif.
- Pour les changements de rang (positions dans un classement), préférez `"bumps"` à `"basic"`.
- Combinez `palette=` avec `"monochrome"` pour aligner les couleurs sur les catégories de marque.

</div>
