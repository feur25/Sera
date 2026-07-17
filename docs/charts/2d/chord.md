# Chord Diagram

<div class="lang-en">

<style>
.sp-cls{display:flex;gap:0;margin:1.6em 0 1.6em 36px;border-radius:14px;background:linear-gradient(180deg,#0a0f1c 0%,#060912 100%);box-shadow:0 18px 50px -12px rgba(0,0,0,.6),0 0 0 1px #1e293b inset;position:relative;overflow:visible}.sp-cls-rail{display:flex;flex-direction:column;background:linear-gradient(180deg,#0d1426,#070b18);border-right:1px solid #1e293b;padding:18px 0;min-width:18px;transition:min-width .28s cubic-bezier(.5,0,.2,1);position:relative;z-index:2;border-radius:14px 0 0 14px;overflow:visible}.sp-cls.sp-open .sp-cls-rail{min-width:170px;padding:18px 8px}.sp-cls-toggle{position:absolute;top:-14px;left:8px;padding:5px 9px;background:#1e293b;color:#a5b4fc;border:1px solid #312e81;border-radius:6px;cursor:pointer;font-size:12px;font-weight:700;transition:all .15s;line-height:1;z-index:5;box-shadow:0 4px 12px -2px rgba(0,0,0,.5)}.sp-cls-tab{position:relative;display:flex;align-items:center;gap:8px;margin:5px 0 5px -34px;padding:11px 16px 11px 14px;background:linear-gradient(90deg,#1a2540 0%,#141d33 70%,#0f172a 100%);color:#94a3b8;font-size:12px;font-weight:600;cursor:pointer;border:none;text-align:left;white-space:nowrap;border-radius:8px 0 0 8px;box-shadow:-6px 4px 14px -4px rgba(0,0,0,.55);transition:all .25s cubic-bezier(.5,0,.2,1);clip-path:polygon(0 0,calc(100% - 10px) 0,100% 50%,calc(100% - 10px) 100%,0 100%);min-height:18px}.sp-cls-tab.sp-cact{background:linear-gradient(90deg,#3730a3 0%,#1e1b4b 50%,#0f172a 100%);color:#f5f3ff;margin-left:-46px;box-shadow:-10px 8px 22px -4px rgba(99,102,241,.35),-3px 0 0 0 #818cf8 inset;font-weight:700;z-index:3}.sp-cls-tab .sp-cic{font-size:13px;flex-shrink:0;color:#a5b4fc;font-weight:900;width:16px;text-align:center}.sp-cls-tab .sp-clb{display:none}.sp-cls.sp-open .sp-cls-tab .sp-clb{display:inline}.sp-cls-body{flex:1;padding:24px 26px 22px;background:#0a0f1c;min-width:0;border-radius:0 14px 14px 0;overflow:hidden}.sp-variant{display:none}.sp-variant.sp-von{display:block}.sp-vmeta{display:flex;flex-wrap:wrap;gap:8px 18px;align-items:center;font-size:13px;color:#94a3b8;margin:6px 0 16px;padding:10px 14px;background:rgba(99,102,241,.06);border-left:3px solid #6366f1;border-radius:0 6px 6px 0}.sp-vmeta strong{color:#a5b4fc;font-weight:700;margin-right:4px;letter-spacing:.04em;text-transform:uppercase;font-size:11px}.sp-vmeta code{background:#1e293b;padding:2px 7px;border-radius:4px;color:#e2e8f0;font-size:12px}.sp-preview-frame{width:100%;height:420px;border:none;border-radius:10px;display:block;background:#0d1117;margin-top:10px}.sp-preview-label{font-size:11px;letter-spacing:.14em;font-weight:700;color:#818cf8;margin:20px 0 8px;text-transform:uppercase}
</style>
<script>
function spCls(scope,name,btn){var root=document.getElementById(scope);root.querySelectorAll('.sp-variant').forEach(function(s){s.classList.remove('sp-von')});root.querySelectorAll('.sp-cls-tab').forEach(function(b){b.classList.remove('sp-cact')});document.getElementById(scope+'-'+name).classList.add('sp-von');btn.classList.add('sp-cact')}
function spClsTog(id){document.getElementById(id).classList.toggle('sp-open')}
</script>

## Signature

`sp.chord(title, labels, matrix, *, variant="basic", **kwargs) -> Chart`

Aliases: `sp.chord`, `sp.chord_chart`, `sp.chord_diagram`

## Description

Chord diagrams show relationships between entities using arcs and ribbons around a circle. The `matrix` is an N×N flow matrix where `matrix[i][j]` is the flow from node `i` to node `j`.

## Variants

| Variant | Description |
|---------|-------------|
| `"basic"` | Standard filled ribbons |
| `"gradient"` | Color-interpolated ribbons |
| `"ribbon"` | Wider ribbon links |
| `"arc"` | Arc-only (no filled ribbons) |
| `"mono"` | Single-color monochrome |

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `labels` | `list[str]` | Node names |
| `matrix` | `list[list[float]]` | N×N flow matrix |
| `variant` | `str` | Rendering style |
| `palette` | `list[int]` | Custom colors |
| `gap_deg` | `float` | Gap between arcs in degrees |
| `arc_width` | `float` | Arc band thickness |
| `width` / `height` | `int` | Chart dimensions (default 700×700) |

## Returns

`Chart` — object with `.html` property and `.show()` method.

## Example

```python
import seraplot as sp
chart = sp.chord(
    "Trade Flows",
    labels=["US", "EU", "China", "Japan"],
    matrix=[
        [0, 40, 30, 20],
        [35, 0, 25, 15],
        [28, 22, 0, 18],
        [18, 12, 16, 0],
    ],
)
chart.show()
```

<div class="sp-cls sp-open" id="chord-en">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('chord-en')" title="Collapse / expand">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('chord-en','basic',this)"><span class="sp-cic">◉</span><span class="sp-clb">Basic</span></button>
<button class="sp-cls-tab" onclick="spCls('chord-en','gradient',this)"><span class="sp-cic">◐</span><span class="sp-clb">Gradient</span></button>
<button class="sp-cls-tab" onclick="spCls('chord-en','ribbon',this)"><span class="sp-cic">▬</span><span class="sp-clb">Ribbon</span></button>
<button class="sp-cls-tab" onclick="spCls('chord-en','arc',this)"><span class="sp-cic">◌</span><span class="sp-clb">Arc</span></button>
<button class="sp-cls-tab" onclick="spCls('chord-en','mono',this)"><span class="sp-cic">○</span><span class="sp-clb">Mono</span></button>
</div>
<div class="sp-cls-body">
<div class="sp-variant sp-von" id="chord-en-basic">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"basic"</code></span><span><strong>Aliases</strong> <code>basic / default / classic</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/chord-basic.html"></iframe>
</div>
<div class="sp-variant" id="chord-en-gradient">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"gradient"</code></span><span><strong>Aliases</strong> <code>gradient / color</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/chord-gradient.html"></iframe>
</div>
<div class="sp-variant" id="chord-en-ribbon">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"ribbon"</code></span><span><strong>Aliases</strong> <code>ribbon / wide</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/chord-ribbon.html"></iframe>
</div>
<div class="sp-variant" id="chord-en-arc">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"arc"</code></span><span><strong>Aliases</strong> <code>arc / outline</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/chord-arc.html"></iframe>
</div>
<div class="sp-variant" id="chord-en-mono">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"mono"</code></span><span><strong>Aliases</strong> <code>mono / single</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/chord-mono.html"></iframe>
</div>
</div>
</div>

</div>

<div class="lang-fr">

## Signature

`sp.chord(title, labels, matrix, *, variant="basic", **kwargs) -> Chart`

Alias : `sp.chord`, `sp.chord_chart`, `sp.chord_diagram`

## Description

Les diagrammes en accords (chord) montrent les relations entre entités à l'aide d'arcs et de rubans autour d'un cercle. `matrix` est une matrice de flux N×N où `matrix[i][j]` est le flux du nœud `i` vers le nœud `j`.

## Variantes

| Variante | Description |
|---------|-------------|
| `"basic"` | Rubans pleins standards |
| `"gradient"` | Rubans à couleur interpolée |
| `"ribbon"` | Liens en rubans plus larges |
| `"arc"` | Arcs seuls (sans rubans pleins) |
| `"mono"` | Monochrome, couleur unique |

## Paramètres

| Paramètre | Type | Description |
|-----------|------|-------------|
| `labels` | `list[str]` | Noms des nœuds |
| `matrix` | `list[list[float]]` | Matrice de flux N×N |
| `variant` | `str` | Style de rendu |
| `palette` | `list[int]` | Couleurs personnalisées |
| `gap_deg` | `float` | Espace entre arcs, en degrés |
| `arc_width` | `float` | Épaisseur de la bande d'arc |
| `width` / `height` | `int` | Dimensions du graphique (défaut 700×700) |

## Retour

`Chart` — objet avec une propriété `.html` et une méthode `.show()`.

## Exemple

```python
import seraplot as sp
chart = sp.chord(
    "Trade Flows",
    labels=["US", "EU", "China", "Japan"],
    matrix=[
        [0, 40, 30, 20],
        [35, 0, 25, 15],
        [28, 22, 0, 18],
        [18, 12, 16, 0],
    ],
)
chart.show()
```

<div class="sp-cls sp-open" id="chord-fr">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('chord-fr')" title="Réduire / déplier">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('chord-fr','basic',this)"><span class="sp-cic">◉</span><span class="sp-clb">Basique</span></button>
<button class="sp-cls-tab" onclick="spCls('chord-fr','gradient',this)"><span class="sp-cic">◐</span><span class="sp-clb">Dégradé</span></button>
<button class="sp-cls-tab" onclick="spCls('chord-fr','ribbon',this)"><span class="sp-cic">▬</span><span class="sp-clb">Ruban</span></button>
<button class="sp-cls-tab" onclick="spCls('chord-fr','arc',this)"><span class="sp-cic">◌</span><span class="sp-clb">Arc</span></button>
<button class="sp-cls-tab" onclick="spCls('chord-fr','mono',this)"><span class="sp-cic">○</span><span class="sp-clb">Mono</span></button>
</div>
<div class="sp-cls-body">
<div class="sp-variant sp-von" id="chord-fr-basic">
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"basic"</code></span><span><strong>Alias</strong> <code>basic / default / classic</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/chord-basic.html"></iframe>
</div>
<div class="sp-variant" id="chord-fr-gradient">
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"gradient"</code></span><span><strong>Alias</strong> <code>gradient / color</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/chord-gradient.html"></iframe>
</div>
<div class="sp-variant" id="chord-fr-ribbon">
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"ribbon"</code></span><span><strong>Alias</strong> <code>ribbon / wide</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/chord-ribbon.html"></iframe>
</div>
<div class="sp-variant" id="chord-fr-arc">
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"arc"</code></span><span><strong>Alias</strong> <code>arc / outline</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/chord-arc.html"></iframe>
</div>
<div class="sp-variant" id="chord-fr-mono">
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"mono"</code></span><span><strong>Alias</strong> <code>mono / single</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/chord-mono.html"></iframe>
</div>
</div>
</div>

</div>
