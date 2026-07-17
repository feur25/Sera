# Event Plot — Discrete Event Ticks per Row

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

`sp.eventplot(title, x_values, categories, *, variant="basic", **kwargs) -> Chart`

Aliases: `sp.eventplot`, `sp.event_plot`, `sp.raster_plot`, `sp.spike_plot`, `sp.build_eventplot`

## Description

`sp.eventplot()` draws a short vertical tick for every discrete event, one row per category — the standard chart for point-in-time occurrences without duration (neuron spike trains, log/error timestamps, user session starts). Unlike [`gantt()`](gantt.md), events have no end time. Reuses `x_values` and the existing `categories` field on `ChartArgs` (the same grouping mechanism used by `bubble()`'s categorical variant) — no new parameter shape, rows are formed automatically from distinct category values in order of first appearance.

## Variants

<div data-sp-registry-table="variants" data-family="eventplot"></div>

Unknown variant strings fall back to the registered default. Variant keys may be prefixed with `en_`, `fr_`, `en-` or `fr-`.

## Data

`x_values` (`list[float]`) — Event positions. `categories` (`list[str]`) — Row assignment per event.


## Parameters

<div data-sp-registry-table="options" data-family="eventplot"></div>

## Themes

<div data-sp-registry-table="themes" data-family="eventplot"></div>


## Returns

`Chart` — object with `.html` property and `.show()` method.


<div class="sp-panel-source">
<h2>Parameters</h2>

<div data-sp-registry-table="variants" data-family="eventplot"></div>
</div>

<div class="sp-cls sp-open" id="eventplot-en">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('eventplot-en')" title="Collapse / expand">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('eventplot-en','basic',this)"><span class="sp-cic">┃</span><span class="sp-clb">Basic</span></button>
<button class="sp-cls-tab" onclick="spCls('eventplot-en','density',this)"><span class="sp-cic">∿</span><span class="sp-clb">Density</span></button>
<button class="sp-cls-tab" onclick="spCls('eventplot-en','gradient',this)"><span class="sp-cic">▤</span><span class="sp-clb">Gradient</span></button>
<button class="sp-cls-tab" onclick="spCls('eventplot-en','connected',this)"><span class="sp-cic">⤳</span><span class="sp-clb">Connected</span></button>
</div>
<div class="sp-cls-body">
<div class="sp-variant sp-von" id="eventplot-en-basic">
<p>Tick marks only, one color per row.</p>
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"basic"</code></span><span><strong>Aliases</strong> <code>basic / default / classic / ticks</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/eventplot-basic.html"></iframe>
</div>
<div class="sp-variant" id="eventplot-en-density">
<p>Tick marks plus a smoothed density curve per row, computed with the same native kernel density estimator as [`kde()`](kde.md) (`scott_bw` bandwidth selection, Gaussian kernel) — a rug plot and a KDE in one chart.</p>
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"density"</code></span><span><strong>Aliases</strong> <code>density / kde / smoothed / rug</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/eventplot-density.html"></iframe>
</div>
<div class="sp-variant" id="eventplot-en-gradient">
<p>Ticks colored by their position on the time axis via `colorscale=` (the 10 shared gradients from the continuous engine), instead of a fixed color per row — highlights when events cluster, not just which row they're in.</p>
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"gradient"</code></span><span><strong>Aliases</strong> <code>gradient / colorscale / heat / timeline</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/eventplot-gradient.html"></iframe>
</div>
<div class="sp-variant" id="eventplot-en-connected">
<p>Draws a thin line connecting each row's events in chronological order, on top of the usual ticks — traces the sequence/trajectory through time, not just where events landed.</p>
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"connected"</code></span><span><strong>Aliases</strong> <code>connected / sequence / path / trajectory</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/eventplot-connected.html"></iframe>
</div>
</div>
</div>

</div>

<div class="lang-fr" style="display:none">

## Signature

`sp.eventplot(title, x_values, categories, *, variant="basic", **kwargs) -> Chart`

Alias : `sp.eventplot`, `sp.event_plot`, `sp.raster_plot`, `sp.spike_plot`, `sp.build_eventplot`

## Description

`sp.eventplot()` trace un court trait vertical pour chaque événement discret, une ligne par catégorie — le graphique de référence pour des occurrences ponctuelles sans durée (trains de potentiels d'action, horodatages de logs/erreurs, débuts de session utilisateur). Contrairement à [`gantt()`](gantt.md), les événements n'ont pas de fin. Réutilise `x_values` et le champ `categories` déjà existant sur `ChartArgs` (le même mécanisme de regroupement que la variante catégorielle de `bubble()`) — aucune nouvelle forme de paramètre, les lignes sont formées automatiquement à partir des valeurs de catégorie distinctes dans leur ordre de première apparition.

## Variantes

<div data-sp-registry-table="variants" data-family="eventplot"></div>

Une variante inconnue retombe sur la valeur par défaut enregistrée. Les clés de variantes peuvent être préfixées par `en_`, `fr_`, `en-` ou `fr-`.

## Données

`x_values` (`list[float]`) — Positions des événements. `categories` (`list[str]`) — Ligne d'appartenance par événement.


## Paramètres

<div data-sp-registry-table="options" data-family="eventplot"></div>

## Thèmes

<div data-sp-registry-table="themes" data-family="eventplot"></div>


## Retour

`Chart` — objet avec une propriété `.html` et une méthode `.show()`.


<div class="sp-panel-source">
<h2>Paramètres</h2>

<div data-sp-registry-table="variants" data-family="eventplot"></div>
</div>

<div class="sp-cls sp-open" id="eventplot-fr">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('eventplot-fr')" title="Collapse / expand">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('eventplot-fr','basic',this)"><span class="sp-cic">┃</span><span class="sp-clb">Basic</span></button>
<button class="sp-cls-tab" onclick="spCls('eventplot-fr','density',this)"><span class="sp-cic">∿</span><span class="sp-clb">Density</span></button>
<button class="sp-cls-tab" onclick="spCls('eventplot-fr','gradient',this)"><span class="sp-cic">▤</span><span class="sp-clb">Gradient</span></button>
<button class="sp-cls-tab" onclick="spCls('eventplot-fr','connected',this)"><span class="sp-cic">⤳</span><span class="sp-clb">Connected</span></button>
</div>
<div class="sp-cls-body">
<div class="sp-variant sp-von" id="eventplot-fr-basic">
<p>Traits verticaux seuls, une couleur par ligne.</p>
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"basic"</code></span><span><strong>Alias</strong> <code>basic / default / classic / ticks</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/eventplot-basic.html"></iframe>
</div>
<div class="sp-variant" id="eventplot-fr-density">
<p>Traits verticaux plus une courbe de densité lissée par ligne, calculée avec le même estimateur de densité par noyau natif que [`kde()`](kde.md) (sélection de bande passante `scott_bw`, noyau gaussien) — un rug plot et un KDE en un seul graphique.</p>
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"density"</code></span><span><strong>Alias</strong> <code>density / kde / smoothed / rug</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/eventplot-density.html"></iframe>
</div>
<div class="sp-variant" id="eventplot-fr-gradient">
<p>Traits colorés selon leur position sur l'axe temporel via `colorscale=` (les 10 dégradés partagés du moteur continu), au lieu d'une couleur fixe par ligne — fait ressortir quand les événements se regroupent, pas seulement dans quelle ligne.</p>
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"gradient"</code></span><span><strong>Alias</strong> <code>gradient / colorscale / heat / timeline</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/eventplot-gradient.html"></iframe>
</div>
<div class="sp-variant" id="eventplot-fr-connected">
<p>Trace un trait fin reliant les événements de chaque ligne dans l'ordre chronologique, par-dessus les graduations habituelles — trace la séquence/trajectoire dans le temps, pas seulement où les événements sont tombés.</p>
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"connected"</code></span><span><strong>Alias</strong> <code>connected / sequence / path / trajectory</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/eventplot-connected.html"></iframe>
</div>
</div>
</div>

</div>
