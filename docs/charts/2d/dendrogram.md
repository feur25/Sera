# Dendrogram

<div class="lang-en">

<style>
.sp-cls{display:flex;gap:0;margin:1.6em 0 1.6em 36px;border-radius:14px;background:linear-gradient(180deg,#0a0f1c 0%,#060912 100%);box-shadow:0 18px 50px -12px rgba(0,0,0,.6),0 0 0 1px #1e293b inset;position:relative;overflow:visible}.sp-cls-rail{display:flex;flex-direction:column;background:linear-gradient(180deg,#0d1426,#070b18);border-right:1px solid #1e293b;padding:18px 0;min-width:18px;transition:min-width .28s;position:relative;z-index:2;border-radius:14px 0 0 14px;overflow:visible}.sp-cls.sp-open .sp-cls-rail{min-width:170px;padding:18px 8px}.sp-cls-toggle{position:absolute;top:-14px;left:8px;padding:5px 9px;background:#1e293b;color:#a5b4fc;border:1px solid #312e81;border-radius:6px;cursor:pointer;font-size:12px;font-weight:700;line-height:1;z-index:5}.sp-cls-tab{position:relative;display:flex;align-items:center;gap:8px;margin:5px 0 5px -34px;padding:11px 16px 11px 14px;background:linear-gradient(90deg,#1a2540,#0f172a);color:#94a3b8;font-size:12px;font-weight:600;cursor:pointer;border:none;text-align:left;white-space:nowrap;border-radius:8px 0 0 8px;transition:all .25s;clip-path:polygon(0 0,calc(100% - 10px) 0,100% 50%,calc(100% - 10px) 100%,0 100%);min-height:18px}.sp-cls-tab.sp-cact{background:linear-gradient(90deg,#3730a3,#0f172a);color:#f5f3ff;margin-left:-46px;box-shadow:-3px 0 0 0 #818cf8 inset;font-weight:700;z-index:3}.sp-cls-tab .sp-cic{font-size:13px;flex-shrink:0;color:#a5b4fc;width:16px;text-align:center}.sp-cls-tab .sp-clb{display:none}.sp-cls.sp-open .sp-cls-tab .sp-clb{display:inline}.sp-cls-body{flex:1;padding:24px 26px 22px;background:#0a0f1c;min-width:0;border-radius:0 14px 14px 0;overflow:hidden}.sp-variant{display:none}.sp-variant.sp-von{display:block}.sp-vmeta{display:flex;flex-wrap:wrap;gap:8px 18px;align-items:center;font-size:13px;color:#94a3b8;margin:6px 0 16px;padding:10px 14px;background:rgba(99,102,241,.06);border-left:3px solid #6366f1;border-radius:0 6px 6px 0}.sp-vmeta strong{color:#a5b4fc;font-weight:700;margin-right:4px;text-transform:uppercase;font-size:11px}.sp-vmeta code{background:#1e293b;padding:2px 7px;border-radius:4px;color:#e2e8f0;font-size:12px}.sp-preview-frame{width:100%;height:400px;border:none;border-radius:10px;display:block;background:#0d1117;margin-top:10px}.sp-preview-label{font-size:11px;letter-spacing:.14em;font-weight:700;color:#818cf8;margin:20px 0 8px;text-transform:uppercase}
</style>
<script>
function spCls(scope,name,btn){var root=document.getElementById(scope);root.querySelectorAll('.sp-variant').forEach(function(s){s.classList.remove('sp-von')});root.querySelectorAll('.sp-cls-tab').forEach(function(b){b.classList.remove('sp-cact')});document.getElementById(scope+'-'+name).classList.add('sp-von');btn.classList.add('sp-cact')}
function spClsTog(id){document.getElementById(id).classList.toggle('sp-open')}
</script>

## Signature

`sp.dendrogram(title, labels, parents, *, variant="vertical", **kwargs) -> Chart`

Aliases: `sp.dendrogram`, `sp.dendro`, `sp.tree`, `sp.tree_diagram`, `sp.hierarchy`, `sp.hierarchical`

## Description

Dendrograms display hierarchical tree structures using right-angle elbow connectors (vertical/horizontal) or smooth bezier curves (elegant) or a radial circular layout. Parent–child relationships are defined by the `parents` list.

## Variants

| Variant | Description |
|---------|-------------|
| `"vertical"` | Root at top, leaves at bottom (elbow connectors) |
| `"horizontal"` | Root at left, leaves at right |
| `"radial"` | Circular radial tree layout |
| `"compact"` | Tighter spacing, smaller font |
| `"elegant"` | Smooth cubic bezier curves |

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `labels` | `list[str]` | Node names |
| `parents` | `list[str]` | Parent name per node (`""` = root) |
| `variant` | `str` | Layout style |
| `palette` | `list[int]` | Custom colors per tree branch |
| `show_labels` | `bool` | Show node labels |
| `line_width` | `float` | Connector line thickness |
| `width` / `height` | `int` | Chart dimensions |

## Returns

`Chart` — object with `.html` property and `.show()` method.

## Example

```python
import seraplot as sp
chart = sp.dendrogram(
    "Organization Chart",
    labels=["CEO", "CTO", "CFO", "Eng", "Design", "Finance", "Legal"],
    parents=["", "CEO", "CEO", "CTO", "CTO", "CFO", "CFO"],
)
chart.show()
```

<div class="sp-cls sp-open" id="dend-en">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('dend-en')" title="Collapse / expand">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('dend-en','vertical',this)"><span class="sp-cic">↓</span><span class="sp-clb">Vertical</span></button>
<button class="sp-cls-tab" onclick="spCls('dend-en','horizontal',this)"><span class="sp-cic">→</span><span class="sp-clb">Horizontal</span></button>
<button class="sp-cls-tab" onclick="spCls('dend-en','radial',this)"><span class="sp-cic">◎</span><span class="sp-clb">Radial</span></button>
<button class="sp-cls-tab" onclick="spCls('dend-en','compact',this)"><span class="sp-cic">≡</span><span class="sp-clb">Compact</span></button>
<button class="sp-cls-tab" onclick="spCls('dend-en','elegant',this)"><span class="sp-cic">∫</span><span class="sp-clb">Elegant</span></button>
</div>
<div class="sp-cls-body">
<div class="sp-variant sp-von" id="dend-en-vertical">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"vertical"</code></span><span><strong>Aliases</strong> <code>vertical / top / default / classic</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/dendrogram-vertical.html"></iframe>
</div>
<div class="sp-variant" id="dend-en-horizontal">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"horizontal"</code></span><span><strong>Aliases</strong> <code>horizontal / left / h</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/dendrogram-horizontal.html"></iframe>
</div>
<div class="sp-variant" id="dend-en-radial">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"radial"</code></span><span><strong>Aliases</strong> <code>radial / circular / polar</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/dendrogram-radial.html"></iframe>
</div>
<div class="sp-variant" id="dend-en-compact">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"compact"</code></span><span><strong>Aliases</strong> <code>compact / dense / tight</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/dendrogram-compact.html"></iframe>
</div>
<div class="sp-variant" id="dend-en-elegant">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"elegant"</code></span><span><strong>Aliases</strong> <code>elegant / smooth / rounded</code></span></div>
<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/dendrogram-elegant.html"></iframe>
</div>
</div>
</div>

</div>

<div class="lang-fr">

## Signature

`sp.dendrogram(title, labels, parents, *, variant="vertical", **kwargs) -> Chart`

Alias : `sp.dendrogram`, `sp.dendro`, `sp.tree`, `sp.tree_diagram`, `sp.hierarchy`, `sp.hierarchical`

## Description

Les dendrogrammes affichent des structures arborescentes hiérarchiques à l'aide de connecteurs en coude à angle droit (vertical/horizontal), de courbes de bézier lisses (elegant), ou d'une disposition radiale circulaire. Les relations parent-enfant sont définies par la liste `parents`.

## Variantes

| Variante | Description |
|---------|-------------|
| `"vertical"` | Racine en haut, feuilles en bas (connecteurs en coude) |
| `"horizontal"` | Racine à gauche, feuilles à droite |
| `"radial"` | Disposition arborescente radiale circulaire |
| `"compact"` | Espacement resserré, police plus petite |
| `"elegant"` | Courbes de bézier cubiques lisses |

## Paramètres

| Paramètre | Type | Description |
|-----------|------|-------------|
| `labels` | `list[str]` | Noms des nœuds |
| `parents` | `list[str]` | Nom du parent de chaque nœud (`""` = racine) |
| `variant` | `str` | Style de disposition |
| `palette` | `list[int]` | Couleurs personnalisées par branche |
| `show_labels` | `bool` | Afficher les labels des nœuds |
| `line_width` | `float` | Épaisseur des lignes de connexion |
| `width` / `height` | `int` | Dimensions du graphique |

## Retour

`Chart` — objet avec une propriété `.html` et une méthode `.show()`.

## Exemple

```python
import seraplot as sp
chart = sp.dendrogram(
    "Organization Chart",
    labels=["CEO", "CTO", "CFO", "Eng", "Design", "Finance", "Legal"],
    parents=["", "CEO", "CEO", "CTO", "CTO", "CFO", "CFO"],
)
chart.show()
```

<div class="sp-cls sp-open" id="dend-fr">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('dend-fr')" title="Réduire / déplier">⇆</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('dend-fr','vertical',this)"><span class="sp-cic">↓</span><span class="sp-clb">Vertical</span></button>
<button class="sp-cls-tab" onclick="spCls('dend-fr','horizontal',this)"><span class="sp-cic">→</span><span class="sp-clb">Horizontal</span></button>
<button class="sp-cls-tab" onclick="spCls('dend-fr','radial',this)"><span class="sp-cic">◎</span><span class="sp-clb">Radial</span></button>
<button class="sp-cls-tab" onclick="spCls('dend-fr','compact',this)"><span class="sp-cic">≡</span><span class="sp-clb">Compact</span></button>
<button class="sp-cls-tab" onclick="spCls('dend-fr','elegant',this)"><span class="sp-cic">∫</span><span class="sp-clb">Élégant</span></button>
</div>
<div class="sp-cls-body">
<div class="sp-variant sp-von" id="dend-fr-vertical">
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"vertical"</code></span><span><strong>Alias</strong> <code>vertical / top / default / classic</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/dendrogram-vertical.html"></iframe>
</div>
<div class="sp-variant" id="dend-fr-horizontal">
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"horizontal"</code></span><span><strong>Alias</strong> <code>horizontal / left / h</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/dendrogram-horizontal.html"></iframe>
</div>
<div class="sp-variant" id="dend-fr-radial">
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"radial"</code></span><span><strong>Alias</strong> <code>radial / circular / polar</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/dendrogram-radial.html"></iframe>
</div>
<div class="sp-variant" id="dend-fr-compact">
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"compact"</code></span><span><strong>Alias</strong> <code>compact / dense / tight</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/dendrogram-compact.html"></iframe>
</div>
<div class="sp-variant" id="dend-fr-elegant">
<div class="sp-vmeta"><span><strong>Variante</strong> <code>"elegant"</code></span><span><strong>Alias</strong> <code>elegant / smooth / rounded</code></span></div>
<div class="sp-preview-label">Aperçu</div>
<iframe class="sp-preview-frame" src="../../previews/dendrogram-elegant.html"></iframe>
</div>
</div>
</div>

</div>
