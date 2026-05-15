(function () {
    var DEBOUNCE_MS = 550;

    var SAMPLES = {
        bar: [
            { name: 'Basic', code: 'sp.bar(\n    "Quarterly Sales",\n    labels=["Q1","Q2","Q3","Q4"],\n    values=[120,190,150,220],\n)' },
            { name: 'Horizontal', code: 'sp.bar(\n    "Top Categories",\n    labels=["Books","Music","Games","Tech","Art"],\n    values=[42,36,55,78,29],\n    variant="horizontal",\n)' },
            { name: 'Grouped', code: 'sp.bar(\n    "Revenue by Region",\n    labels=["Q1","Q2","Q3","Q4"],\n    series=[[40,55,38,62],[30,42,48,55],[22,28,35,40]],\n    series_names=["EU","US","ASIA"],\n    variant="grouped",\n    theme="aurora",\n)' },
            { name: 'Stacked', code: 'sp.bar(\n    "Sales Stack",\n    labels=["Mon","Tue","Wed","Thu","Fri"],\n    series=[[10,20,15,25,18],[8,14,12,20,15],[5,9,11,14,12]],\n    series_names=["A","B","C"],\n    variant="stacked",\n    theme="aurora",\n)' },
            { name: 'Relative', code: 'sp.bar(\n    "Market Share",\n    labels=["Jan","Feb","Mar","Apr"],\n    series=[[30,40,35,50],[20,30,25,35],[15,20,18,25]],\n    series_names=["X","Y","Z"],\n    variant="relative",\n    theme="aurora",\n)' },
            { name: 'Marimekko', code: 'sp.bar(\n    "Marimekko",\n    labels=["A","B","C","D"],\n    series=[[10,20,15,25],[8,14,12,20],[6,10,9,14]],\n    series_names=["L1","L2","L3"],\n    widths=[1,2,1.5,1],\n    variant="marimekko",\n    theme="aurora",\n)' },
            { name: 'Pictogram', code: 'sp.bar(\n    "Users",\n    labels=["A","B","C","D"],\n    values=[40,75,55,90],\n    units_per_icon=10,\n    variant="pictogram",\n)' },
            { name: 'Deluxe', code: 'sp.bar(\n    "Deluxe Style",\n    labels=["Alpha","Beta","Gamma","Delta","Epsilon"],\n    values=[28,52,41,67,35],\n    theme="deluxe",\n)' },
        ],
        line: [
            { name: 'Basic', code: 'sp.line(\n    "Trend",\n    x=[1,2,3,4,5,6,7,8],\n    y=[10,15,12,22,28,25,35,42],\n)' },
            { name: 'Multi-series', code: 'sp.line(\n    "Multi Trend",\n    x=[1,2,3,4,5,6,7,8],\n    series=[[10,15,12,22,28,25,35,42],[8,12,18,24,22,30,28,38],[5,9,14,18,25,28,32,40]],\n    series_names=["Alpha","Beta","Gamma"],\n)' },
            { name: 'Smooth', code: 'sp.line(\n    "Smoothed",\n    x=[0,1,2,3,4,5,6,7,8,9],\n    y=[5,12,8,18,22,15,25,30,28,35],\n    variant="smooth",\n)' },
            { name: 'Area', code: 'sp.line(\n    "Area Fill",\n    x=[1,2,3,4,5,6,7,8],\n    y=[12,18,15,25,30,28,38,45],\n    variant="area",\n    theme="aurora",\n)' },
            { name: 'Stepped', code: 'sp.line(\n    "Stepped",\n    x=[1,2,3,4,5,6,7,8],\n    y=[10,10,18,18,25,25,32,40],\n    variant="step",\n)' },
        ],
        pie: [
            { name: 'Basic', code: 'sp.pie(\n    "Market Share",\n    labels=["Apple","Google","Microsoft","Amazon","Meta"],\n    values=[28,24,22,16,10],\n)' },
            { name: 'Donut', code: 'sp.pie(\n    "Donut",\n    labels=["A","B","C","D"],\n    values=[35,25,22,18],\n    variant="donut",\n)' },
            { name: 'Exploded', code: 'sp.pie(\n    "Exploded",\n    labels=["Core","Plus","Pro","Max","Ultra"],\n    values=[40,25,18,12,5],\n    variant="exploded",\n)' },
        ],
        violin: [
            { name: 'Basic', code: 'sp.violin(\n    "Distribution",\n    labels=["Group A","Group B","Group C"],\n    series=[\n        [12,14,15,16,17,18,19,20,21,22,23,24,25,26,27],\n        [10,12,13,14,15,16,17,18,19,20,21,22,23,24,28],\n        [15,16,17,18,19,20,21,22,23,24,25,26,27,28,30],\n    ],\n)' },
            { name: 'Split', code: 'sp.violin(\n    "Split",\n    labels=["A","B","C","D"],\n    series=[\n        [10,12,13,15,16,18,19,20,22,24],\n        [8,11,13,14,17,18,20,22,23,25],\n        [12,13,15,17,18,20,21,22,24,26],\n        [9,11,12,14,16,18,19,21,23,24],\n    ],\n    variant="split",\n)' },
        ],
        scatter: [
            { name: 'Basic', code: 'sp.scatter(\n    "Cluster",\n    x=[1,2,3,4,5,6,7,8,9,10,11,12],\n    y=[2,3,5,4,7,6,9,8,11,10,13,12],\n)' },
        ],
        histogram: [
            { name: 'Basic', code: 'sp.histogram(\n    "Distribution",\n    values=[1,2,2,3,3,3,4,4,4,4,5,5,5,5,5,6,6,6,6,7,7,7,8,8,9],\n    bins=10,\n)' },
        ],
        heatmap: [
            { name: 'Basic', code: 'sp.heatmap(\n    "Correlation",\n    matrix=[[1.0,0.8,0.3],[0.8,1.0,0.5],[0.3,0.5,1.0]],\n    labels=["A","B","C"],\n)' },
        ],
        boxplot: [
            { name: 'Basic', code: 'sp.boxplot(\n    "Distribution",\n    labels=["A","B","C"],\n    series=[[10,12,15,18,22,25,30],[8,11,14,17,21,28,35],[12,15,18,22,26,30,38]],\n)' },
        ],
        radar: [
            { name: 'Basic', code: 'sp.radar(\n    "Profile",\n    labels=["Speed","Power","Endurance","Skill","Range"],\n    values=[80,65,90,75,55],\n)' },
        ],
    };

    var state = {
        ready: false,
        editor: null,
        iframe: null,
        statusDot: null,
        statusText: null,
        loaderEl: null,
        errEl: null,
        slug: null,
        unifiedFn: null,
        debTimer: null,
        lastSent: '',
        currentVariant: 0,
    };

    function pageSlug() {
        var m = window.location.pathname.match(/\/([^\/]+?)(?:\.html?)?$/);
        return m ? m[1] : null;
    }

    function isChartPage() {
        var slug = pageSlug();
        return /\/charts\//.test(window.location.pathname) && slug && slug !== 'index';
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

    function loadCM(cb) {
        if (window.CodeMirror) { cb(); return; }
        ['https://cdnjs.cloudflare.com/ajax/libs/codemirror/5.65.17/codemirror.min.css',
         'https://cdnjs.cloudflare.com/ajax/libs/codemirror/5.65.17/theme/dracula.min.css'].forEach(function (href) {
            var l = document.createElement('link'); l.rel = 'stylesheet'; l.href = href; document.head.appendChild(l);
        });
        var s1 = document.createElement('script');
        s1.src = 'https://cdnjs.cloudflare.com/ajax/libs/codemirror/5.65.17/codemirror.min.js';
        s1.onload = function () {
            var s2 = document.createElement('script');
            s2.src = 'https://cdnjs.cloudflare.com/ajax/libs/codemirror/5.65.17/mode/python/python.min.js';
            s2.onload = cb;
            document.head.appendChild(s2);
        };
        document.head.appendChild(s1);
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

    function unifiedName(slug) {
        return 'build' + slug.charAt(0).toUpperCase() + slug.slice(1);
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
            '.sp-pg-wrap{position:relative;display:flex;flex-direction:column;border-radius:18px;overflow:hidden;margin:0 0 2.6rem;font-family:"Inter","Segoe UI",sans-serif;background:linear-gradient(135deg,#05060f 0%,#0a0d24 50%,#05060f 100%);box-shadow:0 24px 70px -20px rgba(0,0,0,.7),0 0 0 1px rgba(120,130,200,.08),inset 0 1px 0 rgba(255,255,255,.04)}',
            '.sp-pg-wrap::before{content:"";position:absolute;inset:-1px;border-radius:19px;padding:1px;background:linear-gradient(120deg,rgba(99,102,241,.45),rgba(168,85,247,.35),rgba(34,211,238,.4),rgba(99,102,241,.45));background-size:200% 200%;animation:sp-rotate-bg 14s linear infinite;-webkit-mask:linear-gradient(#000 0 0) content-box,linear-gradient(#000 0 0);-webkit-mask-composite:xor;mask-composite:exclude;pointer-events:none;opacity:.55;z-index:0}',
            '.sp-pg-wrap::after{content:"";position:absolute;width:380px;height:380px;border-radius:50%;background:radial-gradient(circle,rgba(99,102,241,.18),transparent 60%);top:-160px;right:-100px;filter:blur(40px);pointer-events:none;animation:sp-orb 18s ease-in-out infinite;z-index:0}',
            '.sp-pg-hdr,.sp-pg-tabs,.sp-pg-main,.sp-pg-foot{position:relative;z-index:1}',
            '.sp-pg-hdr{display:flex;align-items:center;gap:14px;padding:13px 18px;background:linear-gradient(90deg,rgba(12,18,40,.7) 0%,rgba(14,21,48,.5) 100%);backdrop-filter:blur(14px);-webkit-backdrop-filter:blur(14px);border-bottom:1px solid rgba(120,130,200,.12);flex-wrap:wrap;row-gap:10px}',
            '.sp-pg-title{font-size:10.5px;font-weight:800;letter-spacing:.24em;background:linear-gradient(135deg,#a5b4fc 0%,#67e8f9 100%);-webkit-background-clip:text;background-clip:text;color:transparent;text-transform:uppercase;flex-shrink:0;text-shadow:0 0 18px rgba(99,102,241,.3)}',
            '.sp-pg-spacer{flex:1}',
            '.sp-pg-btn{background:linear-gradient(135deg,#4f46e5,#7c3aed);color:#fff;border:none;border-radius:8px;padding:6px 16px;font-size:12px;font-weight:700;letter-spacing:.06em;text-transform:uppercase;cursor:pointer;transition:transform .15s,box-shadow .15s;font-family:inherit;box-shadow:0 6px 22px -6px rgba(124,58,237,.65),inset 0 1px 0 rgba(255,255,255,.18)}',
            '.sp-pg-btn:hover{transform:translateY(-1px);box-shadow:0 9px 26px -6px rgba(124,58,237,.85),inset 0 1px 0 rgba(255,255,255,.25)}',
            '.sp-pg-btn:active{transform:translateY(0)}',
            '.sp-pg-btn[disabled]{opacity:.45;cursor:not-allowed;transform:none;box-shadow:none}',
            '.sp-pg-conn{display:flex;align-items:center;gap:9px;font-size:11px;font-weight:600;color:#7d8bb8;letter-spacing:.06em;text-transform:uppercase;flex-shrink:0;padding:5px 12px;background:rgba(20,25,55,.5);border:1px solid rgba(120,130,200,.12);border-radius:20px}',
            '.sp-pg-dot{width:8px;height:8px;border-radius:50%;background:#444;flex-shrink:0;transition:background .3s}',
            '.sp-d-loading{background:#f59e0b!important;animation:sp-pulse 1s infinite}',
            '.sp-d-ready{background:#22c55e!important;animation:sp-ripple 2.4s infinite}',
            '.sp-d-err{background:#ef4444!important;box-shadow:0 0 8px #ef444488}',
            '.sp-pg-tabs{display:flex;gap:4px;padding:8px 14px;background:rgba(8,10,28,.5);border-bottom:1px solid rgba(120,130,200,.08);overflow-x:auto;scrollbar-width:none}',
            '.sp-pg-tabs::-webkit-scrollbar{display:none}',
            '.sp-pg-tab{font:600 11px/1 "Inter",sans-serif;letter-spacing:.08em;text-transform:uppercase;padding:8px 14px;border-radius:8px;color:#7d8bb8;background:transparent;border:1px solid transparent;cursor:pointer;transition:all .18s;white-space:nowrap;font-family:inherit}',
            '.sp-pg-tab:hover{color:#c4b5fd;background:rgba(124,58,237,.08)}',
            '.sp-pg-tab.sp-active{color:#fff;background:linear-gradient(135deg,rgba(99,102,241,.35),rgba(124,58,237,.3));border-color:rgba(167,139,250,.45);box-shadow:0 4px 14px -4px rgba(124,58,237,.55),inset 0 1px 0 rgba(255,255,255,.12)}',
            '.sp-pg-main{display:flex;height:480px;background:rgba(5,6,15,.6)}',
            '.sp-pg-ecol{width:46%;min-width:160px;max-width:78%;display:flex;flex-direction:column;flex-shrink:0;overflow:hidden;border-right:1px solid rgba(120,130,200,.08);background:linear-gradient(180deg,rgba(8,10,28,.85) 0%,rgba(5,6,15,.95) 100%)}',
            '.sp-pg-divider{width:6px;background:transparent;cursor:col-resize;flex-shrink:0;position:relative;z-index:3;transition:background .2s}',
            '.sp-pg-divider::before{content:"";position:absolute;left:50%;top:50%;width:2px;height:42px;background:linear-gradient(180deg,transparent,rgba(165,180,252,.45),transparent);transform:translate(-50%,-50%);border-radius:2px;transition:opacity .2s,height .2s;opacity:.5}',
            '.sp-pg-divider:hover::before,.sp-pg-divider.sp-dragging::before{opacity:1;height:90px}',
            '.sp-pg-divider:hover,.sp-pg-divider.sp-dragging{background:rgba(99,102,241,.08)}',
            '.sp-pg-cm-wrap{flex:1;background:transparent;overflow:hidden;position:relative}',
            '.sp-pg-cm-wrap .CodeMirror{background:transparent;font:13px/1.7 "JetBrains Mono","Fira Code","Consolas",monospace;height:100%;color:#d6deff}',
            '.sp-pg-cm-wrap .CodeMirror-scroll{padding:14px 18px;box-sizing:border-box}',
            '.sp-pg-cm-wrap .CodeMirror-gutters{display:none!important;background:transparent}',
            '.sp-pg-cm-wrap .CodeMirror-cursor{border-left:2px solid #c4b5fd;box-shadow:0 0 8px rgba(196,181,253,.6)}',
            '.sp-pg-cm-wrap .CodeMirror-selected{background:rgba(124,58,237,.22)!important}',
            '.sp-pg-cm-wrap .CodeMirror-focused .CodeMirror-selected{background:rgba(124,58,237,.32)!important}',
            '.sp-pg-cm-wrap .cm-s-dracula .cm-keyword{color:#ff79c6;font-weight:600}',
            '.sp-pg-cm-wrap .cm-s-dracula .cm-string{color:#f1fa8c}',
            '.sp-pg-cm-wrap .cm-s-dracula .cm-number{color:#bd93f9}',
            '.sp-pg-cm-wrap .cm-s-dracula .cm-def{color:#50fa7b}',
            '.sp-pg-cm-wrap .cm-s-dracula .cm-comment{color:#3a4d6a;font-style:italic}',
            '.sp-pg-cm-wrap .cm-s-dracula .cm-operator{color:#ff79c6}',
            '.sp-pg-cm-wrap .cm-s-dracula .cm-variable{color:#e6e7ff}',
            '.sp-pg-cm-wrap .cm-s-dracula .cm-property{color:#8be9fd}',
            '.sp-pg-pcol{flex:1;min-width:140px;display:flex;flex-direction:column;position:relative;background:radial-gradient(ellipse at 70% 30%,rgba(99,102,241,.06),transparent 60%),#05060f}',
            '.sp-pg-iframe{flex:1;border:none;background:transparent;width:100%;display:block;animation:sp-fadein .28s cubic-bezier(.2,.8,.2,1)}',
            '.sp-pg-loader{position:absolute;inset:0;display:flex;align-items:center;justify-content:center;background:radial-gradient(ellipse at center,rgba(20,25,55,.4),#05060f);flex-direction:column;gap:18px;color:#7d8bb8;font-size:12.5px;font-weight:600;letter-spacing:.12em;text-transform:uppercase;transition:opacity .25s;z-index:1;text-align:center;padding:0 24px}',
            '.sp-pg-loader.sp-hide{opacity:0;pointer-events:none}',
            '.sp-pg-loader b{color:#c4b5fd;font-weight:700;letter-spacing:.05em}',
            '.sp-pg-spinner{width:38px;height:38px;border:3px solid rgba(124,58,237,.15);border-top-color:#a78bfa;border-right-color:#67e8f9;border-radius:50%;animation:sp-spin .9s cubic-bezier(.5,.1,.5,.9) infinite;filter:drop-shadow(0 0 10px rgba(167,139,250,.5))}',
            '.sp-pg-err{position:absolute;left:18px;right:18px;bottom:18px;background:rgba(40,10,20,.75);border:1px solid rgba(239,68,68,.45);color:#fda4af;font:11.5px/1.55 "JetBrains Mono","Consolas",monospace;padding:10px 14px;border-radius:10px;backdrop-filter:blur(10px);max-height:160px;overflow:auto;white-space:pre-wrap;z-index:5;display:none}',
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
    function hideLoader() {
        if (state.loaderEl) state.loaderEl.classList.add('sp-hide');
    }

    function showErr(msg) {
        if (!state.errEl) return;
        state.errEl.textContent = msg;
        state.errEl.classList.add('sp-show');
    }
    function hideErr() {
        if (state.errEl) state.errEl.classList.remove('sp-show');
    }

    function renderHtmlInIframe(html) {
        if (!state.iframe) return;
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
        if (s === 'None') return null;
        var rangeM = s.match(/^list\s*\(\s*range\s*\(\s*(\d+)\s*(?:,\s*(\d+)\s*)?\)\s*\)$/);
        if (rangeM) {
            var a = +rangeM[1], b = rangeM[2] !== undefined ? +rangeM[2] : null;
            var arr = [], lo = b === null ? 0 : a, hi = b === null ? a : b;
            for (var k = lo; k < hi; k++) arr.push(k);
            return arr;
        }
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
        var args = [], kwargs = {};
        var i = 0, depth = 0, buf = '', inStr = false, q = '';
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
        var title = '';
        if (args.length > 0 && typeof args[0] === 'string') title = args[0];
        return { title: title, kwargs: kwargs };
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
        else {
            var k = s.slice(0, eq).trim();
            var v = parsePyVal(s.slice(eq + 1));
            kwargs[k] = v;
        }
    }

    function extractCall(code) {
        var m = code.match(/sp\.([a-zA-Z_][a-zA-Z0-9_]*)\s*\(/);
        if (!m) return null;
        var fn = m[1], start = m.index + m[0].length, depth = 1, i = start, inStr = false, q = '';
        while (i < code.length && depth > 0) {
            var c = code[i];
            if (inStr) { if (c === '\\') { i += 2; continue; } if (c === q) inStr = false; }
            else {
                if (c === '"' || c === "'") { inStr = true; q = c; }
                else if (c === '(') depth++;
                else if (c === ')') depth--;
            }
            i++;
        }
        return { fn: fn, body: code.slice(start, i - 1) };
    }

    function runOnce(force) {
        if (!state.editor) return;
        var code = state.editor.getValue();
        if (!force && code === state.lastSent) return;
        state.lastSent = code;
        var call = extractCall(code);
        if (!call) { showErr('No sp.<chart>(...) call detected'); return; }
        var sp = window.SeraplotWASM;
        if (!sp || !sp.__ready) { showErr('WASM not ready'); return; }
        var fnName = unifiedName(call.fn);
        var fn = sp[fnName];
        if (typeof fn !== 'function') {
            showErr('No live preview for sp.' + call.fn + '() yet (missing unified WASM entry: ' + fnName + ')');
            hideLoader();
            return;
        }
        var parsed;
        try { parsed = parsePyArgs(call.body); }
        catch (e) { showErr('Parse error: ' + e.message); return; }
        var input = Object.assign({ title: parsed.title }, parsed.kwargs);
        try {
            var html = fn(JSON.stringify(input));
            hideErr();
            hideLoader();
            renderHtmlInIframe(html);
        } catch (e) {
            showErr('Render error: ' + (e && e.message ? e.message : String(e)));
            hideLoader();
        }
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
            var min = 160, max = parentW - 140;
            if (w < min) w = min; if (w > max) w = max;
            ecol.style.width = w + 'px';
            if (state.editor) state.editor.refresh();
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

    function buildSampleCode(sample) {
        return 'import seraplot as sp\n\nc = ' + sample.code + '\n';
    }

    function selectVariant(idx) {
        var samples = SAMPLES[state.slug] || [];
        if (!samples[idx]) return;
        state.currentVariant = idx;
        var tabs = document.querySelectorAll('.sp-pg-tab');
        for (var i = 0; i < tabs.length; i++) tabs[i].classList.toggle('sp-active', i === idx);
        if (state.editor) {
            state.editor.setValue(buildSampleCode(samples[idx]));
            runOnce(true);
        }
    }

    function buildUI() {
        injectStyles();
        var slug = state.slug;
        var samples = SAMPLES[slug] || [{ name: 'Default', code: 'sp.' + slug + '("Demo ' + slug + '")' }];
        var tabsHtml = '';
        for (var i = 0; i < samples.length; i++) {
            tabsHtml += '<button class="sp-pg-tab' + (i === 0 ? ' sp-active' : '') + '" type="button" data-idx="' + i + '">' + samples[i].name + '</button>';
        }
        var wrap = document.createElement('div');
        wrap.className = 'sp-pg-wrap';
        wrap.innerHTML =
            '<div class="sp-pg-hdr">' +
                '<div class="sp-pg-title">Playground · ' + slug + '</div>' +
                '<div class="sp-pg-spacer"></div>' +
                '<button class="sp-pg-btn" type="button">Run</button>' +
                '<div class="sp-pg-conn"><span class="sp-pg-dot"></span><span class="sp-pg-text">Init</span></div>' +
            '</div>' +
            '<div class="sp-pg-tabs">' + tabsHtml + '</div>' +
            '<div class="sp-pg-main">' +
                '<div class="sp-pg-ecol"><div class="sp-pg-cm-wrap"><textarea></textarea></div></div>' +
                '<div class="sp-pg-divider"></div>' +
                '<div class="sp-pg-pcol">' +
                    '<iframe class="sp-pg-iframe" sandbox="allow-scripts allow-same-origin"></iframe>' +
                    '<div class="sp-pg-loader"><div class="sp-pg-spinner"></div><div>Loading WASM</div></div>' +
                    '<div class="sp-pg-err"></div>' +
                '</div>' +
            '</div>';

        var contentRoot = document.querySelector('main') || document.querySelector('.content') || document.body;
        var firstHeading = contentRoot.querySelector('h1, h2');
        if (firstHeading && firstHeading.parentNode) firstHeading.parentNode.insertBefore(wrap, firstHeading.nextSibling);
        else contentRoot.insertBefore(wrap, contentRoot.firstChild);

        var ta = wrap.querySelector('textarea');
        ta.value = buildSampleCode(samples[0]);
        state.iframe = wrap.querySelector('.sp-pg-iframe');
        state.statusDot = wrap.querySelector('.sp-pg-dot');
        state.statusText = wrap.querySelector('.sp-pg-text');
        state.loaderEl = wrap.querySelector('.sp-pg-loader');
        state.errEl = wrap.querySelector('.sp-pg-err');
        var btn = wrap.querySelector('.sp-pg-btn');
        var divider = wrap.querySelector('.sp-pg-divider');
        var ecol = wrap.querySelector('.sp-pg-ecol');
        var tabs = wrap.querySelectorAll('.sp-pg-tab');
        for (var t = 0; t < tabs.length; t++) {
            (function (idx) { tabs[idx].addEventListener('click', function () { selectVariant(idx); }); })(t);
        }
        btn.addEventListener('click', function () { runOnce(true); });

        setStatus('loading', 'Loading WASM');
        loadCM(function () {
            state.editor = window.CodeMirror.fromTextArea(ta, {
                mode: 'python',
                theme: 'dracula',
                lineNumbers: false,
                lineWrapping: true,
                indentUnit: 4,
                tabSize: 4,
                indentWithTabs: false,
            });
            state.editor.on('change', debouncedRun);
            attachResize(divider, ecol);
            ensureWasm(function () {
                state.ready = true;
                setStatus('ready', 'Live · in-browser');
                runOnce(true);
            });
        });
    }

    function init() {
        if (!isChartPage()) return;
        state.slug = pageSlug();
        buildUI();
    }

    if (document.readyState === 'loading') {
        document.addEventListener('DOMContentLoaded', init);
    } else {
        init();
    }
})();
