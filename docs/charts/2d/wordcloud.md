# Word Cloud - Six Rendering Architectures

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

`sp.build_wordcloud(title, words, frequencies, *, variant="basic", shape="rect", **kwargs) -> Chart`


## Description

`sp.build_wordcloud()` packs weighted tokens into six rendering architectures. **Basic** is the canonical spiral packer driven by a parametric `shape=` mask (rect, circle, heart, bird, glasses, diamond, star). **Bubble** gives each word its own color-filled disc sized by frequency - a packed-bubble layout. **Context** is an InfraNodus-style text-network cloud: words positioned by a force-directed layout driven by co-occurrence edges so semantically close words cluster spatially, colored by community. **Image** accepts any binary pixel mask (logo, icon, photo). **LabelMap** draws a datamapplot-style clustered scatter with leader-line labels. **Network** renders a keyword co-occurrence graph with bezier-curved edges.

### Shapes (for variant `basic`)

The `basic` variant accepts a `shape=` argument that selects the silhouette mask:

| Shape | Aliases | Description |
|---|---|---|
| `"rect"` | `rect / rectangle / box / default` | Rectangular Archimedean spiral - the textbook word cloud. |
| `"circle"` | `circle / round / disk / ball` | Words packed inside a perfect disc. |
| `"heart"` | `heart / love / valentine` | Cardioid heart silhouette. |
| `"bird"` | `bird / twitter / tweet / icon` | Composite-disk stylised bird silhouette. |
| `"glasses"` | `glasses / sunglasses / shades / specs` | Sunglasses silhouette (two ellipses + bridge). |
| `"diamond"` | `diamond / rhombus / lozenge` | Rotated square / rhombus silhouette. |
| `"star"` | `star / starburst / 5-point` | 5-pointed star silhouette. |

## Parameters

| Parameter | Used by variants |
|-----------|------------------|
| `height` | all |
| `hover` | all |
| `palette` | all |
| `title` | all |
| `width` | all |

## Returns

`Chart` - object with `.html` property and `.show()` method.

---

</div>

<div class="lang-fr">

<h2>Signature</h2>

`sp.build_wordcloud(title, words, frequencies, *, variant="basic", shape="rect", **kwargs) -> Chart`


<h2>Description</h2>

`sp.build_wordcloud()` propose six architectures de rendu. **Basic** est le packer spirale canonique pilote par un masque `shape=` (rect, circle, heart, bird, glasses, diamond, star). **Bubble** donne a chaque mot un disque colore dimensionne par frequence - un layout bubble-packed. **Context** est un nuage texte-reseau style InfraNodus : mots positionnes par layout force-dirige base sur les aretes de co-occurrence, colores par communaute. **Image** accepte n importe quel masque binaire de pixels. **LabelMap** dessine un scatter clusterise style datamapplot avec etiquettes en lignes de rappel. **Network** rend un graphe de co-occurrence de mots-cles avec aretes bezier.

<h3>Formes (pour la variante `basic`)</h3>

La variante `basic` accepte un argument `shape=` :

| Forme | Alias | Description |
|---|---|---|
| `"rect"` | `rect / rectangle / box / default` | Rectangular Archimedean spiral - the textbook word cloud. |
| `"circle"` | `circle / round / disk / ball` | Words packed inside a perfect disc. |
| `"heart"` | `heart / love / valentine` | Cardioid heart silhouette. |
| `"bird"` | `bird / twitter / tweet / icon` | Composite-disk stylised bird silhouette. |
| `"glasses"` | `glasses / sunglasses / shades / specs` | Sunglasses silhouette (two ellipses + bridge). |
| `"diamond"` | `diamond / rhombus / lozenge` | Rotated square / rhombus silhouette. |
| `"star"` | `star / starburst / 5-point` | 5-pointed star silhouette. |

<h2>Parametres</h2>

| Parametre | Type | Defaut | Description |
|---|---|---|---|
| `title` | `str` | requis | Titre du graphique |
| `words` | `list[str]` | requis | Tokens (toutes variantes sauf labelmap) |
| `frequencies` | `list[float]` | requis | Poids par mot (controle la taille) |
| `variant` | `str` | "basic" | Mode de rendu (voir Variantes) |
| `shape` | `str` | "rect" | Sous-forme pour `basic` (voir Formes) |
| `mask` | `list[int]` | None | Masque binaire (variante `image`) - row-major, 1=interieur |
| `mask_width` | `int` | 0 | Largeur du masque (variante `image`) |
| `mask_height` | `int` | 0 | Hauteur du masque (variante `image`) |
| `points_x` | `list[float]` | None | Coords x du scatter (variante `labelmap`) |
| `points_y` | `list[float]` | None | Coords y du scatter (variante `labelmap`) |
| `category_indices` | `list[int]` | None | Id de cluster par point/mot (`labelmap`, `context`) |
| `cluster_labels` | `list[str]` | None | Etiquette par cluster (variante `labelmap`) |
| `edges_i` | `list[int]` | None | Indices source des aretes (variantes `network`, `context`) |
| `edges_j` | `list[int]` | None | Indices cible des aretes (variantes `network`, `context`) |
| `edges_w` | `list[float]` | None | Poids des aretes (variantes `network`, `context`) |
| `min_font` | `float` | 12.0 | Plus petite taille de police rendue |
| `max_font` | `float` | 72.0 | Plus grande taille de police rendue |
| `palette` | `list[int]` | None | Palette personnalisee |
| `bg_color` | `str` | auto | Couleur de fond |
| `width` | `int` | 900 | Largeur du canvas (px) |
| `height` | `int` | 500 | Hauteur du canvas (px) |

<h2>Retour</h2>

`Chart` - objet avec propriete `.html` et methode `.show()`.

</div>
