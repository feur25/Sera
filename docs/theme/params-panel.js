(function () {
  var POS_KEY       = "sp_params_pos";
  var COL_KEY       = "sp_params_col";
  var SIZE_KEY      = "sp_params_size";
  var PANEL_ID      = "sp-params-panel";

  var pos       = localStorage.getItem(POS_KEY)  || "right";
  var collapsed = localStorage.getItem(COL_KEY)  === "1";
  var size      = localStorage.getItem(SIZE_KEY) || "md";

  function isChartPage() {
    return !!document.querySelector(
      'h2[id="parameters"], h2[id="param-tres"], h2[id="param-tres-param-tres"],' +
      ' h2[id="param-egravtres"], h2[id="param-tres-parametres"]'
    ) || (function () {
      var found = false;
      document.querySelectorAll(".content h2").forEach(function (h) {
        var t = h.textContent.trim().toLowerCase();
        if (t === "parameters" || t === "param\u00e8tres") found = true;
      });
      return found;
    }());
  }

  function findH2(texts) {
    var found = null;
    document.querySelectorAll(".content h2").forEach(function (h) {
      var t = h.textContent.trim().toLowerCase();
      if (!found && texts.indexOf(t) !== -1) found = h;
    });
    return found;
  }

  function extractSection(h2El) {
    if (!h2El) return null;
    var frag = document.createDocumentFragment();
    var sib = h2El.nextElementSibling;
    while (sib && sib.tagName !== "H2") {
      frag.appendChild(sib.cloneNode(true));
      sib = sib.nextElementSibling;
    }
    return frag;
  }

  function buildPanel() {
    if (document.getElementById(PANEL_ID)) return;
    if (!isChartPage()) return;

    var sigH2    = findH2(["signature"]);
    var aliasH2  = findH2(["aliases", "alias"]);
    var paramsH2 = findH2(["parameters", "param\u00e8tres"]);
    var retH2    = findH2(["returns", "retour"]);

    if (!paramsH2) return;

    var panel = document.createElement("div");
    panel.id = PANEL_ID;

    var hd = document.createElement("div");
    hd.className = "sp-ph";

    var htitle = document.createElement("div");
    htitle.className = "sp-ph-left";
    htitle.innerHTML =
      '<svg class="sp-ph-ico" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5">' +
        '<rect x="1" y="3" width="14" height="2" rx="1"/>' +
        '<rect x="1" y="7" width="10" height="2" rx="1"/>' +
        '<rect x="1" y="11" width="12" height="2" rx="1"/>' +
      '</svg>' +
      '<span>Parameters</span>';

    var ctrl = document.createElement("div");
    ctrl.className = "sp-ph-ctrl";

    var sizeBtn = document.createElement("button");
    sizeBtn.className = "sp-pc-btn";
    sizeBtn.id = "sp-pc-size";

    var posBtn = document.createElement("button");
    posBtn.className = "sp-pc-btn";
    posBtn.id = "sp-pc-pos";

    var colBtn = document.createElement("button");
    colBtn.className = "sp-pc-btn";
    colBtn.id = "sp-pc-col";

    ctrl.appendChild(sizeBtn);
    ctrl.appendChild(posBtn);
    ctrl.appendChild(colBtn);
    hd.appendChild(htitle);
    hd.appendChild(ctrl);
    panel.appendChild(hd);

    var body = document.createElement("div");
    body.className = "sp-pb";
    body.id = "sp-pb";

    if (sigH2) {
      var sigSec = extractSection(sigH2);
      if (sigSec) {
        var sw = document.createElement("div");
        sw.className = "sp-psec sp-psec-sig";
        var sl = document.createElement("div");
        sl.className = "sp-psec-lbl";
        sl.textContent = "Signature";
        sw.appendChild(sl);
        sw.appendChild(sigSec);
        body.appendChild(sw);
      }
    }

    if (aliasH2) {
      var alSec = extractSection(aliasH2);
      if (alSec) {
        var aw = document.createElement("div");
        aw.className = "sp-psec sp-psec-alias";
        var al = document.createElement("div");
        al.className = "sp-psec-lbl";
        al.textContent = "Aliases";
        aw.appendChild(al);
        aw.appendChild(alSec);
        body.appendChild(aw);
      }
    }

    if (paramsH2) {
      var pSec = extractSection(paramsH2);
      if (pSec) {
        var pw = document.createElement("div");
        pw.className = "sp-psec sp-psec-params";
        var pl = document.createElement("div");
        pl.className = "sp-psec-lbl";
        pl.textContent = "Parameters";
        pw.appendChild(pl);
        pw.appendChild(pSec);
        body.appendChild(pw);
      }
    }

    if (retH2) {
      var rSec = extractSection(retH2);
      if (rSec) {
        var rw = document.createElement("div");
        rw.className = "sp-psec sp-psec-returns";
        var rl = document.createElement("div");
        rl.className = "sp-psec-lbl";
        rl.textContent = "Returns";
        rw.appendChild(rl);
        rw.appendChild(rSec);
        body.appendChild(rw);
      }
    }

    panel.appendChild(body);
    document.body.appendChild(panel);

    applyPos(panel, posBtn);
    applySize(panel, sizeBtn);
    applyCollapsed(panel, colBtn);

    posBtn.addEventListener("click", function () {
      pos = pos === "right" ? "bottom" : "right";
      localStorage.setItem(POS_KEY, pos);
      applyPos(panel, posBtn);
    });

    sizeBtn.addEventListener("click", function () {
      size = size === "md" ? "lg" : "md";
      localStorage.setItem(SIZE_KEY, size);
      applySize(panel, sizeBtn);
    });

    colBtn.addEventListener("click", function () {
      collapsed = !collapsed;
      localStorage.setItem(COL_KEY, collapsed ? "1" : "0");
      applyCollapsed(panel, colBtn);
    });
  }

  function applyPos(panel, posBtn) {
    panel.classList.remove("sp-p-right", "sp-p-bottom");
    document.body.classList.remove("sp-body-right", "sp-body-bottom");
    panel.classList.add("sp-p-" + pos);
    document.body.classList.add("sp-body-" + pos);
    posBtn.innerHTML = pos === "right"
      ? '<svg viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="2"><path d="M8 3v10M4 9l4 4 4-4"/></svg>'
      : '<svg viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="2"><path d="M3 8h10M9 4l4 4-4 4"/></svg>';
    posBtn.title = pos === "right" ? "Dock to bottom" : "Dock to right";
  }

  function applySize(panel, sizeBtn) {
    panel.classList.remove("sp-p-md", "sp-p-lg");
    panel.classList.add("sp-p-" + size);
    sizeBtn.innerHTML = size === "md"
      ? '<svg viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="2"><path d="M2 8h12M8 2v12"/></svg>'
      : '<svg viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="2"><path d="M3 3l10 10M13 3L3 13"/></svg>';
    sizeBtn.title = size === "md" ? "Expand" : "Shrink";
  }

  function applyCollapsed(panel, colBtn) {
    panel.classList.toggle("sp-p-collapsed", collapsed);
    colBtn.innerHTML = collapsed
      ? '<svg viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="2"><path d="M4 8h8"/><path d="M8 4v8"/></svg>'
      : '<svg viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="2"><path d="M4 8h8"/></svg>';
    colBtn.title = collapsed ? "Expand" : "Collapse";
  }

  var _timer = null;
  function tryBuild() {
    if (document.getElementById(PANEL_ID)) return;
    if (document.readyState !== "loading" && isChartPage()) {
      buildPanel();
    }
  }

  if (document.readyState === "loading") {
    document.addEventListener("DOMContentLoaded", tryBuild);
  } else {
    tryBuild();
  }

  var _obs = new MutationObserver(function () {
    if (_timer) clearTimeout(_timer);
    _timer = setTimeout(function () {
      _timer = null;
      if (!document.getElementById(PANEL_ID)) {
        pos       = localStorage.getItem(POS_KEY)  || "right";
        collapsed = localStorage.getItem(COL_KEY)  === "1";
        size      = localStorage.getItem(SIZE_KEY) || "md";
        tryBuild();
      }
    }, 120);
  });
  _obs.observe(document.body, { childList: true, subtree: true });
})();
