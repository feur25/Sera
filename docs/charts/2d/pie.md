# Pie Charts

<div class="lang-en">

<style>
.sp-preview-frame{width:100%;height:380px;border:none;border-radius:10px;display:block;background:#0d1117;margin-top:10px;box-shadow:0 8px 24px -8px rgba(0,0,0,.5)}
</style>
<script>
function spTab(g,id,btn){var r=document.getElementById(g);r.querySelectorAll('.sp-tc').forEach(function(e){e.classList.remove('sp-on')});r.querySelectorAll('.sp-tb').forEach(function(b){b.classList.remove('sp-act')});document.getElementById(id).classList.add('sp-on');btn.classList.add('sp-act');if(window.hljs)document.getElementById(id).querySelectorAll('code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})}
function spCls(scope,name,btn){var root=document.getElementById(scope);root.querySelectorAll('.sp-variant').forEach(function(s){s.classList.remove('sp-von')});root.querySelectorAll('.sp-cls-tab').forEach(function(b){b.classList.remove('sp-cact')});document.getElementById(scope+'-'+name).classList.add('sp-von');btn.classList.add('sp-cact');if(window.hljs)document.getElementById(scope+'-'+name).querySelectorAll('code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})}
function spClsTog(id){document.getElementById(id).classList.toggle('sp-open')}
document.addEventListener('DOMContentLoaded',function(){if(window.hljs)document.querySelectorAll('.sp-tc.sp-on code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})});
</script>

## Signature

`sp.pie(title, labels=None, values=None, *, variant="basic", series=None, **kwargs) -> Chart`

## Description

`sp.pie()` is the unified entry point for the entire pie-chart family. The `variant` keyword selects the rendering strategy — all other arguments remain consistent across variants.
## Variants

<div data-sp-registry-table="variants" data-family="pie"></div>

## Parameters

<div data-sp-registry-table="options" data-family="pie"></div>

## Floating labels

Pass `labeled=True` to any variant built on the shared angle-sweep engine (`basic`, `donut`, `exploded`, `kpi`, `pattern`, `semi`) to replace the in-wedge percentage text with an outside callout: a thin connector line from the wedge edge to a label showing the percentage and the category name, colored to match its slice. This is the same style used by the reference "total breakdown" donut screenshots — it is not a separate variant, it is a display option any of those variants can turn on.

```python
import seraplot as sp

c = sp.pie(
    labels=["Acquisition", "Conversion", "Retention", "Referral", "Other"],
    values=[42, 26, 16, 11, 5],
    variant="donut",
    center_text="27.3K",
    center_subtext="TOTAL",
    labeled=True,
)
```

---

## Returns

`Chart` — object with `.html` property and `.show()` method.

---

<div class="sp-cls sp-open" id="pie-en">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('pie-en')" title="Collapse / expand">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('pie-en','basic',this)"><span class="sp-cic">●</span><span class="sp-clb">Basic</span></button>
<button class="sp-cls-tab" onclick="spCls('pie-en','donut',this)"><span class="sp-cic">◍</span><span class="sp-clb">Donut</span></button>
<button class="sp-cls-tab" onclick="spCls('pie-en','exploded',this)"><span class="sp-cic">◐</span><span class="sp-clb">Exploded</span></button>
<button class="sp-cls-tab" onclick="spCls('pie-en','subplots',this)"><span class="sp-cic">⊞</span><span class="sp-clb">Subplots</span></button>
<button class="sp-cls-tab" onclick="spCls('pie-en','proportional',this)"><span class="sp-cic">◔</span><span class="sp-clb">Proportional</span></button>
<button class="sp-cls-tab" onclick="spCls('pie-en','semi',this)"><span class="sp-cic">◗</span><span class="sp-clb">Semi</span></button>
<button class="sp-cls-tab" onclick="spCls('pie-en','kpi',this)"><span class="sp-cic">◉</span><span class="sp-clb">KPI</span></button>
<button class="sp-cls-tab" onclick="spCls('pie-en','nested',this)"><span class="sp-cic">◎</span><span class="sp-clb">Nested</span></button>
<button class="sp-cls-tab" onclick="spCls('pie-en','pattern',this)"><span class="sp-cic">▦</span><span class="sp-clb">Pattern</span></button>
<button class="sp-cls-tab" onclick="spCls('pie-en','waffle',this)"><span class="sp-cic">▧</span><span class="sp-clb">Waffle</span></button>
</div>
<div class="sp-cls-body">

<div class="sp-variant sp-von" id="pie-en-basic">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"basic"</code> / <code>"pie"</code></span><span><strong>Required</strong> <code>labels</code>, <code>values</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" data-src="../../previews/pie-basic.html"></iframe>
</div>

<div class="sp-variant" id="pie-en-donut">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"donut"</code> / <code>"ring"</code> / <code>"hole"</code></span><span><strong>Required</strong> <code>labels</code>, <code>values</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" data-src="../../previews/pie-donut.html"></iframe>
</div>

<div class="sp-variant" id="pie-en-exploded">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"exploded"</code> / <code>"pulled"</code> / <code>"explode"</code></span><span><strong>Required</strong> <code>labels</code>, <code>values</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" data-src="../../previews/pie-exploded.html"></iframe>
</div>

<div class="sp-variant" id="pie-en-subplots">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"subplots"</code> / <code>"grid"</code> / <code>"facet"</code> / <code>"multi"</code></span><span><strong>Required</strong> <code>labels</code>, <code>series</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" data-src="../../previews/pie-subplots.html"></iframe>
</div>

<div class="sp-variant" id="pie-en-proportional">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"proportional"</code> / <code>"scaled"</code> / <code>"scalegroup"</code></span><span><strong>Required</strong> <code>labels</code>, <code>series</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" data-src="../../previews/pie-proportional.html"></iframe>
</div>

<div class="sp-variant" id="pie-en-semi">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"semi"</code> / <code>"half_pie"</code></span><span><strong>Required</strong> <code>labels</code>, <code>values</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" data-src="../../previews/pie-semi.html"></iframe>
</div>

<div class="sp-variant" id="pie-en-kpi">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"kpi"</code> / <code>"indicator"</code></span><span><strong>Required</strong> <code>labels</code>, <code>values</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" data-src="../../previews/pie-kpi.html"></iframe>
</div>

<div class="sp-variant" id="pie-en-nested">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"nested"</code> / <code>"concentric"</code></span><span><strong>Required</strong> <code>labels</code>, <code>values</code>, <code>secondary_values</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" data-src="../../previews/pie-nested.html"></iframe>
</div>

<div class="sp-variant" id="pie-en-pattern">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"pattern"</code> / <code>"hatched"</code></span><span><strong>Patterns</strong> <code>"stripes"</code>, <code>"dots"</code>, <code>"diagonal"</code>, <code>"cross"</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" data-src="../../previews/pie-pattern.html"></iframe>
</div>

<div class="sp-variant" id="pie-en-waffle">

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"waffle"</code> / <code>"square"</code> / <code>"pie_square"</code></span><span><strong>Required</strong> <code>labels</code>, <code>values</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">The "square pie" — a 10x10 grid of 100 cells filled proportionally to each category's share (largest-remainder allocation, so the cell count always sums to exactly 100), with a color-matched legend. Easier to read precise percentages from than wedge angles.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" data-src="../../previews/pie-waffle.html"></iframe>
</div>

</div>
</div>

</div><!-- /lang-en -->

<div class="lang-fr" style="display:none">

<h2>Signature</h2>

`sp.pie(title, labels=None, values=None, *, variant="basic", **kwargs) -> Chart`

<h2>Description</h2>

`sp.pie()` est le point d'entrée unique pour toute la famille des camemberts. Le paramètre `variant` sélectionne la stratégie de rendu — donut, éclaté, KPI, semi-cercle ou imbriqué — tout en conservant la même API simple.
<h2>Variantes</h2>

<div data-sp-registry-table="variants" data-family="pie"></div>

<h2>Paramètres</h2>

<div data-sp-registry-table="options" data-family="pie"></div>

<h2>Étiquettes flottantes</h2>

Passe `labeled=True` à n'importe quelle variante construite sur le moteur partagé à secteurs angulaires (`basic`, `donut`, `exploded`, `kpi`, `pattern`, `semi`) pour remplacer le texte de pourcentage à l'intérieur du secteur par une étiquette extérieure : une fine ligne de connexion depuis le bord du secteur jusqu'à un label affichant le pourcentage et le nom de catégorie, colorée comme sa part. C'est le même style que les captures "répartition totale" en donut données en référence — ce n'est pas une variante séparée, c'est une option d'affichage que n'importe laquelle de ces variantes peut activer.

```python
import seraplot as sp

c = sp.pie(
    labels=["Acquisition", "Conversion", "Retention", "Referral", "Other"],
    values=[42, 26, 16, 11, 5],
    variant="donut",
    center_text="27.3K",
    center_subtext="TOTAL",
    labeled=True,
)
```

---

<h2>Retourne</h2>

`Chart` — objet avec la propriété `.html` et la méthode `.show()`.

---

<div class="sp-cls sp-open" id="pie-fr">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('pie-fr')" title="Réduire / déplier">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('pie-fr','basic',this)"><span class="sp-cic">●</span><span class="sp-clb">Basique</span></button>
<button class="sp-cls-tab" onclick="spCls('pie-fr','donut',this)"><span class="sp-cic">◯</span><span class="sp-clb">Donut</span></button>
<button class="sp-cls-tab" onclick="spCls('pie-fr','exploded',this)"><span class="sp-cic">✦</span><span class="sp-clb">Éclaté</span></button>
<button class="sp-cls-tab" onclick="spCls('pie-fr','subplots',this)"><span class="sp-cic">▦</span><span class="sp-clb">Sous-graphiques</span></button>
<button class="sp-cls-tab" onclick="spCls('pie-fr','proportional',this)"><span class="sp-cic">◐</span><span class="sp-clb">Proportionnel</span></button>
<button class="sp-cls-tab" onclick="spCls('pie-fr','semi',this)"><span class="sp-cic">◗</span><span class="sp-clb">Semi</span></button>
<button class="sp-cls-tab" onclick="spCls('pie-fr','kpi',this)"><span class="sp-cic">⊙</span><span class="sp-clb">KPI</span></button>
<button class="sp-cls-tab" onclick="spCls('pie-fr','nested',this)"><span class="sp-cic">◎</span><span class="sp-clb">Imbriqué</span></button>
<button class="sp-cls-tab" onclick="spCls('pie-fr','pattern',this)"><span class="sp-cic">▩</span><span class="sp-clb">Motifs</span></button>
<button class="sp-cls-tab" onclick="spCls('pie-fr','waffle',this)"><span class="sp-cic">▧</span><span class="sp-clb">Gaufre</span></button>
</div>
<div class="sp-cls-body">

<div class="sp-variant sp-von" id="pie-fr-basic"><div class="sp-vmeta"><span><strong>Variante</strong> <code>"basic"</code></span><span><strong>Requis</strong> <code>labels</code>, <code>values</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div><div class="sp-preview-label">Aperçu</div><iframe class="sp-preview-frame" data-src="../../previews/pie-basic.html"></iframe></div>

<div class="sp-variant" id="pie-fr-donut"><div class="sp-vmeta"><span><strong>Variante</strong> <code>"donut"</code> / <code>"ring"</code></span><span><strong>Requis</strong> <code>labels</code>, <code>values</code>, <code>inner_radius</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div><div class="sp-preview-label">Aperçu</div><iframe class="sp-preview-frame" data-src="../../previews/pie-donut.html"></iframe></div>

<div class="sp-variant" id="pie-fr-exploded"><div class="sp-vmeta"><span><strong>Variante</strong> <code>"exploded"</code> / <code>"explode"</code></span><span><strong>Requis</strong> <code>labels</code>, <code>values</code>, <code>pull</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div><div class="sp-preview-label">Aperçu</div><iframe class="sp-preview-frame" data-src="../../previews/pie-exploded.html"></iframe></div>

<div class="sp-variant" id="pie-fr-subplots"><div class="sp-vmeta"><span><strong>Variante</strong> <code>"subplots"</code> / <code>"grid"</code></span><span><strong>Requis</strong> <code>panels</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div><div class="sp-preview-label">Aperçu</div><iframe class="sp-preview-frame" data-src="../../previews/pie-subplots.html"></iframe></div>

<div class="sp-variant" id="pie-fr-proportional"><div class="sp-vmeta"><span><strong>Variante</strong> <code>"proportional"</code> / <code>"prop"</code></span><span><strong>Requis</strong> <code>panels</code>, <code>total_basis</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div><div class="sp-preview-label">Aperçu</div><iframe class="sp-preview-frame" data-src="../../previews/pie-proportional.html"></iframe></div>

<div class="sp-variant" id="pie-fr-semi"><div class="sp-vmeta"><span><strong>Variante</strong> <code>"semi"</code> / <code>"half"</code></span><span><strong>Requis</strong> <code>labels</code>, <code>values</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div><div class="sp-preview-label">Aperçu</div><iframe class="sp-preview-frame" data-src="../../previews/pie-semi.html"></iframe></div>

<div class="sp-variant" id="pie-fr-kpi"><div class="sp-vmeta"><span><strong>Variante</strong> <code>"kpi"</code> / <code>"indicator"</code></span><span><strong>Requis</strong> <code>labels</code>, <code>values</code>, <code>center_text</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div><div class="sp-preview-label">Aperçu</div><iframe class="sp-preview-frame" data-src="../../previews/pie-kpi.html"></iframe></div>

<div class="sp-variant" id="pie-fr-nested"><div class="sp-vmeta"><span><strong>Variante</strong> <code>"nested"</code> / <code>"sunburst"</code></span><span><strong>Requis</strong> <code>rings</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div><div class="sp-preview-label">Aperçu</div><iframe class="sp-preview-frame" data-src="../../previews/pie-nested.html"></iframe></div>

<div class="sp-variant" id="pie-fr-pattern"><div class="sp-vmeta"><span><strong>Variante</strong> <code>"pattern"</code> / <code>"hatch"</code></span><span><strong>Requis</strong> <code>labels</code>, <code>values</code>, <code>patterns</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div><div class="sp-preview-label">Aperçu</div><iframe class="sp-preview-frame" data-src="../../previews/pie-pattern.html"></iframe></div>

<div class="sp-variant" id="pie-fr-waffle"><div class="sp-vmeta"><span><strong>Variante</strong> <code>"waffle"</code> / <code>"square"</code> / <code>"pie_square"</code></span><span><strong>Requis</strong> <code>labels</code>, <code>values</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div><p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Le « pie carré » — une grille 10x10 de 100 cellules remplies proportionnellement à la part de chaque catégorie (allocation par plus grand reste, la somme des cellules vaut donc toujours exactement 100), avec une légende assortie aux couleurs. Plus facile à lire précisément que des angles de camembert.</p><div class="sp-preview-label">Aperçu</div><iframe class="sp-preview-frame" data-src="../../previews/pie-waffle.html"></iframe></div>

</div>
</div>

</div><!-- /lang-fr -->
