# Joint — Bivariate Panel + Marginals

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
.sp-preview-frame{border:none;border-radius:10px;display:block;width:100%;height:560px;background:transparent}
</style>
<script>
function spCls(scope,name,btn){var root=document.getElementById(scope);root.querySelectorAll('.sp-variant').forEach(function(s){s.classList.remove('sp-von')});root.querySelectorAll('.sp-cls-tab').forEach(function(b){b.classList.remove('sp-cact')});document.getElementById(scope+'-'+name).classList.add('sp-von');btn.classList.add('sp-cact')}
function spClsTog(id){document.getElementById(id).classList.toggle('sp-open')}
</script>

## Signature

`sp.joint(title, x, y, *, variant="hexbin", marginal="histogram", panel_variant="", marginal_variant="", bins=24, colorscale=None, categories=None, **kwargs) -> Chart`

Aliases: `sp.joint`, `sp.jointplot`, `sp.joint_plot`, `sp.bivariate`, `sp.build_joint`

## Description

`sp.joint()` composes a bivariate main panel with top and right marginal strips — the SeraPlot equivalent of seaborn's `jointplot()` / `JointGrid`. `variant=` (the panel, "inside") and `marginal=` (the top/right strips, "outside") are **independent, fully open choices** — not a fixed enum. Each accepts the name of *any* registered SeraPlot family from `services/plot/statistical/chart_registry.rs` (currently ~50: `hexbin`, `scatter`, `kde`, `histogram`, `bar`, `violin`, `boxplot`, `heatmap`, `orbita`, `splom`, … the same inventory [`facet()`](facet.md) dispatches through), and `panel_variant=` / `marginal_variant=` forward straight through as that family's own `variant=`, so hexbin's `outlined`/`spaced`/`highlight` styles, histogram's `cumulative`, etc. all work exactly as they do standalone.

```python
sp.joint(x, y, variant="hexbin", marginal="kde")
sp.joint(x, y, variant="kde", marginal="histogram")
sp.joint(x, y, variant="scatter", marginal="bar")
sp.joint(x, y, variant="hexbin", panel_variant="outlined", marginal="kde")
```

Each region — panel, top strip, right strip — is a real, independently-rendered SeraPlot chart embedded in its own frame, calling the exact same native `build_*` function [`facet()`](facet.md) uses for its cells; nothing is reimplemented. That also means regions are **not pixel-aligned** to a shared coordinate system the way a hand-tuned single-SVG composition would be — each chart keeps its own axes/padding — an inherent tradeoff for genuine "any family, any slot" freedom. Not every family produces something meaningful in every slot (e.g. `orbita`, which expects a hierarchy rather than raw points, renders blank as a panel) — that limitation belongs to the target family's own data shape, not to `joint()` itself.

Pass `categories=` alongside `x` / `y` to compare density surfaces across groups with `variant="kde_contour"`, the same `categories=` convention used by [`kde()`](kde.md) and [`histogram()`](histogram.md).

The 8 preset names from earlier releases (`hexbin_marginal`, `heat_scatter` / `joint_histogram` / `histogram2d`, `layered_bivariate`, `joint_kde`, `kde_smooth`, `multiple_bivariate_kde`, `marginal_ticks`, `regression_marginals`) still work as `variant=` values and resolve to a real family under the hood (`resolve_legacy_panel()` in `joint/variant.rs`), so existing code keeps working.

## Data

`x` (`list[float]`) — X coordinates. `y` (`list[float]`) — Y coordinates. `categories` (`list[str]`, optional) — group label per point, for `variant="kde_contour"`.

## Parameters

<div data-sp-registry-table="options" data-family="joint"></div>

## Returns

`Chart` — object with `.html` property and `.show()` method.

