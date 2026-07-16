# Web App (sp.App)

<div class="lang-en">

`sp.App` is a small, dependency-free reactive dashboard server built directly
into the Rust core — no Flask, no Dash, no Node. `.serve()` spins up a
Tokio-based HTTP + WebSocket server (hand-rolled HTTP/1.1 parsing and
RFC&nbsp;6455 framing, no external web framework) that pushes live UI updates
to the browser whenever a registered callback re-runs.

```python
import seraplot as sp

def on_change(period):
    values = {"7d": [12, 19, 15], "30d": [40, 55, 38]}[period]
    chart = sp.line("Sales", labels=["A", "B", "C"], values=values)
    return chart  # .html is extracted automatically

app = sp.App("Sales Dashboard")
app.dropdown("period", ["7d", "30d"], value="7d")
app.chart("out", sp.line("Sales", labels=["A", "B", "C"], values=[12, 19, 15]).html)
app.add_callback(inputs=["period"], output="out", handler=on_change)
app.serve(port=8787)
```

Open `http://127.0.0.1:8787/` — changing the dropdown re-runs `on_change`
server-side and pushes the new chart HTML into the page without a reload.

## How it works

1. `App(title)` creates a single-page app state with an implicit `"/"` page.
   `.page(path, title=None)` registers/switches to additional pages; every
   component call after it attaches to that page until the next `.page()`.
2. Component builders (`dropdown`, `slider`, `button`, `text_input`,
   `checkbox`, `chart`) render server-side HTML for that widget, register its
   initial value, and append it to the current page's layout. All of them
   return `self`, so calls chain.
3. `.add_callback(inputs, output, handler)` wires a Python callable: whenever
   *any* component whose id is in `inputs` changes, `handler` is invoked with
   the **current string value of every input**, positionally, in the order
   given to `inputs`. Its return value becomes the new HTML for `output` —
   either a raw string, or any object exposing an `.html` attribute (a
   `Chart` works directly, no `.html` access needed on the caller's side).
4. `.serve(port=8787, host="127.0.0.1")` blocks and starts the server. The
   browser opens a WebSocket to `/ws`; every input interaction sends
   `{"type":"event","id":...,"value":...}`, the server re-runs matching
   callbacks and pushes back `{"type":"update","id":...,"html":...}`, and a
   ~15-line bootstrap script does `document.getElementById(id).innerHTML =
   html` — no virtual DOM, no client-side framework.

## Component reference

| Method | Signature | Notes |
|---|---|---|
| `App(title="SeraPlot App")` | constructor | |
| `.page(path, title=None)` | `(str, str \| None)` | Creates the page on first call, switches the "current page" on every call |
| `.dropdown(id, options, value=None)` | `(str, list[str], str \| None)` | Defaults to `options[0]` if `value` omitted |
| `.slider(id, min, max, step=1.0, value=None)` | `(str, float, float, float, float \| None)` | Defaults to `min` if `value` omitted |
| `.button(id, label)` | `(str, str)` | Emits value `"click"` on press |
| `.text_input(id, value="", placeholder="")` | `(str, str, str)` | |
| `.checkbox(id, label, checked=False)` | `(str, str, bool)` | Emits `"true"`/`"false"` |
| `.chart(id, html="")` | `(str, str)` | Registers an output slot; typically seeded with a `Chart.html` and refreshed via a callback |
| `.add_callback(inputs, output, handler)` | `(list[str], str, Callable)` | `handler` receives one positional `str` per entry in `inputs` |
| `.serve(port=8787, host="127.0.0.1")` | `(int, str)` | Blocking call |

## Honest limitations

- Every callback argument arrives as a **string** — the server does not know
  a slider's numeric type, so `handler(value: str)` must `float()`/`int()`
  it itself.
- `.chart()` values are only refreshed through a callback's return value;
  there's no server-push independent of a client-originated event (no
  polling, no server timers).
- No routing/auth/session layer — `sp.App` is a local dashboard tool for
  exploration and internal tools, not a production web framework.

</div>

<div class="lang-fr">

`sp.App` est un petit serveur de tableau de bord réactif, sans dépendance,
intégré directement au cœur Rust — pas de Flask, pas de Dash, pas de Node.
`.serve()` démarre un serveur HTTP + WebSocket basé sur Tokio (parsing
HTTP/1.1 et trames RFC&nbsp;6455 écrits à la main, sans framework web
externe) qui pousse les mises à jour de l'interface vers le navigateur à
chaque nouvelle exécution d'un callback enregistré.

