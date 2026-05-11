(function () {
    var WS_URL_OVERRIDE = localStorage.getItem("sp_ws_url");
    var WS_URL_LOCAL = "ws://127.0.0.1:7842";
    var PLAYGROUND_STATUS_URL = "https://raw.githubusercontent.com/feur25/seraplot/main/playground-url.json";
    var MAX_AGE_S = 21600;
    var DEBOUNCE_MS = 650;
    var LANG_KEY = "seraplot_lang";
    var _runId = 0;
    var _ws = null;
    var _wsState = "disc";
    var _wsCallbacks = {};
    var _allFns = [];

    var TPLS = {
        "bar": [
            'import seraplot as sp\nc = sp.bar(',
            '    "Revenue by Month",\n    labels=["Jan","Feb","Mar","Apr","May","Jun","Jul","Aug","Sep","Oct","Nov","Dec"],\n    values=[42,58,73,61,89,97,85,78,91,65,54,70],\n    variant="basic",\n    theme="deluxe",\n    width=860,\n    height=420,',
            ')\nc'
        ],
        "grouped-bar": [
            'import seraplot as sp\nc = sp.grouped_bar(',
            '    "Performance Comparison",\n    labels=["Q1","Q2","Q3","Q4"],\n    series=[[42,58,73,61],[60,45,80,55],[35,70,50,65]],\n    series_names=["Alpha","Beta","Gamma"],\n    theme="deluxe",\n    width=860,\n    height=440,',
            ')\nc'
        ],
        "stacked-bar": [
            'import seraplot as sp\nc = sp.stacked_bar(',
            '    "Revenue Breakdown",\n    labels=["Jan","Feb","Mar","Apr","May","Jun"],\n    series=[[30,40,35,45,50,42],[20,25,30,20,25,28],[10,15,12,18,20,15]],\n    series_names=["Product A","Product B","Product C"],\n    theme="prism",\n    width=860,\n    height=440,',
            ')\nc'
        ],
        "hbar": [
            'import seraplot as sp\nc = sp.hbar(',
            '    "Horizontal Rankings",\n    labels=["Alpha","Beta","Gamma","Delta","Epsilon"],\n    values=[88,74,91,65,83],\n    theme="prism",\n    width=860,\n    height=420,',
            ')\nc'
        ],
        "bubble": [
            'import seraplot as sp\nc = sp.bubble(',
            '    "Market Positioning",\n    x=[1.2,2.4,3.8,5.1,7.0,8.5],\n    y=[4.5,6.2,3.1,8.7,5.5,9.2],\n    sizes=[20,40,15,60,35,25],\n    labels=["A","B","C","D","E","F"],\n    theme="aurora",\n    width=860,\n    height=480,',
            ')\nc'
        ],
        "scatter": [
            'import seraplot as sp\nimport random; random.seed(42)\nc = sp.scatter(',
            '    "Correlation Study",\n    x=[random.gauss(0,1) for _ in range(80)],\n    y=[random.gauss(0,1) for _ in range(80)],\n    theme="prism",\n    width=860,\n    height=480,',
            ')\nc'
        ],
        "line": [
            'import seraplot as sp\nc = sp.line(',
            '    "Trend Analysis",\n    x=list(range(12)),\n    y=[12,19,3,17,28,24,38,35,45,41,52,60],\n    theme="frost",\n    width=860,\n    height=420,',
            ')\nc'
        ],
        "multiline": [
            'import seraplot as sp\nc = sp.multiline(',
            '    "Series Comparison",\n    x=list(range(12)),\n    series=[[12,19,3,17,28,24,38,35,45,41,52,60],[8,15,25,13,21,30,27,32,38,29,44,51]],\n    series_names=["Alpha","Beta"],\n    theme="deluxe",\n    width=860,\n    height=420,',
            ')\nc'
        ],
        "area": [
            'import seraplot as sp\nc = sp.area(',
            '    "Sales Volume",\n    x=list(range(12)),\n    y=[420,580,730,610,890,970,850,780,910,650,540,700],\n    theme="aurora",\n    width=860,\n    height=420,',
            ')\nc'
        ],
        "histogram": [
            'import seraplot as sp\nimport random; random.seed(7)\nc = sp.histogram(',
            '    "Distribution",\n    values=[random.gauss(100,15) for _ in range(500)],\n    bins=30,\n    theme="prism",\n    width=860,\n    height=420,',
            ')\nc'
        ],
        "radar": [
            'import seraplot as sp\nc = sp.radar(',
            '    "Skills Matrix",\n    axes=["Speed","Agility","Strength","Defense","Magic","Stamina"],\n    series=[[85,72,60,91,78,88],[60,55,95,88,45,75]],\n    series_names=["Hero","Warrior"],\n    theme="deluxe",\n    width=620,\n    height=520,',
            ')\nc'
        ],
        "violin": [
            'import seraplot as sp\nimport random; random.seed(3)\nc = sp.violin(',
            '    "Score Distribution",\n    groups=["Alpha","Beta","Gamma"],\n    values=[[random.gauss(65,12) for _ in range(100)],[random.gauss(75,8) for _ in range(100)],[random.gauss(55,20) for _ in range(100)]],\n    theme="prism",\n    width=860,\n    height=480,',
            ')\nc'
        ],
        "boxplot": [
            'import seraplot as sp\nimport random; random.seed(3)\nc = sp.boxplot(',
            '    "Statistical Summary",\n    groups=["Q1","Q2","Q3","Q4"],\n    values=[[random.gauss(50,10) for _ in range(60)],[random.gauss(65,8) for _ in range(60)],[random.gauss(55,12) for _ in range(60)],[random.gauss(70,7) for _ in range(60)]],\n    theme="aurora",\n    width=860,\n    height=480,',
            ')\nc'
        ],
        "heatmap": [
            'import seraplot as sp\nc = sp.heatmap(',
            '    "Correlation Matrix",\n    rows=["A","B","C","D","E"],\n    cols=["V1","V2","V3","V4","V5"],\n    values=[[0.9,0.4,0.2,0.7,0.3],[0.4,1.0,0.6,0.1,0.8],[0.2,0.6,0.9,0.5,0.4],[0.7,0.1,0.5,1.0,0.2],[0.3,0.8,0.4,0.2,1.0]],\n    theme="frost",\n    width=700,\n    height=560,',
            ')\nc'
        ],
        "parallel": [
            'import seraplot as sp\nc = sp.parallel(',
            '    "Multi-Attribute Analysis",\n    axes=["Speed","Power","Agility","Defense"],\n    series=[[82,74,91,65],[55,88,60,78],[70,65,75,82],[90,50,85,58]],\n    series_names=["A","B","C","D"],\n    theme="frost",\n    width=860,\n    height=460,',
            ')\nc'
        ],
        "treemap": [
            'import seraplot as sp\nc = sp.treemap(',
            '    "Portfolio Allocation",\n    labels=["Tech","Finance","Health","Energy","Consumer","Materials"],\n    values=[340,220,180,150,130,90],\n    theme="prism",\n    width=860,\n    height=520,',
            ')\nc'
        ],
        "sunburst": [
            'import seraplot as sp\nc = sp.sunburst(',
            '    "Hierarchy",\n    labels=["Root","A","B","A1","A2","B1","B2"],\n    parents=["","Root","Root","A","A","B","B"],\n    values=[0,60,40,35,25,22,18],\n    theme="aurora",\n    width=700,\n    height=700,',
            ')\nc'
        ],
        "pie": [
            'import seraplot as sp\nc = sp.pie(',
            '    "Market Share",\n    labels=["Product A","Product B","Product C","Other"],\n    values=[38,27,21,14],\n    theme="aurora",\n    width=700,\n    height=520,',
            ')\nc'
        ],
        "donut": [
            'import seraplot as sp\nc = sp.donut(',
            '    "Category Split",\n    labels=["Alpha","Beta","Gamma","Delta"],\n    values=[35,28,22,15],\n    theme="prism",\n    width=700,\n    height=520,',
            ')\nc'
        ],
        "funnel": [
            'import seraplot as sp\nc = sp.funnel(',
            '    "Conversion Funnel",\n    stages=["Impressions","Clicks","Signups","Trials","Paid"],\n    values=[10000,3200,850,240,96],\n    theme="aurora",\n    width=700,\n    height=520,',
            ')\nc'
        ],
        "kde": [
            'import seraplot as sp\nimport random; random.seed(5)\nc = sp.kde(',
            '    "Density Estimate",\n    values=[random.gauss(50,15) for _ in range(200)],\n    theme="deluxe",\n    width=860,\n    height=440,',
            ')\nc'
        ],
        "ridgeline": [
            'import seraplot as sp\nimport random; random.seed(1)\nc = sp.ridgeline(',
            '    "Distribution Ridgeline",\n    groups=["2020","2021","2022","2023","2024"],\n    values=[[random.gauss(50+i*3,10+i) for _ in range(80)] for i in range(5)],\n    theme="prism",\n    width=860,\n    height=560,',
            ')\nc'
        ],
        "lollipop": [
            'import seraplot as sp\nc = sp.lollipop(',
            '    "Top Values",\n    labels=["Alpha","Beta","Gamma","Delta","Epsilon","Zeta"],\n    values=[88,74,91,65,83,57],\n    theme="frost",\n    width=860,\n    height=440,',
            ')\nc'
        ],
        "waterfall": [
            'import seraplot as sp\nc = sp.waterfall(',
            '    "Cash Flow",\n    labels=["Start","Revenue","Costs","Taxes","R&D","End"],\n    values=[100,250,-180,-40,-30,100],\n    theme="prism",\n    width=860,\n    height=460,',
            ')\nc'
        ],
        "slope": [
            'import seraplot as sp\nc = sp.slope(',
            '    "Before vs After",\n    labels=["Alpha","Beta","Gamma","Delta"],\n    before=[42,78,55,90],\n    after=[65,61,82,74],\n    theme="deluxe",\n    width=600,\n    height=500,',
            ')\nc'
        ],
        "bullet": [
            'import seraplot as sp\nc = sp.bullet(',
            '    "KPI Dashboard",\n    labels=["Revenue","Satisfaction","Leads","Retention"],\n    actuals=[82,74,91,65],\n    targets=[90,80,85,70],\n    theme="aurora",\n    width=860,\n    height=380,',
            ')\nc'
        ],
        "scatter3d": [
            'import seraplot as sp\nimport random; random.seed(42)\nn = 60\nc = sp.scatter3d(',
            '    "3D Distribution",\n    x=[random.gauss(0,1) for _ in range(n)],\n    y=[random.gauss(0,1) for _ in range(n)],\n    z=[random.gauss(0,1) for _ in range(n)],\n    theme="deluxe",\n    width=860,\n    height=560,',
            ')\nc'
        ],
        "bar3d": [
            'import seraplot as sp\nc = sp.bar3d(',
            '    "3D Overview",\n    labels=["Q1","Q2","Q3","Q4"],\n    series=[[42,58,73,61],[60,45,80,55]],\n    series_names=["Alpha","Beta"],\n    theme="inferno",\n    width=860,\n    height=560,',
            ')\nc'
        ],
        "bubble3d": [
            'import seraplot as sp\nimport random; random.seed(9)\nn = 40\nc = sp.bubble3d(',
            '    "3D Bubbles",\n    x=[random.gauss(0,1) for _ in range(n)],\n    y=[random.gauss(0,1) for _ in range(n)],\n    z=[random.gauss(0,1) for _ in range(n)],\n    sizes=[random.randint(10,50) for _ in range(n)],\n    theme="aurora",\n    width=860,\n    height=560,',
            ')\nc'
        ],
        "line3d": [
            'import seraplot as sp\nimport math\nt = [i/20*math.pi*4 for i in range(80)]\nc = sp.line3d(',
            '    "3D Helix",\n    x=[math.cos(v) for v in t],\n    y=[math.sin(v) for v in t],\n    z=t,\n    theme="frost",\n    width=860,\n    height=560,',
            ')\nc'
        ],
        "gauge": [
            'import seraplot as sp\nc = sp.gauge(',
            '    "KPI Score",\n    value=72,\n    min_val=0,\n    max_val=100,\n    theme="deluxe",\n    width=500,\n    height=400,',
            ')\nc'
        ],
        "choropleth": [
            'import seraplot as sp\nc = sp.choropleth(',
            '    "Country Metric",\n    countries=["USA","GBR","FRA","DEU","CHN","IND","BRA"],\n    values=[88,74,82,79,91,65,70],\n    theme="aurora",\n    width=860,\n    height=520,',
            ')\nc'
        ],
        "streaming": [
            'import seraplot as sp\nimport time',
            'months = ["Jan","Feb","Mar","Apr","May","Jun"]\nbase = [42,58,73,61,89,97]\n\nfor i in range(8):\n    import random; random.seed(i)\n    vals = [v + random.randint(-10,10) for v in base]\n    c = sp.bar("Live Stream", labels=months, values=vals, theme="inferno", width=860, height=420)\n    _stream(c)\n    time.sleep(0.3)',
            ''
        ],
    };

    var DEFAULT_TPL = [
        'import seraplot as sp\nc = sp.bar(',
        '    "My Chart",\n    labels=["A","B","C","D"],\n    values=[10,25,18,32],',
        ')\nc'
    ];

    function getLang() { return localStorage.getItem(LANG_KEY) || "en"; }

    function pageSlug() {
        var m = window.location.pathname.match(/\/([^\/]+?)(?:\.html?)?$/);
        return m ? m[1] : null;
    }

    function isChartPage() {
        return /\/charts\//.test(window.location.pathname) &&
               pageSlug() !== "index" &&
               pageSlug() !== null;
    }

    function loadCM(cb) {
        if (window.CodeMirror) { cb(); return; }
        [
            "https://cdnjs.cloudflare.com/ajax/libs/codemirror/5.65.17/codemirror.min.css",
            "https://cdnjs.cloudflare.com/ajax/libs/codemirror/5.65.17/theme/dracula.min.css"
        ].forEach(function (href) {
            var l = document.createElement("link");
            l.rel = "stylesheet"; l.href = href;
            document.head.appendChild(l);
        });
        function loadScript(src, next) {
            var s = document.createElement("script");
            s.src = src; s.onload = next; s.onerror = next;
            document.head.appendChild(s);
        }
        loadScript(
            "https://cdnjs.cloudflare.com/ajax/libs/codemirror/5.65.17/codemirror.min.js",
            function () {
                loadScript(
                    "https://cdnjs.cloudflare.com/ajax/libs/codemirror/5.65.17/mode/python/python.min.js",
                    cb
                );
            }
        );
    }

    function resolveWsUrl(cb) {
        if (WS_URL_OVERRIDE) { cb(WS_URL_OVERRIDE); return; }
        if (location.protocol !== "https:") { cb(WS_URL_LOCAL); return; }
        fetch(PLAYGROUND_STATUS_URL + "?_=" + Date.now())
            .then(function (r) { return r.json(); })
            .then(function (d) {
                var age = Math.floor(Date.now() / 1000) - (d.ts || 0);
                cb((d.url && age < MAX_AGE_S) ? d.url : null);
            })
            .catch(function () { cb(null); });
    }

    function wsConnect(onState) {
        if (_ws && (_ws.readyState === 0 || _ws.readyState === 1)) return;
        _wsState = "connecting";
        onState("connecting");
        resolveWsUrl(function (url) {
            if (!url) { _wsState = "disc"; onState("disc"); return; }
            try {
                _ws = new WebSocket(url);
            } catch (e) {
                _wsState = "disc";
                onState("disc");
                return;
            }
            _ws.onopen = function () {
                _wsState = "ok";
                _ws.send(JSON.stringify({ a: "ping" }));
            };
            _ws.onmessage = function (e) {
                try {
                    var msg = JSON.parse(e.data);
                    if (msg.t === "pong") {
                        onState("ok", msg.version);
                        _ws.send(JSON.stringify({ a: "list" }));
                    }
                    if (msg.t === "list" && msg.fns) {
                        _allFns = msg.fns;
                        document.dispatchEvent(new CustomEvent("sp-pg-fns", { detail: msg.fns }));
                    }
                    var cb = _wsCallbacks[msg.id];
                    if (cb) cb(msg);
                    if (msg.t === "done" || msg.t === "err") delete _wsCallbacks[msg.id];
                } catch (ex) {}
            };
            _ws.onclose = _ws.onerror = function () {
                _wsState = "disc";
                onState("disc");
                _ws = null;
                setTimeout(function () { wsConnect(onState); }, 3000);
            };
        });
    }

    function wsSend(payload, cb) {
        if (!_ws || _ws.readyState !== 1) return false;
        if (cb) _wsCallbacks[payload.id] = cb;
        _ws.send(JSON.stringify(payload));
        return true;
    }

    function esc(s) {
        return (s || "").replace(/&/g, "&amp;").replace(/</g, "&lt;").replace(/>/g, "&gt;");
    }

    function buildPG(root) {
        var slug = pageSlug();
        var tpl = (slug && TPLS[slug]) ? TPLS[slug].slice() : DEFAULT_TPL.slice();
        var frames = [];
        var frameIdx = 0;
        var currentRunId = 0;
        var debTimer = null;
        var autoPlay = null;
        var cmEditor = null;
        var previewEl, statusDot, versionEl, statusMsg, streamBar, fcEl;

        injectStyles();
        root.innerHTML = buildHTML(tpl, slug);
        queryRefs();
        bindEvents();
        wsConnect(onWsState);

        loadCM(function () {
            if (!window.CodeMirror) return;
            var cmWrap = root.querySelector(".sp-pg-cm-wrap");
            cmEditor = CodeMirror(cmWrap, {
                value: tpl[1],
                mode: "python",
                theme: "dracula",
                lineNumbers: false,
                indentUnit: 4,
                tabSize: 4,
                indentWithTabs: false,
                viewportMargin: Infinity,
                extraKeys: { Tab: function (cm) { cm.replaceSelection("    "); } }
            });
            cmEditor.on("change", function () {
                clearTimeout(debTimer);
                debTimer = setTimeout(function () {
                    if (_wsState === "ok") run();
                }, DEBOUNCE_MS);
            });
        });

        function queryRefs() {
            previewEl  = root.querySelector(".sp-pg-iframe");
            statusDot  = root.querySelector(".sp-pg-dot");
            versionEl  = root.querySelector(".sp-pg-ver");
            statusMsg  = root.querySelector(".sp-pg-smsg");
            streamBar  = root.querySelector(".sp-pg-sbar");
            fcEl       = root.querySelector(".sp-pg-fc");
        }

        function buildHTML(t, currentSlug) {
            var tplOptions = Object.keys(TPLS).map(function (k) {
                return '<option value="' + k + '"' + (k === currentSlug ? ' selected' : '') + '>' + k + '</option>';
            }).join('');
            return '<div class="sp-pg-wrap">' +
                '<div class="sp-pg-hdr">' +
                    '<span class="sp-pg-title">Playground</span>' +
                    '<select class="sp-pg-fn-sel"><optgroup label="Charts">' + tplOptions + '</optgroup></select>' +
                    '<span class="sp-pg-conn">' +
                        '<span class="sp-pg-dot sp-pg-disc"></span>' +
                        '<span class="sp-pg-ver"></span>' +
                        '<span class="sp-pg-smsg"></span>' +
                    '</span>' +
                '</div>' +
                '<div class="sp-pg-main">' +
                    '<div class="sp-pg-ecol">' +
                        '<pre class="sp-pg-head">' + esc(t[0]) + '</pre>' +
                        '<div class="sp-pg-cm-wrap"></div>' +
                        (t[2] ? '<pre class="sp-pg-tail">' + esc(t[2]) + '</pre>' : '') +
                    '</div>' +
                    '<div class="sp-pg-pcol">' +
                        '<iframe class="sp-pg-iframe" sandbox="allow-scripts allow-same-origin"></iframe>' +
                        '<div class="sp-pg-sbar" style="display:none">' +
                            '<button class="sp-pg-fprev">&#x2039;</button>' +
                            '<span class="sp-pg-fc">1/1</span>' +
                            '<button class="sp-pg-fnext">&#x203a;</button>' +
                            '<button class="sp-pg-fplay">&#x25b6;</button>' +
                        '</div>' +
                    '</div>' +
                '</div>' +
            '</div>';
        }

        function bindEvents() {
            root.querySelector(".sp-pg-fn-sel").addEventListener("change", function () {
                var v = this.value;
                if (!v) return;
                if (v.startsWith("__dyn__")) {
                    loadDynTpl(v.slice(7));
                } else {
                    tpl = TPLS[v] ? TPLS[v].slice() : DEFAULT_TPL.slice();
                    root.querySelector(".sp-pg-head").textContent = tpl[0];
                    if (cmEditor) cmEditor.setValue(tpl[1]);
                    var tailEl = root.querySelector(".sp-pg-tail");
                    if (tailEl) tailEl.textContent = tpl[2];
                    if (_wsState === "ok") run();
                }
            });

            root.querySelector(".sp-pg-fprev").addEventListener("click", function () { showFrame(frameIdx - 1); });
            root.querySelector(".sp-pg-fnext").addEventListener("click", function () { showFrame(frameIdx + 1); });
            root.querySelector(".sp-pg-fplay").addEventListener("click", function () {
                var btn = this;
                if (autoPlay) {
                    clearInterval(autoPlay); autoPlay = null; btn.textContent = "\u25b6";
                } else {
                    btn.textContent = "\u23f8";
                    autoPlay = setInterval(function () {
                        if (frameIdx >= frames.length - 1) {
                            clearInterval(autoPlay); autoPlay = null; btn.textContent = "\u25b6";
                        } else showFrame(frameIdx + 1);
                    }, 400);
                }
            });

            document.addEventListener("sp-pg-fns", function (e) {
                var sel = root.querySelector(".sp-pg-fn-sel");
                if (!sel) return;
                var dyn = e.detail.filter(function (f) { return !TPLS[f]; });
                var grp = sel.querySelector('optgroup[label="All functions"]');
                if (grp) grp.parentNode.removeChild(grp);
                if (dyn.length) {
                    var og = document.createElement("optgroup");
                    og.label = "All functions";
                    dyn.forEach(function (f) {
                        var o = document.createElement("option");
                        o.value = "__dyn__" + f;
                        o.textContent = "sp." + f;
                        og.appendChild(o);
                    });
                    sel.appendChild(og);
                }
            });
        }

        function loadDynTpl(fn) {
            wsSend({ a: "sig", fn: fn }, function (msg) {
                if (msg.t !== "sig" || !msg.params) return;
                var required = msg.params.filter(function (p) { return p.required; });
                var optional = msg.params.filter(function (p) { return !p.required; });
                var head = 'import seraplot as sp\nc = sp.' + fn + '(';
                var body = required.map(function (p) {
                    return '    ' + p.name + '=' + (p.name === 'title' ? '"My ' + fn + '"' : '""') + ',';
                }).concat(
                    optional.slice(0, 6).map(function (p) {
                        return '    ' + p.name + '=' + (p.default || '""') + ',';
                    })
                ).join('\n');
                var tail = ')\nc';
                tpl = [head, body, tail];
                root.querySelector(".sp-pg-head").textContent = head;
                if (cmEditor) cmEditor.setValue(body);
                var tailEl = root.querySelector(".sp-pg-tail");
                if (tailEl) tailEl.textContent = tail;
                if (_wsState === "ok") run();
            });
        }

        function onWsState(state, version) {
            if (!statusDot) return;
            statusDot.className = "sp-pg-dot sp-pg-" + state;
            if (version) versionEl.textContent = "v" + version;
            if (state === "ok") {
                setMsg("ready");
                run();
            } else if (state === "connecting") {
                setMsg("connecting");
                if (previewEl && !previewEl.srcdoc) previewEl.srcdoc = loadingFrame();
            } else {
                setMsg("disc");
                if (previewEl) previewEl.srcdoc = discFrame();
            }
        }

        function fullCode() {
            var body = cmEditor ? cmEditor.getValue() : "";
            return tpl[0] + "\n" + body + "\n" + tpl[2];
        }

        function loadingFrame() {
            return '<!DOCTYPE html><html><head><style>' +
                'body{margin:0;background:#060a14;display:flex;align-items:center;justify-content:center;height:100vh}' +
                '@keyframes spin{to{transform:rotate(360deg)}}' +
                '.r{width:28px;height:28px;border:3px solid #1c2a40;border-top-color:#6366f1;border-radius:50%;animation:spin 0.8s linear infinite}' +
                '</style></head><body><div class="r"></div></body></html>';
        }

        function discFrame() {
            var isFr = getLang() === "fr";
            var msg = isFr ? "Connexion au serveur impossible" : "Could not reach playground server";
            var hint = "pip install seraplot websockets &amp;&amp; python playground_server.py";
            return '<!DOCTYPE html><html><head><style>' +
                'body{margin:0;background:#060a14;display:flex;align-items:center;justify-content:center;height:100vh;font-family:system-ui,sans-serif;flex-direction:column;gap:10px}' +
                'p{color:#334155;font-size:12px;margin:0;text-align:center}' +
                'code{background:#0d1425;color:#475569;padding:3px 9px;border-radius:4px;font-size:11px;border:1px solid #1c2a40}' +
                '</style></head><body>' +
                '<p>' + msg + '</p><code>' + hint + '</code>' +
                '</body></html>';
        }

        function run() {
            if (_wsState !== "ok") return;
            currentRunId = ++_runId;
            frames = []; frameIdx = 0;
            if (autoPlay) { clearInterval(autoPlay); autoPlay = null; }
            setMsg("running");
            var id = currentRunId;
            wsSend({ a: "run", id: id, code: fullCode() }, function (msg) {
                if (msg.id !== id) return;
                if (msg.t === "frame") {
                    frames.push(msg.html);
                    showFrame(frames.length - 1);
                    if (frames.length > 1) setMsg("streaming");
                } else if (msg.t === "done") {
                    setMsg("ready");
                    updateSBar();
                } else if (msg.t === "err") {
                    var lines = (msg.msg || "").split("\n").slice(-6).join("\n");
                    previewEl.srcdoc = "<body style='background:#140505;color:#f87171;font:12.5px/1.5 monospace;padding:20px'><pre>" + esc(lines) + "</pre></body>";
                    setMsg("err");
                }
            });
        }

        function showFrame(idx) {
            idx = Math.max(0, Math.min(idx, frames.length - 1));
            frameIdx = idx;
            if (frames[idx]) previewEl.srcdoc = frames[idx];
            updateSBar();
        }

        function updateSBar() {
            if (streamBar) streamBar.style.display = frames.length > 1 ? "" : "none";
            if (fcEl) fcEl.textContent = (frameIdx + 1) + "/" + frames.length;
        }

        function setMsg(s) {
            if (!statusMsg) return;
            var isFr = getLang() === "fr";
            var m = {
                running:    isFr ? "Execution..." : "Running...",
                streaming:  "Streaming...",
                ready:      isFr ? "Pret" : "Ready",
                err:        isFr ? "Erreur" : "Error",
                connecting: isFr ? "Connexion..." : "Connecting...",
                disc:       ""
            };
            statusMsg.textContent = m[s] !== undefined ? m[s] : s;
            statusMsg.className = "sp-pg-smsg sp-pg-s-" + s;
        }
    }

    function injectStyles() {
        if (document.getElementById("sp-pg-css")) return;
        var s = document.createElement("style");
        s.id = "sp-pg-css";
        s.textContent = [
            ".sp-pg-wrap{font-family:-apple-system,BlinkMacSystemFont,'Segoe UI',Roboto,sans-serif;",
            "background:linear-gradient(180deg,#0c1120,#070c18);border:1px solid #1c2a40;",
            "border-radius:12px;overflow:hidden;margin:0 0 32px;",
            "box-shadow:0 16px 48px -12px rgba(0,0,0,.65),0 0 0 1px rgba(99,102,241,.08) inset}",
            ".sp-pg-hdr{display:flex;align-items:center;gap:10px;padding:9px 14px;",
            "background:#080e1c;border-bottom:1px solid #1c2a40;flex-wrap:wrap}",
            ".sp-pg-title{font-size:11px;font-weight:700;letter-spacing:.1em;text-transform:uppercase;color:#6366f1;flex-shrink:0}",
            ".sp-pg-conn{display:flex;align-items:center;gap:5px;margin-left:auto;min-width:0}",
            ".sp-pg-dot{width:7px;height:7px;border-radius:50%;flex-shrink:0;transition:background .3s,box-shadow .3s}",
            ".sp-pg-ok{background:#22c55e;box-shadow:0 0 7px rgba(34,197,94,.55)}",
            ".sp-pg-connecting{background:#f59e0b;animation:sp-pg-blink 1s infinite}",
            ".sp-pg-disc{background:#1e2d42}",
            "@keyframes sp-pg-blink{0%,100%{opacity:1}50%{opacity:.25}}",
            ".sp-pg-ver{font-size:10.5px;color:#2d3f56}",
            ".sp-pg-smsg{font-size:11px;color:#334155;transition:color .2s;white-space:nowrap}",
            ".sp-pg-s-running{color:#f59e0b}",
            ".sp-pg-s-streaming{color:#22c55e;animation:sp-pg-blink 0.7s infinite}",
            ".sp-pg-s-ready{color:#2d3f56}",
            ".sp-pg-s-err{color:#ef4444}",
            ".sp-pg-s-connecting{color:#f59e0b}",
            ".sp-pg-fn-sel{background:#0d1425;border:1px solid #1e2d42;color:#64748b;",
            "padding:4px 8px;border-radius:6px;font-size:11.5px;cursor:pointer;max-width:180px;outline:none}",
            ".sp-pg-fn-sel:focus{border-color:#4f46e5}",
            ".sp-pg-main{display:grid;grid-template-columns:1fr 1fr;min-height:340px}",
            "@media(max-width:780px){.sp-pg-main{grid-template-columns:1fr}}",
            ".sp-pg-ecol{border-right:1px solid #0f1b2d;display:flex;flex-direction:column;min-width:0;overflow:hidden}",
            ".sp-pg-head,.sp-pg-tail{margin:0;padding:7px 14px;",
            "background:#060a16;color:#243452;font:12px/1.6 'Fira Code','Cascadia Code','Consolas',monospace;",
            "white-space:pre;overflow-x:auto;user-select:none;flex-shrink:0}",
            ".sp-pg-head{border-bottom:1px solid #0f1b2d}",
            ".sp-pg-tail{border-top:1px solid #0f1b2d}",
            ".sp-pg-cm-wrap{flex:1;overflow:hidden;min-height:80px}",
            ".sp-pg-cm-wrap .CodeMirror{height:auto !important;min-height:80px;",
            "background:#080d1e !important;font:13px/1.65 'Fira Code','Cascadia Code','Consolas',monospace !important;",
            "padding:0;border:none !important;color:#cdd6f4}",
            ".sp-pg-cm-wrap .CodeMirror-scroll{min-height:80px}",
            ".sp-pg-cm-wrap .CodeMirror-gutters{display:none !important;width:0 !important}",
            ".sp-pg-cm-wrap .CodeMirror-lines{padding:9px 14px !important}",
            ".sp-pg-cm-wrap .CodeMirror-cursor{border-left:2px solid #818cf8 !important}",
            ".sp-pg-cm-wrap .CodeMirror-selected{background:rgba(99,102,241,.18) !important}",
            ".sp-pg-cm-wrap .cm-s-dracula .cm-keyword{color:#ff79c6}",
            ".sp-pg-cm-wrap .cm-s-dracula .cm-def{color:#50fa7b}",
            ".sp-pg-cm-wrap .cm-s-dracula .cm-string{color:#f1fa8c}",
            ".sp-pg-cm-wrap .cm-s-dracula .cm-number{color:#bd93f9}",
            ".sp-pg-cm-wrap .cm-s-dracula .cm-comment{color:#3d5070;font-style:italic}",
            ".sp-pg-cm-wrap .cm-s-dracula .cm-variable-2{color:#8be9fd}",
            ".sp-pg-cm-wrap .cm-s-dracula .cm-operator{color:#ff79c6}",
            ".sp-pg-pcol{position:relative;display:flex;flex-direction:column;background:#060a14;min-width:0}",
            ".sp-pg-iframe{flex:1;border:none;min-height:340px;background:#060a14;display:block;width:100%}",
            ".sp-pg-sbar{display:flex;align-items:center;gap:8px;padding:5px 12px;",
            "background:#060c1a;border-top:1px solid #0f1b2d;font-size:11.5px;color:#334155}",
            ".sp-pg-sbar button{background:#101926;border:none;color:#4a5e78;",
            "padding:2px 10px;border-radius:4px;cursor:pointer;font-size:13px;line-height:1.5;transition:background .15s}",
            ".sp-pg-sbar button:hover{background:#1a2a3d;color:#94a3b8}",
            ".sp-pg-fc{min-width:38px;text-align:center;color:#2d3f56;font-size:11px}",
        ].join("");
        document.head.appendChild(s);
    }

    function init() {
        if (!isChartPage()) return;
        if (document.querySelector(".sp-pg-wrap")) return;
        var main = document.querySelector(".content main") ||
                   document.querySelector("main") ||
                   document.querySelector(".content");
        if (!main) return;
        var h1 = main.querySelector("h1");
        var anchor = h1 ? h1.nextElementSibling : main.firstElementChild;
        var wrapper = document.createElement("div");
        if (anchor) main.insertBefore(wrapper, anchor);
        else main.appendChild(wrapper);
        buildPG(wrapper);
    }

    if (document.readyState === "loading") {
        document.addEventListener("DOMContentLoaded", init);
    } else {
        init();
    }

    var _obs = new MutationObserver(function () {
        if (!document.querySelector(".sp-pg-wrap") && isChartPage()) setTimeout(init, 80);
    });
    _obs.observe(document.body, { childList: true });
})();