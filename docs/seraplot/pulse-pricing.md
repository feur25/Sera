# Sera Pulse

<div class="lang-en">

Sera Pulse is the streaming layer on top of every native chart: `.push()`, `.push_vector()`, `.serve()`, `.record()`, `.replay()`, `.on_anomaly()`. It's free, open-source, no trial, no license, no activation — same as the chart rendering itself.

```python
import seraplot as sp

chart = sp.scatter("Live feed", x=[], y=[])
chart.serve(port=8787)
chart.push([0, 1, 2], [1.2, 3.4, 2.1])
```

---

## What's included

- **Streaming** — `push()`, `push_vector()`, `serve()`, `record()`/`replay()`, `on_anomaly()`, `export_standalone()`.
- **Sera Secure** — `SecureDFrame`, AES-256-GCM encryption at rest, plaintext never cached or logged.
- **Sera Firehose** — `sp.firehose(...)` + `firehose_push()`, a ring-buffered chart for very high-frequency streams, rendered through WebGL2.
- **Sera Live** — live cursors, pinned comments, shared replay on any `.serve()`'d chart.
- **Sera Board** — `serve_board()`/`export_board_html()`, a Figma-style infinite whiteboard of live, linked charts.
- **SeraReport** — `export_pdf()`, `export_docx()`, `export_pdf_report()`.
- **SeraStudio** — `export_gif()`, `export_video()`, session recording/replay for shareable chart clips.

---

## Sera Secure

`SecureDFrame` is the AES-256-GCM encrypted-at-rest counterpart to `SeraDFrame` — columns stay ciphertext in memory, decrypted only transiently for a single read or chart render:

```python
import seraplot as sp

key = sp.SeraKey.generate()
frame = sp.SecureDFrame({"x": x_values, "y": y_values}, key.to_bytes())
chart_data = frame.to_chart_data("x", "y", key.to_bytes(), max_points=2000)
```

`decrypt()`/`decrypt_column()` give back plaintext on demand; `to_chart_data()`/`to_chart_data_multi()` decrypt, downsample, and hand off chart-ready arrays in one step, without ever materializing a decrypted copy of the rest of the frame.

---

## SeraReport

```python
chart = sp.bar("Sales", labels=["Q1", "Q2", "Q3"], values=[120, 150, 90])
chart.export_pdf("sales.pdf")
chart.export_docx("sales.docx")
```

Direct SVG-to-PDF/DOCX, no headless browser. `export_pdf_report(charts, path, titles=None)` combines several charts into one multi-page PDF. Needs real SVG content — charts large enough to cross their native canvas-rendering threshold render via `<canvas>` instead (same boundary `export_svg()`/`export_png()` have). `interactive=True` (default) also writes a companion `.html` with a clickable link back to the live chart.

<div class="sp-video-card" style="max-width:640px">
<iframe src="../previews/serareport-sample.pdf" style="width:100%;height:420px;border:1px solid var(--theme-popup-border, rgba(0,0,0,.2));border-radius:8px"></iframe>
<p><a href="../previews/serareport-sample.pdf" target="_blank" rel="noopener">Open in a new tab</a> if your browser doesn't preview PDFs inline.</p>
</div>

---

## Sera Firehose

A fixed-size ring buffer chart for absorbing very fast streams — sensor ticks, prices, request latencies — without the tab choking on it. Once full, each new reading overwrites the oldest one:

```python
chart = sp.firehose("CPU %", capacity=2_000, min_val=0.0, max_val=100.0)
chart.serve(port=8787)
for reading in sensor_stream():
    chart.firehose_push([reading])
```

Same rate limiting, `on_anomaly()`, `record()`/`replay()`, `.serve()` broadcast as every other native chart — rendered through WebGL2 instead of `<canvas>`, so per-point cost stays flat no matter how many readings landed since the last frame.

<div class="sp-video-card" style="max-width:640px">
<iframe src="../previews/firehose-demo.html" style="width:100%;height:300px;border:1px solid var(--theme-popup-border, rgba(0,0,0,.2));border-radius:8px;background:#fff"></iframe>
<p><a href="../previews/firehose-demo.html" target="_blank" rel="noopener">Open standalone</a> to see it full-size.</p>
</div>