```python
import seraplot as sp

def on_change(period):
    values = {"7d": [12, 19, 15], "30d": [40, 55, 38]}[period]
    chart = sp.line("Ventes", labels=["A", "B", "C"], values=values)
    return chart  # .html est extrait automatiquement

app = sp.App("Tableau de bord Ventes")
app.dropdown("period", ["7d", "30d"], value="7d")
app.chart("out", sp.line("Ventes", labels=["A", "B", "C"], values=[12, 19, 15]).html)
app.add_callback(inputs=["period"], output="out", handler=on_change)
app.serve(port=8787)
```

Ouvrez `http://127.0.0.1:8787/` — changer le menu déroulant relance
`on_change` côté serveur et pousse le nouveau HTML du graphique dans la page
sans rechargement.

## Fonctionnement

1. `App(title)` crée un état d'application avec une page implicite `"/"`.
   `.page(path, title=None)` enregistre/bascule vers d'autres pages ; chaque
   appel de composant suivant s'attache à cette page jusqu'au `.page()`
   suivant.
2. Les constructeurs de composants (`dropdown`, `slider`, `button`,
   `text_input`, `checkbox`, `chart`) génèrent le HTML côté serveur du
   widget, enregistrent sa valeur initiale et l'ajoutent à la mise en page de
   la page courante. Tous retournent `self`, donc les appels s'enchaînent.
3. `.add_callback(inputs, output, handler)` relie un callable Python : dès
   qu'un composant dont l'id figure dans `inputs` change, `handler` est
   appelé avec la **valeur chaîne courante de chaque input**, en positionnel,
   dans l'ordre de `inputs`. Sa valeur de retour devient le nouveau HTML de
   `output` — une chaîne brute, ou tout objet exposant un attribut `.html`
   (un `Chart` fonctionne directement, sans accès `.html` côté appelant).
4. `.serve(port=8787, host="127.0.0.1")` bloque et démarre le serveur. Le
   navigateur ouvre un WebSocket vers `/ws` ; chaque interaction envoie
   `{"type":"event","id":...,"value":...}`, le serveur relance les callbacks
   correspondants et repousse `{"type":"update","id":...,"html":...}`, et un
   script d'amorçage d'une quinzaine de lignes fait
   `document.getElementById(id).innerHTML = html` — pas de DOM virtuel, pas
   de framework côté client.

## Référence des composants

| Méthode | Signature | Remarques |
|---|---|---|
| `App(title="SeraPlot App")` | constructeur | |
| `.page(path, title=None)` | `(str, str \| None)` | Crée la page au premier appel, bascule la « page courante » à chaque appel |
| `.dropdown(id, options, value=None)` | `(str, list[str], str \| None)` | Vaut `options[0]` par défaut si `value` omis |
| `.slider(id, min, max, step=1.0, value=None)` | `(str, float, float, float, float \| None)` | Vaut `min` par défaut si `value` omis |
| `.button(id, label)` | `(str, str)` | Émet la valeur `"click"` au clic |
| `.text_input(id, value="", placeholder="")` | `(str, str, str)` | |
| `.checkbox(id, label, checked=False)` | `(str, str, bool)` | Émet `"true"`/`"false"` |
| `.chart(id, html="")` | `(str, str)` | Enregistre un emplacement de sortie ; généralement initialisé avec un `Chart.html` et rafraîchi via un callback |
| `.add_callback(inputs, output, handler)` | `(list[str], str, Callable)` | `handler` reçoit un `str` positionnel par entrée de `inputs` |
| `.serve(port=8787, host="127.0.0.1")` | `(int, str)` | Appel bloquant |

## Limites honnêtes

- Chaque argument de callback arrive sous forme de **chaîne** — le serveur
  ne connaît pas le type numérique d'un slider, donc `handler(value: str)`
  doit faire `float()`/`int()` lui-même.
- Les valeurs de `.chart()` ne sont rafraîchies que via la valeur de retour
  d'un callback ; il n'y a pas de push serveur indépendant d'un événement
  initié côté client (pas de polling, pas de timers serveur).
- Aucune couche routage/auth/session — `sp.App` est un outil de tableau de
  bord local pour l'exploration et les outils internes, pas un framework web
  de production.

</div>
