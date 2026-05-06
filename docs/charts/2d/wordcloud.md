# Word Cloud - Weighted Tokens in a Shape

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

`sp.build_wordcloud(title, words, frequencies, *, variant="basic", min_font=12, max_font=72, **kwargs) -> Chart`

Aliases: `sp.build_wordcloud` &middot; `sp.wordcloud` &middot; `sp.word_cloud` &middot; `sp.tag_cloud` &middot; `sp.cloud` &middot; `sp.text_cloud` &middot; `sp.token_cloud`

## Description

`sp.build_wordcloud()` packs weighted tokens inside a parametric shape mask. Font size encodes the frequency, color rotates through the palette, and the `variant` keyword swaps the boundary - rectangular spiral, circle, heart, stylised bird, packed bubbles or sunglasses silhouette - so the same data can be packaged for editorial, dashboard or social contexts.

## Variants

| Variant | Aliases | Description |
|---|---|---|
| `"basic"` | `basic / default / spiral / rect` | Classic Archimedean-spiral packing inside a rectangle - the textbook word cloud. |
| `"circle"` | `circle / round / disk / ball` | Words packed inside a perfect disc - clean editorial silhouette ideal for thumbnails. |
| `"heart"` | `heart / love / valentine` | Cardioid heart silhouette - perfect for sentiment, brand-love or community storytelling. |
| `"bird"` | `bird / twitter / tweet / icon` | Composite-disk silhouette evoking a stylised bird - perfect for social-feed buzz boards. |
| `"bubble"` | `bubble / bubbles / packed / packing / circles` | Each token is rendered as its own colored disc, sized by frequency - editorial bubble pack. |
| `"glasses"` | `glasses / sunglasses / shades / specs` | Sunglasses silhouette (two ellipses + bridge) - playful headline visual for lifestyle decks. |

## Parameters

| Parameter | Type | Default | Description |
|---|---|---|---|
| `title` | `str` | required | Chart title |
| `words` | `list[str]` | required | Tokens to render |
| `frequencies` | `list[float]` | required | Weight per word (controls size) |
| `variant` | `str` | "basic" | Shape mask (see table) |
| `min_font` | `float` | 12.0 | Smallest rendered font size |
| `max_font` | `float` | 72.0 | Largest rendered font size |
| `palette` | `list[int]` | None | Custom palette |
| `bg_color` | `str` | "#1a1a2e" | Background color |
| `width` | `int` | 900 | Canvas width (px) |
| `height` | `int` | 500 | Canvas height (px) |

## Returns

`Chart` - object with `.html` property and `.show()` method.

---

<div class="sp-cls sp-open" id="wordcloud-en">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('wordcloud-en')" title="Toggle">&#x21C6;</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('wordcloud-en','basic',this)"><span class="sp-cic">B</span><span class="sp-clb">Basic</span></button>
<button class="sp-cls-tab" onclick="spCls('wordcloud-en','circle',this)"><span class="sp-cic">C</span><span class="sp-clb">Circle</span></button>
<button class="sp-cls-tab" onclick="spCls('wordcloud-en','heart',this)"><span class="sp-cic">H</span><span class="sp-clb">Heart</span></button>
<button class="sp-cls-tab" onclick="spCls('wordcloud-en','bird',this)"><span class="sp-cic">T</span><span class="sp-clb">Bird</span></button>
<button class="sp-cls-tab" onclick="spCls('wordcloud-en','bubble',this)"><span class="sp-cic">U</span><span class="sp-clb">Bubble</span></button>
<button class="sp-cls-tab" onclick="spCls('wordcloud-en','glasses',this)"><span class="sp-cic">G</span><span class="sp-clb">Glasses</span></button>
</div>
<div class="sp-cls-body">
<div class="sp-variant sp-von" id="wordcloud-en-basic">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"basic"</code></span><span><strong>Aliases</strong> <code>basic / default / spiral / rect</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Classic Archimedean-spiral packing inside a rectangle - the textbook word cloud.</p>

<div class="sp-preview-label">Code</div>

