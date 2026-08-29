# Bubble Map

<div class="lang-en">

<style>
.sp-preview-frame{width:100%;height:420px;border:none;border-radius:10px;display:block;background:#0d1117;margin-top:10px;box-shadow:0 8px 24px -8px rgba(0,0,0,.5)}
</style>
<script>
function spTab(g,id,btn){var r=document.getElementById(g);r.querySelectorAll('.sp-tc').forEach(function(e){e.classList.remove('sp-on')});r.querySelectorAll('.sp-tb').forEach(function(b){b.classList.remove('sp-act')});document.getElementById(id).classList.add('sp-on');btn.classList.add('sp-act');if(window.hljs)document.getElementById(id).querySelectorAll('code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})}
function spCls(scope,name,btn){var root=document.getElementById(scope);root.querySelectorAll('.sp-variant').forEach(function(s){s.classList.remove('sp-von')});root.querySelectorAll('.sp-cls-tab').forEach(function(b){b.classList.remove('sp-cact')});document.getElementById(scope+'-'+name).classList.add('sp-von');btn.classList.add('sp-cact');if(window.hljs)document.getElementById(scope+'-'+name).querySelectorAll('code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})}
function spClsTog(id){document.getElementById(id).classList.toggle('sp-open')}
document.addEventListener('DOMContentLoaded',function(){if(window.hljs)document.querySelectorAll('.sp-tc.sp-on code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})});
</script>

## Signature

`sp.bubble_map(title, labels=None, values=None, *, variant="filled", map="world", region=None, center_lat=None, center_lon=None, **kwargs) -> Chart`

Aliases: `sp.bubble_map`, `sp.bubblemap`, `sp.bubble_map_chart`, `sp.geo_bubble`, `sp.geo_bubble_map`, `sp.build_bubble_map`

## Description

`sp.bubble_map()` marks matched regions on a map. `variant` selects whether the whole region gets filled by category or a proportionally-sized circle marks its centroid; `map` selects which registered geographic region set to draw; `region` optionally restricts drawing to one named group inside that set. All three are read live from the same register/inventory system the rest of the framework uses — nothing below is hardcoded.

## Variants

<div data-sp-registry-table="variants" data-family="bubble_map"></div>

## Region Sets

<div data-sp-registry-table="regions"></div>

## Parameters

<div data-sp-registry-table="options" data-family="bubble_map"></div>

---

## Returns

`Chart` — object with `.html` property and `.show()` method.

---

<div class="sp-cls sp-open" id="bubblemap-en">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('bubblemap-en')" title="Collapse / expand">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('bubblemap-en','filled',this)"><span class="sp-cic">▧</span><span class="sp-clb">Filled</span></button>
<button class="sp-cls-tab" onclick="spCls('bubblemap-en','proportional',this)"><span class="sp-cic">●</span><span class="sp-clb">Proportional</span></button>
<button class="sp-cls-tab" onclick="spCls('bubblemap-en','globe',this)"><span class="sp-cic">◉</span><span class="sp-clb">Globe</span></button>
</div>
<div class="sp-cls-body">

<div class="sp-variant sp-von" id="bubblemap-en-filled">

Each matched region filled solid with its own palette color and labeled at its centroid — reads as a categorical highlight map rather than true bubbles, useful when the regions themselves are what the reader should compare.

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"filled"</code></span><span><strong>Aliases</strong> <code>"filled"</code> / <code>"basic"</code> / <code>"default"</code> / <code>"regions"</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" data-src="../../previews/bubble-map-filled.html"></iframe>
</div>

<div class="sp-variant" id="bubblemap-en-proportional">

The real graduated-symbol map: the base is muted, and a circle sits on each matched region's centroid, its radius scaled by `sqrt(value)` so *area* — not radius — reads proportionally to magnitude. `min_bubble_size` / `max_bubble_size` set the radius range in pixels.

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"proportional"</code></span><span><strong>Aliases</strong> <code>"proportional"</code> / <code>"bubble"</code> / <code>"graduated"</code> / <code>"sized"</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" data-src="../../previews/bubble-map-proportional.html"></iframe>
</div>

<div class="sp-variant" id="bubblemap-en-globe">

Proportional circles projected onto an orthographic sphere — every centroid is converted to real latitude/longitude and re-projected with actual globe math, so bubbles on the far side simply aren't drawn. `center_lat` / `center_lon` aim the view (default `15, 10`). Only region sets that expose a coordinate inversion support this — today, `"world"`.

<div class="sp-vmeta"><span><strong>Variant</strong> <code>"globe"</code></span><span><strong>Aliases</strong> <code>"globe"</code> / <code>"orthographic"</code> / <code>"sphere"</code> / <code>"space"</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" data-src="../../previews/bubble-map-globe.html"></iframe>
</div>

</div>
</div>

### Custom coordinates

```python
import seraplot as sp

chart = sp.bubble_map(
    "City Populations",
    labels=["Paris", "Tokyo", "New York", "Lagos"],
    values=[11, 37, 20, 15],
    latitudes=[48.85, 35.68, 40.71, 6.52],
    longitudes=[2.35, 139.69, -74.01, 3.38],
)
```

### Reading one region group

```python
import seraplot as sp

chart = sp.bubble_map(
    "Northeast Metro Populations (millions)",
    labels=["NY", "PA", "MA", "NJ", "CT"],
    values=[19.6, 12.9, 7.0, 9.3, 3.6],
    map="usa_states",
    region="Northeast",
    variant="proportional",
)
```

`region` accepts any group name a `map`'s own registered set exposes — continents for `"world"`, Census regions for `"usa_states"` (`Northeast`, `Midwest`, `South`, `West`).

</div>

<div class="lang-fr" style="display:none">

## Signature

`sp.bubble_map(title, labels=None, values=None, *, variant="filled", map="world", region=None, center_lat=None, center_lon=None, **kwargs) -> Chart`

Alias : `sp.bubble_map`, `sp.bubblemap`, `sp.bubble_map_chart`, `sp.geo_bubble`, `sp.geo_bubble_map`, `sp.build_bubble_map`

## Description

`sp.bubble_map()` repère les régions correspondantes sur une carte. `variant` sélectionne si la région entière est remplie par catégorie ou si un cercle de taille proportionnelle marque son centroïde ; `map` sélectionne l'ensemble de régions géographiques enregistré à dessiner ; `region` restreint optionnellement le dessin à un seul groupe nommé de cet ensemble. Les trois sont lus en direct depuis le même système de register/inventory utilisé dans tout le framework — rien ci-dessous n'est codé en dur.

## Variantes

<div data-sp-registry-table="variants" data-family="bubble_map"></div>

## Ensembles de régions

<div data-sp-registry-table="regions"></div>

## Paramètres

<div data-sp-registry-table="options" data-family="bubble_map"></div>

---

## Retourne

`Chart` — objet avec une propriété `.html` et une méthode `.show()`.

---

<div class="sp-cls sp-open" id="bubblemap-fr">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('bubblemap-fr')" title="Réduire / déplier">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('bubblemap-fr','filled',this)"><span class="sp-cic">▧</span><span class="sp-clb">Rempli</span></button>
<button class="sp-cls-tab" onclick="spCls('bubblemap-fr','proportional',this)"><span class="sp-cic">●</span><span class="sp-clb">Proportionnel</span></button>
<button class="sp-cls-tab" onclick="spCls('bubblemap-fr','globe',this)"><span class="sp-cic">◉</span><span class="sp-clb">Globe</span></button>
</div>
<div class="sp-cls-body">

<div class="sp-variant sp-von" id="bubblemap-fr-filled">

Chaque région correspondante remplie en aplat de sa propre couleur de palette et étiquetée à son centroïde — se lit comme une carte de mise en évidence catégorielle plutôt que de vraies bulles, utile quand ce sont les régions elles-mêmes que le lecteur doit comparer.

<div class="sp-vmeta"><span><strong>Variante</strong> <code>"filled"</code></span><span><strong>Alias</strong> <code>"filled"</code> / <code>"basic"</code> / <code>"default"</code> / <code>"regions"</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" data-src="../../previews/bubble-map-filled.html"></iframe>
</div>

<div class="sp-variant" id="bubblemap-fr-proportional">

La vraie carte à symboles proportionnels : le fond est estompé, et un cercle se place sur le centroïde de chaque région correspondante, son rayon mis à l'échelle par `sqrt(valeur)` pour que ce soit l'*aire* — pas le rayon — qui soit proportionnelle à la magnitude. `min_bubble_size` / `max_bubble_size` règlent la plage de rayon en pixels.

<div class="sp-vmeta"><span><strong>Variante</strong> <code>"proportional"</code></span><span><strong>Alias</strong> <code>"proportional"</code> / <code>"bubble"</code> / <code>"graduated"</code> / <code>"sized"</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" data-src="../../previews/bubble-map-proportional.html"></iframe>
</div>

<div class="sp-variant" id="bubblemap-fr-globe">

Cercles proportionnels projetés sur une sphère orthographique — chaque centroïde est converti en vraie latitude/longitude puis reprojeté avec une véritable trigonométrie de globe, si bien que les bulles de la face cachée ne sont tout simplement pas dessinées. `center_lat` / `center_lon` orientent la vue (par défaut `15, 10`). Seuls les ensembles de régions exposant une inversion de coordonnées supportent ceci — aujourd'hui, `"world"`.

<div class="sp-vmeta"><span><strong>Variante</strong> <code>"globe"</code></span><span><strong>Alias</strong> <code>"globe"</code> / <code>"orthographic"</code> / <code>"sphere"</code> / <code>"space"</code></span><span><strong>Retourne</strong> <code>Chart</code></span></div>

<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" data-src="../../previews/bubble-map-globe.html"></iframe>
</div>

</div>
</div>

### Coordonnées personnalisées

```python
import seraplot as sp

chart = sp.bubble_map(
    "Populations urbaines",
    labels=["Paris", "Tokyo", "New York", "Lagos"],
    values=[11, 37, 20, 15],
    latitudes=[48.85, 35.68, 40.71, 6.52],
    longitudes=[2.35, 139.69, -74.01, 3.38],
)
```

### Lire un seul groupe de régions

```python
import seraplot as sp

chart = sp.bubble_map(
    "Populations métropolitaines du Nord-Est (millions)",
    labels=["NY", "PA", "MA", "NJ", "CT"],
    values=[19.6, 12.9, 7.0, 9.3, 3.6],
    map="usa_states",
    region="Northeast",
    variant="proportional",
)
```

`region` accepte tout nom de groupe exposé par l'ensemble propre à `map` — les continents pour `"world"`, les régions de recensement pour `"usa_states"` (`Northeast`, `Midwest`, `South`, `West`).

</div>
