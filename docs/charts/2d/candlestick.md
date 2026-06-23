# Candlestick — OHLC Time Series

<div class="lang-en">

<style>
.sp-tabs{border:1px solid #334155;border-radius:8px;overflow:hidden;margin:1.2em 0}
.sp-tab-btns{display:flex;background:#0f172a;border-bottom:1px solid #334155;flex-wrap:wrap}
.sp-tb{padding:8px 14px;border:none;background:none;color:#64748b;cursor:pointer;font-size:12px;font-weight:600;border-bottom:2px solid transparent;transition:color .15s,border-color .15s;white-space:nowrap}
.sp-tb:hover{color:#e2e8f0}
.sp-tb.sp-act{color:#6366f1;border-bottom-color:#6366f1}
.sp-tc{display:none}
.sp-tc.sp-on{display:block}
</style>
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

</div>
