(function () {
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
        parents: '[null,"Root","Root","A","A","B"]',
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
        var els = document.querySelectorAll('link[href*="/theme/"],script[src*="/theme/"]');
        for (var i = 0; i < els.length; i++) {
            var u = els[i].href || els[i].src || '';
            var m = u.match(/(.*\/theme\/)/);
            if (m) return m[1];
        }
        return 'theme/';
    }

    function loadMonaco(cb) {
        if (window.monaco) { cb(); return; }
        if (window.__spMonacoLoading) { window.__spMonacoLoading.then(cb); return; }
        window.__spMonacoLoading = new Promise(function (resolve) {
            var loader = document.createElement('script');
            loader.src = MONACO_BASE + '/loader.js';
            loader.onload = function () {
                window.require.config({ paths: { vs: MONACO_BASE } });
                window.MonacoEnvironment = {
                    getWorkerUrl: function () {
                        return 'data:text/javascript;charset=utf-8,' + encodeURIComponent(
                            'self.MonacoEnvironment={baseUrl:"' + MONACO_BASE + '/"};' +
                            'importScripts("' + MONACO_BASE + '/base/worker/workerMain.js");'
                        );
                    }
                };
                window.require(['vs/editor/editor.main'], function () { resolve(); cb(); });
            };
            document.head.appendChild(loader);
        });
    }

    function ensureWasm(cb) {
        var sp = window.SeraplotWASM;
        if (!sp) { setTimeout(function () { ensureWasm(cb); }, 80); return; }
        if (sp.__ready) { cb(); return; }
        if (sp.__loading) { sp.__loading.then(cb); return; }
        sp.__loading = sp.__init(getThemeBase() + 'seraplot_bg.wasm').then(cb).catch(function (e) {
            setStatus('err', 'WASM load failed');
            showLoader('Failed to load WASM module.<br>' + (e && e.message ? e.message : ''));
        });
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
        var cands = fnCandidates(fn);
        for (var i = 0; i < cands.length; i++) {
            if (typeof sp[cands[i]] === 'function') return { name: cands[i], fn: sp[cands[i]] };
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
        var lang = detectLang();
        var nodes = document.querySelectorAll('.sp-variant[id^="' + slug + '-' + lang + '-"]');
        if (nodes.length === 0) {
            nodes = document.querySelectorAll('.sp-variant[id^="' + slug + '-"]');
        }
        var seen = {}, out = [];
        for (var i = 0; i < nodes.length; i++) {
            var id = nodes[i].id;
            var parts = id.split('-');
            var name = parts[parts.length - 1];
            if (!name || seen[name]) continue;
            seen[name] = true;
            out.push(name);
        }
        return out;
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
            '.sp-pg-wrap{position:relative;display:flex;flex-direction:column;border-radius:14px;overflow:hidden;margin:0 0 2.6rem;font-family:"Inter","Segoe UI",sans-serif;background:#1e1e1e;box-shadow:0 24px 70px -20px rgba(0,0,0,.7),0 0 0 1px #2d2d30,inset 0 1px 0 rgba(255,255,255,.04)}',
            '.sp-pg-wrap::before{content:"";position:absolute;inset:-1px;border-radius:15px;padding:1px;background:linear-gradient(120deg,rgba(99,102,241,.4),rgba(168,85,247,.3),rgba(34,211,238,.35),rgba(99,102,241,.4));background-size:200% 200%;animation:sp-rotate-bg 14s linear infinite;-webkit-mask:linear-gradient(#000 0 0) content-box,linear-gradient(#000 0 0);-webkit-mask-composite:xor;mask-composite:exclude;pointer-events:none;opacity:.5;z-index:0}',
            '.sp-pg-hdr,.sp-pg-tabs,.sp-pg-main{position:relative;z-index:1}',
            '.sp-pg-hdr{display:flex;align-items:center;gap:14px;padding:9px 14px;background:#252526;border-bottom:1px solid #2d2d30;flex-wrap:wrap;row-gap:8px}',
            '.sp-pg-title{font-size:10.5px;font-weight:700;letter-spacing:.22em;color:#cccccc;text-transform:uppercase;flex-shrink:0}',
            '.sp-pg-title b{background:linear-gradient(135deg,#a5b4fc 0%,#67e8f9 100%);-webkit-background-clip:text;background-clip:text;color:transparent;font-weight:800}',
            '.sp-pg-spacer{flex:1}',
            '.sp-pg-btn{background:#0e639c;color:#fff;border:none;border-radius:4px;padding:5px 14px;font-size:12px;font-weight:600;cursor:pointer;transition:background .15s;font-family:inherit}',
            '.sp-pg-btn:hover{background:#1177bb}',
            '.sp-pg-btn:active{background:#0a4d7a}',
            '.sp-pg-btn[disabled]{opacity:.45;cursor:not-allowed}',
            '.sp-pg-conn{display:flex;align-items:center;gap:8px;font-size:11px;font-weight:600;color:#969696;letter-spacing:.05em;text-transform:uppercase;flex-shrink:0;padding:4px 10px;background:#1e1e1e;border:1px solid #2d2d30;border-radius:14px}',
            '.sp-pg-dot{width:8px;height:8px;border-radius:50%;background:#444;flex-shrink:0;transition:background .3s}',
            '.sp-d-loading{background:#dcdcaa!important;animation:sp-pulse 1s infinite}',
            '.sp-d-ready{background:#4ec9b0!important;animation:sp-ripple 2.4s infinite}',
            '.sp-d-err{background:#f48771!important;box-shadow:0 0 8px #f4877188}',
            '.sp-pg-tabs{display:flex;gap:0;padding:0;background:#252526;border-bottom:1px solid #2d2d30;overflow-x:auto;scrollbar-width:none}',
            '.sp-pg-tabs::-webkit-scrollbar{display:none}',
            '.sp-pg-tab{font:600 11.5px/1 "Inter",sans-serif;letter-spacing:.04em;padding:9px 16px;color:#969696;background:#2d2d30;border:none;border-right:1px solid #252526;border-bottom:1px solid #2d2d30;cursor:pointer;transition:all .15s;white-space:nowrap;font-family:inherit;text-transform:capitalize}',
            '.sp-pg-tab:hover{color:#fff;background:#37373d}',
            '.sp-pg-tab.sp-active{color:#fff;background:#1e1e1e;border-bottom-color:#1e1e1e}',
            '.sp-pg-main{display:flex;height:420px;background:#1e1e1e}',
            '.sp-pg-ecol{width:50%;min-width:180px;max-width:80%;display:flex;flex-direction:column;flex-shrink:0;overflow:hidden;border-right:1px solid #2d2d30;background:#1e1e1e}',
            '.sp-pg-ehdr{display:flex;align-items:center;padding:0;background:#252526;border-bottom:1px solid #2d2d30;font:11px/1 "Segoe UI",sans-serif;color:#969696}',
            '.sp-pg-etab{padding:7px 14px;background:#1e1e1e;border:none;border-right:1px solid #2d2d30;color:#fff;font-size:12px;font-weight:500;letter-spacing:.02em;display:flex;align-items:center;gap:8px;font-family:inherit}',
            '.sp-pg-etab::before{content:"🐍";font-size:11px}',
            '.sp-pg-cm-wrap{flex:1;background:#1e1e1e;overflow:hidden;position:relative}',
            '.sp-pg-cm-wrap .monaco-editor{background:#1e1e1e!important}',
            '.sp-pg-cm-wrap .monaco-editor .margin{background:#1e1e1e!important}',
            '.sp-pg-cm-wrap .monaco-editor .monaco-editor-background{background:#1e1e1e!important}',
            '.sp-pg-divider{width:5px;background:#2d2d30;cursor:col-resize;flex-shrink:0;position:relative;z-index:3;transition:background .15s}',
            '.sp-pg-divider:hover,.sp-pg-divider.sp-dragging{background:#0e639c}',
            '.sp-pg-pcol{flex:1;min-width:160px;display:flex;flex-direction:column;position:relative;background:#1e1e1e}',
            '.sp-pg-phdr{display:flex;align-items:center;padding:7px 14px;background:#252526;border-bottom:1px solid #2d2d30;font:11px/1 "Segoe UI",sans-serif;color:#969696;letter-spacing:.04em;text-transform:uppercase;font-weight:700;gap:8px}',
            '.sp-pg-phdr::before{content:"";display:inline-block;width:8px;height:8px;border-radius:50%;background:#4ec9b0;box-shadow:0 0 8px rgba(78,201,176,.6)}',
            '.sp-pg-iframe{flex:1;border:none;background:#fff;width:100%;display:block;animation:sp-fadein .28s cubic-bezier(.2,.8,.2,1)}',
            '.sp-pg-loader{position:absolute;left:0;right:0;top:33px;bottom:0;display:flex;align-items:center;justify-content:center;background:#1e1e1e;flex-direction:column;gap:18px;color:#969696;font-size:12px;font-weight:600;letter-spacing:.1em;text-transform:uppercase;transition:opacity .25s;z-index:1;text-align:center;padding:0 24px}',
            '.sp-pg-loader.sp-hide{opacity:0;pointer-events:none}',
            '.sp-pg-loader b{color:#c4b5fd;font-weight:700}',
            '.sp-pg-spinner{width:38px;height:38px;border:3px solid rgba(124,58,237,.15);border-top-color:#a78bfa;border-right-color:#67e8f9;border-radius:50%;animation:sp-spin .9s cubic-bezier(.5,.1,.5,.9) infinite;filter:drop-shadow(0 0 10px rgba(167,139,250,.5))}',
            '.sp-pg-err{position:absolute;left:14px;right:14px;bottom:14px;background:rgba(40,10,20,.85);border:1px solid #f48771;color:#f48771;font:11px/1.5 "Cascadia Code","Consolas",monospace;padding:9px 12px;border-radius:6px;backdrop-filter:blur(10px);max-height:160px;overflow:auto;white-space:pre-wrap;z-index:5;display:none}',
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
        var fitStyle = '<style>html,body{margin:0;padding:0;width:100%;height:100%;overflow:auto;background:#fff}body>*{max-width:100%!important}svg,canvas{max-width:100%!important;height:auto!important;display:block;margin:0 auto}.chart-container,.plot-container,#chart,#plot{max-width:100%!important;width:100%!important;height:auto!important;box-sizing:border-box}</style>';
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

    function extractAllCalls(code) {
        var calls = [], i = 0;
        while (i < code.length) {
            var idx = code.indexOf('sp.', i);
            if (idx === -1) break;
            var rest = code.slice(idx);
            var m = rest.match(/^sp\.([a-zA-Z_][a-zA-Z0-9_]*)\s*\(/);
            if (!m) { i = idx + 3; continue; }
            var bodyStart = idx + m[0].length, depth = 1, j = bodyStart, inStr = false, q = '';
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
            calls.push({ fn: m[1], body: code.slice(bodyStart, j - 1) });
            i = j;
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

    function selectVariant(idx) {
        var name = state.variants[idx];
        if (!name) return;
        state.currentVariant = idx;
        var tabs = document.querySelectorAll('.sp-pg-wrap .sp-pg-tab');
        for (var i = 0; i < tabs.length; i++) tabs[i].classList.toggle('sp-active', i === idx);
        if (state.editor) {
            state.editor.setValue(buildCode(name));
            runOnce(true);
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

        var tabsHtml = '';
        for (var i = 0; i < variants.length; i++) {
            tabsHtml += '<button class="sp-pg-tab' + (i === 0 ? ' sp-active' : '') + '" type="button">' + variants[i] + '</button>';
        }

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
                    '<div class="sp-pg-ehdr"><div class="sp-pg-etab">' + slug + '.py</div></div>' +
                    '<div class="sp-pg-cm-wrap"><div class="sp-pg-monaco" style="position:absolute;inset:0"></div></div>' +
                '</div>' +
                '<div class="sp-pg-divider"></div>' +
                '<div class="sp-pg-pcol">' +
                    '<div class="sp-pg-phdr">Live Preview</div>' +
                    '<iframe class="sp-pg-iframe" sandbox="allow-scripts allow-same-origin"></iframe>' +
                    '<div class="sp-pg-loader"><div class="sp-pg-spinner"></div><div>Loading WASM</div></div>' +
                    '<div class="sp-pg-err"></div>' +
                '</div>' +
            '</div>';

        var contentRoot = document.querySelector('main') || document.querySelector('.content') || document.body;
        if (isFullPagePlayground()) {
            wrap.style.margin = '0';
            wrap.querySelector('.sp-pg-main').style.height = 'calc(100vh - 200px)';
            var h1 = contentRoot.querySelector('h1');
            if (h1 && h1.parentNode) h1.parentNode.insertBefore(wrap, h1.nextSibling);
            else contentRoot.insertBefore(wrap, contentRoot.firstChild);
        } else {
            contentRoot.appendChild(wrap);
        }

        state.iframe = wrap.querySelector('.sp-pg-iframe');
        state.statusDot = wrap.querySelector('.sp-pg-dot');
        state.statusText = wrap.querySelector('.sp-pg-text');
        state.loaderEl = wrap.querySelector('.sp-pg-loader');
        state.errEl = wrap.querySelector('.sp-pg-err');
        var divider = wrap.querySelector('.sp-pg-divider');
        var ecol = wrap.querySelector('.sp-pg-ecol');
        var monacoHost = wrap.querySelector('.sp-pg-monaco');
        var tabs = wrap.querySelectorAll('.sp-pg-tab');
        for (var t = 0; t < tabs.length; t++) {
            (function (idx) {
                tabs[idx].addEventListener('click', function () {
                    if (isFullPagePlayground()) {
                        state.baseFn = variants[idx];
                        state.currentVariant = 0;
                        for (var ti = 0; ti < tabs.length; ti++) tabs[ti].classList.toggle('sp-active', ti === idx);
                        if (state.editor) { state.editor.setValue(buildCode('basic')); runOnce(true); }
                    } else {
                        selectVariant(idx);
                    }
                });
            })(t);
        }
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

    var VARIANT_ALIASES = {
        'gstack': 'grouped_stacked',
        'h': 'horizontal',
        'grp': 'grouped',
        'stk': 'stacked',
        'rel': 'relative',
        'mek': 'marimekko',
        'pict': 'pictogram',
        'multi': 'multicategory',
    };

    function injectVariantCodeBlocks() {
        var sp = window.SeraplotWASM;
        if (!sp || typeof sp.demo !== 'function') return;
        var slug = state.slug;
        if (!slug || slug === 'playground') return;
        var fn = slug.replace(/-/g, '_');
        var codeStyle = [
            'background:transparent',
            'border-top:1px solid rgba(99,102,241,.18)',
            'border-bottom:none',
            'border-left:none',
            'border-right:none',
            'border-radius:0',
            'padding:12px 2px 4px',
            'margin:0 0 14px',
            'overflow-x:auto',
            'font-family:"Cascadia Code","JetBrains Mono","Fira Code","Consolas",monospace',
            'font-size:12.5px',
            'line-height:1.6',
            'color:#d4d4d4',
            'white-space:pre',
            'position:relative',
        ].join(';');
        var labelStyle = 'font-size:11px;letter-spacing:.14em;font-weight:700;color:#818cf8;margin:20px 0 4px;text-transform:uppercase';
        var copyBtnStyle = [
            'position:absolute',
            'top:4px',
            'right:0',
            'background:#1e293b',
            'border:1px solid #334155',
            'color:#94a3b8',
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
            var variantName = parts[parts.length - 1];
            if (!variantName) continue;
            var v = (variantName === 'default') ? 'basic' : variantName;
            var wasmV = VARIANT_ALIASES[v] || v;
            var code;
            try {
                code = sp.demo(JSON.stringify({ family: fn, variant: wasmV }));
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