```python
import seraplot as sp

words = ["Python", "Rust", "Data", "ML", "AI", "Cloud", "GPU", "Vector",
         "Tensor", "Graph", "Pandas", "NumPy", "Polars", "Arrow"]
freqs = [98, 92, 88, 80, 74, 66, 60, 54, 48, 42, 36, 30, 24, 18]

chart = sp.build_wordcloud(
    title="Tech stack", words=words, frequencies=freqs,
    variant="basic",
    palette=[0x6366F1, 0x22D3EE, 0xF59E0B, 0xEF4444,
             0x10B981, 0xA855F7, 0xEC4899, 0x14B8A6],
    bg_color="#0f172a", min_font=12, max_font=58,
    width=720, height=440,
)
chart.show()
```

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/wordcloud-basic.html"></iframe>
</div>
<div class="sp-variant" id="wordcloud-en-circle">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"circle"</code></span><span><strong>Aliases</strong> <code>circle / round / disk / ball</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Words packed inside a perfect disc - clean editorial silhouette ideal for thumbnails.</p>

<div class="sp-preview-label">Code</div>

```python
import seraplot as sp

words = ["Python", "Rust", "Data", "ML", "AI", "Cloud", "GPU", "Vector",
         "Tensor", "Graph", "Pandas", "NumPy", "Polars", "Arrow"]
freqs = [98, 92, 88, 80, 74, 66, 60, 54, 48, 42, 36, 30, 24, 18]

chart = sp.build_wordcloud(
    title="Tech stack", words=words, frequencies=freqs,
    variant="circle",
    palette=[0x6366F1, 0x22D3EE, 0xF59E0B, 0xEF4444,
             0x10B981, 0xA855F7, 0xEC4899, 0x14B8A6],
    bg_color="#0f172a", min_font=12, max_font=58,
    width=720, height=440,
)
chart.show()
```

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/wordcloud-circle.html"></iframe>
</div>
<div class="sp-variant" id="wordcloud-en-heart">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"heart"</code></span><span><strong>Aliases</strong> <code>heart / love / valentine</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Cardioid heart silhouette - perfect for sentiment, brand-love or community storytelling.</p>

<div class="sp-preview-label">Code</div>

```python
import seraplot as sp

words = ["Python", "Rust", "Data", "ML", "AI", "Cloud", "GPU", "Vector",
         "Tensor", "Graph", "Pandas", "NumPy", "Polars", "Arrow"]
freqs = [98, 92, 88, 80, 74, 66, 60, 54, 48, 42, 36, 30, 24, 18]

chart = sp.build_wordcloud(
    title="Tech stack", words=words, frequencies=freqs,
    variant="heart",
    palette=[0x6366F1, 0x22D3EE, 0xF59E0B, 0xEF4444,
             0x10B981, 0xA855F7, 0xEC4899, 0x14B8A6],
    bg_color="#fdf2f8", min_font=12, max_font=58,
    width=720, height=440,
)
chart.show()
```

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/wordcloud-heart.html"></iframe>
</div>
<div class="sp-variant" id="wordcloud-en-bird">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"bird"</code></span><span><strong>Aliases</strong> <code>bird / twitter / tweet / icon</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Composite-disk silhouette evoking a stylised bird - perfect for social-feed buzz boards.</p>

<div class="sp-preview-label">Code</div>

```python
import seraplot as sp

words = ["Python", "Rust", "Data", "ML", "AI", "Cloud", "GPU", "Vector",
         "Tensor", "Graph", "Pandas", "NumPy", "Polars", "Arrow"]
freqs = [98, 92, 88, 80, 74, 66, 60, 54, 48, 42, 36, 30, 24, 18]

chart = sp.build_wordcloud(
    title="Tech stack", words=words, frequencies=freqs,
    variant="bird",
    palette=[0x6366F1, 0x22D3EE, 0xF59E0B, 0xEF4444,
             0x10B981, 0xA855F7, 0xEC4899, 0x14B8A6],
    bg_color="#0f172a", min_font=12, max_font=58,
    width=720, height=440,
)
chart.show()
```

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/wordcloud-bird.html"></iframe>
</div>
<div class="sp-variant" id="wordcloud-en-bubble">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"bubble"</code></span><span><strong>Aliases</strong> <code>bubble / bubbles / packed / packing / circles</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Each token is rendered as its own colored disc, sized by frequency - editorial bubble pack.</p>

<div class="sp-preview-label">Code</div>

