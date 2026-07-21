# Candlestick — OHLC Time Series

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

`sp.candlestick(title, labels, open, high, low, close, *, variant="basic", palette=None, **kwargs) -> Chart`

## Description

`sp.candlestick()` is the unified entry point for the entire candlestick-chart family. A candlestick chart shows OHLC (Open, High, Low, Close) bars over time and is the de facto standard for financial markets, crypto, commodities, energy spot prices and any time-series with intra-period spread. The `variant` keyword switches the visual style without touching the data — including derived views like Heikin-Ashi smoothing, close-only line, mountain area and high-low range bars.

> **Color convention** — by default green = up (`close >= open`) and red = down. Override with `palette=[up_color, down_color]`. Bars are rendered left-to-right in input order; use `sort_order="asc"` to sort by close price.

## Variants

<div data-sp-registry-table="variants" data-family="candlestick"></div>

## Parameters

<div data-sp-registry-table="options" data-family="candlestick"></div>

## Returns

`Chart` — object with `.html` property and `.show()` method.

---

<div class="sp-cls sp-open" id="candlestick-en">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('candlestick-en')" title="Collapse / expand">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('candlestick-en','basic',this)"><span class="sp-cic">B</span><span class="sp-clb">Basic</span></button>
<button class="sp-cls-tab" onclick="spCls('candlestick-en','hollow',this)"><span class="sp-cic">H</span><span class="sp-clb">Hollow</span></button>
<button class="sp-cls-tab" onclick="spCls('candlestick-en','ohlc',this)"><span class="sp-cic">O</span><span class="sp-clb">OHLC</span></button>
<button class="sp-cls-tab" onclick="spCls('candlestick-en','heikin',this)"><span class="sp-cic">K</span><span class="sp-clb">Heikin-Ashi</span></button>
<button class="sp-cls-tab" onclick="spCls('candlestick-en','outlined',this)"><span class="sp-cic">L</span><span class="sp-clb">Outlined</span></button>
<button class="sp-cls-tab" onclick="spCls('candlestick-en','line',this)"><span class="sp-cic">I</span><span class="sp-clb">Line</span></button>
<button class="sp-cls-tab" onclick="spCls('candlestick-en','mountain',this)"><span class="sp-cic">M</span><span class="sp-clb">Mountain</span></button>
<button class="sp-cls-tab" onclick="spCls('candlestick-en','range',this)"><span class="sp-cic">R</span><span class="sp-clb">Range</span></button>
</div>
<div class="sp-cls-body">

<div class="sp-variant sp-von" id="candlestick-en-basic">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"basic"</code></span><span><strong>Aliases</strong> <code>basic / default / classic / filled</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Classic OHLC candles: solid body, thin wick, green for up bars and red for down bars.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/candlestick-basic.html"></iframe>
</div>

<div class="sp-variant" id="candlestick-en-hollow">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"hollow"</code></span><span><strong>Aliases</strong> <code>hollow / empty / japanese / white_up</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Japanese-style hollow up candles (white fill + colored stroke) and filled down candles.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/candlestick-hollow.html"></iframe>
</div>

<div class="sp-variant" id="candlestick-en-ohlc">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"ohlc"</code></span><span><strong>Aliases</strong> <code>ohlc / western / bar / tick</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Western OHLC bars: vertical wick with left tick = open, right tick = close, no body.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/candlestick-ohlc.html"></iframe>
</div>

<div class="sp-variant" id="candlestick-en-heikin">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"heikin"</code></span><span><strong>Aliases</strong> <code>heikin / heikin_ashi / ha / smoothed</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Heikin-Ashi smoothed candles: filters market noise to highlight trends and reversals clearly.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/candlestick-heikin.html"></iframe>
</div>

<div class="sp-variant" id="candlestick-en-outlined">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"outlined"</code></span><span><strong>Aliases</strong> <code>outlined / outline / stroke / wireframe</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Wireframe candles: translucent body with bold colored stroke; lighter visual footprint.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/candlestick-outlined.html"></iframe>
</div>

<div class="sp-variant" id="candlestick-en-line">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"line"</code></span><span><strong>Aliases</strong> <code>line / close / lineplot / trend</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Close-price line chart with markers — same data, smoother trend reading without OHLC noise.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/candlestick-line.html"></iframe>
</div>

<div class="sp-variant" id="candlestick-en-mountain">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"mountain"</code></span><span><strong>Aliases</strong> <code>mountain / area / filled_area / shade</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Close-price area chart with vertical gradient under the line; great for hero / cover charts.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/candlestick-mountain.html"></iframe>
</div>

<div class="sp-variant" id="candlestick-en-range">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"range"</code></span><span><strong>Aliases</strong> <code>range / hl / highlow / spread</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">High-low range bars only (no open/close), single color — pure volatility visualization.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/candlestick-range.html"></iframe>
</div>

</div>
</div>

</div>

<div class="lang-fr">

<h2>Signature</h2>

`sp.candlestick(title, labels, open, high, low, close, *, variant="basic", palette=None, **kwargs) -> Chart`

<h2>Description</h2>

