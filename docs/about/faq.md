# FAQ / Troubleshooting

<div class="lang-en">

## My chart renders empty / blank

The most common cause is a parameter shape mismatch — check the exact kwarg names and array shapes in that chart's doc page's `## Signature` section. A close second: `NaN`/`Infinity` values in a Python list are sanitized to `0` automatically rather than crashing, so an all-`NaN` column produces a technically-valid but visually empty chart. Call `.dropna()`/`.fillna()` on a `SeraDFrame` before charting if that's the cause. Empty input lists (`labels=[]`, `values=[]`) intentionally render nothing — that's a documented convention, not a bug.

## A method I see in the source isn't callable from Python

A small number of methods present in the Rust source (e.g. some ML estimator classes, `Chart.chart_iframe()`) are not currently wired through to the Python bindings — check `sp.docs()` or `dir(sp)` to confirm a symbol is actually exposed before relying on it; if it's missing, it's a known gap, not something you're doing wrong.

## My chart looks different depending on the browser

SeraPlot charts are tested against real Chromium, Firefox, and WebKit engines (see the accessibility/compatibility notes in the roadmap) and should render identically. If you see a real discrepancy, it's worth reporting — but first check whether a browser extension or a `prefers-contrast`/`prefers-reduced-motion` OS setting is intentionally altering the rendering, since SeraPlot charts respect both.

## `sp.App()` / webapp questions

- **Callback argument types are always strings.** The server has no way to know a slider's value should be a float — `handler(value: str)` must convert it itself.
- **Nothing updates when I click.** Confirm the component `id` you pass to `add_callback(inputs=[...])` matches the id used when creating the component (`app.dropdown("period", ...)` → `inputs=["period"]`), and that the app is actually running (`.serve()` blocks — check the terminal for the bound port).
- **Can I use `sp.App()` in production?** No — see the honest limitations section on its [documentation page](../canvas/webapp.md): no auth, no session isolation, no TLS. It's a local/internal dashboard tool.

## Large datasets are slow or produce huge HTML files

Pass `max_points=N` to `line()`/`bubble()`/`scatter()`/etc. for bounded, lossy downsampling (LTTB algorithm), or rely on the automatic Canvas2D fast path that kicks in above ~3000 points for `line()`/simple `bubble()` (full resolution, no data loss, adds pan/zoom). For `SeraDFrame`, filter/aggregate before charting rather than charting every raw row.

## Where do I report a real bug?

There's no public issue tracker link established yet in this documentation — until one exists, use the contact channel on the [Support & Contact](support.md) page.

</div>

<div class="lang-fr">

## Mon chart s'affiche vide / blanc

La cause la plus fréquente est un décalage de forme de paramètre — vérifiez les noms de kwargs exacts et les formes de tableaux dans la section `## Signature` de la page de doc de ce chart. Deuxième cause fréquente : les valeurs `NaN`/`Infinity` dans une liste Python sont assainies en `0` automatiquement plutôt que de faire planter — une colonne entièrement `NaN` produit donc un chart techniquement valide mais visuellement vide. Appelez `.dropna()`/`.fillna()` sur un `SeraDFrame` avant de tracer si c'est la cause. Les listes vides en entrée (`labels=[]`, `values=[]`) rendent intentionnellement rien — c'est une convention documentée, pas un bug.

## Une méthode que je vois dans le code source n'est pas appelable depuis Python

Un petit nombre de méthodes présentes dans le code source Rust (ex. certaines classes d'estimateurs ML, `Chart.chart_iframe()`) ne sont pas actuellement câblées jusqu'aux bindings Python — vérifiez `sp.docs()` ou `dir(sp)` pour confirmer qu'un symbole est réellement exposé avant de vous y fier ; s'il manque, c'est un manque connu, pas une erreur de votre part.

## Mon chart a un rendu différent selon le navigateur

Les charts SeraPlot sont testés contre de vrais moteurs Chromium, Firefox et WebKit (voir les notes d'accessibilité/compatibilité dans la roadmap) et devraient se rendre à l'identique. Si vous constatez un vrai écart, cela vaut la peine d'être signalé — mais vérifiez d'abord qu'une extension navigateur ou un réglage système `prefers-contrast`/`prefers-reduced-motion` n'altère pas intentionnellement le rendu, car les charts SeraPlot respectent les deux.

## Questions sur `sp.App()` / webapp

- **Les arguments de callback sont toujours des chaînes.** Le serveur n'a aucun moyen de savoir que la valeur d'un slider doit être un flottant — `handler(value: str)` doit le convertir lui-même.
- **Rien ne se met à jour quand je clique.** Vérifiez que l'`id` du composant passé à `add_callback(inputs=[...])` correspond à l'id utilisé à la création du composant (`app.dropdown("period", ...)` → `inputs=["period"]`), et que l'app tourne réellement (`.serve()` est bloquant — vérifiez le terminal pour le port lié).
- **Puis-je utiliser `sp.App()` en production ?** Non — voir la section des limites honnêtes sur sa [page de documentation](../canvas/webapp.md) : pas d'authentification, pas d'isolation de session, pas de TLS. C'est un outil de tableau de bord local/interne.

## Les gros volumes de données sont lents ou produisent des fichiers HTML énormes

Passez `max_points=N` à `line()`/`bubble()`/`scatter()`/etc. pour un sous-échantillonnage borné avec perte (algorithme LTTB), ou reposez-vous sur le chemin rapide Canvas2D automatique qui s'active au-delà d'environ 3000 points pour `line()`/`bubble()` simple (pleine résolution, aucune perte de données, ajoute pan/zoom). Pour `SeraDFrame`, filtrez/agrégez avant de tracer plutôt que de tracer chaque ligne brute.

## Où signaler un vrai bug ?

Aucun lien de suivi d'issues public n'est encore établi dans cette documentation — en attendant, utilisez le canal de contact de la page [Support & Contact](support.md).

</div>
