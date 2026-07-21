# Slideshow - Navigable Chart Carousel

<div class="lang-en">

<style>
.sp-preview-frame{width:100%;height:520px;border:none;border-radius:10px;display:block;background:#0d1117;margin-top:10px;box-shadow:0 8px 24px -8px rgba(0,0,0,.5)}
</style>
## Signature

`sp.build_slideshow(charts, interval_ms=2500, title='', width=900, height=520) -> Chart`

## Description

`sp.build_slideshow()` turns a list of charts into a navigable HTML carousel with previous / next buttons and an auto-advance progress bar. Drop in any pre-built chart objects - the slideshow takes care of layout, timing and progressive reveal so you can pitch a story without a slide deck.

## Parameters

| Parameter | Type | Default | Description |
|---|---|---|---|
| `charts` | `list[Chart]` | required | Charts to use as slides |
| `interval_ms` | `int` | 2500 | Auto-play interval in milliseconds |
| `title` | `str` | "" | Slideshow title shown above the deck |
| `width` | `int` | 900 | Width per slide (px) |
| `height` | `int` | 520 | Height per slide (px) |

## Returns

`Chart` - object with `.html` property and `.show()` method.

---

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/slideshow-basic.html"></iframe>

</div>

<div class="lang-fr">

<h2>Signature</h2>

`sp.build_slideshow(charts, interval_ms=2500, title='', width=900, height=520) -> Chart`

<h2>Description</h2>

`sp.build_slideshow()` transforme une liste de charts en carrousel HTML navigable avec boutons precedent/suivant et barre de progression auto. Pose n importe quels charts deja construits - le slideshow gere layout, timing et reveal progressif pour raconter une histoire sans presentation externe.

<h2>Parametres</h2>

| Parametre | Type | Defaut | Description |
|---|---|---|---|
| `charts` | `list[Chart]` | requis | Charts utilises comme slides |
| `interval_ms` | `int` | 2500 | Intervalle d auto-play (ms) |
| `title` | `str` | "" | Titre affiche au-dessus du carrousel |
| `width` | `int` | 900 | Largeur par slide (px) |
| `height` | `int` | 520 | Hauteur par slide (px) |

<h2>Retour</h2>

`Chart` - objet avec propriete `.html` et methode `.show()`.

</div>
