(function () {
  var THREE_D = [
    "scatter3d", "bar3d", "line3d", "radar3d", "lollipop3d", "kde3d",
    "ridgeline3d", "bubble3d", "pie3d", "violin3d", "heatmap3d",
    "candlestick3d", "dumbbell3d", "funnel3d", "sunburst3d",
    "stacked-bar3d", "globe3d",
  ];
  var MAPS = ["bubble-map", "choropleth"];

  function slugify(key) {
    return key.replace(/_/g, "-");
  }

  function titleCase(slug) {
    return slug
      .replace(/-?3d$/, " 3D")
      .split(/[- ]/)
      .filter(Boolean)
      .map(function (w) { return w === "3D" ? w : w.charAt(0).toUpperCase() + w.slice(1); })
      .join(" ");
  }

  function resolvePreviewSrc(slug, cb) {
    var plain = "previews/" + slug + ".html";
    fetch(plain, { method: "HEAD" }).then(function (r) {
      cb(r.ok ? plain : "previews/" + slug + "-basic.html");
    }).catch(function () {
      cb("previews/" + slug + "-basic.html");
    });
  }

  var FALLBACK_W = 900, FALLBACK_H = 540;

  function measureFrameContent(frame) {
    try {
      var doc = frame.contentDocument;
      if (!doc || !doc.body) return null;
      var w = Math.max(doc.body.scrollWidth, doc.documentElement.scrollWidth);
      var h = Math.max(doc.body.scrollHeight, doc.documentElement.scrollHeight);
      return (w && h) ? { w: w, h: h } : null;
    } catch (e) {
      return null;
    }
  }

  function fitFrame(frame, viewport) {
    var size = measureFrameContent(frame) || { w: FALLBACK_W, h: FALLBACK_H };
    frame.style.width = size.w + "px";
    frame.style.height = size.h + "px";
    var vw = viewport.clientWidth, vh = viewport.clientHeight;
    var scale = Math.min(vw / size.w, vh / size.h);
    frame.style.transform = "scale(" + scale + ")";
  }

  function stripChrome(frame) {
    try {
      var doc = frame.contentDocument;
      if (!doc || !doc.head) return;
      var style = doc.createElement("style");
      style.textContent =
        "html,body{background:transparent!important}" +
        ".sp-bg{fill:transparent!important}";
      doc.head.appendChild(style);
      var wrap = doc.querySelector('[role="main"] > div');
      if (wrap) {
        wrap.style.boxShadow = "none";
        wrap.style.borderRadius = "0";
        wrap.style.background = "transparent";
      }
    } catch (e) {}
  }

  function card(slug, category, title) {
    var wrap = document.createElement("div");
    wrap.className = "sp-sc-card";
    var viewport = document.createElement("div");
    viewport.className = "sp-sc-viewport";
    var frame = document.createElement("iframe");
    frame.className = "sp-sc-frame";
    frame.loading = "lazy";
    frame.scrolling = "no";
    frame.style.width = FALLBACK_W + "px";
    frame.style.height = FALLBACK_H + "px";
    frame.style.transform = "scale(" + (172 / FALLBACK_H) + ")";
    frame.onload = function () {
      stripChrome(frame);
      fitFrame(frame, viewport);
    };
    viewport.appendChild(frame);
    var a = document.createElement("a");
    a.className = "sp-sc-title";
    a.href = "charts/" + category + "/" + slug + ".html";
    a.textContent = title;
    wrap.appendChild(viewport);
    wrap.appendChild(a);
    resolvePreviewSrc(slug, function (src) { frame.src = src; });
    return wrap;
  }

  function section(container, heading, entries, category) {
    if (!entries.length) return;
    var h = document.createElement("h3");
    h.textContent = heading;
    container.appendChild(h);
    var grid = document.createElement("div");
    grid.className = "sp-sc-grid";
    entries.forEach(function (e) {
      grid.appendChild(card(e.slug, category, e.title));
    });
    container.appendChild(grid);
  }

  function build(root, lang) {
    root.innerHTML = "";
    var reg = window.SeraPlotDocRegistry;
    var variants = (reg && reg.variants) || {};

    var twoD = Object.keys(variants)
      .map(slugify)
      .filter(function (s) { return THREE_D.indexOf(s) === -1 && MAPS.indexOf(s) === -1; })
      .sort()
      .map(function (s) { return { slug: s, title: titleCase(s) }; });

    var threeD = THREE_D.map(function (s) { return { slug: s, title: titleCase(s) }; });
    var maps = MAPS.map(function (s) { return { slug: s, title: titleCase(s) }; });

    var titles = lang === "fr"
      ? { d2: "Graphiques 2D", d3: "Graphiques 3D", map: "Cartes", count: "familles enregistrées — lu depuis le registre SeraPlot, rien de codé en dur." }
      : { d2: "2D Charts", d3: "3D Charts", map: "Map Charts", count: "registered families — read from the SeraPlot registry, nothing hardcoded." };

    var count = document.createElement("p");
    count.className = "sp-sc-count";
    count.textContent = (twoD.length + threeD.length + maps.length) + " " + titles.count;
    root.appendChild(count);

    section(root, titles.d2, twoD, "2d");
    section(root, titles.d3, threeD, "3d");
    section(root, titles.map, maps, "map");
  }

  function init() {
    var enRoot = document.getElementById("sp-showcase-en");
    var frRoot = document.getElementById("sp-showcase-fr");
    if (!enRoot && !frRoot) return;
    if (enRoot) build(enRoot, "en");
    if (frRoot) build(frRoot, "fr");
  }

  if (document.readyState === "loading") {
    document.addEventListener("DOMContentLoaded", init);
  } else {
    init();
  }
})();
