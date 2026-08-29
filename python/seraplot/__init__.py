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
    _sera_call_fast,
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
    _chart_methods_json,
    chart_info,
    bind_colors,
    clear_color_bindings,
    __version__,
)

try:
    from .seraplot import App
except ImportError:
    App = None

try:
    from .seraplot import serve_board, export_board_html
except ImportError:
    serve_board = None
    export_board_html = None

try:
    from .seraplot import (
        pulse_activate,
        pulse_status,
        pulse_machine_id,
        pulse_health,
        pulse_serve_health,
        pulse_serve_metrics,
    )
except ImportError:
    pulse_activate = None
    pulse_status = None
    pulse_machine_id = None
    pulse_health = None
    pulse_serve_health = None
    pulse_serve_metrics = None

try:
    from .seraplot import SecureDFrame, SecureDFrameBuilder, SeraKey
except ImportError:
    SecureDFrame = None
    SecureDFrameBuilder = None
    SeraKey = None

try:
    from .seraplot import export_pdf_report
except ImportError:
    export_pdf_report = None

try:
    from .seraplot import SeraDFrameGroupBy
except ImportError:
    SeraDFrameGroupBy = None

import json as _json
from functools import lru_cache, singledispatch, singledispatchmethod


@lru_cache(maxsize=None)
def _params_json_cached(chart, variant):
    return _params_json(chart, variant)


@lru_cache(maxsize=None)
def _required_params_json_cached(chart, variant):
    return _required_params_json(chart, variant)


@lru_cache(maxsize=None)
def _true_required_params_json_cached(chart, variant):
    return _true_required_params_json(chart, variant)


@lru_cache(maxsize=1)
def _chart_variants_json_cached():
    return _chart_variants_json()


@lru_cache(maxsize=1)
def _chart_themes_json_cached():
    return _chart_themes_json()


@lru_cache(maxsize=1)
def _scenes3d_json_cached():
    return _scenes3d_json()


@lru_cache(maxsize=1)
def _docs_json_cached():
    return _docs_json()


@lru_cache(maxsize=1)
def _chart_methods_json_cached():
    return _chart_methods_json()


def params(chart=None, variant=None):
    return _json.loads(_params_json_cached(chart, variant))


def required_params(chart=None, variant=None):
    return _json.loads(_required_params_json_cached(chart, variant))


def true_required_params(chart=None, variant=None):
    return _json.loads(_true_required_params_json_cached(chart, variant))


def chart_variants():
    return _json.loads(_chart_variants_json_cached())


def chart_themes():
    return _json.loads(_chart_themes_json_cached())


def scenes3d():
    return _json.loads(_scenes3d_json_cached())


def docs():
    return _json.loads(_docs_json_cached())


def chart_methods():
    return _json.loads(_chart_methods_json_cached())


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


@singledispatch
def _coerce_call_args(first, kwargs):
    kwargs.setdefault("title", first)
    return kwargs


@_coerce_call_args.register(list)
@_coerce_call_args.register(tuple)
def _(first, kwargs):
    if "charts" not in kwargs:
        kwargs["charts"] = list(first)
    return kwargs


@_coerce_call_args.register(dict)
def _(first, kwargs):
    merged = dict(first)
    merged.update(kwargs)
    return merged


_NATIVE_PUSH_BUILDERS = frozenset({
    "build_bar",
    "build_heatmap",
    "build_candlestick",
    "build_icicle",
    "build_line",
    "build_line_chart",
    "build_area_chart",
    "build_scatter_chart",
    "build_bubble",
})


def _make_fn(name):
    native = name in _NATIVE_PUSH_BUILDERS

    def _fn(*args, **kwargs):
        if args:
            kwargs = _coerce_call_args(args[0], kwargs)
        kwargs = {k: _json_ready(v) for k, v in kwargs.items()}
        if native:
            title = kwargs.pop("title", "") or ""
            labels = kwargs.pop("labels", None)
            values = kwargs.pop("values", None)
            theme = kwargs.pop("theme", None)
            result = _sera_call_fast(name, title, labels, values, theme, **kwargs)
        else:
            result = _sera_call(name, _json.dumps(kwargs))
        stripped = result.lstrip()
        if stripped.startswith("{") or stripped.startswith("["):
            try:
                return _json.loads(result)
            except ValueError:
                raise ValueError(f"Failed to parse JSON result from {name!r}: {result!r}")
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


@lru_cache(maxsize=1)
def _alias_list_json_cached():
    return _alias_list_json()


class _ConfigProxy:
    def __init__(self):
        self._state = {}

    def __call__(self, *args, **kwargs):
        result = _config_fn(*args, **kwargs)
        self._state.update(kwargs)
        return result

    @property
    def aliases(self):
        return _json.loads(_alias_list_json_cached())

    @singledispatchmethod
    def add(self, spec, alias=None):
        raise TypeError(f"config.add() does not support {type(spec).__name__!r}")

    @add.register
    def _(self, spec: str, alias=None):
        return self.add_alias(spec, alias)

    @add.register
    def _(self, spec: dict, alias=None):
        return self.add_aliases(spec)

    def add_alias(self, method_name, alias):
        for a in ([alias] if isinstance(alias, str) else alias):
            _alias_add(method_name, a)
        _alias_list_json_cached.cache_clear()
        return self

    def add_aliases(self, mapping):
        for method_name, alias in dict(mapping).items():
            self.add_alias(method_name, alias)
        return self

    def remove_alias(self, method_name, alias):
        _alias_remove(method_name, alias)
        _alias_list_json_cached.cache_clear()
        return self

    def reset(self):
        _alias_reset()
        _alias_list_json_cached.cache_clear()
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