---

## Sera Live

Turns any `.serve()`'d chart into a shared room — everyone who opens the link sees live cursors, can pin a note or draw directly on the chart, and **Replay together** steps the whole room through a recording in lockstep:

```python
chart = sp.bar("Error rate by region", labels=["us-east", "eu-west", "ap-south"], values=[12, 4, 31])
chart.record("incident.spls")
chart.serve(port=8787)
```

Pins are real `bookmark()` annotations — list them later with `chart.bookmarks()`. `collab=False` turns off presence entirely, nothing about you leaves your machine.

---

## Sera Board

Puts several charts on one infinite, pannable canvas instead of five browser tabs — a sidebar lists them, each sits in its own draggable frame:

```python
errors = sp.bar("Error rate", labels=["us-east", "eu-west", "ap-south"], values=[12, 4, 31])
latency = sp.line("p99 latency (ms)", x=list(range(10)), y=[120, 118, 125, 400, 410, 190, 130, 128, 126, 124])
sp.serve_board([errors, latency], titles=["Errors", "Latency"], session="incident-01", port=8787)
```

`serve_board()` joins Sera Board to Sera Live — shared cursors, pins, and drawings stick to each chart's own frame and move with it. A named **session** persists layout/pins/drawings to disk between runs. `export_board_html()` writes the same board as one self-contained file, no server required.

---

## SeraStudio

Renders a `.spls` recording as a shareable GIF or video — family-correct when the recording carries its source (a themed heatmap keeps its real color grid, not a generic substitute):

```python
chart = sp.bar("Sensors", labels=[str(i) for i in range(600)], values=[10.0] * 600)
chart.record("session.spls")
chart.push([3], [14.5])
chart.stop_record()
chart.export_gif("session.gif", "session.spls")
chart.export_video("session.mp4", "session.spls", format="mp4", fps=30)
```

`sort_by_value=True` gives a bar-chart-race look. `smooth=True` tweens between recorded states instead of snapping. `reveal=True` draws the final state progressively instead of replaying real pushes. `format` is `mp4`/`webm`/`mov`/`mkv`/`avi` (needs ffmpeg on PATH for video).

<div class="sp-video-grid">

<div class="sp-video-card">
<video controls autoplay loop muted playsinline preload="auto" src="videos/serastudio_60fps_bar_pictogram.mp4"></video>
<button class="sp-code-btn" data-sp-code-target="sp-code-pictogram" data-sp-code-title="serastudio_60fps_demo.py">View code</button>
</div>

<div class="sp-video-card">
<video controls autoplay loop muted playsinline preload="auto" src="videos/serastudio_smooth_battle_10s.mp4"></video>
<button class="sp-code-btn" data-sp-code-target="sp-code-battle" data-sp-code-title="serastudio_smooth_battle_demo.py">View code</button>
</div>

<div class="sp-video-card">
<video controls autoplay loop muted playsinline preload="auto" src="videos/serastudio_smooth_scatter_regression_12s.mp4"></video>
<button class="sp-code-btn" data-sp-code-target="sp-code-scatter" data-sp-code-title="serastudio_smooth_scatter_regression_demo.py">View code</button>
</div>

</div>

Full catalog of every variant's own `record()`/`push()`/`export_video()` snippet, generated live from the [Showcase](../showcase.md) registry:

<details class="sp-catalog-details">
<summary>Browse the full catalog</summary>
<div class="sp-catalog" id="sp-serastudio-catalog-en"></div>
</details>

---

## Method reference

<div data-sp-registry-table="methods" data-file="seraplot/pulse-pricing.md" data-group-by="module"></div>

</div>

<div class="lang-fr">

Sera Pulse est la couche de streaming au-dessus de chaque chart natif : `.push()`, `.push_vector()`, `.serve()`, `.record()`, `.replay()`, `.on_anomaly()`. C'est gratuit, open-source, sans essai, sans licence, sans activation — comme le rendu des charts lui-même.

```python
import seraplot as sp

chart = sp.scatter("Live feed", x=[], y=[])
chart.serve(port=8787)
chart.push([0, 1, 2], [1.2, 3.4, 2.1])
```

