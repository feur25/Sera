# Lollipop - Categorical Value Sticks

<div class="lang-en">
<style>
.sp-tabs{border:1px solid #334155;border-radius:8px;overflow:hidden;margin:1.2em 0}
.sp-tab-btns{display:flex;background:#0f172a;border-bottom:1px solid #334155;flex-wrap:wrap}
.sp-tb{padding:8px 14px;border:none;background:none;color:#64748b;cursor:pointer;font-size:12px;font-weight:600;border-bottom:2px solid transparent;transition:color .15s,border-color .15s;white-space:nowrap}
.sp-tb:hover{color:#e2e8f0}
.sp-tb.sp-act{color:#6366f1;border-bottom-color:#6366f1}
.sp-tc{display:none}
.sp-tc.sp-on{display:block}
.sp-cls{display:flex;gap:0;margin:1.6em 0 1.6em 36px;border-radius:14px;background:linear-gradient(180deg,#0a0f1c 0%,#060912 100%);box-shadow:0 18px 50px -12px rgba(0,0,0,.6),0 0 0 1px #1e293b inset;position:relative;overflow:visible}
.sp-cls-rail{display:flex;flex-direction:column;background:linear-gradient(180deg,#0d1426,#070b18);border-right:1px solid #1e293b;padding:18px 0;min-width:18px;transition:min-width .28s cubic-bezier(.5,0,.2,1);position:relative;z-index:2;border-radius:14px 0 0 14px;overflow:visible}
.sp-cls.sp-open .sp-cls-rail{min-width:170px;padding:18px 8px}
.sp-cls-toggle{position:absolute;top:-14px;left:8px;padding:5px 9px;background:#1e293b;color:#a5b4fc;border:1px solid #312e81;border-radius:6px;cursor:pointer;font-size:12px;font-weight:700;transition:all .15s;line-height:1;z-index:5;box-shadow:0 4px 12px -2px rgba(0,0,0,.5)}
.sp-cls-toggle:hover{background:#312e81;color:#e0e7ff;transform:translateY(-1px)}
.sp-cls-tab{position:relative;display:flex;align-items:center;gap:8px;margin:5px 0 5px -34px;padding:11px 16px 11px 14px;background:linear-gradient(90deg,#1a2540 0%,#141d33 70%,#0f172a 100%);color:#94a3b8;font-size:12px;font-weight:600;cursor:pointer;border:none;text-align:left;white-space:nowrap;border-radius:8px 0 0 8px;box-shadow:-6px 4px 14px -4px rgba(0,0,0,.55),inset 0 1px 0 rgba(255,255,255,.04),inset 1px 0 0 rgba(255,255,255,.05);transition:all .25s cubic-bezier(.5,0,.2,1);clip-path:polygon(0 0,calc(100% - 10px) 0,100% 50%,calc(100% - 10px) 100%,0 100%);min-height:18px}
.sp-cls-tab:hover{background:linear-gradient(90deg,#23304d,#1a2540 70%,#141d33);color:#e0e7ff;margin-left:-40px;box-shadow:-8px 6px 18px -4px rgba(0,0,0,.6),inset 0 1px 0 rgba(255,255,255,.06)}
.sp-cls-tab.sp-cact{background:linear-gradient(90deg,#3730a3 0%,#1e1b4b 50%,#0f172a 100%);color:#f5f3ff;margin-left:-46px;box-shadow:-10px 8px 22px -4px rgba(99,102,241,.35),-3px 0 0 0 #818cf8 inset,inset 0 1px 0 rgba(165,180,252,.2);font-weight:700;z-index:3}
.sp-cls-tab .sp-cic{font-size:13px;flex-shrink:0;color:#a5b4fc;font-weight:900;letter-spacing:-1px;width:16px;text-align:center;text-shadow:0 0 6px rgba(165,180,252,.4)}
.sp-cls-tab.sp-cact .sp-cic{color:#e0e7ff;text-shadow:0 0 10px rgba(165,180,252,.7)}
.sp-cls-tab .sp-clb{display:none;font-weight:inherit;letter-spacing:.01em}
.sp-cls.sp-open .sp-cls-tab .sp-clb{display:inline}
.sp-cls-body{flex:1;padding:24px 26px 22px;background:#0a0f1c;min-width:0;position:relative;z-index:1;border-radius:0 14px 14px 0;overflow:hidden}
.sp-variant{display:none}
.sp-variant.sp-von{display:block;animation:spFade .25s ease}
@keyframes spFade{from{opacity:0;transform:translateX(8px)}to{opacity:1;transform:translateX(0)}}
.sp-vmeta{display:flex;flex-wrap:wrap;gap:8px 18px;align-items:center;font-size:13px;color:#94a3b8;margin:6px 0 16px;padding:10px 14px;background:rgba(99,102,241,.06);border-left:3px solid #6366f1;border-radius:0 6px 6px 0}
.sp-vmeta strong{color:#a5b4fc;font-weight:700;margin-right:4px;letter-spacing:.04em;text-transform:uppercase;font-size:11px}
.sp-vmeta code{background:#1e293b;padding:2px 7px;border-radius:4px;color:#e2e8f0;font-size:12px}
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

`sp.lollipop(title, labels, values, *, variant="basic", color_groups=None, highlight_index=-1, **kwargs) -> Chart`

Aliases: `sp.lollipop`, `sp.build_lollipop_chart`

## Description

`sp.lollipop()` is the unified entry point for the lollipop family. Each item becomes a thin stick capped by a dot - lighter ink than a bar chart for the same ranking, and the family includes circular, diverging, focused and grouped editorial layouts (the Office variant reproduces the season-rating panel pattern).

## Variants

| Variant | Aliases | Description |
|---|---|---|
| `"basic"` | `basic / default / classic / vertical` | Vertical sticks topped with dots - the canonical lollipop for ranked categorical values. |
| `"cleveland"` | `cleveland / horizontal / h / row` | Horizontal Cleveland dot plot - long labels read naturally and dots align cleanly along value axis. |
| `"diverging"` | `diverging / div / signed / delta` | Sticks pivot around the mean: green points sit above, red points below - perfect for deviation analysis. |
| `"circular"` | `circular / polar / radial / round` | Polar layout where each category is an angular spoke - eye-catching for small alphabets and dashboard tiles. |
| `"highlight"` | `highlight / spotlight / focus / dim` | All sticks dimmed except one accent (auto-max or `highlight_index`) - ideal for editorial focus. |
| `"office"` | `office / grouped / season / panel` | Group-aware lollipops with per-group mean line and color band - inspired by The Office IMDb season chart. |

## Parameters

| Parameter | Type | Default | Description |
|---|---|---|---|
| `title` | `str` | required | Chart title |
| `labels` | `list[str]` | required | Category labels (one per stick) |
| `values` | `list[float]` | required | Value per label |
| `variant` | `str` | "basic" | Visual style (see table) |
| `color_groups` | `list[str]` | None | Group label per item - enables Office variant grouping |
| `highlight_index` | `int` | -1 | Item to spotlight (Highlight variant); -1 = auto-max |
| `color_hex` | `int` | 0x6366F1 | Default stick color |
| `palette` | `list[int]` | None | Custom palette |
| `show_values` | `bool` | False | Print value next to each dot |
| `gridlines` | `bool` | False | Toggle background grid |
| `sort_order` | `str` | "none" | "none" / "asc" / "desc" / "alpha" |
| `width` | `int` | 900 | Canvas width (px) |
| `height` | `int` | 480 | Canvas height (px) |

## Returns

`Chart` - object with `.html` property and `.show()` method.

---

<div class="sp-cls sp-open" id="lollipop-en">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('lollipop-en')" title="Toggle">&#x21C6;</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('lollipop-en','basic',this)"><span class="sp-cic">B</span><span class="sp-clb">Basic</span></button>
<button class="sp-cls-tab" onclick="spCls('lollipop-en','cleveland',this)"><span class="sp-cic">C</span><span class="sp-clb">Cleveland</span></button>
<button class="sp-cls-tab" onclick="spCls('lollipop-en','diverging',this)"><span class="sp-cic">D</span><span class="sp-clb">Diverging</span></button>
<button class="sp-cls-tab" onclick="spCls('lollipop-en','circular',this)"><span class="sp-cic">O</span><span class="sp-clb">Circular</span></button>
<button class="sp-cls-tab" onclick="spCls('lollipop-en','highlight',this)"><span class="sp-cic">H</span><span class="sp-clb">Highlight</span></button>
<button class="sp-cls-tab" onclick="spCls('lollipop-en','office',this)"><span class="sp-cic">G</span><span class="sp-clb">Office</span></button>
</div>
<div class="sp-cls-body">
<div class="sp-variant sp-von" id="lollipop-en-basic">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"basic"</code></span><span><strong>Aliases</strong> <code>basic / default / classic / vertical</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Vertical sticks topped with dots - the canonical lollipop for ranked categorical values.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/lollipop-basic.html"></iframe>
</div>
<div class="sp-variant" id="lollipop-en-cleveland">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"cleveland"</code></span><span><strong>Aliases</strong> <code>cleveland / horizontal / h / row</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Horizontal Cleveland dot plot - long labels read naturally and dots align cleanly along value axis.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/lollipop-cleveland.html"></iframe>
</div>
<div class="sp-variant" id="lollipop-en-diverging">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"diverging"</code></span><span><strong>Aliases</strong> <code>diverging / div / signed / delta</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Sticks pivot around the mean: green points sit above, red points below - perfect for deviation analysis.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/lollipop-diverging.html"></iframe>
</div>
<div class="sp-variant" id="lollipop-en-circular">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"circular"</code></span><span><strong>Aliases</strong> <code>circular / polar / radial / round</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Polar layout where each category is an angular spoke - eye-catching for small alphabets and dashboard tiles.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/lollipop-circular.html"></iframe>
</div>
<div class="sp-variant" id="lollipop-en-highlight">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"highlight"</code></span><span><strong>Aliases</strong> <code>highlight / spotlight / focus / dim</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">All sticks dimmed except one accent (auto-max or `highlight_index`) - ideal for editorial focus.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/lollipop-highlight.html"></iframe>
</div>
<div class="sp-variant" id="lollipop-en-office">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"office"</code></span><span><strong>Aliases</strong> <code>office / grouped / season / panel</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Group-aware lollipops with per-group mean line and color band - inspired by The Office IMDb season chart.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/lollipop-office.html"></iframe>
</div>
</div>
</div>

</div>

<div class="lang-fr">

<h2>Signature</h2>

`sp.lollipop(title, labels, values, *, variant="basic", color_groups=None, highlight_index=-1, **kwargs) -> Chart`

Aliases: `sp.lollipop`, `sp.build_lollipop_chart`

<h2>Description</h2>

`sp.lollipop()` est le point d entree unique pour la famille lollipop. Chaque item devient un baton fin termine par un point - moins d encre qu un bar chart pour le meme classement, et la famille couvre des layouts circulaires, divergents, focalises et editoriaux groupes (la variante Office reproduit le motif des saisons IMDb de The Office).

<h2>Variantes</h2>

| Variante | Alias | Description |
|---|---|---|
| `"basic"` | `basic / default / classic / vertical` | Batons verticaux surmontes de points - le lollipop canonique pour valeurs categorielles classees. |
| `"cleveland"` | `cleveland / horizontal / h / row` | Cleveland dot plot horizontal - les longs libelles se lisent naturellement, les points s alignent sur l axe des valeurs. |
| `"diverging"` | `diverging / div / signed / delta` | Batons pivotent autour de la moyenne: vert au-dessus, rouge en-dessous - parfait pour les ecarts. |
| `"circular"` | `circular / polar / radial / round` | Disposition polaire ou chaque categorie est un rayon - tres lisible pour petits jeux et dashboards. |
| `"highlight"` | `highlight / spotlight / focus / dim` | Tous les batons grises sauf un accent (auto-max ou `highlight_index`) - ideal pour focus editorial. |
| `"office"` | `office / grouped / season / panel` | Lollipops groupes avec moyenne par groupe et bande de couleur - inspire du chart IMDb de The Office. |

<h2>Parametres</h2>

| Parametre | Type | Defaut | Description |
|---|---|---|---|
| `title` | `str` | requis | Titre du graphique |
| `labels` | `list[str]` | requis | Libelles categoriels (un par baton) |
| `values` | `list[float]` | requis | Valeur par libelle |
| `variant` | `str` | "basic" | Style visuel (voir tableau) |
| `color_groups` | `list[str]` | None | Groupe par item - active le groupage Office |
| `highlight_index` | `int` | -1 | Index a mettre en avant (Highlight); -1 = auto-max |
| `color_hex` | `int` | 0x6366F1 | Couleur par defaut |
| `palette` | `list[int]` | None | Palette personnalisee |
| `show_values` | `bool` | False | Afficher la valeur a cote de chaque point |
| `gridlines` | `bool` | False | Activer la grille de fond |
| `sort_order` | `str` | "none" | "none" / "asc" / "desc" / "alpha" |
| `width` | `int` | 900 | Largeur (px) |
| `height` | `int` | 480 | Hauteur (px) |

<h2>Retour</h2>

`Chart` - objet avec propriete `.html` et methode `.show()`.

---

<div class="sp-cls sp-open" id="lollipop-fr">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('lollipop-fr')" title="Toggle">&#x21C6;</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('lollipop-fr','basic',this)"><span class="sp-cic">B</span><span class="sp-clb">Basic</span></button>
<button class="sp-cls-tab" onclick="spCls('lollipop-fr','cleveland',this)"><span class="sp-cic">C</span><span class="sp-clb">Cleveland</span></button>
<button class="sp-cls-tab" onclick="spCls('lollipop-fr','diverging',this)"><span class="sp-cic">D</span><span class="sp-clb">Diverging</span></button>
<button class="sp-cls-tab" onclick="spCls('lollipop-fr','circular',this)"><span class="sp-cic">O</span><span class="sp-clb">Circular</span></button>
<button class="sp-cls-tab" onclick="spCls('lollipop-fr','highlight',this)"><span class="sp-cic">H</span><span class="sp-clb">Highlight</span></button>
<button class="sp-cls-tab" onclick="spCls('lollipop-fr','office',this)"><span class="sp-cic">G</span><span class="sp-clb">Office</span></button>
</div>
<div class="sp-cls-body">
<div class="sp-variant sp-von" id="lollipop-fr-basic">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"basic"</code></span><span><strong>Aliases</strong> <code>basic / default / classic / vertical</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Batons verticaux surmontes de points - le lollipop canonique pour valeurs categorielles classees.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/lollipop-basic.html"></iframe>
</div>
<div class="sp-variant" id="lollipop-fr-cleveland">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"cleveland"</code></span><span><strong>Aliases</strong> <code>cleveland / horizontal / h / row</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Cleveland dot plot horizontal - les longs libelles se lisent naturellement, les points s alignent sur l axe des valeurs.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/lollipop-cleveland.html"></iframe>
</div>
<div class="sp-variant" id="lollipop-fr-diverging">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"diverging"</code></span><span><strong>Aliases</strong> <code>diverging / div / signed / delta</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Batons pivotent autour de la moyenne: vert au-dessus, rouge en-dessous - parfait pour les ecarts.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/lollipop-diverging.html"></iframe>
</div>
<div class="sp-variant" id="lollipop-fr-circular">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"circular"</code></span><span><strong>Aliases</strong> <code>circular / polar / radial / round</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Disposition polaire ou chaque categorie est un rayon - tres lisible pour petits jeux et dashboards.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/lollipop-circular.html"></iframe>
</div>
<div class="sp-variant" id="lollipop-fr-highlight">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"highlight"</code></span><span><strong>Aliases</strong> <code>highlight / spotlight / focus / dim</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Tous les batons grises sauf un accent (auto-max ou `highlight_index`) - ideal pour focus editorial.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/lollipop-highlight.html"></iframe>
</div>
<div class="sp-variant" id="lollipop-fr-office">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"office"</code></span><span><strong>Aliases</strong> <code>office / grouped / season / panel</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Lollipops groupes avec moyenne par groupe et bande de couleur - inspire du chart IMDb de The Office.</p>

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/lollipop-office.html"></iframe>
</div>
</div>
</div>

</div>