<div class="sp-cls sp-open" id="joint-en">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('joint-en')" title="Collapse / expand">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('joint-en','hexbin_kde',this)"><span class="sp-cic">⬡</span><span class="sp-clb">hexbin + kde</span></button>
<button class="sp-cls-tab" onclick="spCls('joint-en','kde_histogram',this)"><span class="sp-cic">◐</span><span class="sp-clb">kde + histogram</span></button>
<button class="sp-cls-tab" onclick="spCls('joint-en','scatter_bar',this)"><span class="sp-cic">●</span><span class="sp-clb">scatter + bar</span></button>
<button class="sp-cls-tab" onclick="spCls('joint-en','hexbin_outlined_kde',this)"><span class="sp-cic">✦</span><span class="sp-clb">hexbin outlined + kde</span></button>
<button class="sp-cls-tab" onclick="spCls('joint-en','layered_bivariate',this)"><span class="sp-cic">∿</span><span class="sp-clb">legacy: layered_bivariate</span></button>
</div>
<div class="sp-cls-body">
<div class="sp-variant sp-von" id="joint-en-hexbin_kde">
<p>Hexagonal density panel with KDE curve marginals instead of the default histograms.</p>
<div class="sp-vmeta"><span><strong>Call</strong> <code>sp.joint(x, y, variant="hexbin", marginal="kde")</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/joint-hexbin_kde.html"></iframe>
</div>
<div class="sp-variant" id="joint-en-kde_histogram">
<p>A 1D KDE panel with histogram marginals — mixing families freely, not just bivariate-native ones.</p>
<div class="sp-vmeta"><span><strong>Call</strong> <code>sp.joint(x, y, variant="kde", marginal="histogram")</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/joint-kde_histogram.html"></iframe>
</div>
<div class="sp-variant" id="joint-en-scatter_bar">
<p>Scatter panel with bar-chart marginals.</p>
<div class="sp-vmeta"><span><strong>Call</strong> <code>sp.joint(x, y, variant="scatter", marginal="bar")</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/joint-scatter_bar.html"></iframe>
</div>
<div class="sp-variant" id="joint-en-hexbin_outlined_kde">
<p><code>panel_variant=</code> forwards to the panel family's own variant — here hexbin's <code>outlined</code> cell style, combined with KDE marginals.</p>
<div class="sp-vmeta"><span><strong>Call</strong> <code>sp.joint(x, y, variant="hexbin", panel_variant="outlined", marginal="kde")</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/joint-hexbin_outlined_kde.html"></iframe>
</div>
<div class="sp-variant" id="joint-en-layered_bivariate">
<p>The pre-existing <code>layered_bivariate</code> preset name still resolves (to <code>variant="kde"</code> under the hood) — old code keeps working.</p>
<div class="sp-vmeta"><span><strong>Call</strong> <code>sp.joint(x, y, variant="layered_bivariate")</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/joint-layered_bivariate.html"></iframe>
</div>
</div>
</div>

</div>

<div class="lang-fr" style="display:none">

## Signature

`sp.joint(title, x, y, *, variant="hexbin", marginal="histogram", panel_variant="", marginal_variant="", bins=24, colorscale=None, categories=None, **kwargs) -> Chart`

Alias : `sp.joint`, `sp.jointplot`, `sp.joint_plot`, `sp.bivariate`, `sp.build_joint`

## Description

`sp.joint()` compose un panneau bivarié principal avec des bandes marginales en haut et à droite — l'équivalent SeraPlot de `jointplot()` / `JointGrid` de seaborn. `variant=` (le panneau, « intérieur ») et `marginal=` (les bandes haut/droite, « extérieur ») sont des choix **indépendants et totalement ouverts** — pas une énumération figée. Chacun accepte le nom de *n'importe quelle* famille SeraPlot enregistrée dans `services/plot/statistical/chart_registry.rs` (environ 50 actuellement : `hexbin`, `scatter`, `kde`, `histogram`, `bar`, `violin`, `boxplot`, `heatmap`, `orbita`, `splom`, … le même inventaire que celui utilisé par [`facet()`](facet.md)), et `panel_variant=` / `marginal_variant=` sont transmis tels quels comme le `variant=` propre à cette famille.

```python
sp.joint(x, y, variant="hexbin", marginal="kde")
sp.joint(x, y, variant="kde", marginal="histogram")
sp.joint(x, y, variant="scatter", marginal="bar")
sp.joint(x, y, variant="hexbin", panel_variant="outlined", marginal="kde")
```

Chaque région — panneau, bande haute, bande droite — est un vrai graphique SeraPlot rendu indépendamment dans son propre cadre, appelant exactement la même fonction `build_*` native que [`facet()`](facet.md) pour ses cellules ; rien n'est réimplémenté. Cela signifie aussi que les régions ne sont **pas alignées pixel par pixel** sur un système de coordonnées partagé comme le serait une composition SVG unique ajustée à la main — chaque graphique garde ses propres axes/marges — une contrepartie inhérente à une liberté « n'importe quelle famille, n'importe quel emplacement » authentique. Toutes les familles ne produisent pas quelque chose de pertinent dans tous les emplacements (par exemple `orbita`, qui attend une hiérarchie plutôt que des points bruts, reste vide en tant que panneau) — cette limite appartient à la forme de données propre à la famille cible, pas à `joint()` lui-même.

