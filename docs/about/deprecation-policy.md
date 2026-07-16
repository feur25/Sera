# API Deprecation Policy

<div class="lang-en">

SeraPlot is pre-1.0 and under active development (chantier 9 of the internal roadmap tracks the path to production-grade stability). This policy states the rules that will apply once the API surface stabilizes, and the interim rules that apply today.

## Versioning

SeraPlot follows [Semantic Versioning](https://semver.org/): `MAJOR.MINOR.PATCH`.

- **PATCH** (`x.y.Z`): bug fixes only. Never changes behavior a correct caller depends on.
- **MINOR** (`x.Y.z`): additive changes — new chart types, new aliases, new optional parameters with backward-compatible defaults. Never removes or renames a public symbol.
- **MAJOR** (`X.y.z`): may include breaking changes, always accompanied by a migration note in the changelog.

## Pre-1.0 status (current)

Before `1.0.0`, the API is **not yet frozen**. Breaking changes may land in a `MINOR` release rather than waiting for a `MAJOR` bump — this is standard pre-1.0 practice (matching npm/cargo semver conventions), not a departure from semver. Every breaking change is still called out explicitly in the release notes, never silent.

## Deprecation mechanism (from 1.0.0 onward)

1. **Announce**: a symbol scheduled for removal is marked deprecated in-code (the `#[sera_doc]`/`#[params]` macros carry a `deprecated_since`/`removed_in` annotation once implemented) and in its documentation page, with the replacement clearly stated.
2. **Warn**: calling a deprecated symbol emits a Python `DeprecationWarning` (visible via `python -W default` or `pytest -W default`) for at least one full MINOR release cycle before removal.
3. **Remove**: the symbol is removed only in the next MAJOR release, never in a MINOR or PATCH.
4. **Aliases are cheap, keep them**: because SeraPlot's alias mechanism (`#[sera_alias(...)]`) costs nothing at runtime, renaming a function almost never requires deprecating the old name — the old name simply becomes an alias of the new one and both keep working indefinitely. True removals are reserved for functions whose *behavior* changes incompatibly, not just their name.

## What counts as a breaking change

- Removing or renaming a public function, class, or method (without keeping the old name as an alias).
- Changing a parameter's accepted type or removing a parameter without a compatible default.
- Changing the default value of an existing parameter.
- Changing the semantics of existing output for the same input (e.g., a chart rendering differently for identical data) outside of an explicit bug fix.

## What does not count as breaking

- Adding a new optional parameter with a default that preserves prior behavior.
- Adding a new alias for an existing function.
- Fixing a bug where the previous behavior was clearly incorrect (e.g., a documented example that silently rendered nothing) — bug fixes are not held to deprecation cycles, since the "old behavior" was never a contract.
- Performance improvements that do not change output.

</div>

<div class="lang-fr">

SeraPlot est en pré-1.0 et en développement actif (le chantier 9 de la roadmap interne suit le chemin vers une stabilité de qualité production). Cette politique énonce les règles qui s'appliqueront une fois la surface d'API stabilisée, et les règles provisoires en vigueur aujourd'hui.

## Versionnage

SeraPlot suit le [Versionnage Sémantique](https://semver.org/lang/fr/) : `MAJEUR.MINEUR.CORRECTIF`.

- **CORRECTIF** (`x.y.Z`) : corrections de bugs uniquement. Ne change jamais un comportement dont dépend un appelant correct.
- **MINEUR** (`x.Y.z`) : changements additifs — nouveaux types de charts, nouveaux alias, nouveaux paramètres optionnels avec des valeurs par défaut rétrocompatibles. Ne supprime ni ne renomme jamais un symbole public.
- **MAJEUR** (`X.y.z`) : peut inclure des changements cassants, toujours accompagnés d'une note de migration dans le changelog.

## Statut pré-1.0 (actuel)

Avant `1.0.0`, l'API n'est **pas encore figée**. Des changements cassants peuvent arriver dans une version `MINEURE` plutôt que d'attendre un incrément `MAJEUR` — c'est une pratique pré-1.0 standard (cohérente avec les conventions semver de npm/cargo), pas un écart par rapport à semver. Chaque changement cassant reste néanmoins signalé explicitement dans les notes de version, jamais silencieux.

## Mécanisme de dépréciation (à partir de 1.0.0)

1. **Annoncer** : un symbole prévu pour suppression est marqué déprécié dans le code (les macros `#[sera_doc]`/`#[params]` porteront une annotation `deprecated_since`/`removed_in` une fois implémentée) et dans sa page de documentation, avec le remplacement clairement indiqué.
2. **Avertir** : appeler un symbole déprécié émet un `DeprecationWarning` Python (visible via `python -W default` ou `pytest -W default`) pendant au moins un cycle complet de version MINEURE avant suppression.
3. **Supprimer** : le symbole n'est supprimé que dans la MAJEURE suivante, jamais dans une MINEURE ou un CORRECTIF.
4. **Les alias ne coûtent rien, on les garde** : comme le mécanisme d'alias de SeraPlot (`#[sera_alias(...)]`) ne coûte rien à l'exécution, renommer une fonction ne nécessite presque jamais de dépréciation de l'ancien nom — l'ancien nom devient simplement un alias du nouveau et les deux continuent de fonctionner indéfiniment. Les véritables suppressions sont réservées aux fonctions dont le *comportement* change de façon incompatible, pas seulement leur nom.

## Ce qui constitue un changement cassant

- Supprimer ou renommer une fonction, classe ou méthode publique (sans garder l'ancien nom en alias).
- Changer le type accepté d'un paramètre ou supprimer un paramètre sans valeur par défaut compatible.
- Changer la valeur par défaut d'un paramètre existant.
- Changer la sémantique d'une sortie existante pour une même entrée (ex. un chart qui se rend différemment pour des données identiques) en dehors d'une correction de bug explicite.

## Ce qui n'est pas cassant

- Ajouter un nouveau paramètre optionnel avec une valeur par défaut qui préserve le comportement antérieur.
- Ajouter un nouvel alias pour une fonction existante.
- Corriger un bug où le comportement précédent était manifestement incorrect (ex. un exemple documenté qui rendait silencieusement un chart vide) — les corrections de bugs ne sont pas soumises aux cycles de dépréciation, car « l'ancien comportement » n'a jamais été un contrat.
- Améliorations de performance qui ne changent pas la sortie.

</div>
