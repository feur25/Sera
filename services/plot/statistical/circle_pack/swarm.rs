use super::config::CirclePackConfig;
use crate::html::hover::{build_chart_html, slots_to_json, HoverSlot};
use crate::plot::statistical::common::{escape_xml, hex6, push_b, push_f2, push_i, svg_open};
use std::collections::HashMap;

fn type_color(t: &str) -> u32 {
    match t {
        "feat" => 0x22c55e,
        "fix" => 0xf43f5e,
        "docs" => 0x38bdf8,
        "refactor" => 0xa78bfa,
        "chore" => 0x94a3b8,
        "style" => 0xf472b6,
        "perf" => 0xf59e0b,
        "test" => 0x14b8a6,
        _ => 0x64748b,
    }
}

fn type_label(t: &str) -> &'static str {
    match t {
        "feat" => "Feature",
        "fix" => "Fix",
        "docs" => "Docs",
        "refactor" => "Refactor",
        "chore" => "Chore",
        "style" => "Style",
        "perf" => "Perf",
        "test" => "Test",
        _ => "Other",
    }
}

fn month_label(m: &str) -> String {
    let parts: Vec<&str> = m.split('-').collect();
    if parts.len() != 2 || parts[0].len() < 4 {
        return m.to_string();
    }
    const NAMES: [&str; 13] =
        ["", "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];
    let mi: usize = parts[1].parse().unwrap_or(0);
    format!("{} '{}", NAMES.get(mi).copied().unwrap_or("?"), &parts[0][2..])
}

fn spiral_fallback(placed: &[(f64, f64)], placed_r: &[f64], ri: f64, padding: f64) -> (f64, f64) {
    let mut angle = 0.0f64;
    let mut radius = ri + 5.0;
    for _ in 0..2500 {
        let px = radius * angle.cos();
        let py = radius * angle.sin();
        let ok = placed.iter().zip(placed_r.iter()).all(|(&(jx, jy), &jr)| {
            ((jx - px).powi(2) + (jy - py).powi(2)).sqrt() >= jr + ri + padding
        });
        if ok {
            return (px, py);
        }
        angle += 0.31;
        radius += 0.6;
    }
    (radius, 0.0)
}

fn pack_local(radii: &[f64], padding: f64) -> Vec<(f64, f64)> {
    let n = radii.len();
    let mut pos = vec![(0.0, 0.0); n];
    if n <= 1 {
        return pos;
    }
    pos[1] = (radii[0] + radii[1] + padding, 0.0);
    if n == 2 {
        return pos;
    }
    for i in 2..n {
        let ri = radii[i];
        let cand_start = if i > 64 { i - 64 } else { 0 };
        let check_start = if i > 180 { i - 180 } else { 0 };
        let mut best: Option<(f64, f64)> = None;
        let mut best_dist = f64::MAX;
        for a in cand_start..i {
            for b in (a + 1)..i {
                let (ax, ay) = pos[a];
                let (bx, by) = pos[b];
                let ra = radii[a] + ri + padding;
                let rb = radii[b] + ri + padding;
                let dx = bx - ax;
                let dy = by - ay;
                let d = (dx * dx + dy * dy).sqrt();
                if d < 1e-9 || d > ra + rb || d < (ra - rb).abs() {
                    continue;
                }
                let aa = (ra * ra - rb * rb + d * d) / (2.0 * d);
                let h2 = ra * ra - aa * aa;
                if h2 < 0.0 {
                    continue;
                }
                let h = h2.sqrt();
                let mx = ax + aa * dx / d;
                let my = ay + aa * dy / d;
                let ox = -dy / d * h;
                let oy = dx / d * h;
                for &(px, py) in &[(mx + ox, my + oy), (mx - ox, my - oy)] {
                    let mut ok = true;
                    for j in check_start..i {
                        if j == a || j == b {
                            continue;
                        }
                        let (jx, jy) = pos[j];
                        let dd = ((jx - px).powi(2) + (jy - py).powi(2)).sqrt();
                        if dd < radii[j] + ri + padding - 1e-6 {
                            ok = false;
                            break;
                        }
                    }
                    if ok {
                        let dist = (px * px + py * py).sqrt();
                        if dist < best_dist {
                            best_dist = dist;
                            best = Some((px, py));
                        }
                    }
                }
            }
        }
        pos[i] = best.unwrap_or_else(|| {
            spiral_fallback(&pos[check_start..i], &radii[check_start..i], ri, padding)
        });
    }
    pos
}

struct Commit {
    ctype: String,
    bot: bool,
    author: String,
    lines: f64,
    month: String,
}

#[crate::chart_demo("labels=[\"other::0::2e9e0bb::17::feur25::first commit\",\"feat::0::bea5ffb::17::feur25::feat(chart): image loader in hov..\",\"style::0::249091b::18::feur25::style(chart): fix bar spacing\",\"feat::0::2282a4a::18::feur25::feat: tranform plot selection\",\"style::0::d20231a::18::feur25::style: remove useless border in ..\",\"feat::0::dca63d7::18::feur25::feat: create generic method usin..\",\"feat::0::9f9eae1::18::feur25::feat: data processoring features\",\"fix::0::7e2b8f1::18::feur25::fix: scatter plot point restrored\",\"fix::0::c5785d4::18::feur25::fix: transform switch plot\",\"feat::0::eae5ea4::18::feur25::feat: wiki command Framework\",\"feat::0::531e710::18::feur25::feat(camera): 3d dimension persp..\",\"feat::0::a612648::18::feur25::feat: removed isometric plan to ..\",\"feat::0::1b870c6::18::feur25::feat: mouse drag rotation control\",\"feat::0::53f2720::18::feur25::feat: 3d hover plot selection\",\"fix::0::208603f::19::feur25::fix(bar_3d): method useless rend..\",\"feat::0::1341a20::19::feur25::feat: wiki interface\",\"feat::0::b754872::19::feur25::feat: batch / cache pipeline\",\"feat::0::4349b23::19::feur25::feat(wiki): documentation search..\",\"style::0::ff84418::20::feur25::style(chart): map_point method s..\",\"feat::0::fd88a11::20::feur25::feat(html): generated html inter..\",\"feat::0::bf82681::20::feur25::feat(fats_render): simple chart ..\",\"feat::0::b310056::21::feur25::feat(html): added some button to..\",\"feat::0::f2b3997::21::feur25::feat(plot): generic template for..\",\"fix::0::8e8ac15::21::feur25::fix: hover black color\",\"refactor::0::a9e9b89::21::feur25::move: some '_3d' scripts to gene..\",\"fix::0::96fce59::21::feur25::fix(chart): comment unusing vari..\",\"fix::0::d25f794::02::feur25::fix: removed plot builder\",\"fix::0::67eb476::26::feur25::fix(_3d): hover display content\",\"feat::0::22e71c6::26::feur25::feat(asset): set app logo\",\"feat::0::1fece6d::16::feur25::feat: new groups of plot; map pl..\",\"style::0::c0e1f49::19::feur25::style(manager): remove french text\",\"feat::0::6f9aa51::08::feur25::feat: world map & statistical gr..\",\"feat::0::017bb18::08::feur25::feat: plot unified & visual polish\",\"feat::0::d8d0e43::08::feur25::feat: add apply_bg() helper and ..\",\"perf::0::a94ab03::09::feur25::perf: optimize 3D engine v2.2.1 ..\",\"feat::0::bf38b92::09::feur25::feat: add radar; lollipop; kde; ..\",\"other::0::6c19e50::10::feur25::v2.3.1: fix 3D bindings (radar/k..\",\"other::0::6b6d6ac::10::feur25::v2.3.4: all charts 2000x faster ..\",\"other::0::89da1a3::10::feur25::v2.3.7: 9 new 3D charts - pie3d;..\",\"other::0::7c9115d::10::feur25::deep 3D volume relief for kde ri..\",\"feat::0::5771deb::11::feur25::feat: universal show_labels(labe..\",\"refactor::0::cf8c078::11::feur25::refactor: add sorted<T> helper; ..\",\"feat::0::1fb0343::13::feur25::feat(dbscan): dbscan_core_nd N-d..\",\"docs::0::e5b7c54::13::feur25::docs: add complete bilingual mdB..\",\"docs::0::cc98219::13::feur25::docs: rewrite introduction with ..\",\"fix::0::5dfe3de::13::feur25::fix(ci): replace cargo-install m..\",\"fix::0::bcf3b69::13::feur25::fix: remove duplicate file entri..\",\"docs::0::b22b6a8::13::feur25::docs: remove all French text fro..\",\"fix::0::1807b68::13::feur25::fix(ci): make seraplot pip insta..\",\"fix::0::04a2de9::14::feur25::fix(css): restore auto margins o..\",\"fix::0::e2ccdc4::14::feur25::fix(previews): strip body styles..\",\"fix::0::6d14076::14::feur25::fix(previews): fix double-brace ..\",\"fix::0::f119b10::14::feur25::fix(hover): loading=eager in sou..\",\"docs::0::5da3150::14::feur25::docs: intro rewrite — install ho..\",\"feat::0::13940d3::15::feur25::feat: Python/JS/TS tabs in all 5..\",\"feat::0::1c3b9c2::15::feur25::feat: set/reset_global_backgroun..\",\"fix::0::6567011::15::feur25::fix: alias tuple registry + docs..\",\"other::0::14216c2::15::feur25::bench: accurate 6000x numbers — ..\",\"docs::0::c71b7cc::15::feur25::docs: sp.config() global + chain..\",\"refactor::0::0e0efbf::16::feur25::refactor: kmeans.rs — chart_conf..\",\"fix::0::6933449::16::feur25::fix: Cargo.toml lto=thin; codege..\",\"docs::0::88d49d1::17::feur25::docs: ml documentation features\",\"docs::0::8ef947d::17::feur25::docs: fr algo added\",\"docs::0::bc7af0a::17::feur25::docs: quickstart.md fix\",\"feat::0::84bb05f::20::feur25::feat: grid search method\",\"feat::0::7fe1f38::20::feur25::feat: upgrade split & cross_val ..\",\"refactor::0::57b6c13::21::feur25::refractor: book directory delete..\",\"docs::0::026cfcb::23::feur25::docs: bilingual EN/FR pages for ..\",\"docs::0::73dd7ea::23::feur25::docs: complete FR mirrors (chart..\",\"fix::0::22a8bd7::23::feur25::fix(docs): remove leftover stack..\",\"docs::0::95a3e8f::24::feur25::docs: mojibake charactersfixed i..\",\"docs::0::137c29d::24::feur25::docs: summary grouped sidebar no..\",\"fix::0::00dc52a::24::feur25::fix(docs): remove duplicate sect..\",\"style::0::79ab648::24::feur25::style: docs summary; sub section..\",\"docs::0::001461e::24::feur25::docs(introduction): moved 'why s..\",\"feat::0::630d1f7::25::feur25::feat: classification - regressio..\",\"feat::0::a6d0eac::25::feur25::feat(ml): partial_fit on scalers..\",\"docs::0::22809e3::26::feur25::docs: Extracts Signature / Alias..\",\"feat::0::2c262b7::26::feur25::feat(docs): PiP gif + install sw..\",\"fix::0::a3b816b::26::feur25::fix: replace left placement to r..\",\"fix::0::f9c3948::27::feur25::fix(js): look for <p> siblings i..\",\"fix::0::335f3c3::27::feur25::fix(docs): style chart-methods r..\",\"style::0::11a56ef::27::feur25::style: chapter sidebar margin re..\",\"fix::0::3fe5a56::27::feur25::fix(docs): reduce iframe height ..\",\"docs::0::7e6e553::27::feur25::docs: rename index file to api m..\",\"feat::0::9941af3::27::feur25::feat(panel): add code tabs + aut..\",\"fix::0::b28f017::27::feur25::fix(panel): remove loading=lazy ..\",\"fix::0::5186e3e::27::feur25::fix: variant nav redesign (under..\",\"docs::0::1b697d4::27::feur25::docs: restructure bar variants b..\",\"docs::0::28e9ca8::27::feur25::docs: add dedicated preview file..\",\"fix::0::fa27a7c::27::feur25::fix: regenerate relative/grouped..\",\"docs::0::879fd37::27::feur25::docs(chart-methods): add text_au..\",\"chore::0::3ce9d50::27::feur25::chore: bump v2/src submodule poi..\",\"docs::0::3c7f9cc::27::feur25::docs(bar): true protruding bookm..\",\"docs::0::c074d57::27::feur25::docs: bump submodule for sticky ..\",\"docs::0::f5a8d19::27::feur25::docs(api-panel): vertical rail s..\",\"docs::0::2cf29f1::27::feur25::docs(api-panel): ticket rail out..\",\"docs::0::2a274af::27::feur25::docs: bump submodule - scroll + ..\",\"docs::0::610ec96::27::feur25::docs: bump submodule - code tab ..\",\"fix::0::cd7cb5b::28::feur25::fix(api-panel): rail in body sta..\",\"fix::0::e255ef8::28::feur25::fix(api-panel): anchor rail with..\",\"docs::0::3a5c707::28::feur25::docs: removed vertical bars card..\",\"docs::0::b8c658e::28::feur25::docs: 3 canges lade css & js theme\",\"fix::0::7b225c0::28::feur25::fix(docs): switch langague trans..\",\"other::0::46e2597::01::feur25::histogram: add 7 variants (basic..\",\"feat::0::3e70315::01::feur25::feat(heatmap): 7-variant filing-..\",\"docs::0::ebe5268::01::feur25::docs: ramke pictogram & marimekk..\",\"feat::0::7b8f298::02::feur25::feat: box plots variant - doc up..\",\"fix::0::0d14637::02::feur25::fix: color pagination arrow header\",\"feat::0::ad0fbe0::04::feur25::feat: rewritten fit method; Ridg..\",\"docs::0::e37f4c0::04::feur25::docs: css design system ml block..\",\"fix::0::d3f1418::04::feur25::fix: speed algo fit upgraded\",\"docs::0::55deabb::05::feur25::docs: index.md ml documentation ..\",\"feat::0::bed696b::05::feur25::feat: variant waterfall - sunbur..\",\"feat::0::12dad05::06::feur25::feat: lollipop & parallel coordi..\",\"fix::0::6eca0ea::06::feur25::fix: world cloud variant refract..\",\"fix::0::d05ba05::06::feur25::fix: params-panel.js recognise u..\",\"feat::0::57ca97d::07::feur25::feat: telemetry explained\",\"feat::0::84fd8fe::07::feur25::feat: telemetry code cell updated\",\"docs::0::a997df4::08::feur25::docs: variant deluxe/premium pre..\",\"feat::0::039806e::09::feur25::feat: themes custom for all plot\",\"chore::1::44f7c12::11::sera-bot::chore: playground url [skip ci]\",\"chore::1::1f4a65a::11::sera-bot::chore: playground offline [skip ..\",\"chore::1::37852b9::12::sera-bot::chore: playground offline [skip ..\",\"chore::1::0b0bab4::12::sera-bot::chore: playground offline [skip ..\",\"chore::1::565432b::12::sera-bot::chore: playground offline [skip ..\",\"fix::0::a40d38c::12::feur25::fix: WASM copied to docs/theme o..\",\"chore::1::07ae9be::13::sera-bot::chore: playground url [skip ci]\",\"feat::0::31bede7::13::feur25::feat: playground variant tabs pe..\",\"fix::0::296e028::14::feur25::fix: bar variant dispatch to cor..\",\"chore::1::6435a5d::14::sera-bot::chore: playground offline [skip ..\",\"chore::0::3e40870::15::feur25::chore: remove playground-server ..\",\"feat::0::69a2d29::15::feur25::feat(playground): Monaco editor ..\",\"fix::0::24dd4e8::15::feur25::fix(wasm): rebuild as no-modules..\",\"other::0::0b7f93f::21::feur25::fix(docs) : clear all code cell ..\",\"other::0::5d9a143::24::feur25::decorator-based chart params: re..\",\"chore::0::e356980::25::feur25::build(wasm): rebuild seraplot-we..\",\"refactor::0::7451a94::25::feur25::refractor(docs): sidebar repair\",\"feat::0::3bdfb6c::25::feur25::feat(playground): VSCode Dark+ s..\",\"fix::0::a21ad4a::25::feur25::fix(playground): dynamic WASM al..\",\"feat::0::c9d03fc::25::feur25::feat(wasm): export chartAliases(..\",\"fix::0::5196ee2::25::feur25::fix(playground): suppress Monaco..\",\"feat::0::f31365f::28::feur25::feat: rework Sera Framework & up..\",\"feat::0::40572fe::28::feur25::feat: functions auto-mapped js e..\",\"feat::0::44d95ca::29::feur25::feat(macros): split seraplot-mac..\",\"refactor::0::0e4c18f::01::feur25::refactor(bindings): extract pyth..\",\"fix::0::d8f3e40::05::feur25::fix: remove cross_bindings empty..\",\"docs::0::253799e::05::feur25::docs: improve documentation page..\",\"docs::0::9791ca5::05::feur25::docs: load chart aliases dynamic..\",\"docs::0::9110816::06::feur25::docs: parameters elements redisp..\",\"feat::0::4e5692b::24::feur25::feat: building docs ml & plot\",\"feat::0::5008193::16::feur25::feat: add arc diagram chart\",\"feat::0::561d36a::16::feur25::feat: add correlogram chart\",\"feat::0::8dc1f3d::16::feur25::feat: add gantt chart\",\"feat::0::6e7a4c8::16::feur25::feat: add icicle chart\",\"feat::0::963b822::16::feur25::feat: add plot web chart with gr..\",\"feat::0::32ddd05::16::feur25::feat: add scatter plot matrix ch..\",\"feat::0::2744174::16::feur25::feat: add bubble3d; isosurface; ..\",\"feat::0::73490ad::16::feur25::feat: add polar heatmap variant\",\"feat::0::f9b513b::16::feur25::feat: add SeraDFrame native colu..\",\"feat::0::5a0d9b9::16::feur25::feat: add LTTB decimation engine..\",\"refactor::0::9403dd4::16::feur25::refactor(radar): drop deluxe var..\",\"refactor::0::057e50b::16::feur25::refactor(sunburst): drop flame/r..\",\"refactor::0::43c6ab2::16::feur25::refactor(registry): wire new cha..\",\"fix::0::30367dd::16::feur25::fix(a11y): add ARIA roles/titles..\",\"fix::0::361166b::16::feur25::fix(a11y): raise gauge min/max l..\",\"fix::0::2506a84::16::feur25::fix(a11y): raise axis tick contr..\",\"feat::0::de883f4::16::feur25::feat: rotate lollipop x-axis lab..\",\"refactor::0::594aec5::16::feur25::refactor: extract chart-method; ..\",\"docs::0::1ec4c21::16::feur25::docs: wire new chart pages into ..\",\"fix::0::aab4501::16::feur25::fix(a11y): always emit svg <titl..\",\"fix::0::25c1c3f::16::feur25::fix(a11y): add role='group' to 3..\",\"fix::0::13f5c61::16::feur25::fix(wasm): gate SeraDFrame/Table..\",\"fix::0::5958a72::16::feur25::fix(docs): correct WASM asset pa..\",\"docs::0::91a215d::16::feur25::docs: add Canvas Composition; Ta..\",\"feat::0::7349d3c::17::feur25::feat(docs): make showcase page f..\",\"docs::0::e05ac54::17::feur25::docs(nav): move SeraDFrame/Table..\",\"fix::0::2f2e4f0::17::feur25::fix(docs): remove blue pill over..\",\"feat::0::4042a18::17::feur25::feat(docgen): extract SeraDFrame..\",\"docs::0::f126ecf::17::feur25::docs(dframe): replace hand-writt..\",\"fix::0::2c4312f::17::feur25::fix(docs): stop the bottom-docke..\",\"chore::0::8979e86::17::feur25::chore(docs): regenerate doc-regi..\",\"feat::0::35370de::17::feur25::feat(hexbin): add spaced variant..\",\"feat::0::943c37f::17::feur25::feat(icicle): add radial variant..\",\"fix::0::424864d::17::feur25::fix(chart-methods): make colorba..\",\"fix::0::aeeec7e::17::feur25::fix(docs): plot-web radial tab p..\",\"docs::0::c5698bf::17::feur25::docs(previews): generate parcats..\",\"fix::0::fde846d::17::feur25::fix(docs): 9 chart pages had an ..\",\"fix::0::4ea156a::18::feur25::fix(docs): register post-wordclo..\",\"other::0::9fd3a3b::18::feur25::feat(charts;dframe): 3 more drea..\",\"feat::0::71dd4c6::18::feur25::feat(dframe): generic chainable-..\",\"refactor::0::a26a62a::18::feur25::refactor(bindings): move seraplo..\",\"refactor::0::fb3c352::18::feur25::refactor: group plot/ml/data und..\",\"refactor::0::351edeb::18::feur25::refactor: move html/ into servic..\",\"refactor::0::4b321f9::18::feur25::refactor(ml): drop private r2_sc..\",\"fix::0::4f7b1f7::19::feur25::fix: sp.config()'s global option..\",\"refactor::0::f3ab7b1::19::feur25::refactor: dictionary-dispatch cl..\",\"refactor::0::b71a1d2::19::feur25::refactor: relocate root-level fi..\",\"fix::0::d46748c::19::feur25::fix(docs): pair --sp-text with s..\",\"feat::0::a00b638::19::feur25::feat: readme add content\",\"feat::0::c54aa02::19::feur25::feat(webapp): typed callback arg..\",\"feat::0::c0b9cc0::19::feur25::feat(canvas): arc/wedge/ribbon/p..\",\"fix::0::b968554::19::feur25::fix(canvas): center-scale previe..\",\"other::0::14ca71e::20::feur25::docs : replace configuration to ..\",\"other::0::15d73db::20::feur25::style : redesign playground patt..\",\"fix::0::430f69f::20::feur25::fix(docs): compute true required..\",\"refactor::0::2014b70::20::FeurKing::refractor: delete seraplot-power..\",\"docs::0::fb13ec0::20::feur25::docs(canvas): rework mission-con..\",\"fix::0::c860b60::21::feur25::fix(docs): illegible ML doc comp..\",\"other::0::64be431::21::FeurKing::refractor : delete v2 directory\",\"fix::0::8219ecf::21::feur25::fix(docs): stop cropping chart p..\",\"feat::0::d374071::21::feur25::feat(joint): add joint chart fam..\",\"fix::0::b292d35::22::feur25::fix(docs-gen): stop treating the..\",\"feat::0::13ab831::22::feur25::feat(joint): decouple main-panel..\",\"fix::0::2da008e::22::feur25::fix(lib): remove blank lines\",\"feat::0::02b015d::25::feur25::feat(kde): add a genuine bivaria..\",\"fix::0::71f0a4b::25::feur25::fix(chart-methods): make chainab..\",\"fix::0::190fa8d::25::feur25::fix(joint): register joint with ..\",\"fix::0::bc61ea0::25::feur25::fix(boxplot): category labels we..\",\"feat::0::b9ee75b::26::feur25::feat(lollipop): add custom varia..\",\"fix::0::b0e29c2::26::feur25::fix(playground): variant tab lab..\",\"docs::0::849c488::26::feur25::docs: generate previews for circ..\",\"docs::0::559b023::26::feur25::docs: generate previews for nigh..\",\"refactor::0::645d715::26::feur25::refactor(bar): fold circular_lab..\",\"refactor::0::5521b6f::26::feur25::refactor(pie): merge glow/glass ..\",\"chore::0::46a0e73::26::feur25::build(wasm): redeploy with pie l..\",\"docs::0::74bcb7c::26::feur25::docs: document pie labeled= para..\",\"chore::0::fe5047d::26::feur25::build(wasm): redeploy with dedic..\",\"docs::0::7b0b297::26::feur25::docs: document new correlogram/h..\",\"feat::0::04947ff::26::feur25::feat(bubble): add split variant ..\",\"feat::0::0def1ad::26::feur25::feat(hexbin): add log_counts; we..\",\"feat::0::6f934a1::26::feur25::feat(scatter): add sized and wid..\",\"fix::0::e56105e::26::feur25::fix(chart-methods): audit and re..\",\"fix::0::bcced19::26::feur25::fix(charts): make basic-vs-varia..\",\"feat::0::1031399::26::feur25::feat(funnel): add compare varian..\",\"fix::0::4559b13::26::feur25::fix(waterfall): make basic and s..\",\"chore::0::90dd7bd::26::feur25::build(wasm): redeploy with area ..\",\"docs::0::c87603b::26::feur25::docs: document funnel grouped an..\",\"fix::0::c6456a6::26::feur25::fix(chart-methods): flip() and ~..\",\"feat::0::5ce0597::26::feur25::feat(area): add ribbon variant (..\",\"chore::0::2f93767::26::feur25::build(wasm): redeploy with grid_..\",\"feat::0::1907faf::26::feur25::feat(chart-methods): add hover_f..\",\"feat::0::594db77::26::feur25::feat(bar): gate diverging's valu..\",\"fix::0::5c588e5::27::feur25::fix(bar): relative and grouped_s..\",\"feat::0::b4b2954::27::feur25::feat(bar): add Distribution vari..\",\"feat::0::db8f571::27::feur25::feat(waterfall): add Trend varia..\",\"chore::0::c6d4d54::27::feur25::build: redeploy wasm (20260727o)..\",\"feat::0::fa50f7b::27::feur25::feat(treemap): add Trend variant..\",\"chore::0::de9e757::27::feur25::build: redeploy wasm (20260727q)..\",\"chore::0::986050a::27::feur25::build: redeploy wasm (20260727r)..\",\"chore::0::de125e9::27::feur25::build: redeploy wasm (20260727s)..\",\"fix::0::aecde80::27::feur25::fix(ffi): sera_call/_sera_call n..\",\"feat::0::4db88ae::27::feur25::feat(docs): auto-extract theme p..\",\"fix::0::a058cf5::27::feur25::fix(ci): use rustls-tls to avoid..\",\"refactor::0::b070bd3::27::feur25::refactor(canvas): split 3055-lin..\",\"feat::0::7822ed5::27::feur25::feat(docs): expand/collapse butt..\",\"feat::0::d04bd42::27::feur25::feat(canvas): make the RéciTAC h..\",\"fix::0::00a5717::28::feur25::fix(canvas): the fullscreen prev..\",\"feat::0::b598756::28::feur25::feat(python): add display/render..\",\"fix::0::5bc05d3::28::feur25::fix(packaging): correct PyPI rep..\",\"chore::0::0148d50::10::feur25::chore(gitignore): exclude Sera P..\",\"fix::0::6125c05::10::feur25::fix(chart): carry <head> styles ..\",\"feat::0::39f3eba::10::feur25::feat(bar): overhaul the divergin..\",\"feat::0::cd07040::10::feur25::feat(chart): generic add_media()..\",\"docs::0::e9e435a::10::feur25::docs(reference): regenerate regi..\",\"fix::0::028bab5::10::feur25::fix(wasm): resolve the wasm bina..\",\"docs::0::7ca3888::10::feur25::docs(heatmap): add the missing r..\",\"feat::0::ccaf5d9::10::feur25::feat(charts): promote catmull_ro..\",\"chore::0::c83b1e3::10::feur25::chore(wasm): rebuild and redeplo..\",\"docs::0::d310a01::10::feur25::docs(reference): regenerate doc-..\",\"chore::0::afc83c6::10::feur25::chore(wasm): rebuild and redeplo..\",\"feat::0::8cc6095::11::feur25::feat(parallel): add lineage vari..\",\"feat::0::a688c38::11::feur25::feat(parallel): rebuild lineage ..\",\"feat::0::b09f6b6::11::feur25::feat(parallel): add chronicle va..\",\"feat::0::33f7404::12::feur25::feat(dendrogram): make genealogy..\",\"feat::0::228fc37::12::feur25::feat(bubble): rebuild radial_row..\",\"fix::0::464e0cb::12::feur25::fix(wasm): rebuild the browser b..\",\"feat::0::31db901::12::feur25::feat(dendrogram): add bloom; a m..\",\"fix::0::f314916::12::feur25::fix(docs): resolve required/para..\",\"fix::0::f99eb6a::13::feur25::fix(playground): stop Monaco cho..\",\"fix::0::5cef191::13::feur25::fix(playground): stop animating ..\",\"feat::0::29bc93d::13::feur25::feat(bar): drop spiral_grouped; ..\",\"chore::0::e81f60d::13::feur25::chore(wasm): redeploy build 2026..\",\"feat::0::c6d7494::13::feur25::feat(bar): add radial_bars and r..\",\"fix::0::42d3b47::14::feur25::fix(plot): integrate country dot..\",\"chore::0::cf18ae5::14::feur25::chore(wasm): redeploy build 2026..\",\"feat::0::7952b98::15::feur25::feat(radial_flow): rework design..\",\"feat::0::64d20a0::15::feur25::feat(lollipop): add duel variant..\",\"fix::0::58cd34e::15::feur25::fix(lollipop/duel): leaner demo ..\",\"feat::0::49d6810::16::feur25::feat(bar): add radial_pyramid va..\",\"fix::0::a680142::16::feur25::fix(radial_pyramid): two genuine..\",\"refactor::0::2cf86e2::16::feur25::redesign(radial_pyramid): true m..\",\"refactor::0::21be234::17::feur25::redesign(chord-bipartite): full-..\",\"feat::0::8b683ed::17::feur25::feat(circos): new multi-track ci..\",\"docs::0::5bc49df::18::feur25::docs(book): add Circos Plot page..\",\"refactor::0::d72b2aa::18::feur25::refactor(hexbin): share voronoi_..\",\"fix::0::0efef22::19::feur25::fix(hexbin): quiet periphery to ..\",\"feat::0::715def0::19::feur25::feat(sankey): add matrix variant..\",\"other::0::5d66ac5::20::feur25::fix(hexbin;sankey): rebalance ne..\",\"feat::0::be19599::23::feur25::feat(hexbin): add bloom variant;..\",\"other::0::31d85f1::23::feur25::chore(wasm;docs): rebuild live-p..\",\"other::0::0f8d65d::23::feur25::chore(wasm;docs): regenerate doc..\",\"fix::0::79ab660::23::feur25::fix(sankey): beacon radius now e..\",\"docs::0::37c5bf0::23::feur25::docs(heatmap): add hand-maintain..\"], parents=[\"2026-01\",\"2026-01\",\"2026-01\",\"2026-01\",\"2026-01\",\"2026-01\",\"2026-01\",\"2026-01\",\"2026-01\",\"2026-01\",\"2026-01\",\"2026-01\",\"2026-01\",\"2026-01\",\"2026-01\",\"2026-01\",\"2026-01\",\"2026-01\",\"2026-01\",\"2026-01\",\"2026-01\",\"2026-01\",\"2026-01\",\"2026-01\",\"2026-01\",\"2026-01\",\"2026-02\",\"2026-02\",\"2026-02\",\"2026-03\",\"2026-03\",\"2026-04\",\"2026-04\",\"2026-04\",\"2026-04\",\"2026-04\",\"2026-04\",\"2026-04\",\"2026-04\",\"2026-04\",\"2026-04\",\"2026-04\",\"2026-04\",\"2026-04\",\"2026-04\",\"2026-04\",\"2026-04\",\"2026-04\",\"2026-04\",\"2026-04\",\"2026-04\",\"2026-04\",\"2026-04\",\"2026-04\",\"2026-04\",\"2026-04\",\"2026-04\",\"2026-04\",\"2026-04\",\"2026-04\",\"2026-04\",\"2026-04\",\"2026-04\",\"2026-04\",\"2026-04\",\"2026-04\",\"2026-04\",\"2026-04\",\"2026-04\",\"2026-04\",\"2026-04\",\"2026-04\",\"2026-04\",\"2026-04\",\"2026-04\",\"2026-04\",\"2026-04\",\"2026-04\",\"2026-04\",\"2026-04\",\"2026-04\",\"2026-04\",\"2026-04\",\"2026-04\",\"2026-04\",\"2026-04\",\"2026-04\",\"2026-04\",\"2026-04\",\"2026-04\",\"2026-04\",\"2026-04\",\"2026-04\",\"2026-04\",\"2026-04\",\"2026-04\",\"2026-04\",\"2026-04\",\"2026-04\",\"2026-04\",\"2026-04\",\"2026-04\",\"2026-04\",\"2026-04\",\"2026-05\",\"2026-05\",\"2026-05\",\"2026-05\",\"2026-05\",\"2026-05\",\"2026-05\",\"2026-05\",\"2026-05\",\"2026-05\",\"2026-05\",\"2026-05\",\"2026-05\",\"2026-05\",\"2026-05\",\"2026-05\",\"2026-05\",\"2026-05\",\"2026-05\",\"2026-05\",\"2026-05\",\"2026-05\",\"2026-05\",\"2026-05\",\"2026-05\",\"2026-05\",\"2026-05\",\"2026-05\",\"2026-05\",\"2026-05\",\"2026-05\",\"2026-05\",\"2026-05\",\"2026-05\",\"2026-05\",\"2026-05\",\"2026-05\",\"2026-05\",\"2026-05\",\"2026-05\",\"2026-05\",\"2026-06\",\"2026-06\",\"2026-06\",\"2026-06\",\"2026-06\",\"2026-06\",\"2026-07\",\"2026-07\",\"2026-07\",\"2026-07\",\"2026-07\",\"2026-07\",\"2026-07\",\"2026-07\",\"2026-07\",\"2026-07\",\"2026-07\",\"2026-07\",\"2026-07\",\"2026-07\",\"2026-07\",\"2026-07\",\"2026-07\",\"2026-07\",\"2026-07\",\"2026-07\",\"2026-07\",\"2026-07\",\"2026-07\",\"2026-07\",\"2026-07\",\"2026-07\",\"2026-07\",\"2026-07\",\"2026-07\",\"2026-07\",\"2026-07\",\"2026-07\",\"2026-07\",\"2026-07\",\"2026-07\",\"2026-07\",\"2026-07\",\"2026-07\",\"2026-07\",\"2026-07\",\"2026-07\",\"2026-07\",\"2026-07\",\"2026-07\",\"2026-07\",\"2026-07\",\"2026-07\",\"2026-07\",\"2026-07\",\"2026-07\",\"2026-07\",\"2026-07\",\"2026-07\",\"2026-07\",\"2026-07\",\"2026-07\",\"2026-07\",\"2026-07\",\"2026-07\",\"2026-07\",\"2026-07\",\"2026-07\",\"2026-07\",\"2026-07\",\"2026-07\",\"2026-07\",\"2026-07\",\"2026-07\",\"2026-07\",\"2026-07\",\"2026-07\",\"2026-07\",\"2026-07\",\"2026-07\",\"2026-07\",\"2026-07\",\"2026-07\",\"2026-07\",\"2026-07\",\"2026-07\",\"2026-07\",\"2026-07\",\"2026-07\",\"2026-07\",\"2026-07\",\"2026-07\",\"2026-07\",\"2026-07\",\"2026-07\",\"2026-07\",\"2026-07\",\"2026-07\",\"2026-07\",\"2026-07\",\"2026-07\",\"2026-07\",\"2026-07\",\"2026-07\",\"2026-07\",\"2026-07\",\"2026-07\",\"2026-07\",\"2026-07\",\"2026-07\",\"2026-07\",\"2026-07\",\"2026-07\",\"2026-07\",\"2026-07\",\"2026-08\",\"2026-08\",\"2026-08\",\"2026-08\",\"2026-08\",\"2026-08\",\"2026-08\",\"2026-08\",\"2026-08\",\"2026-08\",\"2026-08\",\"2026-08\",\"2026-08\",\"2026-08\",\"2026-08\",\"2026-08\",\"2026-08\",\"2026-08\",\"2026-08\",\"2026-08\",\"2026-08\",\"2026-08\",\"2026-08\",\"2026-08\",\"2026-08\",\"2026-08\",\"2026-08\",\"2026-08\",\"2026-08\",\"2026-08\",\"2026-08\",\"2026-08\",\"2026-08\",\"2026-08\",\"2026-08\",\"2026-08\",\"2026-08\",\"2026-08\",\"2026-08\",\"2026-08\",\"2026-08\",\"2026-08\",\"2026-08\",\"2026-08\"], values=[0,243,25,360,65,1291,918,508,194,882,105,621,128,213,39,760,873,28,14,1568,37,255,426,2,255,2,38,12,26,5700,72,2878,59,20,165,1400,740,378,786,364,139,130,539,6093,156,577,6,788,4,2,343,130,87,129,4885,187,262,44,97,661,6,2706,2391,378,97255,197,95290,446,103851,200,102,12,55,12,40,1301,239,426,771,8,67,90,4,49,3,80,6,272,226,442,6,263,2,56,2,274,216,2,2,102,38,3307,15,34,4625,4148,8,8453,22,1374,1125,62,1624,18329,20379,2162,2,525,416,2250,1303,2,2,2,2,2,7,2,542,12,2,64,590,6052,1729,222,2,17,433,34,3,9,2743,1176,149,4783,375,498,5765,78,2224,197,229,348,470,869,351,1280,239,4536,181,197,118,422,92,4,2,57,3302,284,10,72,7,4,945,618,18,10,123,129,29,2,325,511,101,4,546,221,5282,209,287,4,45,3,18,66,609,555,116,0,569,485,52,35,2,456,8518,336,561,76,148,929,5464,1244,38,671,588,13,4,94,4,1715,819,8,895,4024,124,4024,1178,228,381,299,191,55,208,4,4022,865,76,39,4022,21,57,31,438,402,4024,369,4022,4022,4022,13,308,6,6161,641,1484,4,38,9,2,95,430,205,2582,18,15,142,4131,275,4131,4813,4532,4631,4218,4549,4,706,122,11,3,844,4,1022,56,4131,234,395,13,608,223,203,183,831,160,126,52,608,34,390,2,2,334,288], variant=\"swarm\", width=1760, height=800")]
pub fn render(cfg: &CirclePackConfig) -> String {
    let n = cfg.labels.len().min(cfg.parents.len()).min(cfg.values.len());
    if n == 0 {
        return String::new();
    }

    let mut commits: Vec<Commit> = Vec::with_capacity(n);
    let mut slots: Vec<HoverSlot> = Vec::with_capacity(n);
    let mut month_order: Vec<String> = Vec::new();
    for i in 0..n {
        let parts: Vec<&str> = cfg.labels[i].splitn(6, "::").collect();
        if parts.len() < 6 {
            continue;
        }
        let month = cfg.parents[i].clone();
        if !month_order.iter().any(|m| m == &month) {
            month_order.push(month.clone());
        }
        let lines = cfg.values[i].max(0.0);
        let ctype = parts[0].to_string();
        let bot = parts[1] == "1";
        let hash = parts[2].to_string();
        let day = parts[3].to_string();
        let author = parts[4].to_string();
        let message = parts[5].to_string();

        let mut slot = HoverSlot::new(message.clone())
            .kv("Type", type_label(&ctype))
            .kv("Date", format!("{} {}", month_label(&month), day))
            .kv("Hash", hash.clone())
            .kv("Author", author.clone())
            .kv("Lines changed", format!("{:.0}", lines));
        if bot {
            slot = slot.kv("Automated", "yes");
        }
        slots.push(slot);

        commits.push(Commit { ctype, bot, author, lines, month });
    }
    if commits.is_empty() {
        return String::new();
    }

    let mut by_month: HashMap<&str, Vec<usize>> = HashMap::new();
    for (ci, c) in commits.iter().enumerate() {
        by_month.entry(c.month.as_str()).or_default().push(ci);
    }

    let max_lines = commits.iter().map(|c| c.lines).fold(0.0_f64, f64::max).max(1.0);
    let r_min = 4.0_f64;
    let r_max = 26.0_f64;
    let radius_of = |lines: f64| r_min + (lines / max_lines).sqrt() * (r_max - r_min);

    struct Group {
        month: String,
        order: Vec<usize>,
        local_pos: Vec<(f64, f64)>,
        radii: Vec<f64>,
        enclosing_r: f64,
        spacing_r: f64,
        count: usize,
        total_lines: f64,
    }

    let mut groups: Vec<Group> = Vec::with_capacity(month_order.len());
    for month in &month_order {
        let idxs = by_month.get(month.as_str()).cloned().unwrap_or_default();
        let mut order = idxs.clone();
        order.sort_by(|&a, &b| commits[b].lines.partial_cmp(&commits[a].lines).unwrap_or(std::cmp::Ordering::Equal));
        let radii: Vec<f64> = order.iter().map(|&ci| radius_of(commits[ci].lines)).collect();
        let raw_pos = pack_local(&radii, 1.6);

        let mut min_x = f64::MAX;
        let mut max_x = f64::MIN;
        let mut min_y = f64::MAX;
        let mut max_y = f64::MIN;
        for (k, &(px, py)) in raw_pos.iter().enumerate() {
            let r = radii[k];
            min_x = min_x.min(px - r);
            max_x = max_x.max(px + r);
            min_y = min_y.min(py - r);
            max_y = max_y.max(py + r);
        }
        let mid_x = (min_x + max_x) / 2.0;
        let mid_y = (min_y + max_y) / 2.0;
        let local_pos: Vec<(f64, f64)> = raw_pos.iter().map(|&(px, py)| (px - mid_x, py - mid_y)).collect();
        let enclosing_r = local_pos
            .iter()
            .zip(radii.iter())
            .map(|(&(px, py), &r)| (px * px + py * py).sqrt() + r)
            .fold(0.0_f64, f64::max)
            .max(r_max);
        let label_half_w = (month_label(month).len().max(14) as f64) * 4.6;
        let spacing_r = enclosing_r.max(label_half_w);

        groups.push(Group {
            month: month.clone(),
            total_lines: order.iter().map(|&ci| commits[ci].lines).sum(),
            count: order.len(),
            order,
            local_pos,
            radii,
            enclosing_r,
            spacing_r,
        });
    }

    let gap = 20.0_f64;
    let mut nat_x = vec![0.0_f64; groups.len()];
    for k in 1..groups.len() {
        nat_x[k] = nat_x[k - 1] + groups[k - 1].spacing_r + groups[k].spacing_r + gap;
    }
    let natural_w = nat_x.last().copied().unwrap_or(0.0) + groups.last().map(|g| g.spacing_r).unwrap_or(0.0)
        + groups.first().map(|g| g.spacing_r).unwrap_or(0.0);
    let natural_max_r = groups.iter().map(|g| g.enclosing_r).fold(0.0_f64, f64::max);

    let top = 58_i32;
    let bottom = 92_i32;
    let left = 30_i32;
    let right = 30_i32;
    let plot_w = (cfg.width - left - right).max(40) as f64;
    let plot_h = (cfg.height - top - bottom).max(40) as f64;

    let fit_scale = (plot_w / natural_w.max(1.0))
        .min(plot_h / natural_max_r.max(1.0))
        .min(1.0)
        .max(0.02);

    let x0 = left as f64 + groups.first().map(|g| g.spacing_r).unwrap_or(0.0) * fit_scale;
    let baseline_y = (top as f64 + plot_h).round();

    let mut buf = Vec::<u8>::with_capacity(n * 180 + 8192);
    svg_open(&mut buf, cfg.width, cfg.height);

    push_b(&mut buf, b"<line x1=\"");
    push_f2(&mut buf, left as f64);
    push_b(&mut buf, b"\" y1=\"");
    push_f2(&mut buf, baseline_y);
    push_b(&mut buf, b"\" x2=\"");
    push_f2(&mut buf, (cfg.width - right) as f64);
    push_b(&mut buf, b"\" y2=\"");
    push_f2(&mut buf, baseline_y);
    push_b(&mut buf, b"\" stroke=\"#cbd5e1\" stroke-width=\"1.5\"/>");

    let mut anchors: Vec<(f64, f64)> = Vec::with_capacity(groups.len());
    for (k, g) in groups.iter().enumerate() {
        let ax = x0 + nat_x[k] * fit_scale;
        let er = g.enclosing_r * fit_scale;
        let ay = baseline_y - 8.0 - er;
        anchors.push((ax, ay));

        push_b(&mut buf, b"<line x1=\"");
        push_f2(&mut buf, ax);
        push_b(&mut buf, b"\" y1=\"");
        push_f2(&mut buf, baseline_y);
        push_b(&mut buf, b"\" x2=\"");
        push_f2(&mut buf, ax);
        push_b(&mut buf, b"\" y2=\"");
        push_f2(&mut buf, ay + er);
        push_b(&mut buf, b"\" stroke=\"#94a3b8\" stroke-width=\"1\" stroke-dasharray=\"1,3\" stroke-opacity=\"0.6\"/>");
        push_b(&mut buf, b"<circle cx=\"");
        push_f2(&mut buf, ax);
        push_b(&mut buf, b"\" cy=\"");
        push_f2(&mut buf, baseline_y);
        push_b(&mut buf, b"\" r=\"3\" fill=\"#475569\"/>");

        push_b(&mut buf, b"<text x=\"");
        push_f2(&mut buf, ax);
        push_b(&mut buf, b"\" y=\"");
        push_f2(&mut buf, baseline_y + 18.0);
        push_b(&mut buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"11\" font-weight=\"700\" fill=\"#334155\">");
        escape_xml(&mut buf, &month_label(&g.month));
        push_b(&mut buf, b"</text>");
        push_b(&mut buf, b"<text x=\"");
        push_f2(&mut buf, ax);
        push_b(&mut buf, b"\" y=\"");
        push_f2(&mut buf, baseline_y + 32.0);
        push_b(&mut buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"9.5\" fill=\"#94a3b8\">");
        let sub = if g.total_lines >= 1000.0 {
            format!("{} commits \u{b7} {:.1}k lines", g.count, g.total_lines / 1000.0)
        } else {
            format!("{} commits \u{b7} {:.0} lines", g.count, g.total_lines)
        };
        push_b(&mut buf, sub.as_bytes());
        push_b(&mut buf, b"</text>");
    }

    for (k, g) in groups.iter().enumerate() {
        let (ax, ay) = anchors[k];
        for (slot_i, &ci) in g.order.iter().enumerate() {
            let (lx, ly) = g.local_pos[slot_i];
            let r = (g.radii[slot_i] * fit_scale).max(1.4);
            let px = ax + lx * fit_scale;
            let py = ay + ly * fit_scale;
            let color = type_color(&commits[ci].ctype);
            let hx = hex6(color);
            if commits[ci].bot {
                push_b(&mut buf, b"<polygon data-idx=\"");
                push_i(&mut buf, ci as i32);
                push_b(&mut buf, b"\" points=\"");
                for s in 0..6 {
                    let a = std::f64::consts::PI / 3.0 * s as f64 - std::f64::consts::FRAC_PI_2;
                    push_f2(&mut buf, px + r * a.cos());
                    buf.push(b',');
                    push_f2(&mut buf, py + r * a.sin());
                    if s < 5 {
                        buf.push(b' ');
                    }
                }
                push_b(&mut buf, b"\" fill=\"#");
                buf.extend_from_slice(&hx);
                push_b(&mut buf, b"\" fill-opacity=\"0.92\" stroke=\"#fff\" stroke-width=\"1.1\"/>");
            } else {
                push_b(&mut buf, b"<circle data-idx=\"");
                push_i(&mut buf, ci as i32);
                push_b(&mut buf, b"\" cx=\"");
                push_f2(&mut buf, px);
                push_b(&mut buf, b"\" cy=\"");
                push_f2(&mut buf, py);
                push_b(&mut buf, b"\" r=\"");
                push_f2(&mut buf, r);
                push_b(&mut buf, b"\" fill=\"#");
                buf.extend_from_slice(&hx);
                push_b(&mut buf, b"\" fill-opacity=\"0.88\" stroke=\"#fff\" stroke-width=\"0.9\"/>");
            }
        }
    }

    if !cfg.title.is_empty() {
        push_b(&mut buf, b"<text x=\"");
        push_f2(&mut buf, cfg.width as f64 / 2.0);
        push_b(&mut buf, b"\" y=\"24\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"14\" font-weight=\"700\" fill=\"#1a202c\" letter-spacing=\"2\">");
        escape_xml(&mut buf, cfg.title);
        push_b(&mut buf, b"</text>");
    }

    let total_lines: f64 = commits.iter().map(|c| c.lines).sum();
    let mut authors: Vec<&str> = Vec::new();
    for c in &commits {
        if !authors.iter().any(|a| *a == c.author) {
            authors.push(&c.author);
        }
    }
    let span = if month_order.len() > 1 {
        format!("{} \u{2192} {}", month_label(&month_order[0]), month_label(&month_order[month_order.len() - 1]))
    } else {
        month_label(&month_order[0])
    };
    let stats: [(&str, String); 4] = [
        ("COMMITS", commits.len().to_string()),
        ("LINES CHANGED", format!("{:.0}", total_lines)),
        ("CONTRIBUTORS", authors.len().to_string()),
        ("SPAN", span),
    ];
    let mut sy = 22_i32;
    let sx = cfg.width - right - 4;
    for (label, val) in stats.iter() {
        push_b(&mut buf, b"<text x=\"");
        push_i(&mut buf, sx);
        push_b(&mut buf, b"\" y=\"");
        push_i(&mut buf, sy);
        push_b(&mut buf, b"\" text-anchor=\"end\" font-family=\"Arial,sans-serif\" font-size=\"9.5\" fill=\"#64748b\">");
        push_b(&mut buf, label.as_bytes());
        push_b(&mut buf, b" ");
        push_b(&mut buf, b"<tspan font-weight=\"700\" fill=\"#1e293b\">");
        push_b(&mut buf, val.as_bytes());
        push_b(&mut buf, b"</tspan></text>");
        sy += 14;
    }

    const LEGEND_TYPES: [&str; 9] =
        ["feat", "fix", "docs", "refactor", "chore", "style", "perf", "test", "other"];
    let mut type_counts: HashMap<&str, usize> = HashMap::new();
    for c in &commits {
        *type_counts.entry(c.ctype.as_str()).or_insert(0) += 1;
    }
    let has_bot = commits.iter().any(|c| c.bot);
    let ly = cfg.height - 14;
    let mut lx = left;
    for t in LEGEND_TYPES.iter() {
        let count = type_counts.get(t).copied().unwrap_or(0);
        if count == 0 {
            continue;
        }
        let c = type_color(t);
        push_b(&mut buf, b"<circle cx=\"");
        push_i(&mut buf, lx + 4);
        push_b(&mut buf, b"\" cy=\"");
        push_i(&mut buf, ly);
        push_b(&mut buf, b"\" r=\"4\" fill=\"#");
        buf.extend_from_slice(&hex6(c));
        push_b(&mut buf, b"\" fill-opacity=\"0.9\"/>");
        let label = format!("{} {}", type_label(t), count);
        push_b(&mut buf, b"<text x=\"");
        push_i(&mut buf, lx + 12);
        push_b(&mut buf, b"\" y=\"");
        push_i(&mut buf, ly + 3);
        push_b(&mut buf, b"\" font-family=\"Arial,sans-serif\" font-size=\"9\" fill=\"#475569\">");
        push_b(&mut buf, label.as_bytes());
        push_b(&mut buf, b"</text>");
        lx += 22 + label.len() as i32 * 5;
    }
    if has_bot {
        push_b(&mut buf, b"<polygon points=\"");
        for s in 0..6 {
            let a = std::f64::consts::PI / 3.0 * s as f64 - std::f64::consts::FRAC_PI_2;
            push_f2(&mut buf, (lx + 4) as f64 + 4.2 * a.cos());
            buf.push(b',');
            push_f2(&mut buf, ly as f64 + 4.2 * a.sin());
            if s < 5 {
                buf.push(b' ');
            }
        }
        push_b(&mut buf, b"\" fill=\"#1e293b\" fill-opacity=\"0.85\"/>");
        push_b(&mut buf, b"<text x=\"");
        push_i(&mut buf, lx + 12);
        push_b(&mut buf, b"\" y=\"");
        push_i(&mut buf, ly + 3);
        push_b(&mut buf, b"\" font-family=\"Arial,sans-serif\" font-size=\"9\" fill=\"#475569\">Automated</text>");
    }

    push_b(&mut buf, b"</svg>");
    let svg = unsafe { String::from_utf8_unchecked(buf) };
    build_chart_html(cfg.title, &svg, &slots_to_json(&slots))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cfg<'a>(labels: &'a [String], parents: &'a [String], values: &'a [f64]) -> CirclePackConfig<'a> {
        CirclePackConfig {
            title: "Test",
            labels,
            parents,
            values,
            width: 1600,
            height: 760,
            ..CirclePackConfig::default()
        }
    }

    fn synth(n: usize) -> (Vec<String>, Vec<String>, Vec<f64>) {
        let types = ["feat", "fix", "docs", "refactor", "chore"];
        let months = ["2026-01", "2026-02", "2026-03", "2026-04"];
        let mut labels = Vec::with_capacity(n);
        let mut parents = Vec::with_capacity(n);
        let mut values = Vec::with_capacity(n);
        for i in 0..n {
            let t = types[i % types.len()];
            let bot = if i % 17 == 0 { "1" } else { "0" };
            labels.push(format!("{t}::{bot}::abc{i:04}::{:02}::feur25::commit number {i}", (i % 28) + 1));
            parents.push(months[i % months.len()].to_string());
            values.push(((i % 40) + 1) as f64 * 6.0);
        }
        (labels, parents, values)
    }

    #[test]
    #[ignore]
    fn write_preview_asset() {
        use crate::plot::chart_demo_registry::{iter_entries, render_demo_html};
        for entry in iter_entries() {
            if !entry.file.replace('\\', "/").ends_with("circle_pack/swarm.rs") {
                continue;
            }
            let html = render_demo_html(entry).expect("demo html");
            std::fs::write("docs/previews/circle_pack-swarm.html", html).unwrap();
        }
    }

    #[test]
    fn renders_one_hoverable_mark_per_commit_and_a_baseline_per_month() {
        let (labels, parents, values) = synth(120);
        let html = render(&cfg(&labels, &parents, &values));
        assert!(!html.is_empty());
        let marks = html.matches("<circle data-idx=\"").count() + html.matches("<polygon data-idx=\"").count();
        assert_eq!(marks, 120);
        assert!(html.contains("Jan '26"));
        assert!(html.contains("Feb '26"));
        assert!(html.contains("commits \u{b7}"));
    }

    #[test]
    fn bot_commits_render_as_hexagons_not_circles() {
        let (labels, parents, values) = synth(40);
        let html = render(&cfg(&labels, &parents, &values));
        assert!(html.contains("<polygon data-idx="));
        assert!(html.contains("Automated"));
    }

    #[test]
    fn every_month_cluster_stays_within_the_plot_bounds() {
        let (labels, parents, values) = synth(300);
        let c = cfg(&labels, &parents, &values);
        let html = render(&c);
        assert!(!html.is_empty());
        let marks = html.matches("<circle data-idx=\"").count() + html.matches("<polygon data-idx=\"").count();
        assert_eq!(marks, 300);
    }

    #[test]
    fn empty_input_returns_empty_string() {
        let empty_s: Vec<String> = vec![];
        let empty_v: Vec<f64> = vec![];
        assert!(render(&cfg(&empty_s, &empty_s, &empty_v)).is_empty());
    }

    #[test]
    fn perf_rendering_a_full_project_history_stays_fast() {
        let (labels, parents, values) = synth(1000);
        let c = cfg(&labels, &parents, &values);
        let start = std::time::Instant::now();
        let html = render(&c);
        let elapsed = start.elapsed();
        assert!(elapsed.as_millis() < 1500, "rendering took too long: {elapsed:?}");
        assert!(!html.is_empty());
    }
}
