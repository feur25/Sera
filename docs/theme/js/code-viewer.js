(function () {
  function isEnglish() {
    return (localStorage.getItem("seraplot_lang") || "en").toLowerCase() !== "fr";
  }

  function onKeydown(e) {
    if (e.key === "Escape") closeViewer();
  }

  function closeViewer() {
    var el = document.getElementById("sp-code-viewer");
    if (el && el.parentNode) el.parentNode.removeChild(el);
    document.removeEventListener("keydown", onKeydown);
  }

  function openViewer(code, title) {
    closeViewer();
    var en = isEnglish();

    var overlay = document.createElement("div");
    overlay.id = "sp-code-viewer";
    overlay.setAttribute("role", "dialog");
    overlay.setAttribute("aria-modal", "true");
    overlay.style.cssText = "position:fixed;inset:0;background:rgba(0,0,0,.6);z-index:99999;display:flex;align-items:center;justify-content:center;padding:24px";

    var panel = document.createElement("div");
    panel.style.cssText =
      "width:min(920px,100%);max-height:min(720px,90vh);display:flex;flex-direction:column;background:#1e1e1e;" +
      "border:1px solid #333;border-radius:10px;overflow:hidden;box-shadow:0 20px 60px rgba(0,0,0,.5)";

    var bar = document.createElement("div");
    bar.style.cssText = "display:flex;align-items:center;justify-content:space-between;padding:10px 14px;background:#252526;border-bottom:1px solid #333;flex-shrink:0";

    var dots =
      '<span style="display:inline-flex;gap:6px;margin-right:12px">' +
      '<span style="width:11px;height:11px;border-radius:50%;background:#ff5f56;display:inline-block"></span>' +
      '<span style="width:11px;height:11px;border-radius:50%;background:#ffbd2e;display:inline-block"></span>' +
      '<span style="width:11px;height:11px;border-radius:50%;background:#27c93f;display:inline-block"></span>' +
      "</span>";

    bar.innerHTML =
      '<div style="display:flex;align-items:center;color:#ccc;font-size:12.5px;font-family:system-ui,sans-serif">' + dots + "<span>" + title + "</span></div>" +
      '<div style="display:flex;gap:8px;align-items:center">' +
      '<button id="sp-code-copy" style="background:#2d2d2d;border:1px solid #3d3d3d;color:#ccc;padding:5px 12px;border-radius:6px;font-size:12px;cursor:pointer;font-family:system-ui,sans-serif">' +
      (en ? "Copy" : "Copier") +
      "</button>" +
      '<button id="sp-code-close" aria-label="Close" style="background:none;border:none;color:#999;font-size:20px;cursor:pointer;line-height:1;padding:2px 6px">×</button>' +
      "</div>";

    var body = document.createElement("div");
    body.style.cssText = "overflow:auto;flex:1;background:#1e1e1e !important";

    var pre = document.createElement("pre");
    pre.style.cssText =
      "margin:0 !important;padding:18px 20px !important;font-size:13px !important;line-height:1.6 !important;" +
      "background:#1e1e1e !important;color:#d4d4d4 !important;white-space:pre !important;border:none !important;box-shadow:none !important";
    var codeEl = document.createElement("code");
    codeEl.className = "language-python";
    codeEl.textContent = code;
    codeEl.style.cssText = "background:#1e1e1e !important;color:#d4d4d4 !important;padding:0 !important";
    pre.appendChild(codeEl);
    body.appendChild(pre);

    panel.appendChild(bar);
    panel.appendChild(body);
    overlay.appendChild(panel);
    document.body.appendChild(overlay);

    if (window.hljs) {
      try {
        if (typeof window.hljs.highlightElement === "function") {
          window.hljs.highlightElement(codeEl);
        } else if (typeof window.hljs.highlightBlock === "function") {
          window.hljs.highlightBlock(codeEl);
        }
      } catch (err) {}
    }

    document.getElementById("sp-code-close").addEventListener("click", closeViewer);
    overlay.addEventListener("click", function (e) {
      if (e.target === overlay) closeViewer();
    });
    document.getElementById("sp-code-copy").addEventListener("click", function () {
      var btn = document.getElementById("sp-code-copy");
      var reset = function () {
        setTimeout(function () {
          btn.textContent = en ? "Copy" : "Copier";
        }, 1500);
      };
      navigator.clipboard.writeText(code).then(
        function () {
          btn.textContent = en ? "Copied!" : "Copié !";
          reset();
        },
        function () {
          btn.textContent = en ? "Copy failed" : "Échec";
          reset();
        }
      );
    });

    document.addEventListener("keydown", onKeydown);
  }

  document.addEventListener("click", function (e) {
    var btn = e.target.closest && e.target.closest(".sp-code-btn[data-sp-code-target]");
    if (!btn) return;
    var src = document.getElementById(btn.getAttribute("data-sp-code-target"));
    if (!src) return;
    var title = btn.getAttribute("data-sp-code-title") || "code.py";
    openViewer(src.textContent, title);
  });
})();
