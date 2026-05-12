(function () {
    var DEBOUNCE_MS = 650;
    var _debTimer = null;
    var _wasmReady = false;
    var _streamTimer = null;

    function rng(seed) {
        var s = seed >>> 0;
        return function () { s = (Math.imul(s, 1664525) + 1013904223) >>> 0; return s / 4294967296; };
    }
    function gauss(n, mu, sig, seed) {
        var r = rng(seed), a = [];
        for (var i = 0; i < n; i++) {
            var u1 = r() + 1e-10, u2 = r();
            a.push(+(mu + sig * Math.sqrt(-2 * Math.log(u1)) * Math.cos(2 * Math.PI * u2)).toFixed(2));
        }
        return a;
    }
    function rints(n, lo, hi, seed) {
        var r = rng(seed), a = [];
        for (var i = 0; i < n; i++) a.push(Math.round(lo + r() * (hi - lo)));
        return a;
    }
    function J(v) { return JSON.stringify(v); }

    var _sc = gauss(20, 0, 1, 42), _sc2 = gauss(20, 0, 1, 99);
    var _hist = gauss(50, 100, 15, 7);
    var _kde = gauss(40, 50, 15, 5);
    var _vio = [gauss(25, 65, 12, 3), gauss(25, 75, 8, 4), gauss(25, 55, 20, 5)];
    var _box = [gauss(20, 50, 10, 3), gauss(20, 65, 8, 4), gauss(20, 55, 12, 5), gauss(20, 70, 7, 6)];
    var _rid = [0, 1, 2, 3, 4].map(function (i) { return gauss(18, 50 + i * 3, 10 + i, 10 + i); });
    var _s3x = gauss(15, 0, 1, 42), _s3y = gauss(15, 0, 1, 43), _s3z = gauss(15, 0, 1, 44);
    var _b3x = gauss(12, 0, 1, 9), _b3y = gauss(12, 0, 1, 10), _b3z = gauss(12, 0, 1, 11);
    var _b3s = rints(12, 10, 50, 9);
    var _hx = [], _hy = [], _hz = [];
    for (var _ti = 0; _ti < 40; _ti++) {
        var _t = _ti / 20 * Math.PI * 4;
        _hx.push(+Math.cos(_t).toFixed(3));
        _hy.push(+Math.sin(_t).toFixed(3));
        _hz.push(+_t.toFixed(3));
    }

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
            'import seraplot as sp\nc = sp.scatter(',
            '    "Correlation Study",\n    x=SCATTERX,\n    y=SCATTERY,\n    theme="prism",\n    width=860,\n    height=480,',
            ')\nc'
        ],
        "line": [
            'import seraplot as sp\nc = sp.line(',
            '    "Trend Analysis",\n    x=[0,1,2,3,4,5,6,7,8,9,10,11],\n    y=[12,19,3,17,28,24,38,35,45,41,52,60],\n    theme="frost",\n    width=860,\n    height=420,',
            ')\nc'
        ],
        "multiline": [
            'import seraplot as sp\nc = sp.multiline(',
            '    "Series Comparison",\n    x=[0,1,2,3,4,5,6,7,8,9,10,11],\n    series=[[12,19,3,17,28,24,38,35,45,41,52,60],[8,15,25,13,21,30,27,32,38,29,44,51]],\n    series_names=["Alpha","Beta"],\n    theme="deluxe",\n    width=860,\n    height=420,',
            ')\nc'
        ],
        "area": [
            'import seraplot as sp\nc = sp.area(',
            '    "Sales Volume",\n    x=[0,1,2,3,4,5,6,7,8,9,10,11],\n    y=[420,580,730,610,890,970,850,780,910,650,540,700],\n    theme="aurora",\n    width=860,\n    height=420,',
            ')\nc'
        ],
        "histogram": [
            'import seraplot as sp\nc = sp.histogram(',
            '    "Distribution",\n    values=HIST,\n    bins=20,\n    theme="prism",\n    width=860,\n    height=420,',
            ')\nc'
        ],
        "radar": [
            'import seraplot as sp\nc = sp.radar(',
            '    "Skills Matrix",\n    axes=["Speed","Agility","Strength","Defense","Magic","Stamina"],\n    series=[[85,72,60,91,78,88],[60,55,95,88,45,75]],\n    series_names=["Hero","Warrior"],\n    theme="deluxe",\n    width=620,\n    height=520,',
            ')\nc'
        ],
        "violin": [
            'import seraplot as sp\nc = sp.violin(',
            '    "Score Distribution",\n    groups=["Alpha","Beta","Gamma"],\n    values=VIO,\n    theme="prism",\n    width=860,\n    height=480,',
            ')\nc'
        ],
        "boxplot": [
            'import seraplot as sp\nc = sp.boxplot(',
            '    "Statistical Summary",\n    groups=["Q1","Q2","Q3","Q4"],\n    values=BOX,\n    theme="aurora",\n    width=860,\n    height=480,',
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
            'import seraplot as sp\nc = sp.kde(',
            '    "Density Estimate",\n    values=KDE,\n    theme="deluxe",\n    width=860,\n    height=440,',
            ')\nc'
        ],
        "ridgeline": [
            'import seraplot as sp\nc = sp.ridgeline(',
            '    "Distribution Ridgeline",\n    groups=["2020","2021","2022","2023","2024"],\n    values=RID,\n    theme="prism",\n    width=860,\n    height=560,',
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
            'import seraplot as sp\nc = sp.scatter3d(',
            '    "3D Distribution",\n    x=S3X,\n    y=S3Y,\n    z=S3Z,\n    theme="deluxe",\n    width=860,\n    height=560,',
            ')\nc'
        ],
        "bar3d": [
            'import seraplot as sp\nc = sp.bar3d(',
            '    "3D Overview",\n    labels=["Q1","Q2","Q3","Q4"],\n    series=[[42,58,73,61],[60,45,80,55]],\n    series_names=["Alpha","Beta"],\n    theme="inferno",\n    width=860,\n    height=560,',
            ')\nc'
        ],
        "bubble3d": [
            'import seraplot as sp\nc = sp.bubble3d(',
            '    "3D Bubbles",\n    x=B3X,\n    y=B3Y,\n    z=B3Z,\n    sizes=B3S,\n    theme="aurora",\n    width=860,\n    height=560,',
            ')\nc'
        ],
        "line3d": [
            'import seraplot as sp\nc = sp.line3d(',
            '    "3D Helix",\n    x=HX,\n    y=HY,\n    z=HZ,\n    theme="frost",\n    width=860,\n    height=560,',
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
            'import seraplot as sp',
            '    title="Live Stream",\n    labels=["Jan","Feb","Mar","Apr","May","Jun"],\n    base=[42,58,73,61,89,97],\n    theme="inferno",\n    width=860,\n    height=420,',
            ''
        ],
    };

    function fixTpl(tpl) {
        var b = tpl[1];
        b = b.replace('SCATTERX', J(_sc)).replace('SCATTERY', J(_sc2));
        b = b.replace('HIST', J(_hist));
        b = b.replace('VIO', J(_vio));
        b = b.replace('BOX', J(_box));
        b = b.replace('KDE', J(_kde));
        b = b.replace('RID', J(_rid));
        b = b.replace('S3X', J(_s3x)).replace('S3Y', J(_s3y)).replace('S3Z', J(_s3z));
        b = b.replace('B3X', J(_b3x)).replace('B3Y', J(_b3y)).replace('B3Z', J(_b3z)).replace('B3S', J(_b3s));
        b = b.replace('HX', J(_hx)).replace('HY', J(_hy)).replace('HZ', J(_hz));
        return [tpl[0], b, tpl[2]];
    }

    var DEFAULT_TPL = [
        'import seraplot as sp\nc = sp.bar(',
        '    "My Chart",\n    labels=["A","B","C","D"],\n    values=[10,25,18,32],',
        ')\nc'
    ];

    var CHART_MAP = {
        "bar":          { fn: "buildBarChart",       p: ["labels", "values"] },
        "hbar":         { fn: "buildHbar",            p: ["labels", "values"] },
        "pie":          { fn: "buildPieChart",        p: ["labels", "values"] },
        "donut":        { fn: "buildDonutChart",      p: ["labels", "values"] },
        "lollipop":     { fn: "buildLollipopChart",   p: ["labels", "values"] },
        "waterfall":    { fn: "buildWaterfall",       p: ["labels", "values"] },
        "treemap":      { fn: "buildTreemap",         p: ["labels", "values"] },
        "funnel":       { fn: "buildFunnel",          p: ["labels", "values"],  al: { stages: "labels" } },
        "choropleth":   { fn: "buildChoropleth",      p: ["labels", "values"],  al: { countries: "labels" } },
        "bullet":       { fn: "buildBullet",          p: ["labels", "values"],  al: { actuals: "values" } },
        "scatter":      { fn: "buildScatterChart",    p: ["x", "y"] },
        "line":         { fn: "buildLineChart",       p: ["labels", "values"],  al: { x: "labels", y: "values" } },
        "kde":          { fn: "buildKdeChart",        p: ["values"] },
        "histogram":    { fn: "buildHistogram",       p: ["values"] },
        "gauge":        { fn: "buildGauge",           p: ["value"],             sp: "gauge" },
        "grouped-bar":  { fn: "buildGroupedBar",      p: ["cat", "series"],     al: { labels: "cat" } },
        "grouped_bar":  { fn: "buildGroupedBar",      p: ["cat", "series"],     al: { labels: "cat" } },
        "stacked-bar":  { fn: "buildStackedBar",      p: ["cat", "series"],     al: { labels: "cat" } },
        "stacked_bar":  { fn: "buildStackedBar",      p: ["cat", "series"],     al: { labels: "cat" } },
        "multiline":    { fn: "buildMultilineChart",  p: ["xlabels", "series"], al: { x: "xlabels" } },
        "area":         { fn: "buildAreaChart",       p: ["xlabels", "series"], al: { x: "xlabels" }, sp: "area" },
        "radar":        { fn: "buildRadarChart",      p: ["axes", "series"] },
        "parallel":     { fn: "buildParallel",        p: ["axes", "series"] },
        "violin":       { fn: "buildViolin",          p: ["cat", "values"],     al: { groups: "cat" } },
        "boxplot":      { fn: "buildBoxplot",         p: ["cat", "values"],     al: { groups: "cat" } },
        "heatmap":      { fn: "buildHeatmap",         p: ["labels", "matrix"],  al: { rows: "labels", values: "matrix" }, sp: "heatmap" },
        "sunburst":     { fn: "buildSunburst",        p: ["labels", "parents", "values"] },
        "ridgeline":    { fn: "buildRidgelineChart",  p: ["values", "categories"], al: { groups: "categories" } },
        "slope":        { fn: "buildSlope",           p: ["labels", "left", "right"], al: { before: "left", after: "right" } },
        "bubble":       { fn: "buildBubble",          p: ["x", "y", "sizes"] },
        "scatter3d":    { fn: "buildScatter3dChart",  p: ["x", "y", "z"] },
        "line3d":       { fn: "buildLine3dChart",     p: ["x", "y", "z"] },
        "bubble3d":     { fn: "buildBubble3dChart",   p: ["x", "y", "z", "sizes"] },
        "bar3d":        { fn: "buildBar3dChart",      p: ["x", "y", "z"],       sp: "bar3d" },
    };

    function parsePyVal(s) {
        s = s.trim();
        if (!s) return undefined;
        if (s === "True") return true;
        if (s === "False") return false;
        if (s === "None") return null;
        var rm = s.match(/^list\(range\((\d+)\)\)$/);
        if (rm) { var n = +rm[1], a = []; for (var i = 0; i < n; i++) a.push(i); return a; }
        if ((s[0] === '"' && s[s.length - 1] === '"') || (s[0] === "'" && s[s.length - 1] === "'")) {
            return s.slice(1, -1);
        }
        if (s[0] === '[') return parsePyList(s);
        var num = parseFloat(s);
        if (!isNaN(num) && /^-?\d+\.?\d*$/.test(s)) return num;
        return undefined;
    }

    function parsePyList(s) {
        s = s.trim();
        if (s[0] !== '[' || s[s.length - 1] !== ']') return undefined;
        s = s.slice(1, -1).trim();
        if (!s) return [];
        var items = [], depth = 0, cur = '', inStr = false, sc = '';
        for (var i = 0; i < s.length; i++) {
            var c = s[i];
            if (inStr) { cur += c; if (c === sc && s[i - 1] !== '\\') inStr = false; }
            else if (c === '"' || c === "'") { inStr = true; sc = c; cur += c; }
            else if (c === '[' || c === '(') { depth++; cur += c; }
            else if (c === ']' || c === ')') { depth--; cur += c; }
            else if (c === ',' && depth === 0) { items.push(parsePyVal(cur.trim())); cur = ''; }
            else cur += c;
        }
        if (cur.trim()) items.push(parsePyVal(cur.trim()));
        return items;
    }

    function parsePyArgs(body) {
        var title = null, kwargs = {};
        var flat = body.replace(/\r/g, '');
        var buf = '', depth = 0, inStr = false, sc = '', tokens = [];
        for (var i = 0; i < flat.length; i++) {
            var c = flat[i];
            if (inStr) { buf += c; if (c === sc && flat[i - 1] !== '\\') inStr = false; }
            else if (c === '"' || c === "'") { inStr = true; sc = c; buf += c; }
            else if (c === '[' || c === '(') { depth++; buf += c; }
            else if (c === ']' || c === ')') { depth--; buf += c; }
            else if ((c === ',' || c === '\n') && depth === 0) {
                var t = buf.trim(); if (t) tokens.push(t); buf = '';
            } else buf += c;
        }
        if (buf.trim()) tokens.push(buf.trim());
        tokens.forEach(function (tok, idx) {
            tok = tok.replace(/,$/, '').trim();
            var eq = tok.indexOf('=');
            if (eq === -1) { if (idx === 0 && title === null) title = parsePyVal(tok); }
            else {
                var key = tok.slice(0, eq).trim();
                var val = parsePyVal(tok.slice(eq + 1).trim());
                if (val !== undefined) kwargs[key] = val;
            }
        });
        return { title: title, kwargs: kwargs };
    }

    function callWasm(typeKey, body) {
        var sp = window.SeraplotWASM;
        if (!sp || !sp.__ready) return null;
        var map = CHART_MAP[typeKey];
        if (!map) return null;
        var fn = sp[map.fn];
        if (!fn) return null;
        var parsed = parsePyArgs(body);
        var title = parsed.title || "";
        var kw = parsed.kwargs;
        if (map.al) Object.keys(map.al).forEach(function (from) {
            var to = map.al[from];
            if (kw[from] !== undefined && kw[to] === undefined) kw[to] = kw[from];
        });
        var used = {};
        map.p.forEach(function (k) { used[k] = true; });
        var opts = {};
        Object.keys(kw).forEach(function (k) { if (!used[k]) opts[k] = kw[k]; });
        if (map.al) Object.keys(map.al).forEach(function (k) { delete opts[k]; });
        try {
            if (map.sp === 'gauge') return fn(title, kw.value || 0, J(opts));
            if (map.sp === 'area') {
                return fn(title, J(kw.xlabels || kw.x || []), J(kw.series || (kw.y ? [kw.y] : [])), J(opts));
            }
            if (map.sp === 'bar3d') {
                var labels = kw.labels || kw.x || [];
                var series = kw.series || [];
                var names = kw.series_names || series.map(function (_, i) { return String(i); });
                delete opts.series_names;
                var xs = [], ys = [], zs = [];
                series.forEach(function (row, i) {
                    (row || []).forEach(function (v, j) { xs.push(names[i] || String(i)); ys.push(labels[j] || String(j)); zs.push(v); });
                });
                return fn(title, J(xs), J(ys), J(zs), J(opts));
            }
            if (map.sp === 'heatmap') {
                var rows = kw.labels || kw.rows || [];
                var matrix = kw.matrix || kw.values || [];
                delete opts.rows; delete opts.cols;
                return fn(title, J(rows), J(matrix), J(opts));
            }
            var args = [title];
            map.p.forEach(function (key) { args.push(J(kw[key] !== undefined ? kw[key] : [])); });
            args.push(J(opts));
            return fn.apply(null, args);
        } catch (e) { return null; }
    }

    function streamingFrames(body) {
        var sp = window.SeraplotWASM;
        if (!sp || !sp.__ready) return [];
        var parsed = parsePyArgs(body);
        var kw = parsed.kwargs;
        var title = kw.title || "Live Stream";
        var labels = kw.labels || ["Jan", "Feb", "Mar", "Apr", "May", "Jun"];
        var base = kw.base || [42, 58, 73, 61, 89, 97];
        var theme = kw.theme || "inferno";
        var width = kw.width || 860;
        var height = kw.height || 420;
        var frames = [];
        for (var i = 0; i < 8; i++) {
            var r = rng(i * 7 + 1);
            var vals = base.map(function (v) { return Math.round(v + (r() - 0.5) * 20); });
            try { var html = sp.buildBarChart(title, J(labels), J(vals), J({ theme: theme, width: width, height: height })); if (html) frames.push(html); } catch (e) {}
        }
        return frames;
    }

    function pageSlug() {
        var m = window.location.pathname.match(/\/([^\/]+?)(?:\.html?)?$/);
        return m ? m[1] : null;
    }

    function isChartPage() {
        return /\/charts\//.test(window.location.pathname) && pageSlug() !== "index" && pageSlug() !== null;
    }

    function getThemeBase() {
        var els = document.querySelectorAll('link[href*="/theme/"],script[src*="/theme/"]');
        for (var i = 0; i < els.length; i++) {
            var u = els[i].href || els[i].src || '';
            var m = u.match(/(.*\/theme\/)/);
            if (m) return m[1];
        }
        var depth = window.location.pathname.replace(/\/$/, '').split('/').length - 2;
        return '../'.repeat(Math.max(1, depth)) + 'theme/';
    }

    function loadCM(cb) {
        if (window.CodeMirror) { cb(); return; }
        ["https://cdnjs.cloudflare.com/ajax/libs/codemirror/5.65.17/codemirror.min.css",
         "https://cdnjs.cloudflare.com/ajax/libs/codemirror/5.65.17/theme/dracula.min.css"].forEach(function (href) {
            var l = document.createElement("link"); l.rel = "stylesheet"; l.href = href; document.head.appendChild(l);
        });
        function loadScript(src, next) { var s = document.createElement("script"); s.src = src; s.onload = next; s.onerror = next; document.head.appendChild(s); }
        loadScript("https://cdnjs.cloudflare.com/ajax/libs/codemirror/5.65.17/codemirror.min.js", function () {
            loadScript("https://cdnjs.cloudflare.com/ajax/libs/codemirror/5.65.17/mode/python/python.min.js", cb);
        });
    }

    function initWasm(cb) {
        if (_wasmReady) { cb(); return; }
        var base = getThemeBase();
        function doInit() {
            window.SeraplotWASM.__init(base + 'seraplot_bg.wasm').then(function () {
                window.SeraplotWASM.__ready = true; _wasmReady = true; cb();
            }).catch(function () { cb(); });
        }
        if (window.SeraplotWASM) { doInit(); return; }
        var s = document.createElement('script');
        s.src = base + 'seraplot-web.js';
        s.onload = doInit;
        s.onerror = function () { cb(); };
        document.head.appendChild(s);
    }

    function injectStyles() {
        if (document.getElementById('sp-pg-style')) return;
        var css = '.sp-pg-wrap{display:flex;flex-direction:column;border:1px solid rgba(120,130,200,.13);border-radius:14px;overflow:hidden;margin:2.2rem 0;background:#07091a;font-family:sans-serif;box-shadow:0 8px 40px rgba(0,0,0,.45)}'
            + '.sp-pg-hdr{display:flex;align-items:center;gap:10px;padding:9px 16px;background:linear-gradient(90deg,#0c1228 0%,#0e1530 100%);border-bottom:1px solid rgba(120,130,200,.1)}'
            + '.sp-pg-title{font-size:10px;font-weight:800;letter-spacing:.18em;color:#4a5280;text-transform:uppercase;flex-shrink:0}'
            + '.sp-pg-fn-sel{background:#111828;color:#c9d1ff;border:1px solid rgba(120,130,200,.18);border-radius:7px;padding:4px 12px;font-size:13px;cursor:pointer;max-width:170px;outline:none;transition:border-color .2s}'
            + '.sp-pg-fn-sel:hover,.sp-pg-fn-sel:focus{border-color:rgba(120,130,200,.4)}'
            + '.sp-pg-conn{display:flex;align-items:center;gap:7px;margin-left:auto;font-size:11.5px;color:#4a5280;letter-spacing:.04em}'
            + '.sp-pg-dot{width:7px;height:7px;border-radius:50%;background:#333;flex-shrink:0;transition:background .3s}'
            + '.sp-wasm-loading{background:#f59e0b!important;box-shadow:0 0 8px #f59e0b88;animation:sp-pulse .9s infinite}'
            + '.sp-wasm-ready{background:#22c55e!important;box-shadow:0 0 7px #22c55e66}'
            + '.sp-wasm-err{background:#ef4444!important;box-shadow:0 0 7px #ef444466}'
            + '@keyframes sp-pulse{0%,100%{opacity:1}50%{opacity:.3}}'
            + '@keyframes sp-spin{to{transform:rotate(360deg)}}'
            + '@keyframes sp-fadein{from{opacity:0}to{opacity:1}}'
            + '.sp-pg-main{display:flex;min-height:360px;position:relative}'
            + '.sp-pg-ecol{width:48%;min-width:160px;max-width:80%;display:flex;flex-direction:column;flex-shrink:0}'
            + '.sp-pg-divider{width:5px;background:rgba(120,130,200,.07);cursor:col-resize;flex-shrink:0;transition:background .15s;position:relative;z-index:2}'
            + '.sp-pg-divider:hover,.sp-pg-divider.sp-dragging{background:rgba(120,130,200,.28)}'
            + '.sp-pg-head,.sp-pg-tail{font-family:"Fira Code","Consolas",monospace;font-size:12.5px;line-height:1.7;padding:0 16px;color:#3d4a70;background:#07091a;white-space:pre;user-select:none}'
            + '.sp-pg-head{padding-top:14px;border-bottom:1px solid rgba(120,130,200,.05)}.sp-pg-tail{padding-bottom:14px;border-top:1px solid rgba(120,130,200,.05)}'
            + '.sp-pg-cm-wrap{flex:1;background:#07091a}'
            + '.sp-pg-cm-wrap .CodeMirror{background:#07091a;font:13px/1.7 "Fira Code","Consolas",monospace;height:auto;min-height:80px}'
            + '.sp-pg-cm-wrap .CodeMirror-scroll{padding:2px 16px}'
            + '.sp-pg-cm-wrap .CodeMirror-gutters{display:none!important}'
            + '.sp-pg-cm-wrap .CodeMirror-cursor{border-left-color:#bd93f9}'
            + '.sp-pg-cm-wrap .CodeMirror-selected{background:rgba(189,147,249,.15)!important}'
            + '.sp-pg-cm-wrap .cm-s-dracula .cm-keyword{color:#ff79c6}'
            + '.sp-pg-cm-wrap .cm-s-dracula .cm-string{color:#f1fa8c}'
            + '.sp-pg-cm-wrap .cm-s-dracula .cm-number{color:#bd93f9}'
            + '.sp-pg-cm-wrap .cm-s-dracula .cm-def{color:#50fa7b}'
            + '.sp-pg-cm-wrap .cm-s-dracula .cm-comment{color:#3a4d6a;font-style:italic}'
            + '.sp-pg-cm-wrap .cm-s-dracula .cm-operator{color:#ff79c6}'
            + '.sp-pg-cm-wrap .cm-s-dracula .cm-variable{color:#f8f8f2}'
            + '.sp-pg-pcol{flex:1;min-width:120px;display:flex;flex-direction:column;position:relative;background:#07091a}'
            + '.sp-pg-iframe{flex:1;border:none;background:#07091a;width:100%;display:block;animation:sp-fadein .18s ease}'
            + '.sp-pg-loader{position:absolute;inset:0;display:flex;align-items:center;justify-content:center;background:#07091a;flex-direction:column;gap:14px;color:#4a5280;font-size:13px;transition:opacity .15s;z-index:1}'
            + '.sp-pg-loader.sp-hide{opacity:0;pointer-events:none}'
            + '.sp-pg-spinner{width:30px;height:30px;border:2.5px solid rgba(189,147,249,.15);border-top-color:#bd93f9;border-radius:50%;animation:sp-spin .8s linear infinite}'
            + '.sp-pg-sbar{display:flex;align-items:center;gap:8px;padding:6px 14px;background:#0a0d1e;border-top:1px solid rgba(120,130,200,.08)}'
            + '.sp-pg-sbar button{background:#111828;border:1px solid rgba(120,130,200,.15);color:#8899cc;border-radius:6px;padding:3px 11px;cursor:pointer;font-size:13px;transition:background .15s,color .15s}'
            + '.sp-pg-sbar button:hover{background:#1a2240;color:#c9d1ff}'
            + '.sp-pg-fc{font-size:11px;color:#3d4a70}';
        var st = document.createElement('style'); st.id = 'sp-pg-style'; st.textContent = css; document.head.appendChild(st);
    }

    function buildPG(root) {
        var slug = pageSlug();
        var rawTpl = (slug && TPLS[slug]) ? TPLS[slug] : DEFAULT_TPL;
        var tpl = fixTpl(rawTpl.slice());
        var cmEditor = null;
        var frames = [], frameIdx = 0;
        var streaming = (slug === 'streaming');

        injectStyles();
        var opts = Object.keys(TPLS).map(function (k) {
            return '<option value="' + k + '"' + (k === slug ? ' selected' : '') + '>' + k + '</option>';
        }).join('');
        root.innerHTML = [
            '<div class="sp-pg-hdr">',
            '<span class="sp-pg-title">Playground</span>',
            '<select class="sp-pg-fn-sel">' + opts + '</select>',
            '<span class="sp-pg-conn"><span class="sp-pg-dot sp-wasm-loading"></span><span class="sp-pg-smsg">WASM\u2026</span></span>',
            '</div>',
            '<div class="sp-pg-main">',
            '<div class="sp-pg-ecol"><pre class="sp-pg-head"></pre><div class="sp-pg-cm-wrap"></div><pre class="sp-pg-tail"></pre></div>',
            '<div class="sp-pg-divider"></div>',
            '<div class="sp-pg-pcol">',
            '<div class="sp-pg-loader"><div class="sp-pg-spinner"></div><span>Initialisation\u2026</span></div>',
            '<iframe class="sp-pg-iframe" sandbox="allow-scripts" style="display:none"></iframe>',
            '<div class="sp-pg-sbar" style="display:none"><button class="sp-pg-fprev">\u2039</button><span class="sp-pg-fc">1/1</span><button class="sp-pg-fnext">\u203a</button><button class="sp-pg-fplay">\u25b6</button></div>',
            '</div></div>',
        ].join('');

        var dot = root.querySelector('.sp-pg-dot');
        var smsg = root.querySelector('.sp-pg-smsg');
        var loader = root.querySelector('.sp-pg-loader');
        var iframe = root.querySelector('.sp-pg-iframe');
        var sbar = root.querySelector('.sp-pg-sbar');
        var fcEl = root.querySelector('.sp-pg-fc');
        var headEl = root.querySelector('.sp-pg-head');
        var tailEl = root.querySelector('.sp-pg-tail');
        var cmWrap = root.querySelector('.sp-pg-cm-wrap');
        var sel = root.querySelector('.sp-pg-fn-sel');
        var fplay = root.querySelector('.sp-pg-fplay');

        headEl.textContent = tpl[0];
        tailEl.textContent = tpl[2] || '';

        function setStatus(state) {
            dot.className = 'sp-pg-dot';
            if (state === 'loading') { dot.classList.add('sp-wasm-loading'); smsg.textContent = 'WASM\u2026'; }
            else if (state === 'ready') { dot.classList.add('sp-wasm-ready'); smsg.textContent = 'pr\u00eat'; }
            else { dot.classList.add('sp-wasm-err'); smsg.textContent = 'erreur'; }
        }

        function showFrame(html) {
            loader.classList.add('sp-hide');
            iframe.style.display = 'block';
            iframe.srcdoc = html.replace('<head>', '<head><style>html,body{background:#07091a!important;margin:0}</style>');
        }

        function startPlay() {
            clearInterval(_streamTimer);
            if (fplay) fplay.textContent = '\u23f8';
            _streamTimer = setInterval(function () {
                frameIdx = (frameIdx + 1) % frames.length;
                showFrame(frames[frameIdx]);
                if (fcEl) fcEl.textContent = (frameIdx + 1) + '/' + frames.length;
            }, 600);
        }

        function run() {
            if (!_wasmReady) return;
            clearInterval(_streamTimer);
            if (fplay) fplay.textContent = '\u25b6';
            var body = cmEditor ? cmEditor.getValue() : tpl[1];
            if (streaming) {
                frames = streamingFrames(body);
                if (!frames.length) return;
                frameIdx = 0; showFrame(frames[0]);
                if (fcEl) fcEl.textContent = '1/' + frames.length;
                sbar.style.display = 'flex';
                startPlay();
            } else {
                var typeKey = sel ? sel.value : (slug || 'bar');
                var html = callWasm(typeKey, body);
                if (html) { frames = [html]; frameIdx = 0; showFrame(html); sbar.style.display = 'none'; }
            }
        }

        sel && sel.addEventListener('change', function () {
            var newSlug = sel.value;
            var newRawTpl = TPLS[newSlug] || DEFAULT_TPL;
            var newTpl = fixTpl(newRawTpl.slice());
            headEl.textContent = newTpl[0];
            tailEl.textContent = newTpl[2] || '';
            if (cmEditor) cmEditor.setValue(newTpl[1]);
            streaming = (newSlug === 'streaming');
            clearInterval(_streamTimer);
            setTimeout(run, 50);
        });

        var fprev = root.querySelector('.sp-pg-fprev');
        var fnext = root.querySelector('.sp-pg-fnext');

        var divider = root.querySelector('.sp-pg-divider');
        var ecol = root.querySelector('.sp-pg-ecol');
        divider && divider.addEventListener('mousedown', function (e) {
            e.preventDefault();
            divider.classList.add('sp-dragging');
            var startX = e.clientX, startW = ecol.offsetWidth;
            var totalW = root.querySelector('.sp-pg-main').offsetWidth;
            function onMove(ev) {
                var w = Math.max(160, Math.min(totalW - 160, startW + ev.clientX - startX));
                ecol.style.width = w + 'px'; ecol.style.flex = 'none';
            }
            function onUp() {
                divider.classList.remove('sp-dragging');
                document.removeEventListener('mousemove', onMove);
                document.removeEventListener('mouseup', onUp);
            }
            document.addEventListener('mousemove', onMove);
            document.addEventListener('mouseup', onUp);
        });
        fprev && fprev.addEventListener('click', function () {
            clearInterval(_streamTimer); if (fplay) fplay.textContent = '\u25b6';
            frameIdx = (frameIdx - 1 + frames.length) % frames.length;
            showFrame(frames[frameIdx]);
            if (fcEl) fcEl.textContent = (frameIdx + 1) + '/' + frames.length;
        });
        fnext && fnext.addEventListener('click', function () {
            clearInterval(_streamTimer); if (fplay) fplay.textContent = '\u25b6';
            frameIdx = (frameIdx + 1) % frames.length;
            showFrame(frames[frameIdx]);
            if (fcEl) fcEl.textContent = (frameIdx + 1) + '/' + frames.length;
        });
        fplay && fplay.addEventListener('click', function () {
            if (_streamTimer) { clearInterval(_streamTimer); _streamTimer = null; fplay.textContent = '\u25b6'; }
            else startPlay();
        });

        loadCM(function () {
            if (!window.CodeMirror) return;
            cmEditor = CodeMirror(cmWrap, {
                value: tpl[1], mode: "python", theme: "dracula",
                lineNumbers: false, indentUnit: 4, tabSize: 4, indentWithTabs: false,
                viewportMargin: Infinity,
                extraKeys: { Tab: function (cm) { cm.replaceSelection("    "); } }
            });
            cmEditor.on("change", function () {
                clearTimeout(_debTimer);
                _debTimer = setTimeout(run, DEBOUNCE_MS);
            });
        });

        initWasm(function () {
            if (_wasmReady) { setStatus('ready'); run(); }
            else setStatus('error');
        });
    }

    function init() {
        if (!isChartPage()) return;
        if (document.querySelector('.sp-pg-wrap')) return;
        var root = document.createElement('div');
        root.className = 'sp-pg-wrap';
        var main = document.querySelector('.content main') || document.querySelector('main') || document.body;
        main.appendChild(root);
        buildPG(root);
    }

    if (document.readyState === 'loading') document.addEventListener('DOMContentLoaded', init);
    else init();
})();
