# Gantt — Project Timeline Chart

<div class="lang-en">

<style>
.sp-cls{display:flex;gap:0;margin:1.6em 0 1.6em 36px;border-radius:14px;background:linear-gradient(180deg,#0a0f1c 0%,#060912 100%);box-shadow:0 18px 50px -12px rgba(0,0,0,.6),0 0 0 1px #1e293b inset;position:relative;overflow:visible}.sp-cls-rail{display:flex;flex-direction:column;background:linear-gradient(180deg,#0d1426,#070b18);border-right:1px solid #1e293b;padding:18px 0;min-width:18px;transition:min-width .28s;position:relative;z-index:2;border-radius:14px 0 0 14px;overflow:visible}.sp-cls.sp-open .sp-cls-rail{min-width:170px;padding:18px 8px}.sp-cls-toggle{position:absolute;top:-14px;left:8px;padding:5px 9px;background:#1e293b;color:#a5b4fc;border:1px solid #312e81;border-radius:6px;cursor:pointer;font-size:12px;font-weight:700;line-height:1;z-index:5}.sp-cls-tab{position:relative;display:flex;align-items:center;gap:8px;margin:5px 0 5px -34px;padding:11px 16px 11px 14px;background:linear-gradient(90deg,#1a2540,#0f172a);color:#94a3b8;font-size:12px;font-weight:600;cursor:pointer;border:none;text-align:left;white-space:nowrap;border-radius:8px 0 0 8px;transition:all .25s;clip-path:polygon(0 0,calc(100% - 10px) 0,100% 50%,calc(100% - 10px) 100%,0 100%);min-height:18px}.sp-cls-tab.sp-cact{background:linear-gradient(90deg,#3730a3,#0f172a);color:#f5f3ff;margin-left:-46px;box-shadow:-3px 0 0 0 #818cf8 inset;font-weight:700;z-index:3}.sp-cls-tab .sp-cic{font-size:13px;flex-shrink:0;color:#a5b4fc;width:16px;text-align:center}.sp-cls-tab .sp-clb{display:none}.sp-cls.sp-open .sp-cls-tab .sp-clb{display:inline}.sp-cls-body{flex:1;padding:24px 26px 22px;background:#0a0f1c;min-width:0;border-radius:0 14px 14px 0;overflow:hidden}.sp-variant{display:none}.sp-variant.sp-von{display:block}.sp-vmeta{display:flex;flex-wrap:wrap;gap:8px 18px;align-items:center;font-size:13px;color:#94a3b8;margin:6px 0 16px;padding:10px 14px;background:rgba(99,102,241,.06);border-left:3px solid #6366f1;border-radius:0 6px 6px 0}.sp-vmeta strong{color:#a5b4fc;font-weight:700;margin-right:4px;text-transform:uppercase;font-size:11px}.sp-vmeta code{background:#1e293b;padding:2px 7px;border-radius:4px;color:#e2e8f0;font-size:12px}.sp-preview-frame{width:100%;height:420px;border:none;border-radius:10px;display:block;background:#0d1117;margin-top:10px}.sp-preview-label{font-size:11px;letter-spacing:.14em;font-weight:700;color:#818cf8;margin:20px 0 8px;text-transform:uppercase}
</style>
<script>
function spCls(scope,name,btn){var root=document.getElementById(scope);root.querySelectorAll('.sp-variant').forEach(function(s){s.classList.remove('sp-von')});root.querySelectorAll('.sp-cls-tab').forEach(function(b){b.classList.remove('sp-cact')});document.getElementById(scope+'-'+name).classList.add('sp-von');btn.classList.add('sp-cact')}
function spClsTog(id){document.getElementById(id).classList.toggle('sp-open')}
</script>

## Signature

`sp.gantt(title, labels, start, end, *, variant="basic", categories=None, color_values=None, **kwargs) -> Chart`

Aliases: `sp.gantt`, `sp.gantt_chart`, `sp.broken_barh`, `sp.timeline_chart`, `sp.project_timeline`, `sp.build_gantt`

## Description

`sp.gantt()` draws one horizontal bar per task spanning from `start` to `end` on a shared numeric timeline (day index, epoch, or any consistent unit — no date-typed axis required). Tasks are sorted like every other chart via `sort_order`, and rows can be grouped by an optional `categories` list (e.g. team or phase), which colors bars by category with an automatic legend instead of per-row palette rotation. Reuses the `labels`/`start`/`end`/`categories` fields already on `ChartArgs` (the same `start`/`end` pair used by `dumbbell()`) — no new parameter shape.

## Variants

| Variant | Aliases | Description |
|---|---|---|
| `"basic"` | `basic / default / classic / flat` | Flat colored bar per task. |
| `"progress"` | `progress / percent / completion / filled` | Outlined bar with an inner fill proportional to `color_values` (0–1 completion fraction) and a `%` label. |

## Parameters

| Parameter      | Type          | Default   | Description |
|---|---|---|---|
| `title`        | `str`         | required  | Chart title |
| `labels`       | `list[str]`   | required  | Task names |
| `start`        | `list[float]` | required  | Task start (numeric timeline unit) |
| `end`          | `list[float]` | required  | Task end |
| `variant`      | `str`         | `"basic"` | Visual style (see table) |
| `categories`   | `list[str]`   | `None`    | Optional group per task; colors bars by group with a legend |
| `color_values` | `list[float]` | `None`    | Completion fraction (0–1) per task, used by the `"progress"` variant |
| `sort_order`   | `str`         | `"none"`  | Row ordering, same semantics as every other chart |
| `width`        | `int`         | `1000`    | Canvas width (px) |
| `height`       | `int`         | `520`     | Canvas height (px) |

## Returns

`Chart` — object with `.html` property and `.show()` method.

## Example

```python
import seraplot as sp
chart = sp.gantt(
    "Product Launch",
    labels=["Design", "Build", "Test", "Launch"],
    start=[0, 5, 12, 18],
    end=[6, 14, 19, 22],
    categories=["Plan", "Dev", "Dev", "Plan"],
    variant="progress",
    color_values=[1.0, 0.6, 0.25, 0.0],
)
chart.show()
```

<div class="sp-cls sp-open" id="gantt-en">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('gantt-en')" title="Collapse / expand">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('gantt-en','basic',this)"><span class="sp-cic">▬</span><span class="sp-clb">Basic</span></button>
<button class="sp-cls-tab" onclick="spCls('gantt-en','progress',this)"><span class="sp-cic">▤</span><span class="sp-clb">Progress</span></button>
</div>
<div class="sp-cls-body">
<div class="sp-variant sp-von" id="gantt-en-basic">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"basic"</code></span><span><strong>Aliases</strong> <code>basic / default / classic / flat</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/gantt-basic.html"></iframe>
</div>
<div class="sp-variant" id="gantt-en-progress">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"progress"</code></span><span><strong>Aliases</strong> <code>progress / percent / completion / filled</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/gantt-progress.html"></iframe>
</div>
</div>
</div>

</div>

<div class="lang-fr">

## Signature

`sp.gantt(title, labels, start, end, *, variant="basic", categories=None, color_values=None, **kwargs) -> Chart`

Alias : `sp.gantt`, `sp.gantt_chart`, `sp.broken_barh`, `sp.timeline_chart`, `sp.project_timeline`, `sp.build_gantt`

## Description

`sp.gantt()` trace une barre horizontale par tâche, de `start` à `end` sur une échelle temporelle numérique partagée (index de jour, epoch, ou toute unité cohérente — aucun axe de type date requis). Les tâches sont triées comme sur tous les autres graphiques via `sort_order`, et les lignes peuvent être groupées par une liste `categories` optionnelle (ex : équipe ou phase), qui colore les barres par groupe avec une légende automatique au lieu d'une rotation de palette par ligne. Réutilise les champs `labels`/`start`/`end`/`categories` déjà présents sur `ChartArgs` (la même paire `start`/`end` utilisée par `dumbbell()`) — aucune nouvelle forme de paramètre.

## Variantes

| Variante | Alias | Description |
|---|---|---|
| `"basic"` | `basic / default / classic / flat` | Barre colorée pleine par tâche. |
| `"progress"` | `progress / percent / completion / filled` | Barre en contour avec un remplissage interne proportionnel à `color_values` (fraction de complétion 0–1) et un libellé `%`. |

## Paramètres

| Paramètre      | Type          | Défaut    | Description |
|---|---|---|---|
| `title`        | `str`         | requis    | Titre du graphique |
| `labels`       | `list[str]`   | requis    | Noms des tâches |
| `start`        | `list[float]` | requis    | Début de tâche (unité temporelle numérique) |
| `end`          | `list[float]` | requis    | Fin de tâche |
| `variant`      | `str`         | `"basic"` | Style visuel (voir tableau) |
| `categories`   | `list[str]`   | `None`    | Groupe optionnel par tâche ; colore les barres par groupe avec légende |
| `color_values` | `list[float]` | `None`    | Fraction de complétion (0–1) par tâche, utilisée par la variante `"progress"` |
| `sort_order`   | `str`         | `"none"`  | Ordre des lignes, même sémantique que tous les autres graphiques |
| `width`        | `int`         | `1000`    | Largeur du canvas (px) |
| `height`       | `int`         | `520`     | Hauteur du canvas (px) |

## Retour

`Chart` — objet avec une propriété `.html` et une méthode `.show()`.

<div class="sp-cls sp-open" id="gantt-fr">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('gantt-fr')" title="Collapse / expand">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('gantt-fr','basic',this)"><span class="sp-cic">▬</span><span class="sp-clb">Basic</span></button>
<button class="sp-cls-tab" onclick="spCls('gantt-fr','progress',this)"><span class="sp-cic">▤</span><span class="sp-clb">Progress</span></button>
</div>
<div class="sp-cls-body">
<div class="sp-variant sp-von" id="gantt-fr-basic">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"basic"</code></span><span><strong>Aliases</strong> <code>basic / default / classic / flat</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/gantt-basic.html"></iframe>
</div>
<div class="sp-variant" id="gantt-fr-progress">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"progress"</code></span><span><strong>Aliases</strong> <code>progress / percent / completion / filled</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/gantt-progress.html"></iframe>
</div>
</div>
</div>

</div>
