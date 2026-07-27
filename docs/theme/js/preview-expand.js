(function () {
  function nudgeIframe(frame) {
    var iframe = frame.querySelector('iframe');
    if (!iframe || !iframe.contentWindow) return;
    var fire = function () {
      try {
        iframe.contentWindow.dispatchEvent(new Event('resize'));
      } catch (e) {}
    };
    fire();
    requestAnimationFrame(fire);
    setTimeout(fire, 60);
    setTimeout(fire, 220);
  }

  function collapse(frame, backdrop) {
    frame.classList.remove('sp-preview-expanded');
    backdrop.classList.remove('sp-preview-backdrop-active');
    document.body.classList.remove('sp-preview-lock');
    var btn = frame.querySelector('.sp-preview-expand');
    if (btn) {
      btn.textContent = '⤢';
      btn.setAttribute('aria-label', 'Expand');
    }
    nudgeIframe(frame);
  }

  function expand(frame, backdrop) {
    frame.classList.add('sp-preview-expanded');
    backdrop.classList.add('sp-preview-backdrop-active');
    document.body.classList.add('sp-preview-lock');
    var btn = frame.querySelector('.sp-preview-expand');
    if (btn) {
      btn.textContent = '✕';
      btn.setAttribute('aria-label', 'Collapse');
    }
    nudgeIframe(frame);
  }

  function init(frame, backdrop) {
    var btn = document.createElement('button');
    btn.type = 'button';
    btn.className = 'sp-preview-expand';
    btn.textContent = '⤢';
    btn.setAttribute('aria-label', 'Expand');
    btn.addEventListener('click', function () {
      if (frame.classList.contains('sp-preview-expanded')) {
        collapse(frame, backdrop);
      } else {
        expand(frame, backdrop);
      }
    });
    frame.appendChild(btn);
    backdrop.addEventListener('click', function () {
      collapse(frame, backdrop);
    });
  }

  function mount() {
    var backdrop = document.createElement('div');
    backdrop.className = 'sp-preview-backdrop';
    document.body.appendChild(backdrop);
    var frames = document.querySelectorAll('.sp-preview-frame');
    frames.forEach(function (frame) {
      init(frame, backdrop);
    });
    document.addEventListener('keydown', function (e) {
      if (e.key !== 'Escape') return;
      frames.forEach(function (frame) {
        collapse(frame, backdrop);
      });
    });
  }

  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', mount);
  } else {
    mount();
  }
})();
