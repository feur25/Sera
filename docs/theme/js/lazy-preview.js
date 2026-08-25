(function () {
    function activate(el) {
        var src = el.getAttribute('data-src');
        if (!src || el.getAttribute('src')) return;
        el.setAttribute('src', src);
        el.removeAttribute('data-src');
    }
    function activateVisible() {
        document.querySelectorAll('iframe.sp-preview-frame[data-src]').forEach(function (el) {
            if (el.offsetParent !== null) activate(el);
        });
    }
    function activateDefaults() {
        document.querySelectorAll('.sp-von iframe.sp-preview-frame[data-src]').forEach(activate);
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
                        activate(entry.target);
                        io.unobserve(entry.target);
                    }
                });
            }, { rootMargin: '300px 0px', threshold: 0.01 });
            document.querySelectorAll('iframe.sp-preview-frame[data-src]').forEach(function (el) { io.observe(el); });
        }
    }
    if (document.readyState === 'loading') {
        document.addEventListener('DOMContentLoaded', boot);
    } else {
        boot();
    }
})();
