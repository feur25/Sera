# Funnel — Conversion / Pipeline Chart

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
.sp-preview-frame{width:100%;height:480px;border:none;border-radius:10px;display:block;background:#0d1117;margin-top:10px;box-shadow:0 8px 24px -8px rgba(0,0,0,.5)}
.sp-preview-label{font-size:11px;letter-spacing:.14em;font-weight:700;color:#818cf8;margin:20px 0 8px;text-transform:uppercase}
</style>
<script>
function spTab(g,id,btn){var r=document.getElementById(g);r.querySelectorAll('.sp-tc').forEach(function(e){e.classList.remove('sp-on')});r.querySelectorAll('.sp-tb').forEach(function(b){b.classList.remove('sp-act')});document.getElementById(id).classList.add('sp-on');btn.classList.add('sp-act');if(window.hljs)document.getElementById(id).querySelectorAll('code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})}
function spCls(scope,name,btn){var root=document.getElementById(scope);root.querySelectorAll('.sp-variant').forEach(function(s){s.classList.remove('sp-von')});root.querySelectorAll('.sp-cls-tab').forEach(function(b){b.classList.remove('sp-cact')});document.getElementById(scope+'-'+name).classList.add('sp-von');btn.classList.add('sp-cact');if(window.hljs)document.getElementById(scope+'-'+name).querySelectorAll('code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})}
function spClsTog(id){document.getElementById(id).classList.toggle('sp-open')}
document.addEventListener('DOMContentLoaded',function(){if(window.hljs)document.querySelectorAll('.sp-tc.sp-on code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})});
</script>


## Signature

`sp.funnel(title, labels, values, *, variant="basic", palette=None, show_text=True, **kwargs) -> Chart`

Aliases: `sp.funnel`, `sp.build_funnel_chart`

## Description

`sp.funnel()` renders the entire funnel-chart family: a stacked sequence of stages where each step’s width encodes a value. The `variant` keyword switches the geometry without changing any other parameter. Funnels are the standard for conversion analytics (visitors → signups → paid), recruiting pipelines, sales pipelines, process drop-off and any descending-cohort analysis.

| Variant | Geometry | Best for |
|---|---|---|
| `"basic"`      | Centered trapezoids               | Default conversion view |
| `"stepped"`    | Centered rectangles               | When stage values are similar |
| `"rounded"`    | Pill-shaped trapezoids            | Soft / brand-aligned look |
| `"chevron"`    | Right-pointing pentagons          | Sales pipeline / process flow |
| `"gradient"`   | Vertical linear gradient          | Adds depth / texture |
| `"pyramid"`    | Continuous triangle               | Hierarchical / Maslow-style |
| `"inverted"`   | Flipped vertically                | Growth pyramid / amplification |
| `"conversion"` | Basic + drop-off %                | Executive funnel with KPI overlay |

## Parameters

| Parameter | Type | Default | Description |
|---|---|---|---|
| `title`     | `str`         | required   | Chart title |
| `labels`    | `list[str]`   | required   | Stage labels (top → bottom) |
| `values`    | `list[float]` | required   | Stage values (descending recommended) |
| `variant`   | `str`         | `"basic"` | Geometry variant (see table) |
| `palette`   | `list[int]`   | `None`     | Custom per-stage palette |
| `show_text` | `bool`        | `True`     | Show inside labels + side annotations |
| `width`     | `int`         | `800`      | Canvas width (px) |
| `height`    | `int`         | `480`      | Canvas height (px) |

## Returns

`Chart` — object with `.html` property and `.show()` method.

---

<div class="sp-cls sp-open" id="funnel-en">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('funnel-en')" title="Collapse / expand">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('funnel-en','basic',this)"><span class="sp-cic">B</span><span class="sp-clb">Basic</span></button>
<button class="sp-cls-tab" onclick="spCls('funnel-en','stepped',this)"><span class="sp-cic">S</span><span class="sp-clb">Stepped</span></button>
<button class="sp-cls-tab" onclick="spCls('funnel-en','rounded',this)"><span class="sp-cic">R</span><span class="sp-clb">Rounded</span></button>
<button class="sp-cls-tab" onclick="spCls('funnel-en','chevron',this)"><span class="sp-cic">V</span><span class="sp-clb">Chevron</span></button>
<button class="sp-cls-tab" onclick="spCls('funnel-en','gradient',this)"><span class="sp-cic">G</span><span class="sp-clb">Gradient</span></button>
<button class="sp-cls-tab" onclick="spCls('funnel-en','pyramid',this)"><span class="sp-cic">P</span><span class="sp-clb">Pyramid</span></button>
<button class="sp-cls-tab" onclick="spCls('funnel-en','inverted',this)"><span class="sp-cic">I</span><span class="sp-clb">Inverted</span></button>
<button class="sp-cls-tab" onclick="spCls('funnel-en','conversion',this)"><span class="sp-cic">C</span><span class="sp-clb">Conversion</span></button>
</div>
<div class="sp-cls-body">
<div class="sp-variant sp-von" id="funnel-en-basic">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"basic"</code></span><span><strong>Aliases</strong> <code>basic / default / trapezoid / classic</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Classic centered trapezoid pyramid — each step’s top width inherits the previous bottom width.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/funnel-basic.html"></iframe>
</div>
<div class="sp-variant" id="funnel-en-stepped">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"stepped"</code></span><span><strong>Aliases</strong> <code>stepped / bar / rect / rectangle</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Centered rectangles per stage (no diagonal slope), width ∝ value/max.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/funnel-stepped.html"></iframe>
</div>
<div class="sp-variant" id="funnel-en-rounded">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"rounded"</code></span><span><strong>Aliases</strong> <code>rounded / round / pill / smooth</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Trapezoids with rounded outer corners — softer, pill-like aesthetic.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/funnel-rounded.html"></iframe>
</div>
<div class="sp-variant" id="funnel-en-chevron">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"chevron"</code></span><span><strong>Aliases</strong> <code>chevron / arrow / pipeline / pointer</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Pentagon arrow shapes pointing right — sales-pipeline / process flow style.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/funnel-chevron.html"></iframe>
</div>
<div class="sp-variant" id="funnel-en-gradient">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"gradient"</code></span><span><strong>Aliases</strong> <code>gradient / fade / shade / vgrad</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Per-step vertical linear gradient (opaque top → 50%% bottom) for depth and texture.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/funnel-gradient.html"></iframe>
</div>
<div class="sp-variant" id="funnel-en-pyramid">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"pyramid"</code></span><span><strong>Aliases</strong> <code>pyramid / triangle / cone / point</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Continuous pyramid: each level narrows progressively to a point at the bottom.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/funnel-pyramid.html"></iframe>
</div>
<div class="sp-variant" id="funnel-en-inverted">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"inverted"</code></span><span><strong>Aliases</strong> <code>inverted / inverse / reverse / upside_down</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Vertically flipped funnel — widest stage at the bottom (growth pyramid).</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/funnel-inverted.html"></iframe>
</div>
<div class="sp-variant" id="funnel-en-conversion">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"conversion"</code></span><span><strong>Aliases</strong> <code>conversion / dropoff / rate / steps</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Basic trapezoid plus drop-off percentage between consecutive stages displayed in red.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/funnel-conversion.html"></iframe>
</div>
</div>
</div>

## Tips

- Sort stages descending before passing them in (or use `sort_order="desc"`).
- Use `"conversion"` when the audience cares about stage-to-stage retention rate.
- The `"pyramid"` variant works best when values follow a steep decay.
- For broad audiences, `"chevron"` reads as a sales pipeline more naturally than the trapezoid.

## See also

- [Bar](bar.md) — simple value comparisons
- [Waterfall](waterfall.md) — additive contributions
- [Sankey](sankey.md) — multi-step flow with branching

</div>

<div class="lang-fr">

<h2>Signature</h2>

`sp.funnel(title, labels, values, *, variant="basic", palette=None, show_text=True, **kwargs) -> Chart`

Alias : `sp.funnel`, `sp.build_funnel_chart`

<h2>Description</h2>

`sp.funnel()` produit toute la famille des entonnoirs : une séquence d’étapes empilées dont la largeur encode une valeur. Le mot-clé `variant` permute la géométrie sans changer aucun autre paramètre. Standard pour l’analyse de conversion (visiteurs → inscrits → payants), pipelines de recrutement, pipelines commerciaux, fuites de processus et toute analyse de cohorte décroissante.

| Variante | Géométrie | Idéal pour |
|---|---|---|
| `"basic"`      | Trapèzes centrés                   | Vue de conversion par défaut |
| `"stepped"`    | Rectangles centrés                  | Quand les valeurs sont proches |
| `"rounded"`    | Trapèzes en pilule                  | Look doux / charte graphique |
| `"chevron"`    | Pentagones pointant à droite         | Pipeline commercial / processus |
| `"gradient"`   | Dégradé vertical                     | Profondeur / texture |
| `"pyramid"`    | Triangle continu                       | Hiérarchique / type Maslow |
| `"inverted"`   | Inversé verticalement                | Pyramide de croissance |
| `"conversion"` | Trapèze de base + % de chute        | Entonnoir exécutif avec KPI |

<h2>Paramètres</h2>

| Paramètre | Type | Défaut | Description |
|---|---|---|---|
| `title`     | `str`         | requis     | Titre du graphique |
| `labels`    | `list[str]`   | requis     | Labels d’étape (haut → bas) |
| `values`    | `list[float]` | requis     | Valeurs (décroissantes recommandées) |
| `variant`   | `str`         | `"basic"` | Variante de géométrie |
| `palette`   | `list[int]`   | `None`     | Palette personnalisée par étape |
| `show_text` | `bool`        | `True`     | Afficher labels internes + annotations |
| `width`     | `int`         | `800`      | Largeur (px) |
| `height`    | `int`         | `480`      | Hauteur (px) |

<h2>Retour</h2>

`Chart` — objet avec propriété `.html` et méthode `.show()`.

---

<div class="sp-cls sp-open" id="funnel-fr">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('funnel-fr')" title="Collapse / expand">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('funnel-fr','basic',this)"><span class="sp-cic">B</span><span class="sp-clb">Basic</span></button>
<button class="sp-cls-tab" onclick="spCls('funnel-fr','stepped',this)"><span class="sp-cic">S</span><span class="sp-clb">Stepped</span></button>
<button class="sp-cls-tab" onclick="spCls('funnel-fr','rounded',this)"><span class="sp-cic">R</span><span class="sp-clb">Rounded</span></button>
<button class="sp-cls-tab" onclick="spCls('funnel-fr','chevron',this)"><span class="sp-cic">V</span><span class="sp-clb">Chevron</span></button>
<button class="sp-cls-tab" onclick="spCls('funnel-fr','gradient',this)"><span class="sp-cic">G</span><span class="sp-clb">Gradient</span></button>
<button class="sp-cls-tab" onclick="spCls('funnel-fr','pyramid',this)"><span class="sp-cic">P</span><span class="sp-clb">Pyramid</span></button>
<button class="sp-cls-tab" onclick="spCls('funnel-fr','inverted',this)"><span class="sp-cic">I</span><span class="sp-clb">Inverted</span></button>
<button class="sp-cls-tab" onclick="spCls('funnel-fr','conversion',this)"><span class="sp-cic">C</span><span class="sp-clb">Conversion</span></button>
</div>
<div class="sp-cls-body">
<div class="sp-variant sp-von" id="funnel-fr-basic">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"basic"</code></span><span><strong>Aliases</strong> <code>basic / default / trapezoid / classic</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Pyramide trapézoïdale centrée classique — le haut de chaque étape hérite du bas de la précédente.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/funnel-basic.html"></iframe>
</div>
<div class="sp-variant" id="funnel-fr-stepped">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"stepped"</code></span><span><strong>Aliases</strong> <code>stepped / bar / rect / rectangle</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Rectangles centrés par étape (sans pente diagonale), largeur ∝ valeur/max.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/funnel-stepped.html"></iframe>
</div>
<div class="sp-variant" id="funnel-fr-rounded">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"rounded"</code></span><span><strong>Aliases</strong> <code>rounded / round / pill / smooth</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Trapèzes avec coins arrondis — esthétique douce de type pilule.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/funnel-rounded.html"></iframe>
</div>
<div class="sp-variant" id="funnel-fr-chevron">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"chevron"</code></span><span><strong>Aliases</strong> <code>chevron / arrow / pipeline / pointer</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Pentagones en flèche pointant à droite — style pipeline commercial / processus.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/funnel-chevron.html"></iframe>
</div>
<div class="sp-variant" id="funnel-fr-gradient">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"gradient"</code></span><span><strong>Aliases</strong> <code>gradient / fade / shade / vgrad</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Dégradé linéaire vertical par étape (opaque en haut → 50%% en bas) pour la profondeur.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/funnel-gradient.html"></iframe>
</div>
<div class="sp-variant" id="funnel-fr-pyramid">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"pyramid"</code></span><span><strong>Aliases</strong> <code>pyramid / triangle / cone / point</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Pyramide continue : chaque niveau se rétrécit progressivement jusqu’à une pointe.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/funnel-pyramid.html"></iframe>
</div>
<div class="sp-variant" id="funnel-fr-inverted">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"inverted"</code></span><span><strong>Aliases</strong> <code>inverted / inverse / reverse / upside_down</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Entonnoir inversé verticalement — étape la plus large en bas (pyramide de croissance).</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/funnel-inverted.html"></iframe>
</div>
<div class="sp-variant" id="funnel-fr-conversion">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"conversion"</code></span><span><strong>Aliases</strong> <code>conversion / dropoff / rate / steps</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>
<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Trapèze de base avec le pourcentage de chute entre étapes affiché en rouge.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/funnel-conversion.html"></iframe>
</div>
</div>
</div>

<h2>Astuces</h2>

- Triez les étapes décroissantes avant de les passer (ou utilisez `sort_order="desc"`).
- Utilisez `"conversion"` quand l’audience s’intéresse au taux de rétention entre étapes.
- La variante `"pyramid"` fonctionne mieux avec des valeurs en forte décroissance.
- Pour un public large, `"chevron"` se lit plus naturellement comme un pipeline commercial qu’un trapèze.

<h2>Voir aussi</h2>

- [Bar](bar.md) — comparaisons simples
- [Waterfall](waterfall.md) — contributions additives
- [Sankey](sankey.md) — flux multi-étapes avec branchements

</div>
