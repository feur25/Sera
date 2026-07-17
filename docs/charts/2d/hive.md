# Hive Plot

<div class="lang-en">

<style>
.sp-cls{display:flex;gap:0;margin:1.6em 0 1.6em 36px;border-radius:14px;background:linear-gradient(180deg,#0a0f1c 0%,#060912 100%);box-shadow:0 18px 50px -12px rgba(0,0,0,.6),0 0 0 1px #1e293b inset;position:relative;overflow:visible}.sp-cls-rail{display:flex;flex-direction:column;background:linear-gradient(180deg,#0d1426,#070b18);border-right:1px solid #1e293b;padding:18px 0;min-width:18px;transition:min-width .28s;position:relative;z-index:2;border-radius:14px 0 0 14px;overflow:visible}.sp-cls.sp-open .sp-cls-rail{min-width:170px;padding:18px 8px}.sp-cls-toggle{position:absolute;top:-14px;left:8px;padding:5px 9px;background:#1e293b;color:#a5b4fc;border:1px solid #312e81;border-radius:6px;cursor:pointer;font-size:12px;font-weight:700;line-height:1;z-index:5}.sp-cls-tab{position:relative;display:flex;align-items:center;gap:8px;margin:5px 0 5px -34px;padding:11px 16px 11px 14px;background:linear-gradient(90deg,#1a2540,#0f172a);color:#94a3b8;font-size:12px;font-weight:600;cursor:pointer;border:none;text-align:left;white-space:nowrap;border-radius:8px 0 0 8px;transition:all .25s;clip-path:polygon(0 0,calc(100% - 10px) 0,100% 50%,calc(100% - 10px) 100%,0 100%);min-height:18px}.sp-cls-tab.sp-cact{background:linear-gradient(90deg,#3730a3,#0f172a);color:#f5f3ff;margin-left:-46px;box-shadow:-3px 0 0 0 #818cf8 inset;font-weight:700;z-index:3}.sp-cls-tab .sp-cic{font-size:13px;flex-shrink:0;color:#a5b4fc;width:16px;text-align:center}.sp-cls-tab .sp-clb{display:none}.sp-cls.sp-open .sp-cls-tab .sp-clb{display:inline}.sp-cls-body{flex:1;padding:24px 26px 22px;background:#0a0f1c;min-width:0;border-radius:0 14px 14px 0;overflow:hidden}.sp-variant{display:none}.sp-variant.sp-von{display:block}.sp-vmeta{display:flex;flex-wrap:wrap;gap:8px 18px;align-items:center;font-size:13px;color:#94a3b8;margin:6px 0 16px;padding:10px 14px;background:rgba(99,102,241,.06);border-left:3px solid #6366f1;border-radius:0 6px 6px 0}.sp-vmeta strong{color:#a5b4fc;font-weight:700;margin-right:4px;text-transform:uppercase;font-size:11px}.sp-vmeta code{background:#1e293b;padding:2px 7px;border-radius:4px;color:#e2e8f0;font-size:12px}.sp-preview-frame{width:100%;height:420px;border:none;border-radius:10px;display:block;background:#0d1117;margin-top:10px}.sp-preview-label{font-size:11px;letter-spacing:.14em;font-weight:700;color:#818cf8;margin:20px 0 8px;text-transform:uppercase}
</style>
<script>
function spCls(scope,name,btn){var root=document.getElementById(scope);root.querySelectorAll('.sp-variant').forEach(function(s){s.classList.remove('sp-von')});root.querySelectorAll('.sp-cls-tab').forEach(function(b){b.classList.remove('sp-cact')});document.getElementById(scope+'-'+name).classList.add('sp-von');btn.classList.add('sp-cact')}
function spClsTog(id){document.getElementById(id).classList.toggle('sp-open')}
</script>

## Signature

`sp.hive(title, axes, labels, categories, values, edges_i, edges_j, edges_w, *, variant="basic", **kwargs) -> Chart`

Aliases: `sp.hive`, `sp.hive_plot`, `sp.hive_chart`, `sp.hive_graph`, `sp.radial_network`

## Description

Hive plots organize network nodes on radial axes by category. Each axis corresponds to one node group (`axes`). Node position along the axis is determined by `values` (0–1). Edges between nodes are drawn as straight or curved lines through the center.

## Variants

| Variant | Description |
|---------|-------------|
| `"basic"` | Straight edge lines |
| `"curved"` | Cubic bezier curves through center |
| `"gradient"` | Curved + source-color gradient |
| `"weighted"` | Stroke width proportional to edge weight |
| `"minimal"` | Thin curved edges |

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `axes` | `list[str]` | Axis (group) names |
| `labels` | `list[str]` | Node names |
| `categories` | `list[str]` | Group assignment per node |
| `values` | `list[float]` | Node position along axis (0–1) |
| `edges_i` | `list[int]` | Source node indices |
| `edges_j` | `list[int]` | Target node indices |
| `edges_w` | `list[float]` | Edge weights |
| `variant` | `str` | Rendering style |
| `width` / `height` | `int` | Chart dimensions |

## Returns

`Chart` — object with `.html` property and `.show()` method.

## Example

```python
import seraplot as sp
chart = sp.hive(
    "Protein Interactions",
    axes=["Receptor", "Kinase", "Transcription"],
    labels=["R1","R2","K1","K2","T1","T2"],
    categories=["Receptor","Receptor","Kinase","Kinase","Transcription","Transcription"],
    values=[0.3, 0.7, 0.4, 0.8, 0.5, 0.9],
    edges_i=[0, 1, 2, 3],
    edges_j=[2, 3, 4, 5],
    edges_w=[1.5, 2.0, 1.0, 2.5],
)
chart.show()
```

<div class="sp-cls sp-open" id="hive-en">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('hive-en')" title="Collapse / expand">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('hive-en','basic',this)"><span class="sp-cic">✦</span><span class="sp-clb">Basic</span></button>
<button class="sp-cls-tab" onclick="spCls('hive-en','curved',this)"><span class="sp-cic">∫</span><span class="sp-clb">Curved</span></button>
<button class="sp-cls-tab" onclick="spCls('hive-en','gradient',this)"><span class="sp-cic">◐</span><span class="sp-clb">Gradient</span></button>
<button class="sp-cls-tab" onclick="spCls('hive-en','weighted',this)"><span class="sp-cic">≈</span><span class="sp-clb">Weighted</span></button>
<button class="sp-cls-tab" onclick="spCls('hive-en','minimal',this)"><span class="sp-cic">—</span><span class="sp-clb">Minimal</span></button>
</div>
<div class="sp-cls-body">
<div class="sp-variant sp-von" id="hive-en-basic">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"basic"</code></span><span><strong>Aliases</strong> <code>basic / default / classic</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/hive-basic.html"></iframe>
</div>
<div class="sp-variant" id="hive-en-curved">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"curved"</code></span><span><strong>Aliases</strong> <code>curved / smooth / bezier</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/hive-curved.html"></iframe>
</div>
<div class="sp-variant" id="hive-en-gradient">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"gradient"</code></span><span><strong>Aliases</strong> <code>gradient / color</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/hive-gradient.html"></iframe>
</div>
<div class="sp-variant" id="hive-en-weighted">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"weighted"</code></span><span><strong>Aliases</strong> <code>weighted / width / value</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/hive-weighted.html"></iframe>
</div>
<div class="sp-variant" id="hive-en-minimal">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"minimal"</code></span><span><strong>Aliases</strong> <code>minimal / thin / clean</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/hive-minimal.html"></iframe>
</div>
</div>
</div>

</div>

<div class="lang-fr">

## Signature

`sp.hive(title, axes, labels, categories, values, edges_i, edges_j, edges_w, *, variant="basic", **kwargs) -> Chart`

Alias : `sp.hive`, `sp.hive_plot`, `sp.hive_chart`, `sp.hive_graph`, `sp.radial_network`

## Description

Les hive plots organisent les nœuds d'un réseau sur des axes radiaux par catégorie. Chaque axe correspond à un groupe de nœuds (`axes`). La position d'un nœud le long de l'axe est déterminée par `values` (0–1). Les arêtes entre nœuds sont tracées en lignes droites ou courbes passant par le centre.

## Variantes

| Variante | Description |
|---------|-------------|
| `"basic"` | Arêtes en lignes droites |
| `"curved"` | Courbes de bézier cubiques passant par le centre |
| `"gradient"` | Courbé + dégradé de couleur source |
| `"weighted"` | Épaisseur du trait proportionnelle au poids de l'arête |
| `"minimal"` | Arêtes courbes fines |

## Paramètres

| Paramètre | Type | Description |
|-----------|------|-------------|
| `axes` | `list[str]` | Noms des axes (groupes) |
| `labels` | `list[str]` | Noms des nœuds |
| `categories` | `list[str]` | Groupe assigné à chaque nœud |
| `values` | `list[float]` | Position du nœud le long de l'axe (0–1) |
| `edges_i` | `list[int]` | Indices des nœuds source |
| `edges_j` | `list[int]` | Indices des nœuds cible |
| `edges_w` | `list[float]` | Poids des arêtes |
| `variant` | `str` | Style de rendu |
| `width` / `height` | `int` | Dimensions du graphique |

## Retour

`Chart` — objet avec une propriété `.html` et une méthode `.show()`.

## Exemple

```python
import seraplot as sp
chart = sp.hive(
    "Protein Interactions",
    axes=["Receptor", "Kinase", "Transcription"],
    labels=["R1","R2","K1","K2","T1","T2"],
    categories=["Receptor","Receptor","Kinase","Kinase","Transcription","Transcription"],
    values=[0.3, 0.7, 0.4, 0.8, 0.5, 0.9],
    edges_i=[0, 1, 2, 3],
    edges_j=[2, 3, 4, 5],
    edges_w=[1.5, 2.0, 1.0, 2.5],
)
chart.show()
```

<div class="sp-cls sp-open" id="hive-fr">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('hive-fr')" title="Réduire / déplier">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('hive-fr','basic',this)"><span class="sp-cic">✦</span><span class="sp-clb">Basique</span></button>
<button class="sp-cls-tab" onclick="spCls('hive-fr','curved',this)"><span class="sp-cic">∫</span><span class="sp-clb">Courbé</span></button>
<button class="sp-cls-tab" onclick="spCls('hive-fr','gradient',this)"><span class="sp-cic">◐</span><span class="sp-clb">Dégradé</span></button>
<button class="sp-cls-tab" onclick="spCls('hive-fr','weighted',this)"><span class="sp-cic">≈</span><span class="sp-clb">Pondéré</span></button>
<button class="sp-cls-tab" onclick="spCls('hive-fr','minimal',this)"><span class="sp-cic">—</span><span class="sp-clb">Minimal</span></button>
</div>
<div class="sp-cls-body">
<div class="sp-variant sp-von" id="hive-fr-basic">
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"basic"</code></span><span><strong>Alias</strong> <code>basic / default / classic</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/hive-basic.html"></iframe>
</div>
<div class="sp-variant" id="hive-fr-curved">
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"curved"</code></span><span><strong>Alias</strong> <code>curved / smooth / bezier</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/hive-curved.html"></iframe>
</div>
<div class="sp-variant" id="hive-fr-gradient">
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"gradient"</code></span><span><strong>Alias</strong> <code>gradient / color</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/hive-gradient.html"></iframe>
</div>
<div class="sp-variant" id="hive-fr-weighted">
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"weighted"</code></span><span><strong>Alias</strong> <code>weighted / width / value</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/hive-weighted.html"></iframe>
</div>
<div class="sp-variant" id="hive-fr-minimal">
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"minimal"</code></span><span><strong>Alias</strong> <code>minimal / thin / clean</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/hive-minimal.html"></iframe>
</div>
</div>
</div>

</div>
