# Custom Aliases

<div class="lang-en">

## Signature

```python
sp.config.add_alias(method_name: str, alias: str | list[str]) -> sp.config
sp.config.add_aliases(mapping: dict[str, str | list[str]]) -> sp.config
sp.config.remove_alias(method_name: str, alias: str) -> sp.config
sp.config.reset() -> sp.config
sp.save_config(path: str | None = None) -> str
sp.load_config(path: str | None = None) -> bool
```

---

## Description

`sp.config` is callable exactly like before (`sp.config(font=..., palette=...)` for global
chart defaults), but it now also acts as a registry where you can attach your own custom
names to any framework method — including 2D and 3D chart builders such as
`build_bar3d_chart`, `bar_3d`, `build_bar_chart`, etc.

Aliases are bound immediately as real top-level `sp.<alias>(...)` functions, callable
exactly like the original.

`sp.config.aliases` is a plain `dict[str, list[str]]` mapping each method name to the list
of aliases registered for it — inspect or mutate it directly if you need full control.

Calling `sp.save_config()` writes the current alias registry to
`~/.seraplot/config.json`, **fully overwriting** whatever was saved before — there is no
merge, the file always reflects the in-memory state at the time of the call.

On `import seraplot`, the saved file (if any) is loaded automatically, so custom aliases
defined in a previous session are immediately available again in any new kernel, without
calling anything.

`sp.config.reset()` clears every alias **in memory only** and unbinds the corresponding
`sp.<alias>` functions — it never touches the saved file on disk. Other kernels that already
loaded the previous saved config keep working exactly as before; only calling
`sp.save_config()` afterwards propagates the reset to disk.

---

## Methods

| Method | Description |
|--------|-------------|
| `add_alias(method_name, alias)` | Register one alias (or a list of aliases) for `method_name`. |
| `add_aliases(mapping)` | Bulk-register aliases from a `{method_name: alias_or_list}` dict. |
| `remove_alias(method_name, alias)` | Unregister a single alias and remove the bound function. |
| `reset()` | Clear all aliases in memory and unbind their functions. Does **not** touch the saved file until `save_config()` is called. |
| `save_config(path=None)` | Persist the current alias registry to disk (default `~/.seraplot/config.json`); overwrites any previous save. Returns the path written. |
| `load_config(path=None)` | Load and apply a previously saved alias registry. Returns `True` if a file was found and loaded. |

---

## Examples

```python
import seraplot as sp

sp.config.add_alias("build_bar3d_chart", "mybar3d")
sp.mybar3d("Custom name", labels=["A", "B"], values=[1, 2])

sp.config.add_aliases({
    "build_bar_chart": ["mybar", "qbar"],
    "build_line_chart": "myline",
})

sp.save_config()
```

```python
# In a brand new kernel, later
import seraplot as sp
sp.mybar3d("Still works after restart", labels=["A", "B"], values=[3, 4])
```

</div>

<div class="lang-fr">

## Signature

```python
sp.config.add_alias(method_name: str, alias: str | list[str]) -> sp.config
sp.config.add_aliases(mapping: dict[str, str | list[str]]) -> sp.config
sp.config.remove_alias(method_name: str, alias: str) -> sp.config
sp.config.reset() -> sp.config
sp.save_config(path: str | None = None) -> str
sp.load_config(path: str | None = None) -> bool
```

---

## Description

`sp.config` reste appelable comme avant (`sp.config(font=..., palette=...)` pour les
réglages globaux), mais agit maintenant aussi comme un registre permettant d'attacher vos
propres noms à n'importe quelle méthode du framework — y compris les constructeurs de
charts 2D et 3D comme `build_bar3d_chart`, `bar_3d`, `build_bar_chart`, etc.

Les alias sont immédiatement liés comme de vraies fonctions `sp.<alias>(...)` au premier
niveau, appelables exactement comme l'original.

`sp.config.aliases` est un simple `dict[str, list[str]]` associant chaque nom de méthode à
la liste de ses alias enregistrés — consultable ou modifiable directement si besoin.

Appeler `sp.save_config()` écrit le registre d'alias actuel dans
`~/.seraplot/config.json`, en **écrasant totalement** ce qui était sauvegardé avant — il
n'y a pas de fusion, le fichier reflète toujours l'état en mémoire au moment de l'appel.

Au chargement (`import seraplot`), le fichier sauvegardé (s'il existe) est chargé
automatiquement, donc les alias définis dans une session précédente sont immédiatement
disponibles dans n'importe quel nouveau kernel, sans rien appeler.

`sp.config.reset()` efface tous les alias **en mémoire uniquement** et détache les
fonctions `sp.<alias>` correspondantes — il ne touche jamais au fichier sauvegardé sur
disque. Les autres kernels ayant déjà chargé l'ancienne config sauvegardée continuent de
fonctionner normalement ; seul un appel à `sp.save_config()` après coup propage la
réinitialisation sur disque.

---

## Méthodes

| Méthode | Description |
|---------|--------------|
| `add_alias(method_name, alias)` | Enregistre un alias (ou une liste d'alias) pour `method_name`. |
| `add_aliases(mapping)` | Enregistre en masse des alias depuis un dict `{method_name: alias_ou_liste}`. |
| `remove_alias(method_name, alias)` | Retire un alias et supprime la fonction liée. |
| `reset()` | Efface tous les alias en mémoire et détache leurs fonctions. Ne touche **pas** au fichier sauvegardé tant que `save_config()` n'est pas appelé. |
| `save_config(path=None)` | Sauvegarde le registre d'alias actuel sur disque (par défaut `~/.seraplot/config.json`) ; écrase toute sauvegarde précédente. Retourne le chemin écrit. |
| `load_config(path=None)` | Charge et applique un registre d'alias précédemment sauvegardé. Retourne `True` si un fichier a été trouvé et chargé. |

---

## Exemples

```python
import seraplot as sp

sp.config.add_alias("build_bar3d_chart", "mybar3d")
sp.mybar3d("Nom personnalise", labels=["A", "B"], values=[1, 2])

sp.config.add_aliases({
    "build_bar_chart": ["mybar", "qbar"],
    "build_line_chart": "myline",
})

sp.save_config()
```

```python
# Dans un tout nouveau kernel, plus tard
import seraplot as sp
sp.mybar3d("Fonctionne toujours apres redemarrage", labels=["A", "B"], values=[3, 4])
```

</div>