Passez `categories=` en plus de `x` / `y` pour comparer des surfaces de densité entre groupes avec `variant="kde_contour"`, la même convention `categories=` que [`kde()`](kde.md) et [`histogram()`](histogram.md).

Les 8 noms préréglés des versions précédentes (`hexbin_marginal`, `heat_scatter` / `joint_histogram` / `histogram2d`, `layered_bivariate`, `joint_kde`, `kde_smooth`, `multiple_bivariate_kde`, `marginal_ticks`, `regression_marginals`) fonctionnent toujours comme valeurs de `variant=` et se résolvent vers une vraie famille en interne (`resolve_legacy_panel()` dans `joint/variant.rs`), donc le code existant continue de fonctionner.

## Données

`x` (`list[float]`) — Coordonnées X. `y` (`list[float]`) — Coordonnées Y. `categories` (`list[str]`, optionnel) — étiquette de groupe par point, pour `variant="kde_contour"`.

## Paramètres

<div data-sp-registry-table="options" data-family="joint"></div>

## Retour

`Chart` — objet avec une propriété `.html` et une méthode `.show()`.

<div class="sp-cls sp-open" id="joint-fr">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('joint-fr')" title="Collapse / expand">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('joint-fr','hexbin_kde',this)"><span class="sp-cic">⬡</span><span class="sp-clb">hexbin + kde</span></button>
<button class="sp-cls-tab" onclick="spCls('joint-fr','kde_histogram',this)"><span class="sp-cic">◐</span><span class="sp-clb">kde + histogramme</span></button>
<button class="sp-cls-tab" onclick="spCls('joint-fr','scatter_bar',this)"><span class="sp-cic">●</span><span class="sp-clb">scatter + bar</span></button>
<button class="sp-cls-tab" onclick="spCls('joint-fr','hexbin_outlined_kde',this)"><span class="sp-cic">✦</span><span class="sp-clb">hexbin outlined + kde</span></button>
<button class="sp-cls-tab" onclick="spCls('joint-fr','layered_bivariate',this)"><span class="sp-cic">∿</span><span class="sp-clb">héritage : layered_bivariate</span></button>
</div>
<div class="sp-cls-body">
<div class="sp-variant sp-von" id="joint-fr-hexbin_kde">
<p>Panneau de densité hexagonale avec des marges en courbes de KDE plutôt qu'en histogrammes par défaut.</p>
<div class="sp-vmeta"><span><strong>Appel</strong> <code>sp.joint(x, y, variant="hexbin", marginal="kde")</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/joint-hexbin_kde.html"></iframe>
</div>
<div class="sp-variant" id="joint-fr-kde_histogram">
<p>Un panneau de KDE 1D avec des marges en histogrammes — mélange libre de familles, pas seulement celles nativement bivariées.</p>
<div class="sp-vmeta"><span><strong>Appel</strong> <code>sp.joint(x, y, variant="kde", marginal="histogram")</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/joint-kde_histogram.html"></iframe>
</div>
<div class="sp-variant" id="joint-fr-scatter_bar">
<p>Panneau en nuage de points avec des marges en bar chart.</p>
<div class="sp-vmeta"><span><strong>Appel</strong> <code>sp.joint(x, y, variant="scatter", marginal="bar")</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/joint-scatter_bar.html"></iframe>
</div>
<div class="sp-variant" id="joint-fr-hexbin_outlined_kde">
<p><code>panel_variant=</code> est transmis à la variante propre de la famille du panneau — ici le style de cellule <code>outlined</code> de hexbin, combiné à des marges KDE.</p>
<div class="sp-vmeta"><span><strong>Appel</strong> <code>sp.joint(x, y, variant="hexbin", panel_variant="outlined", marginal="kde")</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/joint-hexbin_outlined_kde.html"></iframe>
</div>
<div class="sp-variant" id="joint-fr-layered_bivariate">
<p>L'ancien nom préréglé <code>layered_bivariate</code> se résout toujours (vers <code>variant="kde"</code> en interne) — le code existant continue de fonctionner.</p>
<div class="sp-vmeta"><span><strong>Appel</strong> <code>sp.joint(x, y, variant="layered_bivariate")</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/joint-layered_bivariate.html"></iframe>
</div>
</div>
</div>

</div>
