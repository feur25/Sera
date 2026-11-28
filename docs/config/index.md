# Configuration

<div class="lang-en">

Two layers, and every page in this section names which one it documents:

- **Global, standalone functions** — `sp.set_global_background(...)`, `sp.theme(...)`, `sp.config(...)` — apply before charts are built and affect everything created afterward. See [Chart Methods](../getting-started/chart-methods.md) for the full generated list.
- **Chainable `Chart` methods** — `chart.export_svg(path)`, `chart.hover_json(payload)`, `chart.downsample()` — apply to one already-built chart and return a new `Chart`, the same way every other per-chart method chains.

Every code sample in this section is checked against a real build rather than
written from memory — where a feature exists only on the JS/WASM side, or
isn't wired up as a Python function yet, that is stated explicitly instead of
shown as if it worked.

</div>

<div class="lang-fr">

Deux couches, et chaque page de cette section précise laquelle elle documente :

- **Fonctions globales autonomes** — `sp.set_global_background(...)`, `sp.theme(...)`, `sp.config(...)` — s'appliquent avant la construction des graphiques et affectent tout ce qui est créé ensuite. Voir [Méthodes des graphiques](../getting-started/chart-methods.md) pour la liste complète générée.
- **Méthodes chaînables sur `Chart`** — `chart.export_svg(path)`, `chart.hover_json(payload)`, `chart.downsample()` — s'appliquent à un chart déjà construit et retournent un nouveau `Chart`, comme toute autre méthode par graphique.

Chaque exemple de code de cette section est vérifié contre un vrai build
plutôt qu'écrit de mémoire — quand une fonctionnalité n'existe que côté
JS/WASM, ou n'est pas encore câblée comme fonction Python, c'est dit
explicitement plutôt que présenté comme si ça fonctionnait.

</div>
