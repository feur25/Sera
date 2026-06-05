# Gauge - Single-Value Arc Indicator

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

`sp.gauge(title, *, value, min_val=0.0, max_val=100.0, label="", variant="basic", comparison=0.0, **kwargs) -> Chart`


## Description

`sp.gauge()` is the unified entry point for the gauge family. A gauge maps a single scalar to a colored arc with optional thresholds - perfect for status / health / utilization KPIs. The `variant` keyword switches the geometry (half, three-quarter, full ring), the embellishments (needle, ticks, glow) and the layering (single arc vs. concentric arcs for value-vs-target).

## Parameters

| Parameter | Used by variants |
|-----------|------------------|
| `comparison` | concentric |
| `max_val` | arc270, concentric, tick |
| `min_val` | arc270, tick |
| `value` | concentric, sleek |

## Returns

`Chart` - object with `.html` property and `.show()` method.

---

</div>

<div class="lang-fr">

<h2>Signature</h2>

`sp.gauge(title, *, value, min_val=0.0, max_val=100.0, label="", variant="basic", comparison=0.0, **kwargs) -> Chart`


<h2>Description</h2>

`sp.gauge()` est le point d entree unique pour la famille jauge. Une jauge associe un scalaire unique a un arc colore avec des seuils optionnels - parfait pour des KPIs de statut / sante / utilisation. Le mot-cle `variant` change la geometrie (demi, trois-quart, anneau complet), les ornements (aiguille, ticks, glow) et la composition (arc simple ou arcs concentriques pour valeur-vs-cible).

<h2>Parametres</h2>

| Parametre | Type | Defaut | Description |
|---|---|---|---|
| `title` | `str` | requis | Titre du graphique |
| `value` | `float` | requis | Valeur courante a afficher |
| `min_val` | `float` | `0.0` | Minimum de l echelle |
| `max_val` | `float` | `100.0` | Maximum de l echelle |
| `variant` | `str` | `"basic"` | Style visuel (voir tableau) |
| `label` | `str` | `""` | Sous-libelle sous la valeur |
| `comparison` | `float` | `0.0` | Valeur de comparaison (pour `concentric`) |
| `seuils` | `list[(float,int)]` | `None` | `[(value, color_hex), ...]` seuils |
| `width` | `int` | `400` | Largeur (px) |
| `height` | `int` | `300` | Hauteur (px) |

<h2>Retour</h2>

`Chart` - objet avec propriete `.html` et methode `.show()`.

---

</div>