`sp.candlestick()` est le point d entree unifie pour toute la famille des chandeliers. Un graphique en chandeliers affiche des barres OHLC (Ouverture, Haut, Bas, Cloture) dans le temps et constitue le standard de fait pour les marches financiers, la crypto, les matieres premieres, le spot energie et toute serie temporelle avec spread intra-periode. Le mot-cle `variant` change le style sans toucher aux donnees — y compris des vues derivees comme le lissage Heikin-Ashi, la ligne de cloture, l aire mountain et les barres haut-bas.

> **Convention de couleur** — par defaut vert = hausse (`close >= open`) et rouge = baisse. Surchargez avec `palette=[couleur_hausse, couleur_baisse]`. Les barres sont rendues de gauche a droite dans l ordre d entree ; `sort_order="asc"` pour trier par prix de cloture.

<h2>Variantes</h2>

<div data-sp-registry-table="variants" data-family="candlestick"></div>

<h2>Paramètres</h2>

<div data-sp-registry-table="options" data-family="candlestick"></div>

<h2>Retour</h2>

`Chart` — objet avec une propriete `.html` et une methode `.show()`.

---

<div class="sp-cls sp-open" id="candlestick-fr">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('candlestick-fr')" title="Collapse / expand">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('candlestick-fr','basic',this)"><span class="sp-cic">B</span><span class="sp-clb">Basic</span></button>
<button class="sp-cls-tab" onclick="spCls('candlestick-fr','hollow',this)"><span class="sp-cic">H</span><span class="sp-clb">Hollow</span></button>
<button class="sp-cls-tab" onclick="spCls('candlestick-fr','ohlc',this)"><span class="sp-cic">O</span><span class="sp-clb">OHLC</span></button>
<button class="sp-cls-tab" onclick="spCls('candlestick-fr','heikin',this)"><span class="sp-cic">K</span><span class="sp-clb">Heikin-Ashi</span></button>
<button class="sp-cls-tab" onclick="spCls('candlestick-fr','outlined',this)"><span class="sp-cic">L</span><span class="sp-clb">Outlined</span></button>
<button class="sp-cls-tab" onclick="spCls('candlestick-fr','line',this)"><span class="sp-cic">I</span><span class="sp-clb">Line</span></button>
<button class="sp-cls-tab" onclick="spCls('candlestick-fr','mountain',this)"><span class="sp-cic">M</span><span class="sp-clb">Mountain</span></button>
<button class="sp-cls-tab" onclick="spCls('candlestick-fr','range',this)"><span class="sp-cic">R</span><span class="sp-clb">Range</span></button>
</div>
<div class="sp-cls-body">

<div class="sp-variant sp-von" id="candlestick-fr-basic">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"basic"</code></span><span><strong>Aliases</strong> <code>basic / default / classic / filled</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Bougies OHLC classiques : corps plein, meche fine, vert pour hausse et rouge pour baisse.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/candlestick-basic.html"></iframe>
</div>

<div class="sp-variant" id="candlestick-fr-hollow">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"hollow"</code></span><span><strong>Aliases</strong> <code>hollow / empty / japanese / white_up</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Style japonais : bougies haussieres creuses (fond blanc + contour colore) et baissieres pleines.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/candlestick-hollow.html"></iframe>
</div>

<div class="sp-variant" id="candlestick-fr-ohlc">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"ohlc"</code></span><span><strong>Aliases</strong> <code>ohlc / western / bar / tick</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Barres OHLC americaines : meche verticale, tick gauche = ouverture, droit = cloture, sans corps.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/candlestick-ohlc.html"></iframe>
</div>

<div class="sp-variant" id="candlestick-fr-heikin">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"heikin"</code></span><span><strong>Aliases</strong> <code>heikin / heikin_ashi / ha / smoothed</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Bougies Heikin-Ashi lissees : filtre le bruit pour mettre en evidence tendances et retournements.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/candlestick-heikin.html"></iframe>
</div>

<div class="sp-variant" id="candlestick-fr-outlined">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"outlined"</code></span><span><strong>Aliases</strong> <code>outlined / outline / stroke / wireframe</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Bougies en fil de fer : corps translucide avec contour colore epais ; rendu plus aere.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/candlestick-outlined.html"></iframe>
</div>

<div class="sp-variant" id="candlestick-fr-line">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"line"</code></span><span><strong>Aliases</strong> <code>line / close / lineplot / trend</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Ligne des prix de cloture avec marqueurs — meme donnee, lecture de tendance plus lisse.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/candlestick-line.html"></iframe>
</div>

<div class="sp-variant" id="candlestick-fr-mountain">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"mountain"</code></span><span><strong>Aliases</strong> <code>mountain / area / filled_area / shade</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Aire des prix de cloture avec degrade vertical sous la courbe ; ideal pour visuel de couverture.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/candlestick-mountain.html"></iframe>
</div>

<div class="sp-variant" id="candlestick-fr-range">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"range"</code></span><span><strong>Aliases</strong> <code>range / hl / highlow / spread</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Barres haut-bas uniquement (sans ouverture/cloture), une seule couleur — visualisation de la volatilite.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/candlestick-range.html"></iframe>
</div>

</div>
</div>

</div>