---

## Ce qui est inclus

- **Streaming** — `push()`, `push_vector()`, `serve()`, `record()`/`replay()`, `on_anomaly()`, `export_standalone()`.
- **Sera Secure** — `SecureDFrame`, chiffrement AES-256-GCM au repos, le texte en clair n'est jamais mis en cache ni journalisé.
- **Sera Firehose** — `sp.firehose(...)` + `firehose_push()`, un chart à buffer circulaire pour les flux très haute fréquence, rendu en WebGL2.
- **Sera Live** — curseurs partagés en direct, commentaires épinglés, replay partagé sur tout chart `.serve()`.
- **Sera Board** — `serve_board()`/`export_board_html()`, une toile infinie façon Figma de charts en direct et reliés.
- **SeraReport** — `export_pdf()`, `export_docx()`, `export_pdf_report()`.
- **SeraStudio** — `export_gif()`, `export_video()`, enregistrement/replay de session pour des clips de chart partageables.

---

## Sera Secure

`SecureDFrame` est l'équivalent chiffré au repos (AES-256-GCM) de `SeraDFrame` — les colonnes restent en texte chiffré en mémoire, déchiffrées seulement le temps d'une lecture ou d'un rendu de chart :

```python
import seraplot as sp

key = sp.SeraKey.generate()
frame = sp.SecureDFrame({"x": x_values, "y": y_values}, key.to_bytes())
chart_data = frame.to_chart_data("x", "y", key.to_bytes(), max_points=2000)
```

`decrypt()`/`decrypt_column()` redonnent du texte en clair à la demande ; `to_chart_data()`/`to_chart_data_multi()` déchiffrent, sous-échantillonnent et retournent des tableaux prêts pour le chart en une étape, sans jamais matérialiser de copie déchiffrée du reste du frame.

---

## SeraReport

```python
chart = sp.bar("Sales", labels=["Q1", "Q2", "Q3"], values=[120, 150, 90])
chart.export_pdf("sales.pdf")
chart.export_docx("sales.docx")
```

Conversion directe SVG vers PDF/DOCX, sans navigateur headless. `export_pdf_report(charts, path, titles=None)` combine plusieurs charts en un seul PDF multi-page. Nécessite un vrai contenu SVG — les charts assez grands pour dépasser leur seuil de rendu canvas natif se rendent en `<canvas>` à la place (même limite que `export_svg()`/`export_png()`). `interactive=True` (défaut) écrit aussi un `.html` compagnon avec un lien cliquable vers le chart en direct.

<div class="sp-video-card" style="max-width:640px">
<iframe src="../previews/serareport-sample.pdf" style="width:100%;height:420px;border:1px solid var(--theme-popup-border, rgba(0,0,0,.2));border-radius:8px"></iframe>
<p><a href="../previews/serareport-sample.pdf" target="_blank" rel="noopener">Ouvrir dans un nouvel onglet</a> si votre navigateur ne prévisualise pas les PDF.</p>
</div>

---

## Sera Firehose

Un chart à buffer circulaire de taille fixe pour absorber des flux très rapides — ticks de capteurs, prix, latences de requêtes — sans bloquer l'onglet. Une fois plein, chaque nouvelle valeur écrase la plus ancienne :

```python
chart = sp.firehose("CPU %", capacity=2_000, min_val=0.0, max_val=100.0)
chart.serve(port=8787)
for reading in sensor_stream():
    chart.firehose_push([reading])
```

Même limitation de débit, `on_anomaly()`, `record()`/`replay()`, diffusion `.serve()` que tout autre chart natif — rendu en WebGL2 plutôt qu'en `<canvas>`, donc le coût par point reste constant peu importe le nombre de valeurs arrivées depuis la dernière frame.

<div class="sp-video-card" style="max-width:640px">
<iframe src="../previews/firehose-demo.html" style="width:100%;height:300px;border:1px solid var(--theme-popup-border, rgba(0,0,0,.2));border-radius:8px;background:#fff"></iframe>
<p><a href="../previews/firehose-demo.html" target="_blank" rel="noopener">Ouvrir en autonome</a> pour le voir en taille réelle.</p>
</div>

