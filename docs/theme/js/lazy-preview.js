(function () {
    var queue = [];
    var draining = false;

    function reallyActivate(el) {
        var src = el.getAttribute('data-src');
        if (!src || el.getAttribute('src')) return;
        el.setAttribute('src', src);
        el.removeAttribute('data-src');
    }
    function drainQueue() {
        if (draining) return;
        draining = true;
        function step() {
            var el = queue.shift();
            if (!el) {
                draining = false;
                return;
            }
            reallyActivate(el);
            if (queue.length) {
                setTimeout(step, 140);
            } else {
                draining = false;
            }
        }
        step();
    }
    function enqueue(el) {
        if (!el || el.getAttribute('src') || queue.indexOf(el) !== -1) return;
        queue.push(el);
        drainQueue();
    }
    function activateVisible() {
        document.querySelectorAll('iframe.sp-preview-frame[data-src]').forEach(function (el) {
            if (el.offsetParent !== null) reallyActivate(el);
        });
    }
    function activateDefaults() {
        document.querySelectorAll('.sp-von iframe.sp-preview-frame[data-src]').forEach(reallyActivate);
    }
    function boot() {
        activateDefaults();
        activateVisible();
        document.addEventListener('click', function (e) {
            var t = e.target;
            var btn = t && t.closest ? t.closest('.sp-cls-tab, .sp-var-tab, [data-sp-tab]') : null;
            if (!btn) return;
            setTimeout(activateVisible, 0);
        }, true);
        if (typeof IntersectionObserver === 'function') {
            var io = new IntersectionObserver(function (entries) {
                entries.forEach(function (entry) {
                    if (entry.isIntersecting) {
                        enqueue(entry.target);
                        io.unobserve(entry.target);
                    }
                });
            }, { rootMargin: '100px 0px', threshold: 0.01 });
            document.querySelectorAll('iframe.sp-preview-frame[data-src]').forEach(function (el) { io.observe(el); });
        }
    }
    if (document.readyState === 'loading') {
        document.addEventListener('DOMContentLoaded', boot);
    } else {
        boot();
    }
})();