```python
import seraplot as sp

words = ["Python", "Rust", "Data", "ML", "AI", "Cloud", "GPU", "Vector",
         "Tensor", "Graph", "Pandas", "NumPy", "Polars", "Arrow"]
freqs = [98, 92, 88, 80, 74, 66, 60, 54, 48, 42, 36, 30, 24, 18]

chart = sp.build_wordcloud(
    title="Tech stack", words=words, frequencies=freqs,
    variant="bubble",
    palette=[0x6366F1, 0x22D3EE, 0xF59E0B, 0xEF4444,
             0x10B981, 0xA855F7, 0xEC4899, 0x14B8A6],
    bg_color="#ffffff", min_font=12, max_font=58,
    width=720, height=440,
)
chart.show()
```

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/wordcloud-bubble.html"></iframe>
</div>
<div class="sp-variant" id="wordcloud-en-glasses">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"glasses"</code></span><span><strong>Aliases</strong> <code>glasses / sunglasses / shades / specs</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Sunglasses silhouette (two ellipses + bridge) - playful headline visual for lifestyle decks.</p>

<div class="sp-preview-label">Code</div>

```python
import seraplot as sp

words = ["Python", "Rust", "Data", "ML", "AI", "Cloud", "GPU", "Vector",
         "Tensor", "Graph", "Pandas", "NumPy", "Polars", "Arrow"]
freqs = [98, 92, 88, 80, 74, 66, 60, 54, 48, 42, 36, 30, 24, 18]

chart = sp.build_wordcloud(
    title="Tech stack", words=words, frequencies=freqs,
    variant="glasses",
    palette=[0x6366F1, 0x22D3EE, 0xF59E0B, 0xEF4444,
             0x10B981, 0xA855F7, 0xEC4899, 0x14B8A6],
    bg_color="#ffffff", min_font=12, max_font=58,
    width=720, height=440,
)
chart.show()
```

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/wordcloud-glasses.html"></iframe>
</div>
</div>
</div>

</div>

<div class="lang-fr">

<h2>Signature</h2>

`sp.build_wordcloud(title, words, frequencies, *, variant="basic", min_font=12, max_font=72, **kwargs) -> Chart`

Aliases: `sp.build_wordcloud` &middot; `sp.wordcloud` &middot; `sp.word_cloud` &middot; `sp.tag_cloud` &middot; `sp.cloud` &middot; `sp.text_cloud` &middot; `sp.token_cloud`

<h2>Description</h2>

`sp.build_wordcloud()` empile des tokens ponderes dans un masque parametrique. La taille de police encode la frequence, la couleur tourne sur la palette, et le mot-cle `variant` change la frontiere - rectangle en spirale, cercle, coeur, oiseau stylise, bulles compactes ou silhouette de lunettes - pour adapter la meme donnee a un usage editorial, dashboard ou reseaux sociaux.

<h2>Variantes</h2>

| Variante | Alias | Description |
|---|---|---|
| `"basic"` | `basic / default / spiral / rect` | Spirale d Archimede classique dans un rectangle - le word cloud canonique. |
| `"circle"` | `circle / round / disk / ball` | Mots empiles dans un disque parfait - silhouette editoriale pour vignettes. |
| `"heart"` | `heart / love / valentine` | Silhouette de coeur (cardioide) - parfait pour sentiment, brand-love, communautes. |
| `"bird"` | `bird / twitter / tweet / icon` | Silhouette composee de disques evoquant un oiseau - boards de buzz reseaux sociaux. |
| `"bubble"` | `bubble / bubbles / packed / packing / circles` | Chaque token devient un disque colore, taille par la frequence - bubble pack editorial. |
| `"glasses"` | `glasses / sunglasses / shades / specs` | Silhouette de lunettes de soleil (deux ellipses + pont) - visuel ludique pour decks lifestyle. |

<h2>Parametres</h2>

| Parametre | Type | Defaut | Description |
|---|---|---|---|
| `title` | `str` | requis | Titre du graphique |
| `words` | `list[str]` | requis | Tokens a afficher |
| `frequencies` | `list[float]` | requis | Poids par mot (taille de police) |
| `variant` | `str` | "basic" | Masque de forme (voir tableau) |
| `min_font` | `float` | 12.0 | Police mini |
| `max_font` | `float` | 72.0 | Police maxi |
| `palette` | `list[int]` | None | Palette personnalisee |
| `bg_color` | `str` | "#1a1a2e" | Couleur de fond |
| `width` | `int` | 900 | Largeur (px) |
| `height` | `int` | 500 | Hauteur (px) |

