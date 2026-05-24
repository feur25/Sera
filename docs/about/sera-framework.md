# Sera — Le Framework

<style>
.sf-hero{margin:1.4em 0 2.2em;padding:32px 36px;border-radius:16px;background:linear-gradient(140deg,#050c1a 0%,#0d1426 40%,#131c35 80%,#0a1020 100%);border:1px solid rgba(99,102,241,.25);position:relative;overflow:hidden;box-shadow:0 24px 60px -20px rgba(0,0,0,.8)}
.sf-hero::before{content:"";position:absolute;top:-30%;right:-8%;width:55%;height:180%;background:radial-gradient(ellipse,rgba(99,102,241,.12) 0%,transparent 65%);pointer-events:none}
.sf-hero::after{content:"";position:absolute;bottom:-20%;left:-5%;width:40%;height:120%;background:radial-gradient(ellipse,rgba(34,211,238,.06) 0%,transparent 60%);pointer-events:none}
.sf-hero-inner{position:relative;z-index:1}
.sf-hero h1{margin:0 0 10px;font-size:30px;background:linear-gradient(135deg,#a5b4fc 0%,#67e8f9 50%,#f0abfc 100%);-webkit-background-clip:text;background-clip:text;color:transparent;font-weight:800;letter-spacing:-.02em;border:none}
.sf-hero p{margin:0;color:#94a3b8;font-size:15px;line-height:1.65;max-width:72ch}

.sf-grid{display:grid;grid-template-columns:repeat(auto-fit,minmax(280px,1fr));gap:16px;margin:2em 0}
.sf-card{padding:22px 24px;background:#070d1c;border:1px solid #1a2540;border-radius:12px;transition:border-color .15s,transform .15s}
.sf-card:hover{border-color:#3730a3;transform:translateY(-2px)}
.sf-card h3{margin:0 0 10px;font-size:14px;color:#a5b4fc;font-weight:700;letter-spacing:.06em;text-transform:uppercase;border:none;display:flex;align-items:center;gap:8px}
.sf-card p{margin:0;color:#64748b;font-size:13px;line-height:1.6}
.sf-card ul{margin:8px 0 0;padding-left:16px;color:#64748b;font-size:13px;line-height:1.7}
.sf-card li{margin:0}

.sf-arch{margin:2em 0;padding:24px 28px;background:#060c1a;border:1px solid #1a2540;border-radius:12px}
.sf-arch h2{margin:0 0 16px;font-size:15px;color:#e2e8f0;font-weight:700;border:none}
.sf-layer{display:flex;align-items:stretch;margin:6px 0;border-radius:8px;overflow:hidden}
.sf-layer-name{flex-shrink:0;width:160px;padding:12px 16px;font-size:12px;font-weight:700;letter-spacing:.08em;text-transform:uppercase;display:flex;align-items:center}
.sf-layer-desc{flex:1;padding:12px 16px;font-size:13px;line-height:1.5;color:#94a3b8;display:flex;align-items:center;border-left:1px solid rgba(255,255,255,.06)}
.sf-l-app{background:linear-gradient(90deg,rgba(99,102,241,.18),rgba(99,102,241,.06));color:#a5b4fc}
.sf-l-api{background:linear-gradient(90deg,rgba(34,211,238,.15),rgba(34,211,238,.04));color:#67e8f9}
.sf-l-core{background:linear-gradient(90deg,rgba(240,171,252,.15),rgba(240,171,252,.04));color:#f0abfc}
.sf-l-sys{background:linear-gradient(90deg,rgba(251,146,60,.15),rgba(251,146,60,.04));color:#fb923c}

.sf-products{display:flex;flex-wrap:wrap;gap:14px;margin:2em 0}
.sf-prod{flex:1;min-width:220px;padding:20px 22px;border-radius:12px;border:1px solid transparent}
.sf-prod-plot{background:linear-gradient(135deg,rgba(99,102,241,.12),rgba(99,102,241,.04));border-color:rgba(99,102,241,.3)}
.sf-prod-ml{background:linear-gradient(135deg,rgba(34,211,238,.1),rgba(34,211,238,.03));border-color:rgba(34,211,238,.25)}
.sf-prod-df{background:linear-gradient(135deg,rgba(240,171,252,.1),rgba(240,171,252,.03));border-color:rgba(240,171,252,.2);opacity:.65}
.sf-prod h3{margin:0 0 8px;font-size:16px;font-weight:800;border:none}
.sf-prod-plot h3{color:#818cf8}
.sf-prod-ml h3{color:#22d3ee}
.sf-prod-df h3{color:#e879f9}
.sf-prod p{margin:0;color:#64748b;font-size:13px;line-height:1.55}
.sf-prod .sf-status{display:inline-block;margin-top:10px;padding:2px 8px;border-radius:999px;font-size:10px;font-weight:700;letter-spacing:.06em}
.sf-s-live{background:rgba(34,197,94,.15);color:#4ade80;border:1px solid rgba(34,197,94,.3)}
.sf-s-soon{background:rgba(251,146,60,.1);color:#fb923c;border:1px solid rgba(251,146,60,.25)}

.sf-quote{margin:2em 0;padding:20px 24px;background:rgba(99,102,241,.06);border-left:4px solid #6366f1;border-radius:0 10px 10px 0;color:#cbd5e1;font-size:14.5px;line-height:1.65;font-style:italic}
.sf-quote strong{color:#a5b4fc;font-style:normal}
</style>

<div class="lang-en">

<div class="sf-hero">
<div class="sf-hero-inner">
<h1>Sera Framework</h1>
<p>Ce document est une explication approfondie des outils que vous serez amené à utiliser en tant que consommateur de <strong style="color:#a5b4fc">Sera</strong>. Un framework de bas niveau, écrit en Rust, sur lequel reposent SeraPlot, SeraML et SeraDFrame.</p>
</div>
</div>

## What is Sera?

<div class="sf-quote">
Sera is a low-level Rust framework on which all three tools rest. It provides CPU/GPU optimisation, memory management, inter-driver allocation, parallel threads, and a macro system for method registration — all natively inherited by every new method added to the framework.
</div>

You have likely noticed: **Sera** is the first word of every product name — SeraPlot, SeraML, SeraDFrame. That prefix is intentional. The three tools are not independent libraries; they are surface layers built on top of the same Rust core, sharing the same allocator, the same cache infrastructure, the same rendering pipeline, and the same macro registry.

<div class="sf-products">

<div class="sf-prod sf-prod-plot">
<h3>SeraPlot</h3>
<p>High-performance data visualisation — 60+ chart types, zero dependencies, 6,000× faster than Plotly. Ships as Python wheel, npm package, WASM, and native Rust crate.</p>
<span class="sf-status sf-s-live">LIVE — pypi.org/project/seraplot</span>
</div>

<div class="sf-prod sf-prod-ml">
<h3>SeraML</h3>
<p>Drop-in Scikit-learn replacement in Rust — KMeans, SVM, GradientBoosting, PCA, GridSearch, full metrics suite. 2–686× faster. GPU and distributed backends built-in.</p>
<span class="sf-status sf-s-live">LIVE — pypi.org/project/seraml</span>
</div>

<div class="sf-prod sf-prod-df">
<h3>SeraDFrame</h3>
<p>A complete Pandas / Polars alternative — columnar data engine, lazy evaluation, native Rust types, zero-copy interop with SeraPlot and SeraML.</p>
<span class="sf-status sf-s-soon">Q4 2027</span>
</div>

</div>

## Architecture

<div class="sf-arch">
<h2>Layers</h2>

<div class="sf-layer">
<div class="sf-layer-name sf-l-app">SeraPlot / SeraML / SeraDFrame</div>
<div class="sf-layer-desc">User-facing APIs — Python, JavaScript, TypeScript, Rust. Each product exposes the Sera capabilities relevant to its domain through language-native bindings.</div>
</div>
<div class="sf-layer">
<div class="sf-layer-name sf-l-api">Bindings Layer</div>
<div class="sf-layer-desc">PyO3 (Python), wasm-bindgen (WASM/JS), C FFI. The same Rust structs are serialised to JSON and passed through a unified builder — no duplicated logic per language.</div>
</div>
<div class="sf-layer">
<div class="sf-layer-name sf-l-core">Sera Core</div>
<div class="sf-layer-desc">Allocator, cache, render pipeline, macro registry. Every chart, model, and transformation is a registered entry — the registry is generated at compile time by <code>build.rs</code> text scanning, zero runtime overhead.</div>
</div>
<div class="sf-layer">
<div class="sf-layer-name sf-l-sys">System / Hardware</div>
<div class="sf-layer-desc">CPU SIMD, GPU compute (CUDA / Metal / Vulkan via wgpu), parallel threads (rayon), memory arenas, inter-driver allocation. Transparent to the user — activated by feature flags.</div>
</div>
</div>

## What the Sera core provides

<div class="sf-grid">

<div class="sf-card">
<h3>⚡ CPU / GPU / RAM</h3>
<p>Native SIMD vectorisation, GPU compute shaders for ML-heavy operations, arena allocators to avoid heap fragmentation on large datasets.</p>
</div>

<div class="sf-card">
<h3>🧠 Cache system</h3>
<p>LRU-based render cache — identical inputs return a cached HTML string without re-rendering. Useful for streaming dashboards and repeated chart variants.</p>
</div>

<div class="sf-card">
<h3>🔗 Allocation (inter-drivers)</h3>
<p>A unified memory pool shared across chart, ML, and data operations avoids redundant copies when passing tensors between the rendering engine and the ML pipeline.</p>
</div>

<div class="sf-card">
<h3>🧵 Parallel threads</h3>
<p>Rayon thread pools parallelise both chart rendering (multi-series) and ML training (tree ensembles, grid search). The pool size adapts to the available cores at runtime.</p>
</div>

<div class="sf-card">
<h3>🏷️ Macro registry</h3>
<p>Every method — chart builder, ML model, data transformer — is registered at compile time via <code>build.rs</code> scanning. The registry is a static array, zero-allocation at runtime.</p>
<ul>
<li>Python bindings auto-generated via PyO3</li>
<li>WASM exports auto-generated via wasm-bindgen</li>
<li>Documentation metadata extracted at compile time</li>
</ul>
</div>

<div class="sf-card">
<h3>🎯 Unified builder</h3>
<p>All nine language bindings share a single JSON-over-FFI protocol. One Rust struct → one JSON schema → Python kwargs, JS object, TypeScript interface, Rust builder — all consistent.</p>
</div>

</div>

## Why Rust?

Rust gives Sera three properties that are very difficult to achieve simultaneously in Python, JavaScript, or C++:

1. **Memory safety without a garbage collector** — no GC pauses on large datasets, deterministic latency.
2. **Zero-cost abstractions** — the macro registry, the unified builder, the arena allocator add zero runtime overhead vs hand-written C.
3. **First-class cross-compilation** — the same crate compiles to native code (Python wheel), to WebAssembly (browser playground), and to a C shared library (future SeraDFrame FFI) with a single `Cargo.toml`.

</div>

<div class="lang-fr">

<div class="sf-hero">
<div class="sf-hero-inner">
<h1>Le Framework Sera</h1>
<p>Ce document est une explication plus complète concernant les outils que vous serez amené à utiliser si vous êtes consommateur de <strong style="color:#a5b4fc">Sera</strong>. Alors qu'est-ce que c'est Sera ?</p>
</div>
</div>

## Qu'est-ce que Sera ?

<div class="sf-quote">
Vous êtes censé l'avoir remarqué : c'est le premier mot de <strong>SeraPlot</strong>, <strong>SeraML</strong>, ou encore <strong>SeraDFrame</strong> — qui sera rendu public fin 2027, alternative complète à Pandas/Polars, à l'instar de SeraPlot pour Plotly &amp; Matplotlib, et SeraML pour tout ce qui relève du machine learning.
</div>

Sera est un **framework de bas niveau codé en Rust** sur lequel mes trois outils reposent. On y retrouvera toute la partie optimisation CPU – GPU – RAM, la mise en cache, le système d'allocation (inter-drivers), les threads parallèles — mais aussi toute la partie macro de méthode register, qui sont toutes nativement héritées dans chaque nouvelle méthode du Framework que je développe.

<div class="sf-products">

<div class="sf-prod sf-prod-plot">
<h3>SeraPlot</h3>
<p>Visualisation haute performance — 60+ types de charts, zéro dépendance, 6 000× plus rapide que Plotly. Disponible en wheel Python, package npm, WASM et crate Rust natif.</p>
<span class="sf-status sf-s-live">LIVE — pypi.org/project/seraplot</span>
</div>

<div class="sf-prod sf-prod-ml">
<h3>SeraML</h3>
<p>Remplacement drop-in de Scikit-learn en Rust — KMeans, SVM, GradientBoosting, PCA, GridSearch, métriques complètes. 2–686× plus rapide. Backends GPU et distribué inclus.</p>
<span class="sf-status sf-s-live">LIVE — pypi.org/project/seraml</span>
</div>

<div class="sf-prod sf-prod-df">
<h3>SeraDFrame</h3>
<p>Alternative complète à Pandas/Polars — moteur colonaire, évaluation paresseuse, types Rust natifs, interop zero-copy avec SeraPlot et SeraML.</p>
<span class="sf-status sf-s-soon">Q4 2027</span>
</div>

</div>

## Architecture

<div class="sf-arch">
<h2>Couches</h2>

<div class="sf-layer">
<div class="sf-layer-name sf-l-app">SeraPlot / SeraML / SeraDFrame</div>
<div class="sf-layer-desc">APIs orientées utilisateur — Python, JavaScript, TypeScript, Rust. Chaque produit expose les capacités Sera pertinentes pour son domaine via des bindings natifs au langage.</div>
</div>
<div class="sf-layer">
<div class="sf-layer-name sf-l-api">Couche Bindings</div>
<div class="sf-layer-desc">PyO3 (Python), wasm-bindgen (WASM/JS), C FFI. Les mêmes structs Rust sont sérialisées en JSON et passées via un builder unifié — aucune logique dupliquée par langage.</div>
</div>
<div class="sf-layer">
<div class="sf-layer-name sf-l-core">Sera Core</div>
<div class="sf-layer-desc">Allocateur, cache, pipeline de rendu, registre de macros. Chaque chart, modèle et transformation est une entrée enregistrée — le registre est généré à la compilation par scan <code>build.rs</code>, zéro overhead runtime.</div>
</div>
<div class="sf-layer">
<div class="sf-layer-name sf-l-sys">Système / Matériel</div>
<div class="sf-layer-desc">SIMD CPU, compute GPU (CUDA / Metal / Vulkan via wgpu), threads parallèles (rayon), arènes mémoire, allocation inter-drivers. Transparent pour l'utilisateur — activé par feature flags.</div>
</div>
</div>

## Ce que le core Sera fournit

<div class="sf-grid">

<div class="sf-card">
<h3>⚡ CPU / GPU / RAM</h3>
<p>Vectorisation SIMD native, compute shaders GPU pour les opérations ML intensives, allocateurs arène pour éviter la fragmentation heap sur les grands datasets.</p>
</div>

<div class="sf-card">
<h3>🧠 Système de cache</h3>
<p>Cache de rendu LRU — des entrées identiques retournent une chaîne HTML mise en cache sans re-rendu. Utile pour les dashboards streaming et les variants de charts répétés.</p>
</div>

<div class="sf-card">
<h3>🔗 Allocation (inter-drivers)</h3>
<p>Un memory pool unifié partagé entre chart, ML et opérations data évite les copies redondantes lors du passage de tenseurs entre moteur de rendu et pipeline ML.</p>
</div>

<div class="sf-card">
<h3>🧵 Threads parallèles</h3>
<p>Les thread pools Rayon parallélisent le rendu de charts (multi-séries) et l'entraînement ML (ensembles d'arbres, grid search). La taille du pool s'adapte aux cœurs disponibles au runtime.</p>
</div>

<div class="sf-card">
<h3>🏷️ Registre de macros</h3>
<p>Chaque méthode — builder de chart, modèle ML, transformateur data — est enregistrée à la compilation via scan <code>build.rs</code>. Le registre est un tableau statique, zéro allocation au runtime.</p>
<ul>
<li>Bindings Python auto-générés via PyO3</li>
<li>Exports WASM auto-générés via wasm-bindgen</li>
<li>Métadonnées de documentation extraites à la compilation</li>
</ul>
</div>

<div class="sf-card">
<h3>🎯 Builder unifié</h3>
<p>Les neuf bindings de langage partagent un unique protocole JSON-over-FFI. Un seul struct Rust → un seul schéma JSON → kwargs Python, objet JS, interface TypeScript, builder Rust — tous cohérents.</p>
</div>

</div>

## Pourquoi Rust ?

Rust donne à Sera trois propriétés très difficiles à obtenir simultanément en Python, JavaScript ou C++ :

1. **Sécurité mémoire sans GC** — aucune pause GC sur les grands datasets, latence déterministe.
2. **Abstractions zero-cost** — le registre de macros, le builder unifié, l'allocateur arène n'ajoutent aucun overhead runtime vs du C écrit à la main.
3. **Cross-compilation first-class** — le même crate compile vers du code natif (wheel Python), WebAssembly (playground navigateur) et bibliothèque partagée C (futur FFI SeraDFrame) avec un seul `Cargo.toml`.

</div>
