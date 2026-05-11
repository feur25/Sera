(function () {
    var WS_URL = "ws://127.0.0.1:7842";
    var CODESPACES_URL = "https://codespaces.new/feur25/seraplot?quickstart=1";
    var DEBOUNCE_MS = 650;
    var LANG_KEY = "seraplot_lang";
    var _runId = 0;
    var _ws = null;
    var _wsState = "disc";
    var _wsCallbacks = {};
    var _allFns = [];

    var TPLS = {
        "bar": [
            'import seraplot as sp\nc = sp.bar(\n    "Revenue by Month",',
            '    labels=["Jan","Feb","Mar","Apr","May","Jun","Jul","Aug","Sep","Oct","Nov","Dec"],\n    values=[42,58,73,61,89,97,85,78,91,65,54,70],\n    variant="basic",\n    theme="deluxe",\n    width=860,\n    height=420,',
            ')\nc'
        ],
        "grouped-bar": [
            'import seraplot as sp\nc = sp.grouped_bar(\n    "Performance Comparison",',
            '    labels=["Q1","Q2","Q3","Q4"],\n    series=[[42,58,73,61],[60,45,80,55],[35,70,50,65]],\n    series_names=["Alpha","Beta","Gamma"],\n    theme="deluxe",\n    width=860,\n    height=440,',
            ')\nc'
        ],
        "stacked-bar": [
            'import seraplot as sp\nc = sp.stacked_bar(\n    "Revenue Breakdown",',
            '    labels=["Jan","Feb","Mar","Apr","May","Jun"],\n    series=[[30,40,35,45,50,42],[20,25,30,20,25,28],[10,15,12,18,20,15]],\n    series_names=["Product A","Product B","Product C"],\n    theme="prism",\n    width=860,\n    height=440,',
            ')\nc'
        ],
        "hbar": [
            'import seraplot as sp\nc = sp.hbar(\n    "Horizontal Rankings",',
            '    labels=["Alpha","Beta","Gamma","Delta","Epsilon"],\n    values=[88,74,91,65,83],\n    theme="prism",\n    width=860,\n    height=420,',
            ')\nc'
        ],
        "bubble": [
            'import seraplot as sp\nc = sp.bubble(\n    "Market Positioning",',
            '    x=[1.2,2.4,3.8,5.1,7.0,8.5],\n    y=[4.5,6.2,3.1,8.7,5.5,9.2],\n    sizes=[20,40,15,60,35,25],\n    labels=["A","B","C","D","E","F"],\n    theme="aurora",\n    width=860,\n    height=480,',
            ')\nc'
        ],
        "scatter": [
            'import seraplot as sp\nimport random; random.seed(42)\nc = sp.scatter(\n    "Correlation Study",',
            '    x=[random.gauss(0,1) for _ in range(80)],\n    y=[random.gauss(0,1) for _ in range(80)],\n    theme="prism",\n    width=860,\n    height=480,',
            ')\nc'
        ],
        "line": [
            'import seraplot as sp\nc = sp.line(\n    "Trend Analysis",',
            '    x=list(range(12)),\n    y=[12,19,3,17,28,24,38,35,45,41,52,60],\n    theme="frost",\n    width=860,\n    height=420,',
            ')\nc'
        ],
        "multiline": [
            'import seraplot as sp\nc = sp.multiline(\n    "Series Comparison",',
            '    x=list(range(12)),\n    series=[[12,19,3,17,28,24,38,35,45,41,52,60],[8,15,25,13,21,30,27,32,38,29,44,51]],\n    series_names=["Alpha","Beta"],\n    theme="deluxe",\n    width=860,\n    height=420,',
            ')\nc'
        ],
        "area": [
            'import seraplot as sp\nc = sp.area(\n    "Sales Volume",',
            '    x=list(range(12)),\n    y=[420,580,730,610,890,970,850,780,910,650,540,700],\n    theme="aurora",\n    width=860,\n    height=420,',
            ')\nc'
        ],
        "histogram": [
            'import seraplot as sp\nimport random; random.seed(7)\nc = sp.histogram(\n    "Distribution",',
            '    values=[random.gauss(100,15) for _ in range(500)],\n    bins=30,\n    theme="prism",\n    width=860,\n    height=420,',
            ')\nc'
        ],
        "radar": [
            'import seraplot as sp\nc = sp.radar(\n    "Skills Matrix",',
            '    axes=["Speed","Agility","Strength","Defense","Magic","Stamina"],\n    series=[[85,72,60,91,78,88],[60,55,95,88,45,75]],\n    series_names=["Hero","Warrior"],\n    theme="deluxe",\n    width=620,\n    height=520,',
            ')\nc'
        ],
        "violin": [
            'import seraplot as sp\nimport random; random.seed(3)\nc = sp.violin(\n    "Score Distribution",',
            '    groups=["Alpha","Beta","Gamma"],\n    values=[[random.gauss(65,12) for _ in range(100)],[random.gauss(75,8) for _ in range(100)],[random.gauss(55,20) for _ in range(100)]],\n    theme="prism",\n    width=860,\n    height=480,',
            ')\nc'
        ],
        "boxplot": [
            'import seraplot as sp\nimport random; random.seed(3)\nc = sp.boxplot(\n    "Statistical Summary",',
            '    groups=["Q1","Q2","Q3","Q4"],\n    values=[[random.gauss(50,10) for _ in range(60)],[random.gauss(65,8) for _ in range(60)],[random.gauss(55,12) for _ in range(60)],[random.gauss(70,7) for _ in range(60)]],\n    theme="aurora",\n    width=860,\n    height=480,',
            ')\nc'
        ],
        "heatmap": [
            'import seraplot as sp\nc = sp.heatmap(\n    "Correlation Matrix",',
            '    rows=["A","B","C","D","E"],\n    cols=["V1","V2","V3","V4","V5"],\n    values=[[0.9,0.4,0.2,0.7,0.3],[0.4,1.0,0.6,0.1,0.8],[0.2,0.6,0.9,0.5,0.4],[0.7,0.1,0.5,1.0,0.2],[0.3,0.8,0.4,0.2,1.0]],\n    theme="frost",\n    width=700,\n    height=560,',
            ')\nc'
        ],
        "parallel": [
            'import seraplot as sp\nc = sp.parallel(\n    "Multi-Attribute Analysis",',
            '    axes=["Speed","Power","Agility","Defense"],\n    series=[[82,74,91,65],[55,88,60,78],[70,65,75,82],[90,50,85,58]],\n    series_names=["A","B","C","D"],\n    theme="frost",\n    width=860,\n    height=460,',
            ')\nc'
        ],
        "treemap": [
            'import seraplot as sp\nc = sp.treemap(\n    "Portfolio Allocation",',
            '    labels=["Tech","Finance","Health","Energy","Consumer","Materials"],\n    values=[340,220,180,150,130,90],\n    theme="prism",\n    width=860,\n    height=520,',
            ')\nc'
        ],
        "sunburst": [
            'import seraplot as sp\nc = sp.sunburst(\n    "Hierarchy",',
            '    labels=["Root","A","B","A1","A2","B1","B2"],\n    parents=["","Root","Root","A","A","B","B"],\n    values=[0,60,40,35,25,22,18],\n    theme="aurora",\n    width=700,\n    height=700,',
            ')\nc'
        ],
        "pie": [
            'import seraplot as sp\nc = sp.pie(\n    "Market Share",',
            '    labels=["Product A","Product B","Product C","Other"],\n    values=[38,27,21,14],\n    theme="aurora",\n    width=700,\n    height=520,',
            ')\nc'
        ],
        "donut": [
            'import seraplot as sp\nc = sp.donut(\n    "Category Split",',
            '    labels=["Alpha","Beta","Gamma","Delta"],\n    values=[35,28,22,15],\n    theme="prism",\n    width=700,\n    height=520,',
            ')\nc'
        ],
        "funnel": [
            'import seraplot as sp\nc = sp.funnel(\n    "Conversion Funnel",',
            '    stages=["Impressions","Clicks","Signups","Trials","Paid"],\n    values=[10000,3200,850,240,96],\n    theme="aurora",\n    width=700,\n    height=520,',
            ')\nc'
        ],
        "kde": [
            'import seraplot as sp\nimport random; random.seed(5)\nc = sp.kde(\n    "Density Estimate",',
            '    values=[random.gauss(50,15) for _ in range(200)],\n    theme="deluxe",\n    width=860,\n    height=440,',
            ')\nc'
        ],
        "ridgeline": [
            'import seraplot as sp\nimport random; random.seed(1)\nc = sp.ridgeline(\n    "Distribution Ridgeline",',
            '    groups=["2020","2021","2022","2023","2024"],\n    values=[[random.gauss(50+i*3,10+i) for _ in range(80)] for i in range(5)],\n    theme="prism",\n    width=860,\n    height=560,',
            ')\nc'
        ],
        "lollipop": [
            'import seraplot as sp\nc = sp.lollipop(\n    "Top Values",',
            '    labels=["Alpha","Beta","Gamma","Delta","Epsilon","Zeta"],\n    values=[88,74,91,65,83,57],\n    theme="frost",\n    width=860,\n    height=440,',
            ')\nc'
        ],
        "waterfall": [
            'import seraplot as sp\nc = sp.waterfall(\n    "Cash Flow",',
            '    labels=["Start","Revenue","Costs","Taxes","R&D","End"],\n    values=[100,250,-180,-40,-30,100],\n    theme="prism",\n    width=860,\n    height=460,',
            ')\nc'
        ],
        "slope": [
            'import seraplot as sp\nc = sp.slope(\n    "Before vs After",',
            '    labels=["Alpha","Beta","Gamma","Delta"],\n    before=[42,78,55,90],\n    after=[65,61,82,74],\n    theme="deluxe",\n    width=600,\n    height=500,',
            ')\nc'
        ],
        "bullet": [
            'import seraplot as sp\nc = sp.bullet(\n    "KPI Dashboard",',
            '    labels=["Revenue","Satisfaction","Leads","Retention"],\n    actuals=[82,74,91,65],\n    targets=[90,80,85,70],\n    theme="aurora",\n    width=860,\n    height=380,',
            ')\nc'
        ],
        "scatter3d": [
            'import seraplot as sp\nimport random; random.seed(42)\nn = 60\nc = sp.scatter3d(\n    "3D Distribution",',
            '    x=[random.gauss(0,1) for _ in range(n)],\n    y=[random.gauss(0,1) for _ in range(n)],\n    z=[random.gauss(0,1) for _ in range(n)],\n    theme="deluxe",\n    width=860,\n    height=560,',
            ')\nc'
        ],
        "bar3d": [
            'import seraplot as sp\nc = sp.bar3d(\n    "3D Overview",',
            '    labels=["Q1","Q2","Q3","Q4"],\n    series=[[42,58,73,61],[60,45,80,55]],\n    series_names=["Alpha","Beta"],\n    theme="inferno",\n    width=860,\n    height=560,',
            ')\nc'
        ],
        "bubble3d": [
            'import seraplot as sp\nimport random; random.seed(9)\nn = 40\nc = sp.bubble3d(\n    "3D Bubbles",',
            '    x=[random.gauss(0,1) for _ in range(n)],\n    y=[random.gauss(0,1) for _ in range(n)],\n    z=[random.gauss(0,1) for _ in range(n)],\n    sizes=[random.randint(10,50) for _ in range(n)],\n    theme="aurora",\n    width=860,\n    height=560,',
            ')\nc'
        ],
        "line3d": [
            'import seraplot as sp\nimport math\nt = [i/20*math.pi*4 for i in range(80)]\nc = sp.line3d(\n    "3D Helix",',
            '    x=[math.cos(v) for v in t],\n    y=[math.sin(v) for v in t],\n    z=t,\n    theme="frost",\n    width=860,\n    height=560,',
            ')\nc'
        ],
        "gauge": [
            'import seraplot as sp\nc = sp.gauge(\n    "KPI Score",',
            '    value=72,\n    min_val=0,\n    max_val=100,\n    theme="deluxe",\n    width=500,\n    height=400,',
            ')\nc'
        ],
        "choropleth": [
            'import seraplot as sp\nc = sp.choropleth(\n    "Country Metric",',
            '    countries=["USA","GBR","FRA","DEU","CHN","IND","BRA"],\n    values=[88,74,82,79,91,65,70],\n    theme="aurora",\n    width=860,\n    height=520,',
            ')\nc'
        ],
    };

    var DEFAULT_TPL = [
        'import seraplot as sp\nc = sp.bar(',
        '    "My Chart",\n    labels=["A","B","C","D"],\n    values=[10,25,18,32],',
        ')\nc'
    ];

    var STREAMING_TPL = [
        'import seraplot as sp\nimport time',
        'months = ["Jan","Feb","Mar","Apr","May","Jun"]\nbase = [42,58,73,61,89,97]\n\nfor i in range(8):\n    import random; random.seed(i)\n    vals = [v + random.randint(-10,10) for v in base]\n    c = sp.bar("Live Stream", labels=months, values=vals, theme="inferno", width=860, height=420)\n    _stream(c)\n    time.sleep(0.3)',
        ''
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

    function wsConnect(onState) {
        if (_ws && (_ws.readyState === 0 || _ws.readyState === 1)) return;
        _wsState = "connecting";
        onState("connecting");
        try {
            _ws = new WebSocket(WS_URL);
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
    }

    function wsSend(payload, cb) {
        if (!_ws || _ws.readyState !== 1) return false;
        if (cb) _wsCallbacks[payload.id] = cb;
        _ws.send(JSON.stringify(payload));
        return true;
    }

    function esc(s) {
        return (s || "").replace(/&/g,"&amp;").replace(/</g,"&lt;").replace(/>/g,"&gt;");
    }

    function buildPG(root) {
        var slug = pageSlug();
        var tpl = (slug && TPLS[slug]) ? TPLS[slug].slice() : DEFAULT_TPL.slice();
        var frames = [];
        var frameIdx = 0;
        var currentRunId = 0;
        var debTimer = null;
        var autoEnabled = true;
        var autoPlay = null;
        var editorEl, previewEl, statusDot, versionEl, statusMsg, offlineBanner, streamBar, fcEl;

        injectStyles();
        root.innerHTML = buildHTML(tpl, slug);
        queryRefs();
        bindEvents();

        wsConnect(onWsState);

        function queryRefs() {
            editorEl     = root.querySelector(".sp-pg-body");
            previewEl    = root.querySelector(".sp-pg-iframe");
            statusDot    = root.querySelector(".sp-pg-dot");
            versionEl    = root.querySelector(".sp-pg-ver");
            statusMsg    = root.querySelector(".sp-pg-smsg");
            offlineBanner = root.querySelector(".sp-pg-offline");
            streamBar    = root.querySelector(".sp-pg-sbar");
            fcEl         = root.querySelector(".sp-pg-fc");
        }

        function buildHTML(t, currentSlug) {
            var lang = getLang();
            var isFr = lang === "fr";
            var runLbl    = isFr ? "▶ Exécuter"   : "▶ Run";
            var autoLbl   = isFr ? "⟳ Auto"        : "⟳ Auto";
            var stmLbl    = isFr ? "⚡ Stream"      : "⚡ Stream";
            var fnLbl     = isFr ? "Choisir…"       : "Pick chart…";
            var offMsg    = isFr
                ? '<span>Exécution locale non disponible.</span>' +
                  '<div class="sp-pg-off-actions">' +
                  '<a class="sp-pg-cs-btn" href="' + CODESPACES_URL + '" target="_blank" rel="noopener">&#9654; Ouvrir dans Codespaces</a>' +
                  '<span class="sp-pg-off-sep">ou</span>' +
                  '<span class="sp-pg-off-cmd"><code>pip install seraplot</code><code>python playground_server.py</code></span>' +
                  '</div>'
                : '<span>Live execution not available.</span>' +
                  '<div class="sp-pg-off-actions">' +
                  '<a class="sp-pg-cs-btn" href="' + CODESPACES_URL + '" target="_blank" rel="noopener">&#9654; Open in Codespaces</a>' +
                  '<span class="sp-pg-off-sep">or run locally:</span>' +
                  '<span class="sp-pg-off-cmd"><code>pip install seraplot</code><code>python playground_server.py</code></span>' +
                  '</div>';

            var tplOptions = Object.keys(TPLS).map(function (k) {
                return '<option value="' + k + '"' + (k === currentSlug ? ' selected' : '') + '>' + k + '</option>';
            }).join('');

            var dynOptions = _allFns.filter(function (f) { return !TPLS[f]; }).map(function (f) {
                return '<option value="__dyn__' + f + '">sp.' + f + '</option>';
            }).join('');

            return '<div class="sp-pg-wrap">' +
                '<div class="sp-pg-hdr">' +
                    '<span class="sp-pg-title">Playground</span>' +
                    '<select class="sp-pg-fn-sel"><optgroup label="Chart types">' + tplOptions + '</optgroup>' +
                        (dynOptions ? '<optgroup label="All functions">' + dynOptions + '</optgroup>' : '') +
                    '</select>' +
                    '<span class="sp-pg-conn"><span class="sp-pg-dot sp-pg-disc"></span><span class="sp-pg-ver"></span></span>' +
                '</div>' +
                '<div class="sp-pg-main">' +
                    '<div class="sp-pg-ecol">' +
                        '<pre class="sp-pg-head">' + esc(t[0]) + '</pre>' +
                        '<textarea class="sp-pg-body" spellcheck="false" autocorrect="off" autocapitalize="off">' + t[1] + '</textarea>' +
                        '<pre class="sp-pg-tail">' + esc(t[2]) + '</pre>' +
                    '</div>' +
                    '<div class="sp-pg-pcol">' +
                        '<iframe class="sp-pg-iframe" sandbox="allow-scripts allow-same-origin"></iframe>' +
                        '<div class="sp-pg-sbar" style="display:none">' +
                            '<button class="sp-pg-fprev">‹</button>' +
                            '<span class="sp-pg-fc">1/1</span>' +
                            '<button class="sp-pg-fnext">›</button>' +
                            '<button class="sp-pg-fplay">▶</button>' +
                            '<span class="sp-pg-fps">400ms</span>' +
                        '</div>' +
                    '</div>' +
                '</div>' +
                '<div class="sp-pg-ftr">' +
                    '<button class="sp-pg-run sp-pg-btn">' + runLbl + '</button>' +
                    '<button class="sp-pg-auto sp-pg-btn sp-pg-sec sp-pg-act">' + autoLbl + '</button>' +
                    '<button class="sp-pg-stm sp-pg-btn sp-pg-sec">' + stmLbl + '</button>' +
                    '<button class="sp-pg-copy sp-pg-btn sp-pg-sec">⧉</button>' +
                    '<span class="sp-pg-smsg"></span>' +
                    '<div class="sp-pg-offline" style="display:none">' + offMsg + '</div>' +
                '</div>' +
            '</div>';
        }

        function bindEvents() {
            editorEl.addEventListener("input", function () {
                autoResize();
                if (autoEnabled) {
                    clearTimeout(debTimer);
                    debTimer = setTimeout(run, DEBOUNCE_MS);
                }
            });
            editorEl.addEventListener("keydown", function (e) {
                if (e.key === "Tab") {
                    e.preventDefault();
                    var s = editorEl.selectionStart, v = editorEl.value;
                    editorEl.value = v.slice(0, s) + "    " + v.slice(editorEl.selectionEnd);
                    editorEl.selectionStart = editorEl.selectionEnd = s + 4;
                }
            });

            root.querySelector(".sp-pg-run").addEventListener("click", run);

            root.querySelector(".sp-pg-auto").addEventListener("click", function () {
                autoEnabled = !autoEnabled;
                this.classList.toggle("sp-pg-act", autoEnabled);
            });

            root.querySelector(".sp-pg-stm").addEventListener("click", function () {
                tpl = STREAMING_TPL.slice();
                root.querySelector(".sp-pg-head").textContent = tpl[0];
                editorEl.value = tpl[1];
                root.querySelector(".sp-pg-tail").textContent = tpl[2];
                autoResize();
                run();
            });

            root.querySelector(".sp-pg-copy").addEventListener("click", function () {
                if (navigator.clipboard) navigator.clipboard.writeText(fullCode());
            });

            root.querySelector(".sp-pg-fn-sel").addEventListener("change", function () {
                var v = this.value;
                if (!v) return;
                if (v.startsWith("__dyn__")) {
                    var fn = v.slice(7);
                    loadDynTpl(fn);
                } else {
                    tpl = TPLS[v] ? TPLS[v].slice() : DEFAULT_TPL.slice();
                    root.querySelector(".sp-pg-head").textContent = tpl[0];
                    editorEl.value = tpl[1];
                    root.querySelector(".sp-pg-tail").textContent = tpl[2];
                    autoResize();
                    run();
                }
            });

            root.querySelector(".sp-pg-fprev").addEventListener("click", function () { showFrame(frameIdx - 1); });
            root.querySelector(".sp-pg-fnext").addEventListener("click", function () { showFrame(frameIdx + 1); });
            root.querySelector(".sp-pg-fplay").addEventListener("click", function () {
                var btn = this;
                if (autoPlay) {
                    clearInterval(autoPlay); autoPlay = null; btn.textContent = "▶";
                } else {
                    btn.textContent = "⏸";
                    autoPlay = setInterval(function () {
                        if (frameIdx >= frames.length - 1) {
                            clearInterval(autoPlay); autoPlay = null; btn.textContent = "▶";
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

            autoResize();
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
                editorEl.value = body;
                root.querySelector(".sp-pg-tail").textContent = tail;
                autoResize();
                run();
            });
            wsSend({ a: "sig", fn: fn }, null);
        }

        function onWsState(state, version) {
            if (!statusDot) return;
            statusDot.className = "sp-pg-dot sp-pg-" + state;
            if (version) versionEl.textContent = "v" + version;
            if (offlineBanner) offlineBanner.style.display = state === "disc" ? "" : "none";
            if (state === "disc") {
                root.querySelector(".sp-pg-main").classList.add("sp-pg-offline-mode");
                if (previewEl) previewEl.srcdoc = buildOfflinePreview();
            } else {
                root.querySelector(".sp-pg-main").classList.remove("sp-pg-offline-mode");
                run();
            }
        }

        function fullCode() {
            return tpl[0] + "\n" + editorEl.value + "\n" + tpl[2];
        }

        function buildOfflinePreview() {
            var code = esc(fullCode());
            var isFr = getLang() === "fr";
            var msg = isFr ? "Copiez et exécutez localement ou ouvrez dans Codespaces" : "Copy and run locally, or open in Codespaces";
            return '<!DOCTYPE html><html><head><style>' +
                'body{margin:0;background:#060a14;display:flex;align-items:center;justify-content:center;height:100vh;font-family:system-ui,sans-serif}' +
                '.wrap{text-align:center;padding:24px;max-width:480px}' +
                'p{color:#475569;font-size:12px;margin:0 0 16px}' +
                'a{display:inline-block;background:linear-gradient(135deg,#22863a,#2ea44f);color:#fff;text-decoration:none;padding:7px 20px;border-radius:6px;font-size:12px;font-weight:700}' +
                'a:hover{opacity:.85}' +
                '</style></head><body><div class="wrap"><p>' + msg + '</p>' +
                '<a href="' + CODESPACES_URL + '" target="_blank" rel="noopener">&#9654; Codespaces</a>' +
                '</div></body></html>';
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
            var m = { running: "Running…", streaming: "Streaming…", ready: "Ready", err: "Error" };
            statusMsg.textContent = m[s] || s;
            statusMsg.className = "sp-pg-smsg sp-pg-s-" + s;
        }

        function autoResize() {
            editorEl.style.height = "auto";
            editorEl.style.height = Math.max(80, editorEl.scrollHeight) + "px";
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
            ".sp-pg-title{font-size:11px;font-weight:700;letter-spacing:.1em;text-transform:uppercase;",
            "color:#6366f1;flex-shrink:0}",
            ".sp-pg-conn{display:flex;align-items:center;gap:5px;margin-left:auto}",
            ".sp-pg-dot{width:7px;height:7px;border-radius:50%;flex-shrink:0;transition:all .3s}",
            ".sp-pg-ok{background:#22c55e;box-shadow:0 0 6px rgba(34,197,94,.5)}",
            ".sp-pg-connecting{background:#f59e0b;animation:sp-pg-blink 1s infinite}",
            ".sp-pg-disc{background:#334155}",
            "@keyframes sp-pg-blink{0%,100%{opacity:1}50%{opacity:.3}}",
            ".sp-pg-ver{font-size:10.5px;color:#475569}",

            ".sp-pg-fn-sel{background:#0d1425;border:1px solid #243049;color:#94a3b8;",
            "padding:4px 8px;border-radius:6px;font-size:11.5px;cursor:pointer;max-width:180px}",
            ".sp-pg-fn-sel:focus{outline:none;border-color:#6366f1}",

            ".sp-pg-main{display:grid;grid-template-columns:1fr 1fr;min-height:320px}",
            ".sp-pg-main.sp-pg-offline-mode{grid-template-columns:1fr 1fr}",
            "@media(max-width:780px){.sp-pg-main{grid-template-columns:1fr}}",

            ".sp-pg-ecol{border-right:1px solid #1c2a40;display:flex;flex-direction:column;min-width:0}",

            ".sp-pg-head,.sp-pg-tail{margin:0;padding:7px 14px;",
            "background:#060a16;color:#3d4f6a;font:12px/1.6 'Fira Code','Cascadia Code','Consolas',monospace;",
            "white-space:pre;overflow-x:auto;user-select:none;flex-shrink:0}",
            ".sp-pg-head{border-bottom:1px dashed #1c2a40}",
            ".sp-pg-tail{border-top:1px dashed #1c2a40}",

            ".sp-pg-body{flex:1;margin:0;padding:9px 14px;resize:none;overflow:hidden;",
            "background:#090e1f;color:#b8c8e0;",
            "font:13px/1.65 'Fira Code','Cascadia Code','Consolas',monospace;",
            "border:none;outline:none;min-height:80px;tab-size:4;white-space:pre}",
            ".sp-pg-body:focus{background:#0a1022;outline:none}",

            ".sp-pg-pcol{position:relative;display:flex;flex-direction:column;background:#060a14;min-width:0}",
            ".sp-pg-iframe{flex:1;border:none;min-height:340px;background:#060a14;display:block}",

            ".sp-pg-sbar{display:flex;align-items:center;gap:8px;padding:5px 12px;",
            "background:#060c1a;border-top:1px solid #1c2a40;font-size:11.5px;color:#475569}",
            ".sp-pg-sbar button{background:#162035;border:none;color:#94a3b8;",
            "padding:2px 10px;border-radius:4px;cursor:pointer;font-size:13px;line-height:1.5}",
            ".sp-pg-sbar button:hover{background:#243049}",
            ".sp-pg-fc{min-width:38px;text-align:center;color:#6b7a93}",
            ".sp-pg-fps{font-size:10.5px;color:#334155}",

            ".sp-pg-ftr{display:flex;align-items:center;gap:8px;padding:9px 14px;",
            "background:#060c1a;border-top:1px solid #1c2a40;flex-wrap:wrap}",

            ".sp-pg-btn{padding:5px 14px;border-radius:6px;border:none;cursor:pointer;",
            "font-size:11.5px;font-weight:600;transition:all .15s;white-space:nowrap}",
            ".sp-pg-run{background:linear-gradient(135deg,#3730a3,#6366f1);color:#fff}",
            ".sp-pg-run:hover{background:linear-gradient(135deg,#2d2a8a,#4f46e5);transform:translateY(-1px)}",
            ".sp-pg-sec{background:#131e30;color:#64748b}",
            ".sp-pg-sec:hover{background:#1c2a40;color:#94a3b8}",
            ".sp-pg-sec.sp-pg-act{background:#1e1b4b;color:#a5b4fc;border:1px solid #3730a3}",

            ".sp-pg-smsg{font-size:11px;margin-left:auto;transition:color .2s}",
            ".sp-pg-s-running{color:#f59e0b}",
            ".sp-pg-s-streaming{color:#22c55e;animation:sp-pg-blink 0.8s infinite}",
            ".sp-pg-s-ready{color:#475569}",
            ".sp-pg-s-err{color:#f87171}",

            ".sp-pg-offline{font-size:11px;color:#94a3b8;padding:6px 10px;width:100%;",
            "background:rgba(15,23,42,.6);border-radius:6px;border:1px solid #1c2a40;",
            "display:flex;flex-direction:column;gap:5px}",
            ".sp-pg-off-actions{display:flex;align-items:center;gap:8px;flex-wrap:wrap;margin-top:2px}",
            ".sp-pg-cs-btn{background:linear-gradient(135deg,#22863a,#2ea44f);color:#fff;",
            "text-decoration:none;padding:4px 12px;border-radius:5px;font-size:11px;font-weight:700;",
            "white-space:nowrap;transition:opacity .15s}",
            ".sp-pg-cs-btn:hover{opacity:.85}",
            ".sp-pg-off-sep{color:#475569;font-size:10.5px;white-space:nowrap}",
            ".sp-pg-off-cmd{display:flex;gap:5px;flex-wrap:wrap}",
            ".sp-pg-off-cmd code{background:#0d1425;border:1px solid #1c2a40;padding:2px 7px;",
            "border-radius:4px;color:#64748b;font-size:10.5px;white-space:nowrap}",
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

    var _initObs = new MutationObserver(function () {
        if (!document.querySelector(".sp-pg-wrap") && isChartPage()) {
            setTimeout(init, 80);
        }
    });
    _initObs.observe(document.body, { childList: true });
})();