---

## Sera Live

Transforme tout chart `.serve()` en une salle partagée — chaque personne qui ouvre le lien voit les curseurs en direct, peut épingler une note ou dessiner directement sur le chart, et **Replay together** fait avancer toute la salle dans un enregistrement au même rythme :

```python
chart = sp.bar("Error rate by region", labels=["us-east", "eu-west", "ap-south"], values=[12, 4, 31])
chart.record("incident.spls")
chart.serve(port=8787)
```

Les épingles sont de vraies annotations `bookmark()` — à lister ensuite avec `chart.bookmarks()`. `collab=False` désactive entièrement la présence, rien ne quitte votre machine.

---

## Sera Board

Place plusieurs charts sur une seule toile infinie et déplaçable au lieu de cinq onglets de navigateur — une barre latérale les liste, chacun dans son propre cadre déplaçable :

```python
errors = sp.bar("Error rate", labels=["us-east", "eu-west", "ap-south"], values=[12, 4, 31])
latency = sp.line("p99 latency (ms)", x=list(range(10)), y=[120, 118, 125, 400, 410, 190, 130, 128, 126, 124])
sp.serve_board([errors, latency], titles=["Errors", "Latency"], session="incident-01", port=8787)
```

`serve_board()` relie Sera Board à Sera Live — curseurs, épingles et dessins partagés collent au cadre de chaque chart et le suivent. Une **session** nommée persiste la disposition/épingles/dessins sur disque entre deux exécutions. `export_board_html()` écrit le même tableau en un seul fichier autonome, sans serveur requis.

---

## SeraStudio

Rend un enregistrement `.spls` en GIF ou vidéo partageable — fidèle à la famille du chart quand l'enregistrement porte sa source (un heatmap thématisé garde sa vraie grille de couleurs, pas un substitut générique) :

```python
chart = sp.bar("Sensors", labels=[str(i) for i in range(600)], values=[10.0] * 600)
chart.record("session.spls")
chart.push([3], [14.5])
chart.stop_record()
chart.export_gif("session.gif", "session.spls")
chart.export_video("session.mp4", "session.spls", format="mp4", fps=30)
```

`sort_by_value=True` donne un effet course de barres. `smooth=True` interpole entre les états enregistrés au lieu de sauter. `reveal=True` dessine l'état final progressivement au lieu de rejouer les vrais push. `format` vaut `mp4`/`webm`/`mov`/`mkv`/`avi` (ffmpeg requis dans le PATH pour la vidéo).

<div class="sp-video-grid">

<div class="sp-video-card">
<video controls autoplay loop muted playsinline preload="auto" src="videos/serastudio_60fps_bar_pictogram.mp4"></video>
<button class="sp-code-btn" data-sp-code-target="sp-code-pictogram-fr" data-sp-code-title="serastudio_60fps_demo.py">Voir le code</button>
</div>

<div class="sp-video-card">
<video controls autoplay loop muted playsinline preload="auto" src="videos/serastudio_smooth_battle_10s.mp4"></video>
<button class="sp-code-btn" data-sp-code-target="sp-code-battle-fr" data-sp-code-title="serastudio_smooth_battle_demo.py">Voir le code</button>
</div>

<div class="sp-video-card">
<video controls autoplay loop muted playsinline preload="auto" src="videos/serastudio_smooth_scatter_regression_12s.mp4"></video>
<button class="sp-code-btn" data-sp-code-target="sp-code-scatter-fr" data-sp-code-title="serastudio_smooth_scatter_regression_demo.py">Voir le code</button>
</div>

</div>

Catalogue complet du snippet `record()`/`push()`/`export_video()` propre à chaque variante, généré en direct depuis le registre du [Showcase](../showcase.md) :

<details class="sp-catalog-details">
<summary>Parcourir le catalogue complet</summary>
<div class="sp-catalog" id="sp-serastudio-catalog-fr"></div>
</details>

---

## Référence des méthodes

<div data-sp-registry-table="methods" data-file="seraplot/pulse-pricing.md" data-group-by="module"></div>

</div>
