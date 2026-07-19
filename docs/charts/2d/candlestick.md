# Candlestick — OHLC Time Series

<div class="lang-en">

<style>

.sp-tabs{border:1px solid var(--sp-border);border-radius:8px;overflow:hidden;margin:1.2em 0}
.sp-tab-btns{display:flex;background:var(--sp-surface);border-bottom:1px solid var(--sp-border);flex-wrap:wrap}
.sp-tb{padding:8px 14px;border:none;background:none;color:var(--sp-text-muted);cursor:pointer;font-size:12px;font-weight:600;border-bottom:2px solid transparent;transition:color .15s,border-color .15s;white-space:nowrap}
.sp-tb:hover{color:var(--sp-text)}
.sp-tb.sp-act{color:var(--sp-accent);border-bottom-color:var(--sp-accent)}
.sp-tc{display:none}
.sp-tc.sp-on{display:block}
.sp-cls{display:flex;gap:0;margin:1.6em 0 1.6em 36px;border-radius:14px;background:linear-gradient(180deg,#0a0f1c 0%,#060912 100%);box-shadow:0 18px 50px -12px rgba(0,0,0,.6),0 0 0 1px #1e293b inset;position:relative;overflow:visible}
.sp-cls-rail{display:flex;flex-direction:column;background:linear-gradient(180deg,#0d1426,#070b18);border-right:1px solid #1e293b;padding:18px 0;min-width:18px;transition:min-width .28s cubic-bezier(.5,0,.2,1);position:relative;z-index:2;border-radius:14px 0 0 14px;overflow:visible}
.sp-cls.sp-open .sp-cls-rail{min-width:170px;padding:18px 8px}
.sp-cls-toggle{position:absolute;top:-14px;left:8px;padding:5px 9px;background:#1e293b;color:#a5b4fc;border:1px solid #312e81;border-radius:6px;cursor:pointer;font-size:12px;font-weight:700;transition:all .15s;line-height:1;z-index:5;box-shadow:0 4px 12px -2px rgba(0,0,0,.5)}
.sp-cls-toggle:hover{background:#312e81;color:#e0e7ff;transform:translateY(-1px)}
.sp-cls-tab{position:relative;display:flex;align-items:center;gap:8px;margin:5px 0 5px -34px;padding:11px 16px 11px 14px;background:linear-gradient(90deg,#1a2540 0%,#141d33 70%,var(--sp-surface) 100%);color:#94a3b8;font-size:12px;font-weight:600;cursor:pointer;border:none;text-align:left;white-space:nowrap;border-radius:8px 0 0 8px;box-shadow:-6px 4px 14px -4px rgba(0,0,0,.55),inset 0 1px 0 rgba(255,255,255,.04),inset 1px 0 0 rgba(255,255,255,.05);transition:all .25s cubic-bezier(.5,0,.2,1);clip-path:polygon(0 0,calc(100% - 10px) 0,100% 50%,calc(100% - 10px) 100%,0 100%);min-height:18px}
.sp-cls-tab:hover{background:linear-gradient(90deg,#23304d,#1a2540 70%,#141d33);color:#e0e7ff;margin-left:-40px;box-shadow:-8px 6px 18px -4px rgba(0,0,0,.6),inset 0 1px 0 rgba(255,255,255,.06)}
.sp-cls-tab.sp-cact{background:linear-gradient(90deg,#3730a3 0%,#1e1b4b 50%,var(--sp-surface) 100%);color:#f5f3ff;margin-left:-46px;box-shadow:-10px 8px 22px -4px rgba(99,102,241,.35),-3px 0 0 0 #818cf8 inset,inset 0 1px 0 rgba(165,180,252,.2);font-weight:700;z-index:3}
.sp-cls-tab .sp-cic{font-size:13px;flex-shrink:0;color:#a5b4fc;font-weight:900;letter-spacing:-1px;width:16px;text-align:center;text-shadow:0 0 6px rgba(165,180,252,.4)}
.sp-cls-tab.sp-cact .sp-cic{color:#e0e7ff;text-shadow:0 0 10px rgba(165,180,252,.7)}
.sp-cls-tab .sp-clb{display:none;font-weight:inherit;letter-spacing:.01em}
.sp-cls.sp-open .sp-cls-tab .sp-clb{display:inline}
.sp-cls-body{flex:1;padding:24px 26px 22px;background:#0a0f1c;min-width:0;position:relative;z-index:1;border-radius:0 14px 14px 0;overflow:hidden}
.sp-variant{display:none}
.sp-variant.sp-von{display:block;animation:spFade .25s ease}
@keyframes spFade{from{opacity:0;transform:translateX(8px)}to{opacity:1;transform:translateX(0)}}
.sp-vmeta{display:flex;flex-wrap:wrap;gap:8px 18px;align-items:center;font-size:13px;color:#94a3b8;margin:6px 0 16px;padding:10px 14px;background:rgba(99,102,241,.06);border-left:3px solid var(--sp-accent);border-radius:0 6px 6px 0}
.sp-vmeta strong{color:#a5b4fc;font-weight:700;margin-right:4px;letter-spacing:.04em;text-transform:uppercase;font-size:11px}
.sp-vmeta code{background:#1e293b;padding:2px 7px;border-radius:4px;color:var(--sp-text);font-size:12px}
.sp-preview-frame{width:100%;height:520px;border:none;border-radius:10px;display:block;background:#0d1117;margin-top:10px;box-shadow:0 8px 24px -8px rgba(0,0,0,.5)}
.sp-preview-label{font-size:11px;letter-spacing:.14em;font-weight:700;color:#818cf8;margin:20px 0 8px;text-transform:uppercase}
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
