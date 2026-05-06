# Slideshow

<div class="lang-en">

## Signature

```python
sp.build_slideshow(
    charts: list[Chart],
    *,
    title: str = "",
    width: int = 1000,
    height: int = 600,
    background: str | None = None,
    autoplay: bool = False,
    interval_ms: int = 3000,
) -> Chart
```

Aliases: `sp.slideshow`

---

## Description

A slideshow wraps multiple `Chart` objects into a single interactive carousel with Prev/Next navigation buttons. All charts are pre-rendered inline in the output HTML, so the slideshow works fully offline without any external dependencies. When `autoplay` is `True`, slides advance automatically every `interval_ms` milliseconds and loop back to the first after the last. The slideshow is ideal for presentations, dashboards, and reports where multiple charts share equal importance and should be viewed sequentially.

**Ideal for:**
- Business reviews and presentations requiring multiple charts in a single embed
- Report sections where charts follow a narrative sequence
- Dashboard demos cycling through key metrics automatically

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `charts` | `list[Chart]` | required | Ordered list of charts to display |
| `title` | `str` | `""` | Slideshow header title |
| `width` | `int` | `1000` | Canvas width in pixels |
| `height` | `int` | `600` | Canvas height in pixels |
| `background` | `str \| None` | `None` | Background color or `None` = transparent |
| `autoplay` | `bool` | `False` | Auto-advance slides |
| `interval_ms` | `int` | `3000` | Milliseconds between auto-advances |

---

## Returns

`Chart`

---

<style>.sp-tabs{border:1px solid #334155;border-radius:8px;overflow:hidden;margin:1.5em 0}.sp-tab-btns{display:flex;background:#0f172a;border-bottom:1px solid #334155;flex-wrap:wrap}.sp-tb{padding:7px 14px;border:none;background:none;color:#64748b;cursor:pointer;font-size:12px;font-weight:600;border-bottom:2px solid transparent;transition:color .15s,border-color .15s;white-space:nowrap}.sp-tb:hover{color:#e2e8f0}.sp-tb.sp-act{color:#6366f1;border-bottom-color:#6366f1}.sp-tc{display:none}.sp-tc.sp-on{display:block}</style>
<script>function spTab(g,id,btn){var r=document.getElementById(g);r.querySelectorAll('.sp-tc').forEach(function(e){e.classList.remove('sp-on')});r.querySelectorAll('.sp-tb').forEach(function(b){b.classList.remove('sp-act')});document.getElementById(id).classList.add('sp-on');btn.classList.add('sp-act');if(window.hljs)document.getElementById(id).querySelectorAll('code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})}document.addEventListener('DOMContentLoaded',function(){if(window.hljs)document.querySelectorAll('.sp-tc.sp-on code').forEach(function(c){try{(hljs.highlightElement||hljs.highlightBlock).call(hljs,c)}catch(e){}})});</script>


</div>

<iframe src="../../previews/slideshow.html" style="width:100%;height:380px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

---

## See also

- [Grid Layout](grid.md) — `sp.build_grid()`

</div>

<div class="lang-fr">

<h2>Signature</h2>

```python
sp.build_slideshow(
    charts: list[Chart],
    *,
    title: str = "",
    width: int = 1000,
    height: int = 600,
    background: str | None = None,
    autoplay: bool = False,
    interval_ms: int = 3000,
) -> Chart
```

Aliases: `sp.slideshow`

---

<h2>Description</h2>

Un slideshow enveloppe plusieurs objets `Chart` dans un seul carousel interactif avec des boutons de navigation Précédent/Suivant. Tous les graphiques sont pré-rendus en ligne dans le HTML de sortie, le slideshow fonctionne donc entièrement hors ligne. Quand `autoplay` est `True`, les diapositives avancent automatiquement toutes les `interval_ms` millisecondes et rebouclent au début après la dernière.

**Idéal pour :**
- Bilans d'activité et présentations nécessitant plusieurs graphiques dans un seul embed
- Sections de rapport où les graphiques suivent une séquence narrative
- Démonstrations de tableau de bord faisant défiler automatiquement les indicateurs clés

---

<h2>Paramètres</h2>

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `charts` | `list[Chart]` | requis | Liste ordonnée des graphiques à afficher |
| `title` | `str` | `""` | Titre d'en-tête du slideshow |
| `width` | `int` | `1000` | Largeur du canvas en pixels |
| `height` | `int` | `600` | Hauteur du canvas en pixels |
| `background` | `str \| None` | `None` | Couleur de fond ou `None` = transparent |
| `autoplay` | `bool` | `False` | Avance automatique des diapositives |
| `interval_ms` | `int` | `3000` | Millisecondes entre les avancements automatiques |

---

<h2>Retourne</h2>

`Chart`

---


</div>

<iframe src="../../previews/slideshow.html" style="width:100%;height:380px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

---

<h2>Voir aussi</h2>

- [Grille de graphiques](grid.md) — `sp.build_grid()`

</div>
