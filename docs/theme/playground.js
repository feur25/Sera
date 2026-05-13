(function () {
    var DEBOUNCE_MS = 650;
    var _debTimer = null;
    var _wasmReady = false;
    var _skipDebounce = false;

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

    var PAGE_VARIANTS = {
        "bar": [
            { id: "basic",         label: "Basic",          body: '    "Revenue by Month",\n    labels=["Jan","Feb","Mar","Apr","May","Jun","Jul","Aug","Sep","Oct","Nov","Dec"],\n    values=[42,58,73,61,89,97,85,78,91,65,54,70],\n    variant="basic",\n    theme="deluxe",\n    width=860,\n    height=420,' },
            { id: "horizontal",    label: "Horizontal",     body: '    "Horizontal Rankings",\n    labels=["Alpha","Beta","Gamma","Delta","Epsilon","Zeta"],\n    values=[88,74,91,65,83,57],\n    variant="horizontal",\n    theme="prism",\n    width=860,\n    height=420,' },
            { id: "grouped",       label: "Grouped",        body: '    "Performance Comparison",\n    labels=["Q1","Q2","Q3","Q4"],\n    series=[[42,58,73,61],[60,45,80,55],[35,70,50,65]],\n    series_names=["Alpha","Beta","Gamma"],\n    variant="grouped",\n    theme="deluxe",\n    width=860,\n    height=440,' },
            { id: "stacked",       label: "Stacked",        body: '    "Revenue Breakdown",\n    labels=["Jan","Feb","Mar","Apr","May","Jun"],\n    series=[[30,40,35,45,50,42],[20,25,30,20,25,28],[10,15,12,18,20,15]],\n    series_names=["Product A","Product B","Product C"],\n    variant="stacked",\n    theme="prism",\n    width=860,\n    height=440,' },
            { id: "relative",      label: "Relative",       body: '    "Profit & Loss",\n    labels=["Jan","Feb","Mar","Apr","May","Jun"],\n    series=[[30,40,-10,45,50,42],[20,-25,30,20,25,-18],[10,15,12,-8,20,15]],\n    series_names=["Product A","Product B","Product C"],\n    variant="relative",\n    theme="frost",\n    width=860,\n    height=440,' },
            { id: "marimekko",     label: "Marimekko",      body: '    "Market Share",\n    series=[[40,30,20,10],[35,25,30,10]],\n    series_names=["Category A","Category B"],\n    widths=[1,2,1.5,0.8],\n    variant="marimekko",\n    theme="aurora",\n    width=860,\n    height=440,' },
            { id: "pictogram",     label: "Pictogram",      body: '    "Sales Units",\n    labels=["Alpha","Beta","Gamma","Delta"],\n    values=[50,30,70,45],\n    units_per_icon=10,\n    variant="pictogram",\n    theme="deluxe",\n    width=860,\n    height=420,' },
            { id: "multicategory", label: "Multicategory",  body: '    "Regional Sales",\n    labels=["Jan","Feb","Mar","Jan","Feb","Mar"],\n    values=[42,58,73,61,89,97],\n    super_categories=["North","North","North","South","South","South"],\n    variant="multicategory",\n    theme="prism",\n    width=860,\n    height=440,' },
            { id: "deluxe",        label: "Deluxe",         body: '    "Neon Revenue",\n    labels=["Jan","Feb","Mar","Apr","May","Jun","Jul","Aug","Sep","Oct","Nov","Dec"],\n    values=[42,58,73,61,89,97,85,78,91,65,54,70],\n    variant="deluxe",\n    theme="deluxe",\n    width=860,\n    height=420,' },
        ],
        "scatter": [
            { id: "basic",       label: "Basic",       body: '    "Correlation Study",\n    x_values=SCATTERX,\n    y_values=SCATTERY,\n    variant="basic",\n    theme="prism",\n    width=860,\n    height=480,' },
            { id: "categorical", label: "Categorical", body: '    "Grouped Points",\n    x_values=SCATTERX,\n    y_values=SCATTERY,\n    categories=["A","B","A","B","A","B","A","B","A","B","A","B","A","B","A","B","A","B","A","B"],\n    variant="categorical",\n    theme="frost",\n    width=860,\n    height=480,' },
            { id: "gradient",    label: "Gradient",   body: '    "Color Scale",\n    x_values=SCATTERX,\n    y_values=SCATTERY,\n    color_values=SCATTERY,\n    variant="gradient",\n    theme="aurora",\n    width=860,\n    height=480,' },
            { id: "symbols",     label: "Symbols",    body: '    "Shape Groups",\n    x_values=SCATTERX,\n    y_values=SCATTERY,\n    categories=["A","B","C","A","B","C","A","B","C","A","B","C","A","B","C","A","B","C","A","B"],\n    variant="symbols",\n    theme="prism",\n    width=860,\n    height=480,' },
            { id: "labeled",     label: "Labeled",    body: '    "Labeled Points",\n    x_values=SCATTERX,\n    y_values=SCATTERY,\n    labels=["P1","P2","P3","P4","P5","P6","P7","P8","P9","P10","P11","P12","P13","P14","P15","P16","P17","P18","P19","P20"],\n    variant="labeled",\n    theme="frost",\n    width=860,\n    height=480,' },
            { id: "regression",  label: "Regression", body: '    "Trend Line",\n    x_values=SCATTERX,\n    y_values=SCATTERY,\n    regression_type="linear",\n    variant="regression",\n    theme="deluxe",\n    width=860,\n    height=480,' },
            { id: "deluxe",      label: "Deluxe",     body: '    "Stellar Points",\n    x_values=SCATTERX,\n    y_values=SCATTERY,\n    variant="deluxe",\n    theme="deluxe",\n    width=860,\n    height=480,' },
            { id: "galaxy",      label: "Galaxy",     body: '    "Galaxy Scatter",\n    x_values=SCATTERX,\n    y_values=SCATTERY,\n    variant="galaxy",\n    theme="deluxe",\n    width=860,\n    height=480,' },
            { id: "nova",        label: "Nova",       body: '    "Nova Scatter",\n    x_values=SCATTERX,\n    y_values=SCATTERY,\n    variant="nova",\n    theme="deluxe",\n    width=860,\n    height=480,' },
        ],
        "line": [
            { id: "basic",             label: "Basic",             body: '    "Trend Analysis",\n    labels=["Jan","Feb","Mar","Apr","May","Jun","Jul","Aug","Sep","Oct","Nov","Dec"],\n    values=[12,19,3,17,28,24,38,35,45,41,52,60],\n    variant="basic",\n    theme="frost",\n    width=860,\n    height=420,' },
            { id: "multi",             label: "Multi",             body: '    "Series Comparison",\n    x_labels=["Jan","Feb","Mar","Apr","May","Jun","Jul","Aug","Sep","Oct","Nov","Dec"],\n    series=[[12,19,3,17,28,24,38,35,45,41,52,60],[8,15,25,13,21,30,27,32,38,29,44,51]],\n    series_names=["Alpha","Beta"],\n    variant="multi",\n    theme="deluxe",\n    width=860,\n    height=420,' },
            { id: "stepped",           label: "Stepped",           body: '    "Step Signal",\n    labels=["00:00","04:00","08:00","12:00","16:00","20:00","24:00"],\n    values=[2,2,5,8,8,3,1],\n    variant="stepped",\n    theme="aurora",\n    width=860,\n    height=420,' },
            { id: "spline",            label: "Spline",            body: '    "Smooth Curve",\n    labels=["Jan","Feb","Mar","Apr","May","Jun","Jul","Aug","Sep","Oct","Nov","Dec"],\n    values=[12,19,3,17,28,24,38,35,45,41,52,60],\n    variant="spline",\n    theme="prism",\n    width=860,\n    height=420,' },
            { id: "filled",            label: "Filled",            body: '    "Area Fill",\n    labels=["Jan","Feb","Mar","Apr","May","Jun","Jul","Aug","Sep","Oct","Nov","Dec"],\n    values=[12,19,3,17,28,24,38,35,45,41,52,60],\n    variant="filled",\n    theme="frost",\n    width=860,\n    height=420,' },
            { id: "dashed",            label: "Dashed",            body: '    "Dashed Line",\n    labels=["Jan","Feb","Mar","Apr","May","Jun","Jul","Aug","Sep","Oct","Nov","Dec"],\n    values=[12,19,3,17,28,24,38,35,45,41,52,60],\n    variant="dashed",\n    theme="deluxe",\n    width=860,\n    height=420,' },
            { id: "connected_scatter", label: "Connected Scatter", body: '    "Points + Line",\n    labels=["Jan","Feb","Mar","Apr","May","Jun","Jul","Aug","Sep","Oct","Nov","Dec"],\n    values=[12,19,3,17,28,24,38,35,45,41,52,60],\n    marker_size=6,\n    variant="connected_scatter",\n    theme="aurora",\n    width=860,\n    height=420,' },
        ],
        "histogram": [
            { id: "basic",      label: "Basic",      body: '    "Distribution",\n    values=HIST,\n    bins=20,\n    variant="basic",\n    theme="prism",\n    width=860,\n    height=420,' },
            { id: "horizontal", label: "Horizontal", body: '    "Horizontal Distribution",\n    values=HIST,\n    bins=20,\n    variant="horizontal",\n    theme="frost",\n    width=860,\n    height=420,' },
            { id: "normalized", label: "Normalized", body: '    "Density",\n    values=HIST,\n    bins=20,\n    variant="normalized",\n    theme="aurora",\n    width=860,\n    height=420,' },
            { id: "cumulative", label: "Cumulative", body: '    "CDF",\n    values=HIST,\n    bins=20,\n    variant="cumulative",\n    theme="prism",\n    width=860,\n    height=420,' },
            { id: "step",       label: "Step",       body: '    "Step Outline",\n    values=HIST,\n    bins=20,\n    variant="step",\n    theme="deluxe",\n    width=860,\n    height=420,' },
            { id: "deluxe",     label: "Deluxe",     body: '    "Neon Distribution",\n    values=HIST,\n    bins=20,\n    variant="deluxe",\n    theme="deluxe",\n    width=860,\n    height=420,' },
        ],
        "violin": [
            { id: "basic",    label: "Basic",    body: '    "Score Distribution",\n    labels=["Alpha","Beta","Gamma"],\n    values=VIO,\n    variant="basic",\n    theme="prism",\n    width=860,\n    height=480,' },
            { id: "box",      label: "Box",      body: '    "Violin + Box",\n    labels=["Alpha","Beta","Gamma"],\n    values=VIO,\n    variant="box",\n    theme="deluxe",\n    width=860,\n    height=480,' },
            { id: "quartile", label: "Quartile", body: '    "Quartile Lines",\n    labels=["Alpha","Beta","Gamma"],\n    values=VIO,\n    variant="quartile",\n    theme="frost",\n    width=860,\n    height=480,' },
            { id: "mean",     label: "Mean",     body: '    "Mean Line",\n    labels=["Alpha","Beta","Gamma"],\n    values=VIO,\n    variant="mean",\n    theme="aurora",\n    width=860,\n    height=480,' },
            { id: "points",   label: "Points",   body: '    "Points Jitter",\n    labels=["Alpha","Beta","Gamma"],\n    values=VIO,\n    variant="points",\n    theme="prism",\n    width=860,\n    height=480,' },
        ],
        "boxplot": [
            { id: "basic",      label: "Basic",      body: '    "Statistical Summary",\n    labels=["Q1","Q2","Q3","Q4"],\n    values=BOX,\n    variant="basic",\n    theme="aurora",\n    width=860,\n    height=480,' },
            { id: "horizontal", label: "Horizontal", body: '    "Horizontal Boxes",\n    labels=["Q1","Q2","Q3","Q4"],\n    values=BOX,\n    variant="horizontal",\n    theme="frost",\n    width=860,\n    height=480,' },
            { id: "notched",    label: "Notched",    body: '    "Confidence Intervals",\n    labels=["Q1","Q2","Q3","Q4"],\n    values=BOX,\n    variant="notched",\n    theme="prism",\n    width=860,\n    height=480,' },
            { id: "strip",      label: "Strip",      body: '    "Strip Plot",\n    labels=["Q1","Q2","Q3","Q4"],\n    values=BOX,\n    variant="strip",\n    theme="aurora",\n    width=860,\n    height=480,' },
        ],
        "radar": [
            { id: "basic",     label: "Basic",     body: '    "Skills Matrix",\n    axes=["Speed","Agility","Strength","Defense","Magic","Stamina"],\n    series=[[85,72,60,91,78,88],[60,55,95,88,45,75]],\n    series_names=["Hero","Warrior"],\n    variant="basic",\n    theme="deluxe",\n    width=620,\n    height=520,' },
            { id: "lines",     label: "Lines",     body: '    "Outline Radar",\n    axes=["Speed","Agility","Strength","Defense","Magic","Stamina"],\n    series=[[85,72,60,91,78,88],[60,55,95,88,45,75]],\n    series_names=["Hero","Warrior"],\n    variant="lines",\n    theme="prism",\n    width=620,\n    height=520,' },
            { id: "filled",    label: "Filled",    body: '    "Filled Radar",\n    axes=["Speed","Agility","Strength","Defense","Magic","Stamina"],\n    series=[[85,72,60,91,78,88],[60,55,95,88,45,75]],\n    series_names=["Hero","Warrior"],\n    variant="filled",\n    theme="aurora",\n    width=620,\n    height=520,' },
            { id: "dashed",    label: "Dashed",    body: '    "Dashed Radar",\n    axes=["Speed","Agility","Strength","Defense","Magic","Stamina"],\n    series=[[85,72,60,91,78,88],[60,55,95,88,45,75]],\n    series_names=["Hero","Warrior"],\n    variant="dashed",\n    theme="frost",\n    width=620,\n    height=520,' },
            { id: "polar_bar", label: "Polar Bar", body: '    "Radial Bars",\n    axes=["Speed","Agility","Strength","Defense","Magic","Stamina"],\n    series=[[85,72,60,91,78,88]],\n    series_names=["Hero"],\n    variant="polar_bar",\n    theme="deluxe",\n    width=620,\n    height=520,' },
            { id: "gradient",  label: "Gradient",  body: '    "Gradient Radar",\n    axes=["Speed","Agility","Strength","Defense","Magic","Stamina"],\n    series=[[85,72,60,91,78,88]],\n    series_names=["Hero"],\n    variant="gradient",\n    theme="aurora",\n    width=620,\n    height=520,' },
        ],
        "heatmap": [
            { id: "basic",       label: "Basic",       body: '    "Correlation Matrix",\n    rows=["A","B","C","D","E"],\n    cols=["V1","V2","V3","V4","V5"],\n    values=[[0.9,0.4,0.2,0.7,0.3],[0.4,1.0,0.6,0.1,0.8],[0.2,0.6,0.9,0.5,0.4],[0.7,0.1,0.5,1.0,0.2],[0.3,0.8,0.4,0.2,1.0]],\n    variant="basic",\n    theme="frost",\n    width=700,\n    height=560,' },
            { id: "annotated",   label: "Annotated",   body: '    "Annotated Heatmap",\n    rows=["A","B","C","D","E"],\n    cols=["V1","V2","V3","V4","V5"],\n    values=[[0.9,0.4,0.2,0.7,0.3],[0.4,1.0,0.6,0.1,0.8],[0.2,0.6,0.9,0.5,0.4],[0.7,0.1,0.5,1.0,0.2],[0.3,0.8,0.4,0.2,1.0]],\n    show_values=True,\n    variant="annotated",\n    theme="frost",\n    width=700,\n    height=560,' },
            { id: "correlation", label: "Correlation", body: '    "Correlation",\n    rows=["A","B","C","D","E"],\n    cols=["A","B","C","D","E"],\n    values=[[1.0,0.4,0.2,0.7,0.3],[0.4,1.0,0.6,-0.1,0.8],[0.2,0.6,1.0,0.5,-0.4],[0.7,-0.1,0.5,1.0,0.2],[0.3,0.8,-0.4,0.2,1.0]],\n    variant="correlation",\n    theme="prism",\n    width=700,\n    height=560,' },
            { id: "log",         label: "Log Scale",   body: '    "Log Heatmap",\n    rows=["A","B","C","D","E"],\n    cols=["V1","V2","V3","V4","V5"],\n    values=[[900,4,2,700,3],[40,1000,600,1,800],[2,60,900,50,4],[700,1,50,1000,2],[3,800,4,2,1000]],\n    variant="log",\n    theme="aurora",\n    width=700,\n    height=560,' },
        ],
        "bubble": [
            { id: "basic",       label: "Basic",       body: '    "Market Positioning",\n    x_values=[1.2,2.4,3.8,5.1,7.0,8.5],\n    y_values=[4.5,6.2,3.1,8.7,5.5,9.2],\n    sizes=[20,40,15,60,35,25],\n    variant="basic",\n    theme="aurora",\n    width=860,\n    height=480,' },
            { id: "categorical", label: "Categorical", body: '    "Grouped Bubbles",\n    x_values=[1.2,2.4,3.8,5.1,7.0,8.5],\n    y_values=[4.5,6.2,3.1,8.7,5.5,9.2],\n    sizes=[20,40,15,60,35,25],\n    categories=["A","B","A","B","A","B"],\n    variant="categorical",\n    theme="frost",\n    width=860,\n    height=480,' },
            { id: "labeled",     label: "Labeled",     body: '    "Labeled Bubbles",\n    x_values=[1.2,2.4,3.8,5.1,7.0,8.5],\n    y_values=[4.5,6.2,3.1,8.7,5.5,9.2],\n    sizes=[20,40,15,60,35,25],\n    labels=["Alpha","Beta","Gamma","Delta","Epsilon","Zeta"],\n    variant="labeled",\n    theme="prism",\n    width=860,\n    height=480,' },
            { id: "deluxe",      label: "Deluxe",      body: '    "Iridescent Bubbles",\n    x_values=[1.2,2.4,3.8,5.1,7.0,8.5],\n    y_values=[4.5,6.2,3.1,8.7,5.5,9.2],\n    sizes=[20,40,15,60,35,25],\n    variant="deluxe",\n    theme="deluxe",\n    width=860,\n    height=480,' },
            { id: "plasma",      label: "Plasma",      body: '    "Plasma Bubbles",\n    x_values=[1.2,2.4,3.8,5.1,7.0,8.5],\n    y_values=[4.5,6.2,3.1,8.7,5.5,9.2],\n    sizes=[20,40,15,60,35,25],\n    variant="plasma",\n    theme="deluxe",\n    width=860,\n    height=480,' },
        ],
        "pie": [
            { id: "basic",    label: "Basic",    body: '    "Market Share",\n    labels=["Product A","Product B","Product C","Other"],\n    values=[38,27,21,14],\n    variant="basic",\n    theme="aurora",\n    width=700,\n    height=520,' },
            { id: "donut",    label: "Donut",    body: '    "Category Split",\n    labels=["Alpha","Beta","Gamma","Delta"],\n    values=[35,28,22,15],\n    variant="donut",\n    theme="prism",\n    width=700,\n    height=520,' },
            { id: "exploded", label: "Exploded", body: '    "Pulled Slice",\n    labels=["Product A","Product B","Product C","Other"],\n    values=[38,27,21,14],\n    pull=[0.1,0,0,0],\n    variant="exploded",\n    theme="frost",\n    width=700,\n    height=520,' },
        ],
        "funnel": [
            { id: "basic",      label: "Basic",      body: '    "Conversion Funnel",\n    stages=["Impressions","Clicks","Signups","Trials","Paid"],\n    values=[10000,3200,850,240,96],\n    variant="basic",\n    theme="aurora",\n    width=700,\n    height=520,' },
            { id: "stepped",    label: "Stepped",    body: '    "Step Funnel",\n    stages=["Impressions","Clicks","Signups","Trials","Paid"],\n    values=[10000,3200,850,240,96],\n    variant="stepped",\n    theme="prism",\n    width=700,\n    height=520,' },
            { id: "chevron",    label: "Chevron",    body: '    "Sales Pipeline",\n    stages=["Impressions","Clicks","Signups","Trials","Paid"],\n    values=[10000,3200,850,240,96],\n    variant="chevron",\n    theme="frost",\n    width=700,\n    height=520,' },
            { id: "pyramid",    label: "Pyramid",    body: '    "Pyramid",\n    stages=["Level 1","Level 2","Level 3","Level 4","Level 5"],\n    values=[10000,3200,850,240,96],\n    variant="pyramid",\n    theme="deluxe",\n    width=700,\n    height=520,' },
            { id: "conversion", label: "Conversion", body: '    "KPI Funnel",\n    stages=["Impressions","Clicks","Signups","Trials","Paid"],\n    values=[10000,3200,850,240,96],\n    variant="conversion",\n    theme="aurora",\n    width=700,\n    height=520,' },
        ],
        "treemap": [
            { id: "basic",    label: "Basic",    body: '    "Portfolio Allocation",\n    labels=["Tech","Finance","Health","Energy","Consumer","Materials"],\n    values=[340,220,180,150,130,90],\n    variant="basic",\n    theme="prism",\n    width=860,\n    height=520,' },
            { id: "flat",     label: "Flat",     body: '    "Mosaic",\n    labels=["Tech","Finance","Health","Energy","Consumer","Materials"],\n    values=[340,220,180,150,130,90],\n    variant="flat",\n    theme="frost",\n    width=860,\n    height=520,' },
            { id: "gradient", label: "Gradient", body: '    "Gradient Tiles",\n    labels=["Tech","Finance","Health","Energy","Consumer","Materials"],\n    values=[340,220,180,150,130,90],\n    variant="gradient",\n    theme="aurora",\n    width=860,\n    height=520,' },
            { id: "heat",     label: "Heatmap",  body: '    "Value Heatmap",\n    labels=["Tech","Finance","Health","Energy","Consumer","Materials"],\n    values=[340,220,180,150,130,90],\n    variant="heat",\n    theme="prism",\n    width=860,\n    height=520,' },
            { id: "mono",     label: "Mono",     body: '    "Monochrome",\n    labels=["Tech","Finance","Health","Energy","Consumer","Materials"],\n    values=[340,220,180,150,130,90],\n    variant="mono",\n    theme="deluxe",\n    width=860,\n    height=520,' },
        ],
        "sunburst": [
            { id: "basic",   label: "Basic",   body: '    "Hierarchy",\n    labels=["Root","A","B","A1","A2","B1","B2"],\n    parents=["","Root","Root","A","A","B","B"],\n    values=[0,60,40,35,25,22,18],\n    variant="basic",\n    theme="aurora",\n    width=700,\n    height=700,' },
            { id: "donut",   label: "Donut",   body: '    "Donut Sunburst",\n    labels=["Root","A","B","A1","A2","B1","B2"],\n    parents=["","Root","Root","A","A","B","B"],\n    values=[0,60,40,35,25,22,18],\n    variant="donut",\n    theme="prism",\n    width=700,\n    height=700,' },
            { id: "flame",   label: "Flame",   body: '    "Flame Sunburst",\n    labels=["Root","A","B","A1","A2","B1","B2"],\n    parents=["","Root","Root","A","A","B","B"],\n    values=[0,60,40,35,25,22,18],\n    variant="flame",\n    theme="deluxe",\n    width=700,\n    height=700,' },
            { id: "rainbow", label: "Rainbow", body: '    "Rainbow Sunburst",\n    labels=["Root","A","B","A1","A2","B1","B2"],\n    parents=["","Root","Root","A","A","B","B"],\n    values=[0,60,40,35,25,22,18],\n    variant="rainbow",\n    theme="aurora",\n    width=700,\n    height=700,' },
        ],
        "waterfall": [
            { id: "basic",      label: "Basic",      body: '    "Cash Flow",\n    labels=["Start","Revenue","Costs","Taxes","R&D","End"],\n    values=[100,250,-180,-40,-30,100],\n    variant="basic",\n    theme="prism",\n    width=860,\n    height=460,' },
            { id: "stepped",    label: "Stepped",    body: '    "Staircase",\n    labels=["Start","Revenue","Costs","Taxes","R&D","End"],\n    values=[100,250,-180,-40,-30,100],\n    variant="stepped",\n    theme="frost",\n    width=860,\n    height=460,' },
            { id: "lollipop",   label: "Lollipop",   body: '    "Lollipop Waterfall",\n    labels=["Start","Revenue","Costs","Taxes","R&D","End"],\n    values=[100,250,-180,-40,-30,100],\n    variant="lollipop",\n    theme="aurora",\n    width=860,\n    height=460,' },
            { id: "arrowed",    label: "Arrowed",    body: '    "Directional",\n    labels=["Start","Revenue","Costs","Taxes","R&D","End"],\n    values=[100,250,-180,-40,-30,100],\n    variant="arrowed",\n    theme="deluxe",\n    width=860,\n    height=460,' },
            { id: "horizontal", label: "Horizontal", body: '    "Horizontal Flow",\n    labels=["Start","Revenue","Costs","Taxes","R&D","End"],\n    values=[100,250,-180,-40,-30,100],\n    variant="horizontal",\n    theme="prism",\n    width=860,\n    height=460,' },
        ],
        "ridgeline": [
            { id: "basic",     label: "Basic",     body: '    "Distribution Ridgeline",\n    groups=["2020","2021","2022","2023","2024"],\n    values=RID,\n    variant="basic",\n    theme="prism",\n    width=860,\n    height=560,' },
            { id: "gradient",  label: "Gradient",  body: '    "Gradient Ridges",\n    groups=["2020","2021","2022","2023","2024"],\n    values=RID,\n    variant="gradient",\n    theme="aurora",\n    width=860,\n    height=560,' },
            { id: "lines",     label: "Lines",     body: '    "Outline Ridges",\n    groups=["2020","2021","2022","2023","2024"],\n    values=RID,\n    variant="lines",\n    theme="frost",\n    width=860,\n    height=560,' },
            { id: "quartiles", label: "Quartiles", body: '    "Quartile Markers",\n    groups=["2020","2021","2022","2023","2024"],\n    values=RID,\n    variant="quartiles",\n    theme="deluxe",\n    width=860,\n    height=560,' },
            { id: "heatmap",   label: "Heatmap",   body: '    "Heatmap Ridges",\n    groups=["2020","2021","2022","2023","2024"],\n    values=RID,\n    variant="heatmap",\n    theme="prism",\n    width=860,\n    height=560,' },
        ],
        "lollipop": [
            { id: "basic",     label: "Basic",     body: '    "Top Values",\n    labels=["Alpha","Beta","Gamma","Delta","Epsilon","Zeta"],\n    values=[88,74,91,65,83,57],\n    variant="basic",\n    theme="frost",\n    width=860,\n    height=440,' },
            { id: "cleveland", label: "Cleveland", body: '    "Cleveland Dot Plot",\n    labels=["Alpha","Beta","Gamma","Delta","Epsilon","Zeta"],\n    values=[88,74,91,65,83,57],\n    variant="cleveland",\n    theme="aurora",\n    width=860,\n    height=440,' },
            { id: "diverging", label: "Diverging", body: '    "Deviation Plot",\n    labels=["Alpha","Beta","Gamma","Delta","Epsilon","Zeta"],\n    values=[88,74,91,65,83,57],\n    variant="diverging",\n    theme="prism",\n    width=860,\n    height=440,' },
            { id: "circular",  label: "Circular",  body: '    "Radial Lollipop",\n    labels=["Alpha","Beta","Gamma","Delta","Epsilon","Zeta"],\n    values=[88,74,91,65,83,57],\n    variant="circular",\n    theme="deluxe",\n    width=640,\n    height=640,' },
            { id: "highlight", label: "Highlight", body: '    "Spotlight",\n    labels=["Alpha","Beta","Gamma","Delta","Epsilon","Zeta"],\n    values=[88,74,91,65,83,57],\n    variant="highlight",\n    theme="frost",\n    width=860,\n    height=440,' },
        ],
        "kde": [
            { id: "basic",      label: "Basic",      body: '    "Density Estimate",\n    values=KDE,\n    variant="basic",\n    theme="deluxe",\n    width=860,\n    height=440,' },
            { id: "outline",    label: "Outline",    body: '    "Outline Density",\n    values=KDE,\n    variant="outline",\n    theme="prism",\n    width=860,\n    height=440,' },
            { id: "rug",        label: "Rug",        body: '    "Rug + KDE",\n    values=KDE,\n    variant="rug",\n    theme="aurora",\n    width=860,\n    height=440,' },
            { id: "histogram",  label: "Histogram",  body: '    "KDE on Histogram",\n    values=KDE,\n    variant="histogram",\n    theme="frost",\n    width=860,\n    height=440,' },
            { id: "cumulative", label: "Cumulative", body: '    "CDF",\n    values=KDE,\n    variant="cumulative",\n    theme="deluxe",\n    width=860,\n    height=440,' },
            { id: "gradient",   label: "Gradient",   body: '    "Gradient Fill",\n    values=KDE,\n    variant="gradient",\n    theme="prism",\n    width=860,\n    height=440,' },
        ],
        "slope": [
            { id: "basic",       label: "Basic",       body: '    "Before vs After",\n    labels=["Alpha","Beta","Gamma","Delta"],\n    before=[42,78,55,90],\n    after=[65,61,82,74],\n    variant="basic",\n    theme="deluxe",\n    width=600,\n    height=500,' },
            { id: "monochrome",  label: "Monochrome",  body: '    "Monochrome Slope",\n    labels=["Alpha","Beta","Gamma","Delta"],\n    before=[42,78,55,90],\n    after=[65,61,82,74],\n    variant="monochrome",\n    theme="frost",\n    width=600,\n    height=500,' },
            { id: "highlighted", label: "Highlighted", body: '    "Spotlight Slope",\n    labels=["Alpha","Beta","Gamma","Delta"],\n    before=[42,78,55,90],\n    after=[65,61,82,74],\n    variant="highlighted",\n    theme="prism",\n    width=600,\n    height=500,' },
            { id: "bumps",       label: "Bumps",       body: '    "Rank Changes",\n    labels=["Alpha","Beta","Gamma","Delta"],\n    before=[1,4,3,2],\n    after=[2,3,1,4],\n    variant="bumps",\n    theme="aurora",\n    width=600,\n    height=500,' },
            { id: "curved",      label: "Curved",      body: '    "Smooth Slope",\n    labels=["Alpha","Beta","Gamma","Delta"],\n    before=[42,78,55,90],\n    after=[65,61,82,74],\n    variant="curved",\n    theme="deluxe",\n    width=600,\n    height=500,' },
        ],
        "bullet": [
            { id: "basic",    label: "Basic",    body: '    "KPI Dashboard",\n    labels=["Revenue","Satisfaction","Leads","Retention"],\n    actuals=[82,74,91,65],\n    targets=[90,80,85,70],\n    variant="basic",\n    theme="aurora",\n    width=860,\n    height=380,' },
            { id: "stacked",  label: "Stacked",  body: '    "Zones KPI",\n    labels=["Revenue","Satisfaction","Leads","Retention"],\n    actuals=[82,74,91,65],\n    targets=[90,80,85,70],\n    variant="stacked",\n    theme="prism",\n    width=860,\n    height=380,' },
            { id: "thermo",   label: "Thermo",   body: '    "Thermometer",\n    labels=["Revenue","Satisfaction","Leads","Retention"],\n    actuals=[82,74,91,65],\n    targets=[90,80,85,70],\n    variant="thermo",\n    theme="deluxe",\n    width=860,\n    height=380,' },
            { id: "progress", label: "Progress", body: '    "Progress Pills",\n    labels=["Revenue","Satisfaction","Leads","Retention"],\n    actuals=[82,74,91,65],\n    targets=[90,80,85,70],\n    variant="progress",\n    theme="aurora",\n    width=860,\n    height=380,' },
        ],
        "parallel": [
            { id: "basic",       label: "Basic",       body: '    "Multi-Attribute Analysis",\n    axes=["Speed","Power","Agility","Defense"],\n    series=[[82,74,91,65],[55,88,60,78],[70,65,75,82],[90,50,85,58]],\n    series_names=["A","B","C","D"],\n    variant="basic",\n    theme="frost",\n    width=860,\n    height=460,' },
            { id: "smooth",      label: "Smooth",      body: '    "Smooth Parallel",\n    axes=["Speed","Power","Agility","Defense"],\n    series=[[82,74,91,65],[55,88,60,78],[70,65,75,82],[90,50,85,58]],\n    series_names=["A","B","C","D"],\n    variant="smooth",\n    theme="aurora",\n    width=860,\n    height=460,' },
            { id: "categorical", label: "Categorical", body: '    "Color by Group",\n    axes=["Speed","Power","Agility","Defense"],\n    series=[[82,74,91,65],[55,88,60,78],[70,65,75,82],[90,50,85,58]],\n    series_names=["A","B","C","D"],\n    variant="categorical",\n    theme="prism",\n    width=860,\n    height=460,' },
            { id: "highlight",   label: "Highlight",   body: '    "Spotlight Series",\n    axes=["Speed","Power","Agility","Defense"],\n    series=[[82,74,91,65],[55,88,60,78],[70,65,75,82],[90,50,85,58]],\n    series_names=["A","B","C","D"],\n    variant="highlight",\n    theme="deluxe",\n    width=860,\n    height=460,' },
        ],
        "gauge": [
            { id: "basic",     label: "Basic",     body: '    "KPI Score",\n    value=72,\n    min_val=0,\n    max_val=100,\n    variant="basic",\n    theme="deluxe",\n    width=500,\n    height=400,' },
            { id: "radial",    label: "Radial",    body: '    "Donut Gauge",\n    value=72,\n    min_val=0,\n    max_val=100,\n    variant="radial",\n    theme="aurora",\n    width=500,\n    height=400,' },
            { id: "arc270",    label: "Arc 270\u00b0", body: '    "Wide Arc",\n    value=72,\n    min_val=0,\n    max_val=100,\n    variant="arc270",\n    theme="prism",\n    width=500,\n    height=400,' },
            { id: "sleek",     label: "Sleek",     body: '    "Clean KPI",\n    value=72,\n    min_val=0,\n    max_val=100,\n    variant="sleek",\n    theme="frost",\n    width=500,\n    height=400,' },
            { id: "segmented", label: "Segmented", body: '    "Battery Gauge",\n    value=72,\n    min_val=0,\n    max_val=100,\n    variant="segmented",\n    theme="deluxe",\n    width=500,\n    height=400,' },
            { id: "glow",      label: "Glow",      body: '    "Neon Gauge",\n    value=72,\n    min_val=0,\n    max_val=100,\n    variant="glow",\n    theme="deluxe",\n    width=500,\n    height=400,' },
        ],
        "choropleth": [
            { id: "basic", label: "Basic", body: '    "Country Metric",\n    countries=["USA","GBR","FRA","DEU","CHN","IND","BRA"],\n    values=[88,74,82,79,91,65,70],\n    theme="aurora",\n    width=860,\n    height=520,' },
        ],
        "scatter3d": [
            { id: "basic", label: "Basic", body: '    "3D Distribution",\n    x=S3X,\n    y=S3Y,\n    z=S3Z,\n    theme="deluxe",\n    width=860,\n    height=560,' },
        ],
        "bar3d": [
            { id: "basic", label: "Basic", body: '    "3D Overview",\n    labels=["Q1","Q2","Q3","Q4"],\n    series=[[42,58,73,61],[60,45,80,55]],\n    series_names=["Alpha","Beta"],\n    theme="inferno",\n    width=860,\n    height=560,' },
        ],
        "bubble3d": [
            { id: "basic", label: "Basic", body: '    "3D Bubbles",\n    x=B3X,\n    y=B3Y,\n    z=B3Z,\n    sizes=B3S,\n    theme="aurora",\n    width=860,\n    height=560,' },
        ],
        "line3d": [
            { id: "basic", label: "Basic", body: '    "3D Helix",\n    x=HX,\n    y=HY,\n    z=HZ,\n    theme="frost",\n    width=860,\n    height=560,' },
        ],
    };

    var PAGE_FN = {
        "bar":        'import seraplot as sp\nc = sp.bar(',
        "scatter":    'import seraplot as sp\nc = sp.scatter(',
        "line":       'import seraplot as sp\nc = sp.line(',
        "histogram":  'import seraplot as sp\nc = sp.histogram(',
        "violin":     'import seraplot as sp\nc = sp.violin(',
        "boxplot":    'import seraplot as sp\nc = sp.boxplot(',
        "radar":      'import seraplot as sp\nc = sp.radar(',
        "heatmap":    'import seraplot as sp\nc = sp.heatmap(',
        "bubble":     'import seraplot as sp\nc = sp.bubble(',
        "pie":        'import seraplot as sp\nc = sp.pie(',
        "funnel":     'import seraplot as sp\nc = sp.funnel(',
        "treemap":    'import seraplot as sp\nc = sp.treemap(',
        "sunburst":   'import seraplot as sp\nc = sp.sunburst(',
        "waterfall":  'import seraplot as sp\nc = sp.waterfall(',
        "ridgeline":  'import seraplot as sp\nc = sp.ridgeline(',
        "lollipop":   'import seraplot as sp\nc = sp.lollipop(',
        "kde":        'import seraplot as sp\nc = sp.kde(',
        "slope":      'import seraplot as sp\nc = sp.slope(',
        "bullet":     'import seraplot as sp\nc = sp.bullet(',
        "parallel":   'import seraplot as sp\nc = sp.parallel(',
        "gauge":      'import seraplot as sp\nc = sp.gauge(',
        "choropleth": 'import seraplot as sp\nc = sp.choropleth(',
        "scatter3d":  'import seraplot as sp\nc = sp.scatter3d(',
        "bar3d":      'import seraplot as sp\nc = sp.bar3d(',
        "bubble3d":   'import seraplot as sp\nc = sp.bubble3d(',
        "line3d":     'import seraplot as sp\nc = sp.line3d(',
    };

    var CHART_MAP = {
        "bar":        { fn: "buildBarChart",       p: ["labels", "values"], sp: "bar" },
        "scatter":    { fn: "buildScatterChart",   p: ["x", "y"],              al: { x_values: "x", y_values: "y" } },
        "line":       { fn: "buildLineChart",      p: ["labels", "values"],     al: { x: "labels", y: "values", x_labels: "labels" } },
        "histogram":  { fn: "buildHistogram",      p: ["values"] },
        "violin":     { fn: "buildViolin",         p: ["cat", "values"],        al: { labels: "cat", groups: "cat" } },
        "boxplot":    { fn: "buildBoxplot",        p: ["cat", "values"],        al: { labels: "cat", groups: "cat" } },
        "radar":      { fn: "buildRadarChart",     p: ["axes", "series"] },
        "heatmap":    { fn: "buildHeatmap",        p: ["labels", "matrix"],     al: { rows: "labels", values: "matrix" }, sp: "heatmap" },
        "bubble":     { fn: "buildBubble",         p: ["x", "y", "sizes"],      al: { x_values: "x", y_values: "y" } },
        "pie":        { fn: "buildPieChart",       p: ["labels", "values"] },
        "funnel":     { fn: "buildFunnel",         p: ["labels", "values"],     al: { stages: "labels" } },
        "treemap":    { fn: "buildTreemap",        p: ["labels", "values"] },
        "sunburst":   { fn: "buildSunburst",       p: ["labels", "parents", "values"] },
        "waterfall":  { fn: "buildWaterfall",      p: ["labels", "values"] },
        "ridgeline":  { fn: "buildRidgelineChart", p: ["values", "categories"], al: { groups: "categories" } },
        "lollipop":   { fn: "buildLollipopChart",  p: ["labels", "values"] },
        "kde":        { fn: "buildKdeChart",       p: ["values"] },
        "slope":      { fn: "buildSlope",          p: ["labels", "left", "right"], al: { before: "left", after: "right" } },
        "bullet":     { fn: "buildBullet",         p: ["labels", "values"],     al: { actuals: "values" } },
        "parallel":   { fn: "buildParallel",       p: ["axes", "series"] },
        "gauge":      { fn: "buildGauge",          p: ["value"],                sp: "gauge" },
        "choropleth": { fn: "buildChoropleth",     p: ["labels", "values"],     al: { countries: "labels" } },
        "scatter3d":  { fn: "buildScatter3dChart", p: ["x", "y", "z"] },
        "line3d":     { fn: "buildLine3dChart",    p: ["x", "y", "z"] },
        "bubble3d":   { fn: "buildBubble3dChart",  p: ["x", "y", "z", "sizes"] },
        "bar3d":      { fn: "buildBar3dChart",     p: ["x", "y", "z"],          sp: "bar3d" },
    };

    function fixBody(body) {
        return body
            .replace(/\bSCATTERX\b/g, J(_sc)).replace(/\bSCATTERY\b/g, J(_sc2))
            .replace(/\bHIST\b/g, J(_hist))
            .replace(/\bVIO\b/g, J(_vio))
            .replace(/\bBOX\b/g, J(_box))
            .replace(/\bKDE\b/g, J(_kde))
            .replace(/\bRID\b/g, J(_rid))
            .replace(/\bS3X\b/g, J(_s3x)).replace(/\bS3Y\b/g, J(_s3y)).replace(/\bS3Z\b/g, J(_s3z))
            .replace(/\bB3X\b/g, J(_b3x)).replace(/\bB3Y\b/g, J(_b3y)).replace(/\bB3Z\b/g, J(_b3z)).replace(/\bB3S\b/g, J(_b3s))
            .replace(/\bHX\b/g, J(_hx)).replace(/\bHY\b/g, J(_hy)).replace(/\bHZ\b/g, J(_hz));
    }

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
            if (map.sp === 'bar') {
                var variant = kw.variant || 'basic';
                var barOpts = {};
                Object.keys(kw).forEach(function(k) { if (k !== 'labels' && k !== 'values' && k !== 'series') barOpts[k] = kw[k]; });
                if (variant === 'horizontal') return sp.buildHbar(title, J(kw.labels || []), J(kw.values || []), J(barOpts));
                if (variant === 'grouped') return sp.buildGroupedBar(title, J(kw.labels || []), J(kw.series || []), J(barOpts));
                if (variant === 'stacked') return sp.buildStackedBar(title, J(kw.labels || []), J(kw.series || []), J(barOpts));
                if (variant === 'relative') { barOpts.relative = true; return sp.buildStackedBar(title, J(kw.labels || []), J(kw.series || []), J(barOpts)); }
                return sp.buildBarChart(title, J(kw.labels || []), J(kw.values || []), J(barOpts));
            }
            if (map.sp === 'gauge') return fn(title, kw.value || 0, J(opts));
            if (map.sp === 'heatmap') {
                var rows = kw.labels || kw.rows || [];
                var matrix = kw.matrix || kw.values || [];
                delete opts.rows; delete opts.cols;
                return fn(title, J(rows), J(matrix), J(opts));
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
            var args = [title];
            map.p.forEach(function (key) { args.push(J(kw[key] !== undefined ? kw[key] : [])); });
            args.push(J(opts));
            return fn.apply(null, args);
        } catch (e) { return null; }
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
        var css = '.sp-pg-wrap{display:flex;flex-direction:column;border:1px solid rgba(120,130,200,.13);border-radius:14px;overflow:hidden;margin:0 0 2.4rem;background:#07091a;font-family:sans-serif;box-shadow:0 8px 40px rgba(0,0,0,.45)}'
            + '.sp-pg-hdr{display:flex;align-items:center;gap:10px;padding:9px 14px;background:linear-gradient(90deg,#0c1228 0%,#0e1530 100%);border-bottom:1px solid rgba(120,130,200,.1);flex-wrap:wrap;row-gap:8px}'
            + '.sp-pg-title{font-size:10px;font-weight:800;letter-spacing:.18em;color:#4a5280;text-transform:uppercase;flex-shrink:0}'
            + '.sp-pg-vtabs{display:flex;gap:4px;flex-wrap:wrap;flex:1;min-width:0}'
            + '.sp-pg-vtab{background:transparent;border:1px solid rgba(120,130,200,.15);color:#6475a0;border-radius:6px;padding:3px 10px;font-size:12px;font-weight:600;cursor:pointer;transition:all .15s;white-space:nowrap;letter-spacing:.02em}'
            + '.sp-pg-vtab:hover{background:rgba(120,130,200,.1);color:#c9d1ff;border-color:rgba(120,130,200,.35)}'
            + '.sp-pg-vtab.sp-vact{background:linear-gradient(135deg,#3730a3,#1e1b4b);color:#e0e7ff;border-color:#6366f1;box-shadow:0 0 10px rgba(99,102,241,.3)}'
            + '.sp-pg-conn{display:flex;align-items:center;gap:7px;margin-left:auto;font-size:11.5px;color:#4a5280;letter-spacing:.04em;flex-shrink:0}'
            + '.sp-pg-dot{width:7px;height:7px;border-radius:50%;background:#333;flex-shrink:0;transition:background .3s}'
            + '.sp-wasm-loading{background:#f59e0b!important;box-shadow:0 0 8px #f59e0b88;animation:sp-pulse .9s infinite}'
            + '.sp-wasm-ready{background:#22c55e!important;box-shadow:0 0 7px #22c55e66}'
            + '.sp-wasm-err{background:#ef4444!important;box-shadow:0 0 7px #ef444466}'
            + '@keyframes sp-pulse{0%,100%{opacity:1}50%{opacity:.3}}'
            + '@keyframes sp-spin{to{transform:rotate(360deg)}}'
            + '@keyframes sp-fadein{from{opacity:0}to{opacity:1}}'
            + '.sp-pg-main{display:flex;height:440px;position:relative}'
            + '.sp-pg-ecol{width:44%;min-width:140px;max-width:78%;display:flex;flex-direction:column;flex-shrink:0;overflow:hidden;border-right:1px solid rgba(120,130,200,.08)}'
            + '.sp-pg-divider{width:4px;background:transparent;cursor:col-resize;flex-shrink:0;position:relative;z-index:2;transition:background .15s}'
            + '.sp-pg-divider:hover,.sp-pg-divider.sp-dragging{background:rgba(120,130,200,.22)}'
            + '.sp-pg-head,.sp-pg-tail{font-family:"Fira Code","Consolas",monospace;font-size:12px;line-height:1.65;padding:10px 14px 6px;color:#2d3a5a;background:#07091a;white-space:pre;user-select:none;flex-shrink:0;letter-spacing:.01em}'
            + '.sp-pg-tail{padding:6px 14px 10px;border-top:1px solid rgba(120,130,200,.05)}'
            + '.sp-pg-head{border-bottom:1px solid rgba(120,130,200,.05)}'
            + '.sp-pg-cm-wrap{flex:1;background:#07091a;overflow:hidden;position:relative}'
            + '.sp-pg-cm-wrap .CodeMirror{background:#07091a;font:12.5px/1.65 "Fira Code","Consolas",monospace;height:100%}'
            + '.sp-pg-cm-wrap .CodeMirror-scroll{padding:4px 14px;box-sizing:border-box}'
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
            + '.sp-pg-spinner{width:30px;height:30px;border:2.5px solid rgba(189,147,249,.15);border-top-color:#bd93f9;border-radius:50%;animation:sp-spin .8s linear infinite}';
        var st = document.createElement('style'); st.id = 'sp-pg-style'; st.textContent = css; document.head.appendChild(st);
    }

    function buildPG(root) {
        var slug = pageSlug();
        var variants = (slug && PAGE_VARIANTS[slug]) ? PAGE_VARIANTS[slug] : null;
        var headFn = (slug && PAGE_FN[slug]) ? PAGE_FN[slug] : 'import seraplot as sp\nc = sp.bar(';
        var varIdx = 0;
        var cmEditor = null;

        injectStyles();

        var vtabsHtml = variants ? variants.map(function (v, i) {
            return '<button class="sp-pg-vtab' + (i === 0 ? ' sp-vact' : '') + '" data-vi="' + i + '">' + v.label + '</button>';
        }).join('') : '';

        root.innerHTML = [
            '<div class="sp-pg-hdr">',
            '<span class="sp-pg-title">Playground</span>',
            '<div class="sp-pg-vtabs">' + vtabsHtml + '</div>',
            '<span class="sp-pg-conn"><span class="sp-pg-dot sp-wasm-loading"></span><span class="sp-pg-smsg">WASM\u2026</span></span>',
            '</div>',
            '<div class="sp-pg-main">',
            '<div class="sp-pg-ecol"><pre class="sp-pg-head"></pre><div class="sp-pg-cm-wrap"></div><pre class="sp-pg-tail"></pre></div>',
            '<div class="sp-pg-divider"></div>',
            '<div class="sp-pg-pcol">',
            '<div class="sp-pg-loader"><div class="sp-pg-spinner"></div><span>Initialisation\u2026</span></div>',
            '<iframe class="sp-pg-iframe" sandbox="allow-scripts" style="display:none"></iframe>',
            '</div></div>',
        ].join('');

        var dot = root.querySelector('.sp-pg-dot');
        var smsg = root.querySelector('.sp-pg-smsg');
        var loader = root.querySelector('.sp-pg-loader');
        var iframe = root.querySelector('.sp-pg-iframe');
        var headEl = root.querySelector('.sp-pg-head');
        var tailEl = root.querySelector('.sp-pg-tail');
        var cmWrap = root.querySelector('.sp-pg-cm-wrap');

        headEl.textContent = headFn;
        tailEl.textContent = ')\nc';

        function currentBody() {
            if (!variants) return '';
            return fixBody(variants[varIdx].body);
        }

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

        function run() {
            if (!_wasmReady) return;
            var body = cmEditor ? cmEditor.getValue() : currentBody();
            var html = callWasm(slug || 'bar', body);
            if (html) showFrame(html);
        }

        root.querySelectorAll('.sp-pg-vtab').forEach(function (btn) {
            btn.addEventListener('click', function () {
                varIdx = parseInt(btn.getAttribute('data-vi'), 10);
                root.querySelectorAll('.sp-pg-vtab').forEach(function (b) { b.classList.remove('sp-vact'); });
                btn.classList.add('sp-vact');
                clearTimeout(_debTimer);
                var body = fixBody(variants[varIdx].body);
                if (cmEditor) {
                    _skipDebounce = true;
                    cmEditor.setValue(body);
                    _skipDebounce = false;
                }
                run();
            });
        });

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

        loadCM(function () {
            if (!window.CodeMirror) return;
            cmEditor = CodeMirror(cmWrap, {
                value: currentBody(), mode: "python", theme: "dracula",
                lineNumbers: false, indentUnit: 4, tabSize: 4, indentWithTabs: false,
                lineWrapping: true,
                extraKeys: { Tab: function (cm) { cm.replaceSelection("    "); } }
            });
            cmEditor.setSize('100%', '100%');
            cmEditor.on("change", function () {
                if (_skipDebounce) return;
                clearTimeout(_debTimer);
                _debTimer = setTimeout(run, DEBOUNCE_MS);
            });
            if (_wasmReady) run();
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
        var first = main.firstChild;
        if (first) main.insertBefore(root, first);
        else main.appendChild(root);
        buildPG(root);
    }

    if (document.readyState === 'loading') document.addEventListener('DOMContentLoaded', init);
    else init();
})();
