# About & Support

<style>
/* ── Support page ──────────────────────────────────────────── */
.sp-hero-card {
  display: flex; align-items: flex-start; gap: 20px;
  background: linear-gradient(135deg, #0d1117 55%, #111827);
  border: 1px solid #1e293b; border-radius: 14px;
  padding: 26px; margin: 1.5em 0;
}
.sp-hero-ava {
  width: 52px; height: 52px; border-radius: 50%; flex-shrink: 0;
  background: linear-gradient(135deg, #6366f1, #8b5cf6);
  display: flex; align-items: center; justify-content: center;
  font-size: 22px; box-shadow: 0 0 0 3px rgba(99,102,241,.2);
}
.sp-hero-body h3 { margin: 0 0 8px; font-size: 15px; color: #e2e8f0; font-weight: 700; }
.sp-hero-body p  { margin: 0; font-size: 13px; color: #94a3b8; line-height: 1.7; }
.sp-goals {
  display: flex; flex-wrap: wrap; gap: 8px; margin-top: 16px;
}
.sp-goal {
  background: #161b27; border: 1px solid #1e293b; border-radius: 20px;
  padding: 5px 13px; font-size: 12px; font-weight: 600; color: #94a3b8;
  display: flex; align-items: center; gap: 6px;
}
.sp-goal::before { content: ''; width: 7px; height: 7px; border-radius: 50%; background: #6366f1; flex-shrink: 0; }

/* contact card */
.sp-contact-card {
  border: 1px solid #1e293b; border-radius: 12px;
  padding: 22px 24px; margin: 1.5em 0; background: #0d1117;
}
.sp-contact-card .sp-card-head {
  display: flex; align-items: center; gap: 9px;
  margin-bottom: 8px; font-size: 15px; font-weight: 700; color: #e2e8f0;
}
.sp-contact-card p { margin: 0 0 16px; font-size: 13px; color: #64748b; line-height: 1.6; }
.sp-email-row { display: flex; align-items: center; gap: 10px; flex-wrap: wrap; }
.sp-email-link {
  display: inline-flex; align-items: center; gap: 8px;
  padding: 10px 18px; background: #0f172a;
  border: 1.5px solid #30363d; border-radius: 8px;
  color: #6366f1 !important; text-decoration: none !important;
  font-size: 14px; font-weight: 700; transition: border-color .15s, background .15s;
}
.sp-email-link:hover { border-color: #6366f1; background: #1e1b3a; }
.sp-copy-btn {
  padding: 9px 14px; background: #161b27;
  border: 1px solid #30363d; border-radius: 7px;
  color: #64748b; font-size: 12px; font-weight: 600;
  cursor: pointer; font-family: inherit;
  transition: color .15s, border-color .15s;
}
.sp-copy-btn:hover { color: #e2e8f0; border-color: #475569; }
.sp-copy-btn.sp-copied { color: #22c55e !important; border-color: #22c55e !important; }

/* donate card */
.sp-donate-card {
  border: 1px solid #1e293b; border-radius: 12px;
  padding: 22px 24px; margin: 1.5em 0; background: #0d1117;
}
.sp-donate-card .sp-card-head {
  display: flex; align-items: center; gap: 9px;
  margin-bottom: 8px; font-size: 15px; font-weight: 700; color: #e2e8f0;
}
.sp-donate-card p { margin: 0 0 16px; font-size: 13px; color: #64748b; line-height: 1.6; }
.sp-donate-btn {
  display: inline-flex; align-items: center; gap: 9px;
  padding: 11px 22px;
  background: linear-gradient(135deg, #0070ba, #003087);
  color: #fff !important; text-decoration: none !important;
  border-radius: 8px; font-size: 14px; font-weight: 700;
  box-shadow: 0 2px 14px rgba(0,112,186,.35);
  transition: filter .15s, box-shadow .15s;
}
.sp-donate-btn:hover { filter: brightness(1.15); box-shadow: 0 4px 20px rgba(0,112,186,.5); }
.sp-donate-note {
  display: flex; align-items: center; gap: 6px;
  margin-top: 12px; font-size: 12px; color: #475569;
}

.sp-thanks {
  margin-top: 2em; text-align: center;
  font-size: 13px; color: #475569; letter-spacing: .03em;
}
</style>

<div class="lang-en">

<div class="sp-hero-card">
<div class="sp-hero-ava">👨‍💻</div>
<div class="sp-hero-body">
<h3>A solo project</h3>
<p>SeraPlot is built <strong>entirely on my own</strong>, on top of a day job. I regularly rework the core of the framework to keep improving it — not because I'm the best at low-level optimisation, but because I care about delivering a <strong>complete solution</strong> that originally answered my own needs and hopefully answers yours too.</p>
<div class="sp-goals">
<span class="sp-goal">Ultra-performant</span>
<span class="sp-goal">Fully customisable</span>
<span class="sp-goal">Complete</span>
</div>
</div>
</div>

## Get in touch

<div class="sp-contact-card">
<div class="sp-card-head">✉️ Email only</div>
<p>If you'd like a specific mechanic, feature or chart type, don't hesitate to reach out — <strong>by email only</strong>. I can't promise I'll build everything, but I'll do as much as I can.</p>
<div class="sp-email-row">
<a class="sp-email-link" href="mailto:feur09@gmail.com">📧 feur09@gmail.com</a>
<button class="sp-copy-btn" id="sp-copy-en" onclick="spCopyEmail('feur09@gmail.com','sp-copy-en','✓ Copied!')">Copy address</button>
</div>
</div>

## Support the project

<div class="sp-donate-card">
<div class="sp-card-head">☕ Buy me a coffee</div>
<p>I work on SeraPlot <strong>for free</strong>, on top of my day job. If it saves you time, a donation is very welcome — but you absolutely <strong>don't need to donate</strong> to send me a feature request.</p>
<a class="sp-donate-btn" href="https://paypal.me/seraplot" target="_blank" rel="noopener">
  <svg width="18" height="18" viewBox="0 0 24 24" fill="currentColor" style="opacity:.9"><path d="M7.076 21.337H2.47a.641.641 0 01-.633-.74L4.944.901C5.026.382 5.474 0 5.998 0h7.46c2.57 0 4.578.543 5.69 1.81 1.01 1.15 1.304 2.42 1.012 4.287-.023.143-.047.288-.077.437-.983 5.05-4.349 6.797-8.647 6.797h-2.19c-.524 0-.968.382-1.05.9l-1.12 7.106zm14.146-14.42a3.35 3.35 0 00-.607-.541c-.013.076-.026.175-.041.254-.93 4.778-4.005 7.201-9.138 7.201h-2.19a.563.563 0 00-.556.479l-1.187 7.527h-.506l-.24 1.516a.56.56 0 00.554.647h3.882c.46 0 .85-.334.922-.788.06-.26.76-4.852.816-5.09a.932.932 0 01.923-.788h.58c3.76 0 6.705-1.528 7.565-5.946.36-1.847.174-3.388-.777-4.471z"/></svg>
  Donate via PayPal
</a>
</div>

<p class="sp-thanks">Thanks for using SeraPlot ✨</p>

</div>

<div class="lang-fr">

<div class="sp-hero-card">
<div class="sp-hero-ava">👨‍💻</div>
<div class="sp-hero-body">
<h3>Un projet en solo</h3>
<p>SeraPlot est <strong>entièrement réalisé seul</strong>, en plus d'un travail à plein temps. Je retravaille régulièrement le corps du framework pour continuer à l'améliorer — pas parce que je suis le meilleur en optimisation bas-niveau, mais parce que je tiens à offrir une <strong>solution complète</strong> qui répondait au départ à mes propres besoins et qui répond, j'espère, aussi aux vôtres.</p>
<div class="sp-goals">
<span class="sp-goal">Ultra-performant</span>
<span class="sp-goal">Entièrement personnalisable</span>
<span class="sp-goal">Complet</span>
</div>
</div>
</div>

## Me contacter

<div class="sp-contact-card">
<div class="sp-card-head">✉️ Par mail uniquement</div>
<p>Si vous voulez une mécanique, une fonctionnalité ou un type de graphique particulier, n'hésitez pas — <strong>par mail exclusivement</strong>. Je ne dis pas que je serai en capacité de tout réaliser, mais je ferai le plus possible.</p>
<div class="sp-email-row">
<a class="sp-email-link" href="mailto:feur09@gmail.com">📧 feur09@gmail.com</a>
<button class="sp-copy-btn" id="sp-copy-fr" onclick="spCopyEmail('feur09@gmail.com','sp-copy-fr','✓ Copié !')">Copier l'adresse</button>
</div>
</div>

## Soutenir le projet

<div class="sp-donate-card">
<div class="sp-card-head">☕ Offrez-moi un café</div>
<p>Je travaille sur SeraPlot <strong>gratuitement</strong>, en plus de mon travail. Si cela vous fait gagner du temps, un don est le bienvenu — mais il n'y a <strong>aucun besoin de faire un don</strong> pour me faire une demande de fonctionnalité.</p>
<a class="sp-donate-btn" href="https://paypal.me/seraplot" target="_blank" rel="noopener">
  <svg width="18" height="18" viewBox="0 0 24 24" fill="currentColor" style="opacity:.9"><path d="M7.076 21.337H2.47a.641.641 0 01-.633-.74L4.944.901C5.026.382 5.474 0 5.998 0h7.46c2.57 0 4.578.543 5.69 1.81 1.01 1.15 1.304 2.42 1.012 4.287-.023.143-.047.288-.077.437-.983 5.05-4.349 6.797-8.647 6.797h-2.19c-.524 0-.968.382-1.05.9l-1.12 7.106zm14.146-14.42a3.35 3.35 0 00-.607-.541c-.013.076-.026.175-.041.254-.93 4.778-4.005 7.201-9.138 7.201h-2.19a.563.563 0 00-.556.479l-1.187 7.527h-.506l-.24 1.516a.56.56 0 00.554.647h3.882c.46 0 .85-.334.922-.788.06-.26.76-4.852.816-5.09a.932.932 0 01.923-.788h.58c3.76 0 6.705-1.528 7.565-5.946.36-1.847.174-3.388-.777-4.471z"/></svg>
  Soutenir via PayPal
</a>
</div>

<p class="sp-thanks">Merci d'utiliser SeraPlot ✨</p>

</div>

<script>
function spCopyEmail(text, btnId, label) {
  var btn = document.getElementById(btnId);
  var orig = btn.textContent;
  function done() {
    btn.textContent = label;
    btn.classList.add('sp-copied');
    setTimeout(function () { btn.textContent = orig; btn.classList.remove('sp-copied'); }, 2200);
  }
  if (navigator.clipboard && navigator.clipboard.writeText) {
    navigator.clipboard.writeText(text).then(done).catch(function () {
      var ta = document.createElement('textarea');
      ta.value = text; document.body.appendChild(ta); ta.select();
      document.execCommand('copy'); document.body.removeChild(ta); done();
    });
  } else {
    var ta = document.createElement('textarea');
    ta.value = text; document.body.appendChild(ta); ta.select();
    document.execCommand('copy'); document.body.removeChild(ta); done();
  }
}
</script>