<h2>Retour</h2>

`Chart` - objet avec propriete `.html` et methode `.show()`.

<div class="sp-cls sp-open" id="wordcloud-fr">
<div class="sp-cls-rail">
<button class="sp-cls-toggle" onclick="spClsTog('wordcloud-fr')" title="Toggle">&#x21C6;</button>
<button class="sp-cls-tab sp-cact" onclick="spCls('wordcloud-fr','basic',this)"><span class="sp-cic">B</span><span class="sp-clb">Basic</span></button>
<button class="sp-cls-tab" onclick="spCls('wordcloud-fr','circle',this)"><span class="sp-cic">C</span><span class="sp-clb">Circle</span></button>
<button class="sp-cls-tab" onclick="spCls('wordcloud-fr','heart',this)"><span class="sp-cic">H</span><span class="sp-clb">Heart</span></button>
<button class="sp-cls-tab" onclick="spCls('wordcloud-fr','bird',this)"><span class="sp-cic">T</span><span class="sp-clb">Bird</span></button>
<button class="sp-cls-tab" onclick="spCls('wordcloud-fr','bubble',this)"><span class="sp-cic">U</span><span class="sp-clb">Bubble</span></button>
<button class="sp-cls-tab" onclick="spCls('wordcloud-fr','glasses',this)"><span class="sp-cic">G</span><span class="sp-clb">Glasses</span></button>
</div>
<div class="sp-cls-body">
<div class="sp-variant sp-von" id="wordcloud-fr-basic">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"basic"</code></span><span><strong>Aliases</strong> <code>basic / default / spiral / rect</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Spirale d Archimede classique dans un rectangle - le word cloud canonique.</p>

<div class="sp-preview-label">Code</div>

```python
import seraplot as sp

words = ["Python", "Rust", "Data", "ML", "AI", "Cloud", "GPU", "Vector",
         "Tensor", "Graph", "Pandas", "NumPy", "Polars", "Arrow"]
freqs = [98, 92, 88, 80, 74, 66, 60, 54, 48, 42, 36, 30, 24, 18]

chart = sp.build_wordcloud(
    title="Tech stack", words=words, frequencies=freqs,
    variant="basic",
    palette=[0x6366F1, 0x22D3EE, 0xF59E0B, 0xEF4444,
             0x10B981, 0xA855F7, 0xEC4899, 0x14B8A6],
    bg_color="#0f172a", min_font=12, max_font=58,
    width=720, height=440,
)
chart.show()
```

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/wordcloud-basic.html"></iframe>
</div>
<div class="sp-variant" id="wordcloud-fr-circle">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"circle"</code></span><span><strong>Aliases</strong> <code>circle / round / disk / ball</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Mots empiles dans un disque parfait - silhouette editoriale pour vignettes.</p>

<div class="sp-preview-label">Code</div>

```python
import seraplot as sp

words = ["Python", "Rust", "Data", "ML", "AI", "Cloud", "GPU", "Vector",
         "Tensor", "Graph", "Pandas", "NumPy", "Polars", "Arrow"]
freqs = [98, 92, 88, 80, 74, 66, 60, 54, 48, 42, 36, 30, 24, 18]

chart = sp.build_wordcloud(
    title="Tech stack", words=words, frequencies=freqs,
    variant="circle",
    palette=[0x6366F1, 0x22D3EE, 0xF59E0B, 0xEF4444,
             0x10B981, 0xA855F7, 0xEC4899, 0x14B8A6],
    bg_color="#0f172a", min_font=12, max_font=58,
    width=720, height=440,
)
chart.show()
```

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/wordcloud-circle.html"></iframe>
</div>
<div class="sp-variant" id="wordcloud-fr-heart">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"heart"</code></span><span><strong>Aliases</strong> <code>heart / love / valentine</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Silhouette de coeur (cardioide) - parfait pour sentiment, brand-love, communautes.</p>

<div class="sp-preview-label">Code</div>

