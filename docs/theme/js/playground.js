(function () {
    var SELF_SRC = document.currentScript ? document.currentScript.src : "";
    var DEBOUNCE_MS = 500;
    var MONACO_BASE = 'https://cdn.jsdelivr.net/npm/monaco-editor@0.45.0/min/vs';

    var DEFAULT_VALUES = {
        labels: '["Alpha","Beta","Gamma","Delta","Epsilon","Zeta","Eta","Theta"]',
        category_labels: '["Alpha","Beta","Gamma","Delta","Epsilon"]',
        categories: '["Alpha","Alpha","Beta","Beta","Gamma","Gamma","Delta","Delta"]',
        super_categories: '["G1","G1","G2","G2","G3","G3","G4","G4"]',
        names: '["Alpha","Beta","Gamma","Delta","Epsilon"]',
        values: '[12,28,18,34,22,15,30,20]',
        series_values: '[12,28,18,34,22,8,16,12,22,15,5,11,9,17,13]',
        series: '[[12,28,18,34,22],[8,16,12,22,15],[5,11,9,17,13]]',
        series_names: '["Alpha","Beta","Gamma"]',
        widths: '[1,2,1.5,1,1.2]',
        weights: '[1,2,3,4,5,6,7,8]',
        x: '[10,25,40,55,70,85,100,115]',
        x_values: '[10,25,40,55,70,85,100,115]',
        y: '[18,42,28,55,38,72,48,85]',
        y_values: '[18,42,28,55,38,72,48,85]',
        z: '[2,4,3,6,8,5,7,9]',
        z_values: '[2,4,3,6,8,5,7,9]',
        sizes: '[18,42,28,55,38,72,48,85]',
        size: '[18,42,28,55,38,72,48,85]',
        opens: '[100,105,110,108,115,112,118,122]',
        highs: '[108,112,118,114,122,120,125,128]',
        lows: '[98,102,106,104,112,110,115,118]',
        closes: '[105,110,108,115,118,116,122,126]',
        matrix: '[[1.0,0.85,0.42,0.18],[0.85,1.0,0.65,0.31],[0.42,0.65,1.0,0.74],[0.18,0.31,0.74,1.0]]',
        data: '[[1,2,3,4,5],[5,4,3,2,1],[2,3,4,3,2]]',
        bins: '15',
        units_per_icon: '10',
        icon_size: '24',
        max_icons_per_column: '10',
        parents: '["","Root","Root","A","A","B"]',
        ids: '["Root","A","B","A1","A2","B1"]',
        offset_groups: '["G1","G1","G2","G2"]',
        regions: '["FR","DE","IT","ES","UK"]',
        positions: '[1,2,3,4,5]',
        actuals: '[60,75,82,55,68]',
        targets: '[70,80,80,60,75]',
        ranges: '[[40,70,90],[50,75,95],[55,80,100],[35,60,85],[45,70,90]]',
        words: '["data","chart","plot","graph","viz","render","scale","axis"]',
        frequencies: '[40,32,28,24,20,16,12,10]',
        groups: '["A","B","A","B","A","B"]',
        color_groups: '["A","B","A","B","C","C","A","B"]',
        color_values: '[10,25,40,55,70,85,100,115]',
        sort_order: '"none"',
        legend_position: '"right"',
    };

    var state = {
        editor: null,
        iframe: null,
        statusDot: null,
        statusText: null,
        loaderEl: null,
        errEl: null,
        slug: null,
        baseFn: null,
        baseParams: [],
        variants: [],
        currentVariant: 0,
        debTimer: null,
        lastSent: '',
        wasmAliases: null,
    };

    function pageSlug() {
        var m = window.location.pathname.match(/\/([^\/]+?)(?:\.html?)?$/);
        return m ? m[1] : null;
    }

    function isChartPage() {
        var slug = pageSlug();
        return (/\/charts\//.test(window.location.pathname) && slug && slug !== 'index') ||
               slug === 'playground';
    }

    function isFullPagePlayground() {
        return pageSlug() === 'playground';
    }

    function detectLang() {
        var visEn = document.querySelector('.lang-en');
        var visFr = document.querySelector('.lang-fr');
        if (visFr && visEn) {
            var stEn = window.getComputedStyle(visEn).display;
            if (stEn === 'none') return 'fr';
        }
        return 'en';
    }

    function getThemeBase() {
        if (SELF_SRC) return SELF_SRC.replace(/[^/]*$/, '') + '../wasm/';
        var parts = window.location.pathname.split('/').filter(Boolean);
        parts.pop();
        return new Array(parts.length).join('../') + 'docs/theme/wasm/';
    }

    function loadMonaco(cb) {
        if (window.monaco) { cb(); return; }
        if (window.__spMonacoLoading) { window.__spMonacoLoading.then(cb); return; }
        window.__spMonacoLoading = new Promise(function (resolve) {
            var loader = document.createElement('script');
            loader.src = MONACO_BASE + '/loader.js';
            loader.onload = function () {
                window.require.config({ paths: { vs: MONACO_BASE } });
                window.MonacoEnvironment = { getWorkerUrl: function () { return 'data:text/javascript;charset=utf-8,'; } };
                window.require(['vs/editor/editor.main'], function () { resolve(); cb(); });
            };
            document.head.appendChild(loader);
        });
    }

    function ensureWasm(cb) {
        var sp = window.SeraplotWASM;
        if (!sp) { setTimeout(function () { ensureWasm(cb); }, 80); return; }
        if (sp.__ready) { if (!state.wasmAliases) state.wasmAliases = buildDynamicAliases(sp); cb(); return; }
        if (sp.__loading) {
            sp.__loading.then(function () { if (!state.wasmAliases) state.wasmAliases = buildDynamicAliases(window.SeraplotWASM); cb(); }, function () {
                sp.__loading = null;
                setStatus('err', 'WASM load failed');
                showLoader('Failed to load WASM module.');
            });
            return;
        }
        sp.__loading = sp.__init(getThemeBase() + 'seraplot_bg.wasm?v=' + (window.SP_WASM_BUILD || '0')).then(function () { state.wasmAliases = buildDynamicAliases(window.SeraplotWASM); cb(); }).catch(function (e) {
            sp.__loading = null;
            setStatus('err', 'WASM load failed');
            showLoader('Failed to load WASM module.<br>' + (e && e.message ? e.message : ''));
        });
    }

    function buildDynamicAliases(sp) {
        if (typeof sp.chartAliases === 'function') {
            try {
                var pairs = JSON.parse(sp.chartAliases());
                var fromRegistry = {};
                for (var p = 0; p < pairs.length; p++) {
                    var alias = pairs[p][0];
                    var snakeFn = pairs[p][1];
                    var camelFn = snakeFn.replace(/_([a-z0-9])/g, function (_m, c) { return c.toUpperCase(); });
                    if (typeof sp[camelFn] === 'function') fromRegistry[alias] = camelFn;
                }
                if (Object.keys(fromRegistry).length) return fromRegistry;
            } catch (e) {}
        }
        var aliases = {};
        var keys = Object.keys(sp);
        for (var i = 0; i < keys.length; i++) {
            var k = keys[i];
            if (typeof sp[k] !== 'function') continue;
            var snake = k.replace(/[A-Z]/g, function (c) { return '_' + c.toLowerCase(); });
            var base = snake.indexOf('build_') === 0 ? snake.slice(6) : snake;
            var stripped = base.length > 6 && base.slice(-6) === '_chart' ? base.slice(0, -6) : base;
            aliases[base] = k;
            if (stripped !== base) aliases[stripped] = k;
            if (base.slice(-1) !== 's') { aliases[base + 's'] = k; }
            if (stripped !== base && stripped.slice(-1) !== 's') { aliases[stripped + 's'] = k; }
        }
        return aliases;
    }

    function fnCandidates(fn) {
        var camel = fn.split('_').map(function (s, i) {
            return i === 0 ? s : s.charAt(0).toUpperCase() + s.slice(1);
        }).join('');
        var base = camel.charAt(0).toUpperCase() + camel.slice(1);
        var out = [];
        if (camel.indexOf('build') === 0) out.push(camel);
        else { out.push('build' + base); out.push('build' + base + 'Chart'); out.push(camel); }
        out.push('build' + base + 'Chart');
        var seen = {}, dedup = [];
        for (var i = 0; i < out.length; i++) {
            if (!seen[out[i]]) { seen[out[i]] = 1; dedup.push(out[i]); }
        }
        return dedup;
    }

    function resolveWasmFn(fn) {
        var sp = window.SeraplotWASM;
        if (!sp) return null;
        var aliases = state.wasmAliases;
        var resolved = (aliases && aliases[fn]) ? aliases[fn] : null;
        if (resolved && typeof sp[resolved] === 'function') return { name: resolved, fn: sp[resolved] };
        var cands = fnCandidates(fn);
        for (var i = 0; i < cands.length; i++) {
            if (typeof sp[cands[i]] === 'function') return { name: cands[i], fn: sp[cands[i]] };
        }
        if (fn.slice(-1) === 's') {
            var singular = fn.slice(0, -1);
            var cands2 = fnCandidates(singular);
            for (var j = 0; j < cands2.length; j++) {
                if (typeof sp[cands2[j]] === 'function') return { name: cands2[j], fn: sp[cands2[j]] };
            }
        }
        return null;
    }

    function parseSignature(sig) {
        var m = sig.match(/sp\.([a-zA-Z_][a-zA-Z0-9_]*)\s*\(([\s\S]*?)\)\s*(?:->|$)/);
        if (!m) {
            m = sig.match(/sp\.([a-zA-Z_][a-zA-Z0-9_]*)\s*\(([\s\S]*)\)/);
            if (!m) return null;
        }
        var fn = m[1], raw = m[2], params = [], depth = 0, buf = '', inStr = false, q = '';
        for (var i = 0; i < raw.length; i++) {
            var c = raw[i];
            if (inStr) {
                if (c === '\\' && i + 1 < raw.length) { buf += c + raw[i + 1]; i++; continue; }
                if (c === q) inStr = false;
                buf += c;
            } else {
                if (c === '"' || c === "'") { inStr = true; q = c; buf += c; }
                else if (c === '(' || c === '[' || c === '{') { depth++; buf += c; }
                else if (c === ')' || c === ']' || c === '}') { depth--; buf += c; }
                else if (c === ',' && depth === 0) { addParam(buf, params); buf = ''; }
                else buf += c;
            }
        }
        if (buf.trim().length) addParam(buf, params);
        return { fn: fn, params: params };
    }

    function addParam(raw, out) {
        var s = raw.trim();
        if (!s.length || s === '*' || s === '/' || s.indexOf('**') === 0) return;
        if (s.charAt(0) === '*') s = s.replace(/^\*+/, '');
        var nm = s.match(/^([a-zA-Z_][a-zA-Z0-9_]*)/);
        if (!nm) return;
        var name = nm[1];
        if (name === 'self' || name === 'cls' || name === 'kwargs' || name === 'args') return;
        var def = null;
        var eq = s.indexOf('=');
        if (eq !== -1) {
            var rhs = s.slice(eq + 1).trim();
            var qm = rhs.match(/^"([^"]*)"|^'([^']*)'/);
            if (qm) def = '"' + (qm[1] || qm[2]) + '"';
            else if (/^(True|False|None)$/.test(rhs)) def = rhs;
            else if (/^-?\d/.test(rhs)) { var nm2 = rhs.match(/^-?[\d.]+/); if (nm2) def = nm2[0]; }
        }
        out.push({ name: name, defaultValue: def });
    }

    function findPageSignature() {
        var nodes = document.querySelectorAll('p code, pre code, pre, code');
        for (var i = 0; i < nodes.length; i++) {
            var node = nodes[i];
            if (node.closest && node.closest('.sp-variant')) continue;
            var t = node.textContent || '';
            if (t.indexOf('sp.') === -1) continue;
            var p = parseSignature(t);
            if (p && p.params.length >= 1) return p;
        }
        return null;
    }

    function discoverVariants() {
        var slug = state.slug;
        var fn = state.baseFn || (slug ? slug.replace(/-/g, '_') : '');
        var sp = window.SeraplotWASM;
        if (sp && typeof sp.chartVariants === 'function' && fn) {
            try {
                var map = JSON.parse(sp.chartVariants());
                var fromWasm = normalizeVariantList(map && map[fn]);
                if (fromWasm.length) return fromWasm;
            } catch (e) {}
        }
        var lang = detectLang();
        var nodes = document.querySelectorAll('.sp-variant[id^="' + slug + '-' + lang + '-"]');
        if (nodes.length === 0) {
            nodes = document.querySelectorAll('.sp-variant[id^="' + slug + '-"]');
        }
        if (nodes.length === 0) {
            nodes = document.querySelectorAll('.sp-variant[data-variant]');
        }
        var seen = {}, out = [];
        for (var i = 0; i < nodes.length; i++) {
            var id = nodes[i].id;
            var parts = id.split('-');
            var name = nodes[i].getAttribute('data-variant') || parts[parts.length - 1];
            name = normalizeVariantName(name);
            if (!name || seen[name]) continue;
            seen[name] = true;
            out.push({ key: name, aliases: [name] });
        }
        return out;
    }

    function normalizeVariantName(name) {
        return String(name || '').trim().replace(/-/g, '_');
    }

    function normalizeVariantList(raw) {
        if (!raw) return [];
        if (Array.isArray(raw)) {
            return raw.map(function (v) {
                if (typeof v === 'string') return { key: normalizeVariantName(v), aliases: [normalizeVariantName(v)] };
                return {
                    key: normalizeVariantName(v.key || v.name || ''),
                    aliases: Array.isArray(v.aliases) ? v.aliases.slice() : []
                };
            }).filter(function (v) { return v.key; });
        }
        if (raw.variants) return normalizeVariantList(raw.variants);
        return [];
    }

    function variantKey(item) {
        return typeof item === 'string' ? normalizeVariantName(item) : normalizeVariantName(item && item.key);
    }

    function variantLabel(item) {
        if (typeof item === 'string') return item;
        var aliases = item && item.aliases;
        return aliases && aliases.length ? aliases[0] : variantKey(item);
    }

    function escAttr(s) {
        return String(s || '').replace(/&/g, '&amp;').replace(/"/g, '&quot;').replace(/</g, '&lt;').replace(/>/g, '&gt;');
    }

    function findVariantParamHints() {
        var hints = {};
        var tables = document.querySelectorAll('table');
        for (var t = 0; t < tables.length; t++) {
            var table = tables[t];
            if (table.closest && table.closest('.sp-variant')) continue;
            var headRow = table.querySelector('thead tr') || table.querySelector('tr');
            if (!headRow) continue;
            var headCells = headRow.querySelectorAll('th, td');
            var head = [];
            for (var h = 0; h < headCells.length; h++) head.push((headCells[h].textContent || '').trim().toLowerCase());
            var variantCol = -1, extrasCol = -1;
            for (var hh = 0; hh < head.length; hh++) {
                if (head[hh] === 'variant' && variantCol === -1) variantCol = hh;
                if (/extra|key.*arg|required/.test(head[hh])) extrasCol = hh;
            }
            if (variantCol === -1 || extrasCol === -1) continue;
            var rows = table.querySelectorAll('tbody tr');
            if (!rows.length) rows = table.querySelectorAll('tr');
            for (var r = 0; r < rows.length; r++) {
                var cells = rows[r].querySelectorAll('td');
                if (cells.length <= Math.max(variantCol, extrasCol)) continue;
                var nameCell = cells[variantCol];
                var nameCodes = nameCell.querySelectorAll('code');
                var names = [];
                if (nameCodes.length) {
                    for (var nc = 0; nc < nameCodes.length; nc++) names.push((nameCodes[nc].textContent || '').replace(/["']/g, '').trim());
                } else {
                    names.push((nameCell.textContent || '').replace(/["']/g, '').trim());
                }
                var extraCell = cells[extrasCol];
                var paramCodes = extraCell.querySelectorAll('code');
                var params = [];
                for (var pc = 0; pc < paramCodes.length; pc++) {
                    var pn = (paramCodes[pc].textContent || '').trim();
                    if (/^[a-zA-Z_][a-zA-Z0-9_]*$/.test(pn)) params.push(pn);
                }
                for (var nn = 0; nn < names.length; nn++) {
                    if (names[nn]) hints[names[nn]] = params;
                }
            }
        }
        return hints;
    }

    function buildCode(variantName) {
        variantName = variantKey(variantName);
        var fn = state.baseFn || (state.slug ? state.slug.replace(/-/g, '_') : 'bar');
        if (isFullPagePlayground()) fn = state.baseFn || 'bar';
        try {
            var sp = window.SeraplotWASM;
            if (sp && typeof sp.demo === 'function') {
                var v = (variantName === 'default' || !variantName) ? 'basic' : variantName;
                var code = sp.demo(JSON.stringify({ family: fn, variant: v }));
                if (code && code.trim()) return code;
            }
        } catch (e) {}
        var lines = ['import seraplot as sp', '', 'c = sp.' + fn + '('];
        lines.push('    "' + capitalize(variantName === 'default' ? state.slug : variantName) + ' demo",');
        var paramSet = {};
        var ordered = [];
        function add(n) { if (!paramSet[n]) { paramSet[n] = 1; ordered.push(n); } }
        var hints = state.variantHints[variantName] || [];
        for (var h = 0; h < hints.length; h++) add(hints[h]);
        for (var i = 0; i < state.baseParams.length; i++) {
            var n = state.baseParams[i].name;
            if (n === 'title' || n === 'variant') continue;
            if (state.baseParams[i].defaultValue === null) add(n);
        }
        for (var j = 0; j < state.baseParams.length; j++) {
            var nm = state.baseParams[j].name;
            if (nm === 'title' || nm === 'variant') continue;
            if (DEFAULT_VALUES.hasOwnProperty(nm)) add(nm);
        }
        if (ordered.length === 0) {
            var dataKeys = ['labels', 'values', 'x_values', 'y_values', 'sizes', 'category_labels', 'series_values', 'series', 'matrix'];
            for (var d = 0; d < dataKeys.length; d++) {
                if (DEFAULT_VALUES.hasOwnProperty(dataKeys[d])) add(dataKeys[d]);
            }
        }
        for (var k = 0; k < ordered.length; k++) {
            var pn = ordered[k];
            if (DEFAULT_VALUES.hasOwnProperty(pn)) {
                lines.push('    ' + pn + '=' + DEFAULT_VALUES[pn] + ',');
            }
        }
        if (variantName && variantName !== 'default') {
            lines.push('    variant="' + variantName + '",');
        }
        lines.push(')\n');
        return lines.join('\n');
    }

    function capitalize(s) {
        if (!s) return 'Demo';
        return s.charAt(0).toUpperCase() + s.slice(1);
    }

    function injectStyles() {
        if (document.getElementById('sp-pg-style')) return;
        var css = [
            '@keyframes sp-spin{to{transform:rotate(360deg)}}',
            '@keyframes sp-fadein{from{opacity:0;transform:scale(.985)}to{opacity:1;transform:scale(1)}}',
            '@keyframes sp-pulse{0%,100%{opacity:1;transform:scale(1)}50%{opacity:.45;transform:scale(.85)}}',
            '@keyframes sp-ripple{0%{box-shadow:0 0 0 0 rgba(34,197,94,.55)}100%{box-shadow:0 0 0 14px rgba(34,197,94,0)}}',
            '@keyframes sp-orb{0%{transform:translate3d(0,0,0)}50%{transform:translate3d(40px,-30px,0)}100%{transform:translate3d(0,0,0)}}',
            '@keyframes sp-rotate-bg{0%{background-position:0% 50%}100%{background-position:200% 50%}}',
            '.sp-pg-wrap{position:relative;display:flex;flex-direction:column;border-radius:14px;overflow:hidden;margin:1.6em 0 2em;font-family:"Inter","Segoe UI",sans-serif;background:var(--sp-surface);box-shadow:inset 0 6px 20px rgba(0,0,0,.6),inset 0 -1px 0 rgba(255,255,255,.03),0 -3px 8px rgba(0,0,0,.12),0 1px 2px rgba(0,0,0,.3),0 0 0 1px var(--sp-border)}',
            '.sp-pg-wrap.sp-pg-flush{margin-top:0}',
            '.sp-pg-wrap::before{content:"";position:absolute;inset:-1px;border-radius:15px;padding:1px;background:linear-gradient(120deg,rgba(99,102,241,.4),rgba(168,85,247,.3),rgba(34,211,238,.35),rgba(99,102,241,.4));background-size:200% 200%;animation:sp-rotate-bg 14s linear infinite;-webkit-mask:linear-gradient(#000 0 0) content-box,linear-gradient(#000 0 0);-webkit-mask-composite:xor;mask-composite:exclude;pointer-events:none;opacity:.35;z-index:0}',
            '.sp-pg-hdr,.sp-pg-tabs,.sp-pg-main{position:relative;z-index:1}',
            '.sp-pg-hdr{display:flex;align-items:center;gap:12px;padding:6px 12px;background:var(--sp-surface-2);border-bottom:1px solid var(--sp-border);flex-wrap:wrap;row-gap:6px}',
            '.sp-pg-title{font-size:10px;font-weight:700;letter-spacing:.16em;color:var(--sp-text);text-transform:uppercase;flex-shrink:0}',
            '.sp-pg-title b{background:linear-gradient(135deg,#a5b4fc 0%,#67e8f9 100%);-webkit-background-clip:text;background-clip:text;color:transparent;font-weight:800}',
            '.sp-pg-spacer{flex:1}',
            '.sp-pg-btn{background:var(--sp-accent);color:#fff;border:none;border-radius:4px;padding:5px 14px;font-size:12px;font-weight:600;cursor:pointer;transition:filter .15s;font-family:inherit}',
            '.sp-pg-btn:hover{filter:brightness(1.15)}',
            '.sp-pg-btn:active{filter:brightness(.82)}',
            '.sp-pg-btn[disabled]{opacity:.45;cursor:not-allowed}',
            '.sp-pg-conn{display:flex;align-items:center;gap:8px;font-size:11px;font-weight:600;color:var(--sp-text-muted);letter-spacing:.05em;text-transform:uppercase;flex-shrink:0;padding:4px 10px;background:var(--sp-surface);border:1px solid var(--sp-border);border-radius:14px}',
            '.sp-pg-dot{width:8px;height:8px;border-radius:50%;background:var(--sp-text-muted);flex-shrink:0;transition:background .3s}',
            '.sp-d-loading{background:var(--sp-warn)!important;animation:sp-pulse 1s infinite}',
            '.sp-d-ready{background:var(--sp-ok)!important;animation:sp-ripple 2.4s infinite}',
            '.sp-d-err{background:var(--sp-danger)!important;box-shadow:0 0 8px var(--sp-danger)}',
            '.sp-pg-tabs{display:flex;gap:0;padding:0;background:var(--sp-surface-2);border-bottom:1px solid var(--sp-border);box-shadow:inset 0 3px 8px rgba(0,0,0,.35);overflow-x:auto;scrollbar-width:none}',
            '.sp-pg-tabs::-webkit-scrollbar{display:none}',
            '.sp-pg-tab{font:600 11.5px/1 "Inter",sans-serif;letter-spacing:.04em;padding:9px 16px;color:var(--sp-text-muted);background:var(--sp-surface);border:none;border-right:1px solid var(--sp-surface-2);border-bottom:1px solid var(--sp-border);cursor:pointer;transition:all .15s;white-space:nowrap;font-family:inherit;text-transform:capitalize}',
            '.sp-pg-tab:hover{color:var(--sp-text);background:var(--sp-surface-hover)}',
            '.sp-pg-tab.sp-active{color:#e0e7ff;background:#0a0f1c;border-bottom-color:#0a0f1c;box-shadow:inset 0 2px 5px rgba(0,0,0,.4)}',
            '.sp-pg-main{display:flex;height:520px;background:#0a0f1c}',
            '.sp-pg-ecol{width:50%;min-width:180px;max-width:80%;display:flex;flex-direction:column;flex-shrink:0;overflow:hidden;border-right:1px solid #1e293b;background:#0a0f1c}',
            '.sp-pg-ehdr{display:flex;align-items:center;padding:0;background:#0d1426;border-bottom:1px solid #1e293b;box-shadow:inset 0 3px 8px rgba(0,0,0,.4);font:11px/1 "Segoe UI",sans-serif;color:#94a3b8}',
            '.sp-pg-etab{padding:7px 14px;background:#0a0f1c;border:none;border-right:1px solid #1e293b;color:#e0e7ff;font-size:12px;font-weight:500;letter-spacing:.02em;display:flex;align-items:center;gap:8px;font-family:inherit}',
            '.sp-pg-etab::before{content:"🐍";font-size:11px}',
            '.sp-pg-cm-wrap{flex:1;background:#0a0f1c;overflow:hidden;position:relative}',
            '.sp-pg-cm-wrap .monaco-editor{background:#0a0f1c!important}',
            '.sp-pg-cm-wrap .monaco-editor .margin{background:#0a0f1c!important}',
            '.sp-pg-cm-wrap .monaco-editor .monaco-editor-background{background:#0a0f1c!important}',
            '.sp-pg-divider{width:5px;background:var(--sp-border);cursor:col-resize;flex-shrink:0;position:relative;z-index:3;transition:background .15s}',
            '.sp-pg-divider:hover,.sp-pg-divider.sp-dragging{background:var(--sp-accent)}',
            '.sp-pg-pcol{flex:1;min-width:160px;display:flex;flex-direction:column;position:relative;background:var(--sp-surface)}',
            '.sp-pg-phdr{display:flex;align-items:center;padding:7px 14px;background:var(--sp-surface-2);border-bottom:1px solid var(--sp-border);box-shadow:inset 0 3px 8px rgba(0,0,0,.35);font:11px/1 "Segoe UI",sans-serif;color:var(--sp-text-muted);letter-spacing:.04em;text-transform:uppercase;font-weight:700;gap:8px}',
            '.sp-pg-phdr::before{content:"";display:inline-block;width:8px;height:8px;border-radius:50%;background:var(--sp-ok);box-shadow:0 0 8px rgba(45,212,191,.6)}',
            '.sp-pg-iframe{flex:1;border:none;background:#fff;width:100%;display:block;animation:sp-fadein .28s cubic-bezier(.2,.8,.2,1)}',
            '.sp-pg-loader{position:absolute;left:0;right:0;top:33px;bottom:0;display:flex;align-items:center;justify-content:center;background:var(--sp-surface);flex-direction:column;gap:18px;color:var(--sp-text-muted);font-size:12px;font-weight:600;letter-spacing:.1em;text-transform:uppercase;transition:opacity .25s;z-index:1;text-align:center;padding:0 24px}',
            '.sp-pg-loader.sp-hide{opacity:0;pointer-events:none}',
            '.sp-pg-loader b{color:var(--sp-important);font-weight:700}',
            '.sp-pg-spinner{width:38px;height:38px;border:3px solid rgba(124,58,237,.15);border-top-color:#a78bfa;border-right-color:#67e8f9;border-radius:50%;animation:sp-spin .9s cubic-bezier(.5,.1,.5,.9) infinite;filter:drop-shadow(0 0 10px rgba(167,139,250,.5))}',
            '.sp-pg-err{position:absolute;left:14px;right:14px;bottom:14px;background:var(--sp-code-bg);border:1px solid var(--sp-danger);color:var(--sp-danger);font:11px/1.5 "Cascadia Code","Consolas",monospace;padding:9px 12px;border-radius:6px;backdrop-filter:blur(10px);max-height:160px;overflow:auto;white-space:pre-wrap;z-index:5;display:none}',
            '.sp-pg-err.sp-show{display:block;animation:sp-fadein .2s}',
        ].join('');
        var st = document.createElement('style'); st.id = 'sp-pg-style'; st.textContent = css; document.head.appendChild(st);
    }

    function setStatus(kind, txt) {
        if (!state.statusDot) return;
        state.statusDot.classList.remove('sp-d-loading', 'sp-d-ready', 'sp-d-err');
        if (kind) state.statusDot.classList.add('sp-d-' + kind);
        state.statusText.textContent = txt;
    }

    function showLoader(msg) {
        if (!state.loaderEl) return;
        state.loaderEl.innerHTML = '<div class="sp-pg-spinner"></div><div>' + msg + '</div>';
        state.loaderEl.classList.remove('sp-hide');
    }
    function hideLoader() { if (state.loaderEl) state.loaderEl.classList.add('sp-hide'); }
    function showErr(msg) { if (state.errEl) { state.errEl.textContent = msg; state.errEl.classList.add('sp-show'); } }
    function hideErr() { if (state.errEl) state.errEl.classList.remove('sp-show'); }

    function renderHtmlInIframe(html) {
        if (!state.iframe) return;
        var fitStyle = '<style>html,body{margin:0;padding:0;width:100%;height:100%;overflow:auto;background:#fff}body{display:flex;flex-direction:column;align-items:center;justify-content:center;box-sizing:border-box}body>*{max-width:100%!important;max-height:100%!important;box-sizing:border-box}svg,canvas{max-width:100%!important;max-height:100%!important;width:auto!important;height:auto!important;display:block;margin:0 auto}.chart-container,.plot-container,#chart,#plot{max-width:100%!important;max-height:100%!important;width:auto!important;height:auto!important;box-sizing:border-box;display:flex;flex-direction:column;align-items:center;justify-content:center}</style>';
        if (/<\/head>/i.test(html)) {
            html = html.replace(/<\/head>/i, fitStyle + '</head>');
        } else if (/<body[^>]*>/i.test(html)) {
            html = html.replace(/<body[^>]*>/i, function (m) { return m + fitStyle; });
        } else {
            html = fitStyle + html;
        }
        var blob = new Blob([html], { type: 'text/html' });
        var url = URL.createObjectURL(blob);
        var prev = state.iframe.dataset.blobUrl;
        state.iframe.src = url;
        state.iframe.dataset.blobUrl = url;
        if (prev) setTimeout(function () { URL.revokeObjectURL(prev); }, 500);
    }

    function parsePyVal(s) {
        s = s.trim();
        if (!s.length) return null;
        if (s[0] === '"' || s[0] === "'") {
            var q = s[0], out = '', i = 1;
            while (i < s.length && s[i] !== q) {
                if (s[i] === '\\' && i + 1 < s.length) { out += s[i + 1]; i += 2; }
                else { out += s[i]; i++; }
            }
            return out;
        }
        if (s[0] === '[') return parsePyList(s);
        if (s === 'True') return true;
        if (s === 'False') return false;
        if (s === 'None' || s === 'null') return null;
        var rangeM = s.match(/^list\s*\(\s*range\s*\(\s*(\d+)\s*(?:,\s*(\d+)\s*)?\)\s*\)$/);
        if (rangeM) {
            var aa = +rangeM[1], bb = rangeM[2] !== undefined ? +rangeM[2] : null;
            var arr = [], lo = bb === null ? 0 : aa, hi = bb === null ? aa : bb;
            for (var k = lo; k < hi; k++) arr.push(k);
            return arr;
        }
        if (/^0x[0-9a-fA-F]+$/.test(s)) return parseInt(s, 16);
        var n = Number(s);
        if (!isNaN(n)) return n;
        return s;
    }

    function parsePyList(s) {
        s = s.trim();
        if (s[0] !== '[') return null;
        var out = [], i = 1, depth = 1, buf = '', inStr = false, q = '';
        while (i < s.length && depth > 0) {
            var c = s[i];
            if (inStr) {
                buf += c;
                if (c === '\\' && i + 1 < s.length) { buf += s[i + 1]; i += 2; continue; }
                if (c === q) inStr = false;
            } else {
                if (c === '"' || c === "'") { inStr = true; q = c; buf += c; }
                else if (c === '[' || c === '(' || c === '{') { depth++; buf += c; }
                else if (c === ']' || c === ')' || c === '}') { depth--; if (depth === 0) { if (buf.trim().length) out.push(parsePyVal(buf)); break; } buf += c; }
                else if (c === ',' && depth === 1) { if (buf.trim().length) out.push(parsePyVal(buf)); buf = ''; }
                else buf += c;
            }
            i++;
        }
        return out;
    }

    function parsePyArgs(body) {
        var args = [], kwargs = {}, i = 0, depth = 0, buf = '', inStr = false, q = '';
        while (i < body.length) {
            var c = body[i];
            if (inStr) {
                buf += c;
                if (c === '\\' && i + 1 < body.length) { buf += body[i + 1]; i += 2; continue; }
                if (c === q) inStr = false;
            } else {
                if (c === '"' || c === "'") { inStr = true; q = c; buf += c; }
                else if (c === '(' || c === '[' || c === '{') { depth++; buf += c; }
                else if (c === ')' || c === ']' || c === '}') { depth--; buf += c; }
                else if (c === ',' && depth === 0) { pushArg(buf, args, kwargs); buf = ''; }
                else buf += c;
            }
            i++;
        }
        if (buf.trim().length) pushArg(buf, args, kwargs);
        return { args: args, kwargs: kwargs };
    }

    function pushArg(raw, args, kwargs) {
        var s = raw.trim();
        if (!s.length) return;
        var eq = -1, depth = 0, inStr = false, q = '';
        for (var i = 0; i < s.length; i++) {
            var c = s[i];
            if (inStr) { if (c === '\\') { i++; continue; } if (c === q) inStr = false; }
            else {
                if (c === '"' || c === "'") { inStr = true; q = c; }
                else if (c === '(' || c === '[' || c === '{') depth++;
                else if (c === ')' || c === ']' || c === '}') depth--;
                else if (c === '=' && depth === 0 && s[i + 1] !== '=' && s[i - 1] !== '=' && s[i - 1] !== '!' && s[i - 1] !== '<' && s[i - 1] !== '>') { eq = i; break; }
            }
        }
        if (eq === -1) args.push(parsePyVal(s));
        else { var k = s.slice(0, eq).trim(); var v = parsePyVal(s.slice(eq + 1)); kwargs[k] = v; }
    }

    function readParenBody(code, bodyStart) {
        var depth = 1, j = bodyStart, inStr = false, q = '';
        while (j < code.length && depth > 0) {
            var c = code[j];
            if (inStr) { if (c === '\\') { j += 2; continue; } if (c === q) inStr = false; }
            else {
                if (c === '"' || c === "'") { inStr = true; q = c; }
                else if (c === '(') depth++;
                else if (c === ')') depth--;
            }
            j++;
        }
        return j;
    }

    function readChain(code, start) {
        var chain = [], i = start;
        while (true) {
            var k = i;
            while (k < code.length && /\s/.test(code[k])) k++;
            var rest = code.slice(k);
            var m = rest.match(/^\.([a-zA-Z_][a-zA-Z0-9_]*)\s*\(/);
            if (!m) break;
            var bodyStart = k + m[0].length;
            var j = readParenBody(code, bodyStart);
            chain.push({ fn: m[1], body: code.slice(bodyStart, j - 1) });
            i = j;
        }
        return { chain: chain, end: i };
    }

    function extractAllCalls(code) {
        var calls = [], i = 0;
        while (i < code.length) {
            var idx = code.indexOf('sp.', i);
            if (idx === -1) break;
            var rest = code.slice(idx);
            var m = rest.match(/^sp\.([a-zA-Z_][a-zA-Z0-9_]*)\s*\(/);
            if (!m) { i = idx + 3; continue; }
            var bodyStart = idx + m[0].length;
            var j = readParenBody(code, bodyStart);
            var chained = readChain(code, j);
            calls.push({ fn: m[1], body: code.slice(bodyStart, j - 1), chain: chained.chain });
            i = chained.end;
        }
        return calls;
    }

    function runOnce(force) {
        if (!state.editor) return;
        var code = state.editor.getValue();
        if (!force && code === state.lastSent) return;
        state.lastSent = code;
        var calls = extractAllCalls(code);
        if (!calls.length) { showErr('No sp.<chart>(...) call detected'); return; }
        var sp = window.SeraplotWASM;
        if (!sp || !sp.__ready) { showErr('WASM not ready'); return; }
        var lastHtml = '';
        for (var c = 0; c < calls.length; c++) {
            var call = calls[c];
            try {
                if (call.fn === 'theme') {
                    var parsedTheme = parsePyArgs(call.body);
                    var name = (parsedTheme.args[0] || parsedTheme.kwargs.name || '').toString();
                    if (typeof sp.setTheme === 'function') sp.setTheme(name);
                    continue;
                }
                if (call.fn === 'reset_theme' || call.fn === 'resetTheme') {
                    if (typeof sp.resetTheme === 'function') sp.resetTheme('');
                    continue;
                }
                var entry = resolveWasmFn(call.fn);
                if (!entry) { showErr('No WASM entry for sp.' + call.fn + '() (tried: ' + fnCandidates(call.fn).join(', ') + ')'); hideLoader(); return; }
                var parsed = parsePyArgs(call.body);
                var input = {};
                if (parsed.args.length > 0 && typeof parsed.args[0] === 'string') input.title = parsed.args[0];
                for (var key in parsed.kwargs) if (parsed.kwargs.hasOwnProperty(key)) input[key] = parsed.kwargs[key];
                state.lastInput = input;
                state.lastFnName = entry.name;
                lastHtml = entry.fn(JSON.stringify(input));
                if (lastHtml && call.chain && call.chain.length && typeof sp.applyChartMethod === 'function') {
                    for (var mIdx = 0; mIdx < call.chain.length; mIdx++) {
                        var mCall = call.chain[mIdx];
                        var mParsed = parsePyArgs(mCall.body);
                        var mArgs = {};
                        for (var mKey in mParsed.kwargs) if (mParsed.kwargs.hasOwnProperty(mKey)) mArgs[mKey] = mParsed.kwargs[mKey];
                        lastHtml = sp.applyChartMethod(lastHtml, mCall.fn, JSON.stringify(mArgs));
                    }
                }
            } catch (e) {
                showErr('Render error in sp.' + call.fn + ': ' + (e && e.message ? e.message : String(e)));
                hideLoader();
                return;
            }
        }
        if (!lastHtml) {
            var keys = Object.keys(state.lastInput || {}).join(', ');
            showErr('Empty render output from ' + (state.lastFnName || '?') + '(). The chart returned an empty string — usually means a required field is missing or has invalid shape.\nFields sent: ' + keys);
            hideLoader();
            return;
        }
        hideErr();
        hideLoader();
        renderHtmlInIframe(lastHtml);
    }

    function debouncedRun() {
        if (state.debTimer) clearTimeout(state.debTimer);
        state.debTimer = setTimeout(function () { runOnce(false); }, DEBOUNCE_MS);
    }

    function attachResize(divider, ecol) {
        var dragging = false, startX = 0, startW = 0, parentW = 0;
        function down(e) {
            dragging = true;
            startX = e.clientX || (e.touches && e.touches[0].clientX) || 0;
            startW = ecol.getBoundingClientRect().width;
            parentW = ecol.parentElement.getBoundingClientRect().width;
            divider.classList.add('sp-dragging');
            document.body.style.userSelect = 'none';
            e.preventDefault();
        }
        function move(e) {
            if (!dragging) return;
            var x = e.clientX || (e.touches && e.touches[0].clientX) || 0;
            var w = startW + (x - startX);
            var min = 180, max = parentW - 160;
            if (w < min) w = min; if (w > max) w = max;
            ecol.style.width = w + 'px';
            if (state.editor && state.editor.layout) state.editor.layout();
        }
        function up() {
            if (!dragging) return;
            dragging = false;
            divider.classList.remove('sp-dragging');
            document.body.style.userSelect = '';
        }
        divider.addEventListener('mousedown', down);
        document.addEventListener('mousemove', move);
        document.addEventListener('mouseup', up);
        divider.addEventListener('touchstart', down, { passive: false });
        document.addEventListener('touchmove', move, { passive: false });
        document.addEventListener('touchend', up);
    }

    function tabsHtmlFor(variants) {
        var out = '';
        for (var i = 0; i < variants.length; i++) {
            out += '<button class="sp-pg-tab' + (i === 0 ? ' sp-active' : '') + '" type="button">' + escAttr(variantLabel(variants[i])) + '</button>';
        }
        return out;
    }

    function wireTabs(tabs, variants, wrap, slug, fileTab) {
        for (var t = 0; t < tabs.length; t++) {
            (function (idx) {
                tabs[idx].addEventListener('click', function () {
                    if (isFullPagePlayground()) {
                        state.baseFn = variantKey(variants[idx]);
                        state.currentVariant = 0;
                        for (var ti = 0; ti < tabs.length; ti++) tabs[ti].classList.toggle('sp-active', ti === idx);
                        if (fileTab) fileTab.textContent = state.baseFn + '.py';
                        if (state.editor) { state.editor.setValue(buildCode('basic')); runOnce(true); }
                    } else {
                        if (fileTab) fileTab.textContent = slug + '-' + variantLabel(variants[idx]) + '.py';
                        selectVariant(idx);
                    }
                });
            })(t);
        }
    }

    function selectVariant(idx) {
        var item = state.variants[idx];
        var name = variantKey(item);
        if (!name) return;
        state.currentVariant = idx;
        var tabs = document.querySelectorAll('.sp-pg-wrap .sp-pg-tab');
        for (var i = 0; i < tabs.length; i++) tabs[i].classList.toggle('sp-active', i === idx);
        if (state.editor) {
            state.editor.setValue(buildCode(item));
            runOnce(true);
        }
    }

    function fitWrapWidth(wrap) {
        var main = wrap.closest('main');
        var content = main ? (main.closest('.content') || main.parentElement) : null;
        if (!main || !content) return;
        var pad = parseFloat(getComputedStyle(content).paddingLeft || '0') + parseFloat(getComputedStyle(content).paddingRight || '0');
        var avail = content.clientWidth - pad;
        var mainW = main.clientWidth;
        if (avail <= mainW + 4) {
            wrap.style.width = '';
            wrap.style.marginLeft = '';
            wrap.style.marginRight = '';
            return;
        }
        var offset = (avail - mainW) / 2;
        wrap.style.width = avail + 'px';
        wrap.style.marginLeft = (-offset) + 'px';
        wrap.style.marginRight = (-offset) + 'px';
    }

    function watchWrapWidth(wrap) {
        var scheduled = false;
        function schedule() {
            if (scheduled) return;
            scheduled = true;
            requestAnimationFrame(function () { scheduled = false; fitWrapWidth(wrap); });
        }
        window.addEventListener('resize', schedule);
        var toggle = document.getElementById('mdbook-sidebar-toggle-anchor');
        if (toggle) toggle.addEventListener('change', function () { setTimeout(schedule, 320); });
        var pageWrapper = document.querySelector('.page-wrapper');
        if (pageWrapper) pageWrapper.addEventListener('transitionend', schedule);
        if (window.ResizeObserver) {
            var main = wrap.closest('main');
            if (main) new ResizeObserver(schedule).observe(main);
        }
    }

    function buildUI() {
        injectStyles();
        var slug = state.slug;
        var sig = findPageSignature();
        if (sig) { state.baseFn = sig.fn; state.baseParams = sig.params; }
        else { state.baseFn = isFullPagePlayground() ? 'bar' : slug.replace(/-/g, '_'); state.baseParams = []; }
        state.variantHints = findVariantParamHints();
        var variants = discoverVariants();
        if (variants.length === 0) {
            if (isFullPagePlayground()) {
                variants = ['bar','line','scatter','bubble','histogram','heatmap','pie','violin','radar','waterfall','sunburst','treemap','candlestick','funnel','boxplot','gauge'];
            } else {
                variants = ['default'];
            }
        }
        state.variants = variants;

        var tabsHtml = tabsHtmlFor(variants);

        var wrap = document.createElement('div');
        wrap.className = 'sp-pg-wrap';
        wrap.innerHTML =
            '<div class="sp-pg-hdr">' +
                '<div class="sp-pg-title">Playground · <b>' + slug + '</b></div>' +
                '<div class="sp-pg-spacer"></div>' +
                '<div class="sp-pg-conn"><span class="sp-pg-dot"></span><span class="sp-pg-text">Init</span></div>' +
            '</div>' +
            (variants.length > 1 ? '<div class="sp-pg-tabs">' + tabsHtml + '</div>' : '') +
            '<div class="sp-pg-main">' +
                '<div class="sp-pg-ecol">' +
                    '<div class="sp-pg-ehdr"><div class="sp-pg-etab">' + slug + '-' + escAttr(variantLabel(variants[0])) + '.py</div></div>' +
                    '<div class="sp-pg-cm-wrap"><div class="sp-pg-monaco" style="position:absolute;inset:0"></div></div>' +
                '</div>' +
                '<div class="sp-pg-divider"></div>' +
                '<div class="sp-pg-pcol">' +
                    '<div class="sp-pg-phdr">Live Preview</div>' +
                    '<iframe class="sp-pg-iframe" sandbox="allow-scripts"></iframe>' +
                    '<div class="sp-pg-loader"><div class="sp-pg-spinner"></div><div>Loading WASM</div></div>' +
                    '<div class="sp-pg-err"></div>' +
                '</div>' +
            '</div>';

        var contentRoot = document.querySelector('main') || document.querySelector('.content') || document.body;
        if (isFullPagePlayground()) {
            wrap.classList.add('sp-pg-flush');
            wrap.querySelector('.sp-pg-main').style.height = 'calc(100vh - 200px)';
            var h1 = contentRoot.querySelector('h1');
            if (h1 && h1.parentNode) h1.parentNode.insertBefore(wrap, h1.nextSibling);
            else contentRoot.insertBefore(wrap, contentRoot.firstChild);
        } else {
            contentRoot.appendChild(wrap);
        }

        fitWrapWidth(wrap);
        watchWrapWidth(wrap);

        state.iframe = wrap.querySelector('.sp-pg-iframe');
        state.statusDot = wrap.querySelector('.sp-pg-dot');
        state.statusText = wrap.querySelector('.sp-pg-text');
        state.loaderEl = wrap.querySelector('.sp-pg-loader');
        state.errEl = wrap.querySelector('.sp-pg-err');
        var divider = wrap.querySelector('.sp-pg-divider');
        var ecol = wrap.querySelector('.sp-pg-ecol');
        var monacoHost = wrap.querySelector('.sp-pg-monaco');
        var tabs = wrap.querySelectorAll('.sp-pg-tab');
        var fileTab = wrap.querySelector('.sp-pg-etab');
        wireTabs(tabs, variants, wrap, slug, fileTab);
        setStatus('loading', 'Loading editor');
        loadMonaco(function () {
            setStatus('loading', 'Loading WASM');
            state.editor = window.monaco.editor.create(monacoHost, {
                value: buildCode(variants[0]),
                language: 'python',
                theme: 'vs-dark',
                automaticLayout: true,
                fontFamily: '"Cascadia Code","JetBrains Mono","Fira Code","Consolas",monospace',
                fontSize: 13,
                lineHeight: 21,
                minimap: { enabled: false },
                scrollBeyondLastLine: false,
                renderLineHighlight: 'all',
                tabSize: 4,
                insertSpaces: true,
                wordWrap: 'on',
                bracketPairColorization: { enabled: true },
                guides: { indentation: true, bracketPairs: true },
                padding: { top: 10, bottom: 10 },
                smoothScrolling: true,
                cursorBlinking: 'smooth',
                fixedOverflowWidgets: true,
            });
            state.editor.onDidChangeModelContent(debouncedRun);
            attachResize(divider, ecol);
            ensureWasm(function () {
                var fresh = discoverVariants();
                if (fresh.length > variants.length) {
                    variants = fresh;
                    state.variants = fresh;
                    var tabsBar = wrap.querySelector('.sp-pg-tabs');
                    if (tabsBar) {
                        tabsBar.innerHTML = tabsHtmlFor(variants);
                        tabs = wrap.querySelectorAll('.sp-pg-tab');
                        wireTabs(tabs, variants, wrap, slug, fileTab);
                    }
                }
                setStatus('ready', 'Live · in-browser');
                runOnce(true);
                injectVariantCodeBlocks();
            });
        });
    }

    function escHtml(s) {
        return s.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;');
    }

    var PY_KW = /^(import|from|as|def|return|class|if|elif|else|for|while|in|not|and|or|is|None|True|False|lambda|with|try|except|finally|raise|pass|break|continue|yield|global|nonlocal|del|assert|print|self|super)$/;

    function highlightPython(src) {
        var out = '';
        var i = 0;
        var n = src.length;
        while (i < n) {
            var ch = src[i];
            if (ch === '#') {
                var e = src.indexOf('\n', i);
                if (e === -1) e = n;
                out += '<span style="color:#6a9955">' + escHtml(src.slice(i, e)) + '</span>';
                i = e;
            } else if (src.slice(i, i + 3) === '"""' || src.slice(i, i + 3) === "'''") {
                var q = src.slice(i, i + 3);
                var e = src.indexOf(q, i + 3);
                if (e === -1) e = n - 3; else e += 3;
                out += '<span style="color:#ce9178">' + escHtml(src.slice(i, e)) + '</span>';
                i = e;
            } else if (ch === '"' || ch === "'") {
                var q = ch;
                var j = i + 1;
                while (j < n && src[j] !== q && src[j] !== '\n') { if (src[j] === '\\') j++; j++; }
                if (j < n && src[j] === q) j++;
                out += '<span style="color:#ce9178">' + escHtml(src.slice(i, j)) + '</span>';
                i = j;
            } else if (ch >= '0' && ch <= '9') {
                var j = i;
                while (j < n && ((src[j] >= '0' && src[j] <= '9') || src[j] === '.' || src[j] === '_' || src[j] === 'x' || src[j] === 'X')) j++;
                out += '<span style="color:#b5cea8">' + escHtml(src.slice(i, j)) + '</span>';
                i = j;
            } else if ((ch >= 'a' && ch <= 'z') || (ch >= 'A' && ch <= 'Z') || ch === '_') {
                var j = i;
                while (j < n && ((src[j] >= 'a' && src[j] <= 'z') || (src[j] >= 'A' && src[j] <= 'Z') || (src[j] >= '0' && src[j] <= '9') || src[j] === '_')) j++;
                var word = src.slice(i, j);
                var after = (src.slice(j).match(/^[\t ]*\(/) !== null);
                if (PY_KW.test(word)) {
                    out += '<span style="color:#569cd6">' + escHtml(word) + '</span>';
                } else if (after) {
                    out += '<span style="color:#dcdcaa">' + escHtml(word) + '</span>';
                } else if (word[0] >= 'A' && word[0] <= 'Z') {
                    out += '<span style="color:#4ec9b0">' + escHtml(word) + '</span>';
                } else {
                    out += '<span style="color:#9cdcfe">' + escHtml(word) + '</span>';
                }
                i = j;
            } else if (ch === '=' || ch === '+' || ch === '-' || ch === '*' || ch === '/' || ch === '%' || ch === '<' || ch === '>' || ch === '!' || ch === '&' || ch === '|' || ch === '^' || ch === '~') {
                out += '<span style="color:#d4d4d4">' + escHtml(ch) + '</span>';
                i++;
            } else {
                out += escHtml(ch);
                i++;
            }
        }
        return out;
    }

    function injectVariantCodeBlocks() {
        var sp = window.SeraplotWASM;
        if (!sp || typeof sp.demo !== 'function') return;
        var slug = state.slug;
        if (!slug || slug === 'playground') return;
        var fn = state.baseFn || slug.replace(/-/g, '_');
        var codeStyle = [
            'background:var(--sp-code-bg)',
            'border:1px solid var(--sp-code-border)',
            'border-radius:8px',
            'padding:14px 16px',
            'margin:0 0 14px',
            'overflow-x:auto',
            'width:100%',
            'box-sizing:border-box',
            'font-family:"Cascadia Code","JetBrains Mono","Fira Code","Consolas",monospace',
            'font-size:12.5px',
            'line-height:1.6',
            'color:var(--sp-code-fg)',
            'white-space:pre',
            'position:relative',
        ].join(';');
        var labelStyle = 'font-size:11px;letter-spacing:.14em;font-weight:700;color:var(--sp-important);margin:20px 0 4px;text-transform:uppercase';
        var copyBtnStyle = [
            'position:absolute',
            'top:4px',
            'right:0',
            'background:var(--sp-surface)',
            'border:1px solid var(--sp-border)',
            'color:var(--sp-text-muted)',
            'border-radius:5px',
            'padding:3px 9px',
            'font-size:11px',
            'cursor:pointer',
            'font-family:inherit',
            'transition:all .15s',
        ].join(';');
        var variants = document.querySelectorAll('.sp-variant[id]');
        for (var i = 0; i < variants.length; i++) {
            var el = variants[i];
            if (el.querySelector('.sp-demo-code-wrap')) continue;
            var id = el.id;
            var parts = id.split('-');
            var v = el.getAttribute('data-variant') || parts[parts.length - 1];
            v = normalizeVariantName(v);
            if (!v || v === 'default') v = 'basic';
            var code;
            try {
                code = sp.demo(JSON.stringify({ family: fn, variant: v }));
            } catch (e) {}
            if (!code || !code.trim()) {
                code = buildCode(v);
            }
            if (!code || !code.trim()) continue;
            var wrap = document.createElement('div');
            wrap.className = 'sp-demo-code-wrap';
            var lbl = document.createElement('div');
            lbl.style.cssText = labelStyle;
            lbl.textContent = 'Code';
            var pre = document.createElement('pre');
            pre.style.cssText = codeStyle;
            var codeEl = document.createElement('code');
            codeEl.innerHTML = highlightPython(code);
            var copyBtn = document.createElement('button');
            copyBtn.style.cssText = copyBtnStyle;
            copyBtn.textContent = 'Copy';
            copyBtn.addEventListener('click', (function (txt, btn) {
                return function () {
                    navigator.clipboard && navigator.clipboard.writeText(txt).then(function () {
                        btn.textContent = 'Copied!';
                        setTimeout(function () { btn.textContent = 'Copy'; }, 1500);
                    });
                };
            })(code, copyBtn));
            pre.appendChild(codeEl);
            pre.appendChild(copyBtn);
            wrap.appendChild(lbl);
            wrap.appendChild(pre);
            var preview = el.querySelector('.sp-preview-label');
            if (preview) {
                el.insertBefore(wrap, preview);
            } else {
                var meta = el.querySelector('.sp-vmeta');
                if (meta && meta.nextSibling) { el.insertBefore(wrap, meta.nextSibling); }
                else { el.appendChild(wrap); }
            }
        }
    }

    function init() {
        if (!isChartPage()) return;
        state.slug = pageSlug();
        buildUI();
    }

    if (document.readyState === 'loading') document.addEventListener('DOMContentLoaded', init);
    else init();
})();
