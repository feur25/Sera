(function () {
    var URL_JSON_REMOTE = "https://raw.githubusercontent.com/feur25/seraplot/main/playground-url.json";
    var URL_JSON_LOCAL = "playground-url.json";
    var DEBOUNCE_MS = 700;
    var RECONNECT_MS = 2500;

    var state = {
        ws: null,
        wsUrl: null,
        connecting: false,
        ready: false,
        runId: 0,
        lastSent: "",
        debTimer: null,
        editor: null,
        iframe: null,
        statusDot: null,
        statusText: null,
        version: "",
        loaderEl: null,
        errEl: null,
    };

    function pageSlug() {
        var m = window.location.pathname.match(/\/([^\/]+?)(?:\.html?)?$/);
        return m ? m[1] : null;
    }

    function isChartPage() {
        return /\/charts\//.test(window.location.pathname) && pageSlug() && pageSlug() !== "index";
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
        ["https://cdnjs.cloudflare.com/ajax/libs/codemirror/5.65.17/codemirror.min.css",
         "https://cdnjs.cloudflare.com/ajax/libs/codemirror/5.65.17/theme/dracula.min.css"].forEach(function (href) {
            var l = document.createElement("link"); l.rel = "stylesheet"; l.href = href; document.head.appendChild(l);
        });
        var s1 = document.createElement("script");
        s1.src = "https://cdnjs.cloudflare.com/ajax/libs/codemirror/5.65.17/codemirror.min.js";
        s1.onload = function () {
            var s2 = document.createElement("script");
            s2.src = "https://cdnjs.cloudflare.com/ajax/libs/codemirror/5.65.17/mode/python/python.min.js";
            s2.onload = cb;
            document.head.appendChild(s2);
        };
        document.head.appendChild(s1);
    }

    function defaultCode(slug) {
        return 'import seraplot as sp\n\nc = sp.' + slug + '(\n    "Demo ' + slug + '",\n    labels=["A","B","C","D","E"],\n    values=[12,24,18,30,21],\n)\n';
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
            '.sp-pg-hdr,.sp-pg-main,.sp-pg-foot{position:relative;z-index:1}',
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

    function fetchPlaygroundUrl() {
        return fetch(URL_JSON_REMOTE, { cache: 'no-store' })
            .catch(function () { return fetch(URL_JSON_LOCAL, { cache: 'no-store' }); })
            .then(function (r) { return r.ok ? r.json() : null; })
            .catch(function () { return null; });
    }

    function connect() {
        if (state.connecting || state.ready) return;
        state.connecting = true;
        setStatus('loading', 'Connecting');
        fetchPlaygroundUrl().then(function (info) {
            if (!info || !info.url) {
                state.connecting = false;
                setStatus('err', 'Server offline');
                showLoader('Playground server is <b>offline</b>.<br>It cycles every ~6h via GitHub Actions.<br>Check back soon.');
                setTimeout(connect, 30000);
                return;
            }
            state.wsUrl = info.url;
            try { state.ws = new WebSocket(info.url); }
            catch (e) {
                state.connecting = false;
                setStatus('err', 'Connect failed');
                setTimeout(connect, RECONNECT_MS);
                return;
            }
            state.ws.onopen = function () {
                state.connecting = false;
                state.ready = true;
                setStatus('ready', 'Live');
                state.ws.send(JSON.stringify({ a: 'ping' }));
                runOnce(true);
            };
            state.ws.onmessage = function (ev) {
                try { handleMsg(JSON.parse(ev.data)); } catch (e) { }
            };
            state.ws.onerror = function () {
                setStatus('err', 'Error');
            };
            state.ws.onclose = function () {
                state.ready = false;
                state.connecting = false;
                state.ws = null;
                setStatus('err', 'Disconnected');
                setTimeout(connect, RECONNECT_MS);
            };
        });
    }

    function handleMsg(m) {
        if (m.t === 'pong') {
            state.version = m.version || '';
            setStatus('ready', 'Live · sp ' + state.version);
        } else if (m.t === 'frame') {
            hideErr();
            hideLoader();
            renderHtmlInIframe(m.html);
        } else if (m.t === 'err') {
            showErr(m.msg || 'Execution error');
            hideLoader();
        } else if (m.t === 'done') {
            hideLoader();
        }
    }

    function runOnce(force) {
        if (!state.ready || !state.editor) return;
        var code = state.editor.getValue();
        if (!force && code === state.lastSent) return;
        state.lastSent = code;
        state.runId++;
        showLoader('Running');
        try { state.ws.send(JSON.stringify({ a: 'run', id: state.runId, code: code })); }
        catch (e) { setStatus('err', 'Send failed'); }
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

    function buildUI() {
        injectStyles();
        var slug = pageSlug();
        var wrap = document.createElement('div');
        wrap.className = 'sp-pg-wrap';
        wrap.innerHTML =
            '<div class="sp-pg-hdr">' +
                '<div class="sp-pg-title">Playground · ' + slug + '</div>' +
                '<div class="sp-pg-spacer"></div>' +
                '<button class="sp-pg-btn" type="button">Run</button>' +
                '<div class="sp-pg-conn"><span class="sp-pg-dot"></span><span class="sp-pg-text">Init</span></div>' +
            '</div>' +
            '<div class="sp-pg-main">' +
                '<div class="sp-pg-ecol"><div class="sp-pg-cm-wrap"><textarea></textarea></div></div>' +
                '<div class="sp-pg-divider"></div>' +
                '<div class="sp-pg-pcol">' +
                    '<iframe class="sp-pg-iframe" sandbox="allow-scripts allow-same-origin"></iframe>' +
                    '<div class="sp-pg-loader"><div class="sp-pg-spinner"></div><div>Booting</div></div>' +
                    '<div class="sp-pg-err"></div>' +
                '</div>' +
            '</div>';

        var contentRoot = document.querySelector('main') || document.querySelector('.content') || document.body;
        var firstHeading = contentRoot.querySelector('h1, h2');
        if (firstHeading && firstHeading.parentNode) firstHeading.parentNode.insertBefore(wrap, firstHeading.nextSibling);
        else contentRoot.insertBefore(wrap, contentRoot.firstChild);

        var ta = wrap.querySelector('textarea');
        ta.value = defaultCode(slug);
        state.iframe = wrap.querySelector('.sp-pg-iframe');
        state.statusDot = wrap.querySelector('.sp-pg-dot');
        state.statusText = wrap.querySelector('.sp-pg-text');
        state.loaderEl = wrap.querySelector('.sp-pg-loader');
        state.errEl = wrap.querySelector('.sp-pg-err');
        var btn = wrap.querySelector('.sp-pg-btn');
        var divider = wrap.querySelector('.sp-pg-divider');
        var ecol = wrap.querySelector('.sp-pg-ecol');

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
            connect();
        });

        btn.addEventListener('click', function () { runOnce(true); });
    }

    function init() {
        if (!isChartPage()) return;
        buildUI();
    }

    if (document.readyState === 'loading') {
        document.addEventListener('DOMContentLoaded', init);
    } else {
        init();
    }
})();