```python
import seraplot as sp

words = ["Python", "Rust", "Data", "ML", "AI", "Cloud", "GPU", "Vector",
         "Tensor", "Graph", "Pandas", "NumPy", "Polars", "Arrow"]
freqs = [98, 92, 88, 80, 74, 66, 60, 54, 48, 42, 36, 30, 24, 18]

chart = sp.build_wordcloud(
    title="Tech stack", words=words, frequencies=freqs,
    variant="heart",
    palette=[0x6366F1, 0x22D3EE, 0xF59E0B, 0xEF4444,
             0x10B981, 0xA855F7, 0xEC4899, 0x14B8A6],
    bg_color="#fdf2f8", min_font=12, max_font=58,
    width=720, height=440,
)
chart.show()
```

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/wordcloud-heart.html"></iframe>
</div>
<div class="sp-variant" id="wordcloud-fr-bird">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"bird"</code></span><span><strong>Aliases</strong> <code>bird / twitter / tweet / icon</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Silhouette composee de disques evoquant un oiseau - boards de buzz reseaux sociaux.</p>

<div class="sp-preview-label">Code</div>

```python
import seraplot as sp

words = ["Python", "Rust", "Data", "ML", "AI", "Cloud", "GPU", "Vector",
         "Tensor", "Graph", "Pandas", "NumPy", "Polars", "Arrow"]
freqs = [98, 92, 88, 80, 74, 66, 60, 54, 48, 42, 36, 30, 24, 18]

chart = sp.build_wordcloud(
    title="Tech stack", words=words, frequencies=freqs,
    variant="bird",
    palette=[0x6366F1, 0x22D3EE, 0xF59E0B, 0xEF4444,
             0x10B981, 0xA855F7, 0xEC4899, 0x14B8A6],
    bg_color="#0f172a", min_font=12, max_font=58,
    width=720, height=440,
)
chart.show()
```

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/wordcloud-bird.html"></iframe>
</div>
<div class="sp-variant" id="wordcloud-fr-bubble">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"bubble"</code></span><span><strong>Aliases</strong> <code>bubble / bubbles / packed / packing / circles</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Chaque token devient un disque colore, taille par la frequence - bubble pack editorial.</p>

<div class="sp-preview-label">Code</div>

```python
import seraplot as sp

words = ["Python", "Rust", "Data", "ML", "AI", "Cloud", "GPU", "Vector",
         "Tensor", "Graph", "Pandas", "NumPy", "Polars", "Arrow"]
freqs = [98, 92, 88, 80, 74, 66, 60, 54, 48, 42, 36, 30, 24, 18]

chart = sp.build_wordcloud(
    title="Tech stack", words=words, frequencies=freqs,
    variant="bubble",
    palette=[0x6366F1, 0x22D3EE, 0xF59E0B, 0xEF4444,
             0x10B981, 0xA855F7, 0xEC4899, 0x14B8A6],
    bg_color="#ffffff", min_font=12, max_font=58,
    width=720, height=440,
)
chart.show()
```

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/wordcloud-bubble.html"></iframe>
</div>
<div class="sp-variant" id="wordcloud-fr-glasses">
<div class="sp-vmeta"><span><strong>Variant</strong> <code>"glasses"</code></span><span><strong>Aliases</strong> <code>glasses / sunglasses / shades / specs</code></span><span><strong>Returns</strong> <code>Chart</code></span></div>

<p style="color:#94a3b8;font-size:13px;margin:0 0 14px">Silhouette de lunettes de soleil (deux ellipses + pont) - visuel ludique pour decks lifestyle.</p>

<div class="sp-preview-label">Code</div>

```python
import seraplot as sp

words = ["Python", "Rust", "Data", "ML", "AI", "Cloud", "GPU", "Vector",
         "Tensor", "Graph", "Pandas", "NumPy", "Polars", "Arrow"]
freqs = [98, 92, 88, 80, 74, 66, 60, 54, 48, 42, 36, 30, 24, 18]

chart = sp.build_wordcloud(
    title="Tech stack", words=words, frequencies=freqs,
    variant="glasses",
    palette=[0x6366F1, 0x22D3EE, 0xF59E0B, 0xEF4444,
             0x10B981, 0xA855F7, 0xEC4899, 0x14B8A6],
    bg_color="#ffffff", min_font=12, max_font=58,
    width=720, height=440,
)
chart.show()
```

<div class="sp-preview-label">Preview</div>
<iframe class="sp-preview-frame" src="../../previews/wordcloud-glasses.html"></iframe>
</div>
</div>
</div>

</div>
