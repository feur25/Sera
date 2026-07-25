from .seraplot import (
    Chart,
    Canvas,
    canvas,
    canvas_load,
    canvas_save_named,
    canvas_load_named,
    canvas_default_dir,
    Dataset,
    DatasetStats,
    Table,
    SeraDFrame,
    DFrameBuilder,
    LiveStream,
    theme,
    config as _config_fn,
    _sera_call,
    _sera_list,
    _sera_aliases,
    _alias_add,
    _alias_remove,
    _alias_reset,
    _alias_list_json,
    _alias_resolve,
    _themes as themes,
    _reset_theme as reset_theme,
    _set_global_background as set_background,
    _reset_global_background as reset_background,
    _set_global_background as set_global_background,
    _reset_global_background as reset_global_background,
    _set_auto_display as set_auto_display,
    _reset_config as reset_config,
    _demos as demos,
    _demo as demo,
    _params_json,
    _required_params_json,
    _true_required_params_json,
    _chart_variants_json,
    _chart_themes_json,
    _scenes3d_json,
    _docs_json,
    chart_info,
    bind_colors,
    clear_color_bindings,
    __version__,
)

try:
    from .seraplot import App
except ImportError:
    App = None

import json as _json


def params(chart=None, variant=None):
    return _json.loads(_params_json(chart, variant))


def required_params(chart=None, variant=None):
    return _json.loads(_required_params_json(chart, variant))


def true_required_params(chart=None, variant=None):
    return _json.loads(_true_required_params_json(chart, variant))


def chart_variants():
    return _json.loads(_chart_variants_json())


def chart_themes():
    return _json.loads(_chart_themes_json())


def scenes3d():
    return _json.loads(_scenes3d_json())


def docs():
    return _json.loads(_docs_json())


def _json_ready(value):
    if isinstance(value, Chart):
        return value.html
    if isinstance(value, (list, tuple)):
        return [_json_ready(v) for v in value]
    if isinstance(value, dict):
        return {k: _json_ready(v) for k, v in value.items()}
    if hasattr(value, "tolist"):
        return value.tolist()
    return value


def _make_fn(name):
    def _fn(*args, **kwargs):
        if args:
            first = args[0]
            if isinstance(first, (list, tuple)) and "charts" not in kwargs:
                kwargs["charts"] = first
            else:
                kwargs.setdefault("title", first)
        kwargs = {k: _json_ready(v) for k, v in kwargs.items()}
        result = _sera_call(name, _json.dumps(kwargs))
        stripped = result.lstrip()
        if stripped.startswith("{") or stripped.startswith("["):
            try:
                return _json.loads(result)
            except ValueError:
                pass
        return Chart(result)
    _fn.__name__ = name
    _fn.__qualname__ = name
    return _fn

_alias_map = dict(_sera_aliases())

for _n in _sera_list():
    globals()[_n] = _make_fn(_n)

for _alias, _target in _alias_map.items():
    if _alias not in globals() and _target in globals():
        globals()[_alias] = globals()[_target]

del _make_fn, _alias_map, _n, _alias, _target


import os as _os


def _default_visual_config_path():
    return _os.path.join(_os.path.expanduser("~"), ".seraplot", "config.json")


class _ConfigProxy:
    def __init__(self):
        self._state = {}

    def __call__(self, *args, **kwargs):
        result = _config_fn(*args, **kwargs)
        self._state.update(kwargs)
        return result

    @property
    def aliases(self):
        return _json.loads(_alias_list_json())

    def add_alias(self, method_name, alias):
        for a in ([alias] if isinstance(alias, str) else alias):
            _alias_add(method_name, a)
        return self

    def add_aliases(self, mapping):
        for method_name, alias in dict(mapping).items():
            self.add_alias(method_name, alias)
        return self

    def remove_alias(self, method_name, alias):
        _alias_remove(method_name, alias)
        return self

    def reset(self):
        _alias_reset()
        return self

    def save(self, path=None):
        target = path or _default_visual_config_path()
        _os.makedirs(_os.path.dirname(target) or ".", exist_ok=True)
        payload = {"visual": self._state, "aliases": self.aliases}
        with open(target, "w", encoding="utf-8") as f:
            _json.dump(payload, f, indent=2)
        return target

    def load(self, path=None):
        target = path or _default_visual_config_path()
        try:
            with open(target, "r", encoding="utf-8") as f:
                payload = _json.load(f)
        except FileNotFoundError:
            return False
        visual = payload.get("visual", {})
        if visual:
            _config_fn(**visual)
            self._state.update(visual)
        for method_name, alias_list in payload.get("aliases", {}).items():
            self.add_alias(method_name, alias_list)
        return True


config = _ConfigProxy()


def save_config(path=None):
    return config.save(path)


def load_config(path=None):
    return config.load(path)


config.load()


def fetch_image(url: str) -> str:
    import urllib.request as _ur, base64 as _b64, mimetypes as _mt
    with _ur.urlopen(url) as resp:
        data = resp.read()
        ct = resp.headers.get_content_type() or _mt.guess_type(url)[0] or "image/jpeg"
    return "data:" + ct + ";base64," + _b64.b64encode(data).decode()


def __getattr__(name):
    target = _alias_resolve(name)
    if target is not None and target in globals():
        return globals()[target]
    raise AttributeError(f"module {__name__!r} has no attribute {name!r}")
