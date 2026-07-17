# Gantt — Project Timeline Chart

<div class="lang-en">

<style>
.sp-panel-source{display:none!important}
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

<div data-sp-registry-table="variants" data-family="gantt"></div>

Unknown variant strings fall back to the registered default. Variant keys may be prefixed with `en_`, `fr_`, `en-` or `fr-`.

## Data

`labels` (`list[str]`) — Task names. `start` (`list[float]`) — Task start (numeric timeline unit). `end` (`list[float]`) — Task end. `categories` (`list[str]`) — Optional group per task; colors bars by group with a legend. `color_values` (`list[float]`) — Completion fraction (0–1) per task, used by the `"progress"` variant.


## Parameters

<div data-sp-registry-table="options" data-family="gantt"></div>

## Themes

<div data-sp-registry-table="themes" data-family="gantt"></div>


## Returns

`Chart` — object with `.html` property and `.show()` method.


<div class="sp-panel-source">
<h2>Parameters</h2>

<div data-sp-registry-table="variants" data-family="gantt"></div>
</div>

<div class="sp-cls sp-open" id="gantt-en">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('gantt-en')" title="Collapse / expand">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('gantt-en','basic',this)"><span class="sp-cic">▬</span><span class="sp-clb">Basic</span></button>
<button class="sp-cls-tab" onclick="spCls('gantt-en','progress',this)"><span class="sp-cic">▤</span><span class="sp-clb">Progress</span></button>
<button class="sp-cls-tab" onclick="spCls('gantt-en','gradient',this)"><span class="sp-cic">▥</span><span class="sp-clb">Gradient</span></button>
<button class="sp-cls-tab" onclick="spCls('gantt-en','milestone',this)"><span class="sp-cic">◆</span><span class="sp-clb">Milestone</span></button>
</div>
<div class="sp-cls-body">
<div class="sp-variant sp-von" id="gantt-en-basic">
<p>Flat colored bar per task.</p>
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"basic"</code></span><span><strong>Aliases</strong> <code>basic / default / classic / flat</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/gantt-basic.html"></iframe>
</div>
<div class="sp-variant" id="gantt-en-progress">
<p>Outlined bar with an inner fill proportional to `color_values` (0–1 completion fraction) and a `%` label.</p>
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"progress"</code></span><span><strong>Aliases</strong> <code>progress / percent / completion / filled</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/gantt-progress.html"></iframe>
</div>
<div class="sp-variant" id="gantt-en-gradient">
<p>Bars colored by task duration via `colorscale=` (any of the 10 shared colorscales) instead of by category — the longest and shortest tasks stand out at a glance.</p>
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"gradient"</code></span><span><strong>Aliases</strong> <code>gradient / duration / colorscale / heat</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/gantt-gradient.html"></iframe>
</div>
<div class="sp-variant" id="gantt-en-milestone">
<p>Renders zero-duration tasks (`start == end`) as a diamond marker instead of a degenerate bar — the standard way project-planning tools distinguish milestones from work items.</p>
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"milestone"</code></span><span><strong>Aliases</strong> <code>milestone / diamonds / checkpoints / markers</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/gantt-milestone.html"></iframe>
</div>
</div>
</div>

</div>

<div class="lang-fr" style="display:none">

## Signature

`sp.gantt(title, labels, start, end, *, variant="basic", categories=None, color_values=None, **kwargs) -> Chart`

Alias : `sp.gantt`, `sp.gantt_chart`, `sp.broken_barh`, `sp.timeline_chart`, `sp.project_timeline`, `sp.build_gantt`

## Description

`sp.gantt()` trace une barre horizontale par tâche, de `start` à `end` sur une échelle temporelle numérique partagée (index de jour, epoch, ou toute unité cohérente — aucun axe de type date requis). Les tâches sont triées comme sur tous les autres graphiques via `sort_order`, et les lignes peuvent être groupées par une liste `categories` optionnelle (ex : équipe ou phase), qui colore les barres par groupe avec une légende automatique au lieu d'une rotation de palette par ligne. Réutilise les champs `labels`/`start`/`end`/`categories` déjà présents sur `ChartArgs` (la même paire `start`/`end` utilisée par `dumbbell()`) — aucune nouvelle forme de paramètre.

## Variantes

<div data-sp-registry-table="variants" data-family="gantt"></div>

Une variante inconnue retombe sur la valeur par défaut enregistrée. Les clés de variantes peuvent être préfixées par `en_`, `fr_`, `en-` ou `fr-`.

## Données

`labels` (`list[str]`) — Noms des tâches. `start` (`list[float]`) — Début de tâche (unité temporelle numérique). `end` (`list[float]`) — Fin de tâche. `categories` (`list[str]`) — Groupe optionnel par tâche ; colore les barres par groupe avec légende. `color_values` (`list[float]`) — Fraction de complétion (0–1) par tâche, utilisée par la variante `"progress"`.


## Paramètres

<div data-sp-registry-table="options" data-family="gantt"></div>

## Thèmes

<div data-sp-registry-table="themes" data-family="gantt"></div>


## Retour

`Chart` — objet avec une propriété `.html` et une méthode `.show()`.


<div class="sp-panel-source">
<h2>Paramètres</h2>

<div data-sp-registry-table="variants" data-family="gantt"></div>
</div>

<div class="sp-cls sp-open" id="gantt-fr">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('gantt-fr')" title="Collapse / expand">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('gantt-fr','basic',this)"><span class="sp-cic">▬</span><span class="sp-clb">Basic</span></button>
<button class="sp-cls-tab" onclick="spCls('gantt-fr','progress',this)"><span class="sp-cic">▤</span><span class="sp-clb">Progress</span></button>
<button class="sp-cls-tab" onclick="spCls('gantt-fr','gradient',this)"><span class="sp-cic">▥</span><span class="sp-clb">Gradient</span></button>
<button class="sp-cls-tab" onclick="spCls('gantt-fr','milestone',this)"><span class="sp-cic">◆</span><span class="sp-clb">Milestone</span></button>
</div>
<div class="sp-cls-body">
<div class="sp-variant sp-von" id="gantt-fr-basic">
<p>Barre colorée pleine par tâche.</p>
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"basic"</code></span><span><strong>Alias</strong> <code>basic / default / classic / flat</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/gantt-basic.html"></iframe>
</div>
<div class="sp-variant" id="gantt-fr-progress">
<p>Barre en contour avec un remplissage interne proportionnel à `color_values` (fraction de complétion 0–1) et un libellé `%`.</p>
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"progress"</code></span><span><strong>Alias</strong> <code>progress / percent / completion / filled</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/gantt-progress.html"></iframe>
</div>
<div class="sp-variant" id="gantt-fr-gradient">
<p>Barres colorées selon la durée de la tâche via `colorscale=`, au lieu d'une couleur par catégorie — les tâches les plus longues et les plus courtes ressortent immédiatement.</p>
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"gradient"</code></span><span><strong>Alias</strong> <code>gradient / duration / colorscale / heat</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/gantt-gradient.html"></iframe>
</div>
<div class="sp-variant" id="gantt-fr-milestone">
<p>Affiche les tâches de durée nulle (`start == end`) sous forme de losange plutôt qu'une barre dégénérée — la façon standard dont les outils de planification distinguent les jalons des tâches.</p>
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"milestone"</code></span><span><strong>Alias</strong> <code>milestone / diamonds / checkpoints / markers</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/gantt-milestone.html"></iframe>
</div>
</div>
</div>

</div>
