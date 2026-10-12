use super::annotations::{self, Annotation};
use super::history::{self, Comparison};
use crate::plot::anomaly_registry::{self, AnomalyEvent};
use crate::Chart;
use pyo3::prelude::*;
use seraplot_macros::sera_doc_impl;

enum Block {
    Heading(u8, String),
    Text(String),
    List(Vec<String>),
    Table(Vec<&'static str>, Vec<Vec<String>>),
}

fn oneline(s: &str) -> String {
    s.replace(['\n', '\r'], " ")
}

fn format_values(values: &[f64]) -> String {
    let parts: Vec<String> = values.iter().map(|v| format!("{v:.4}")).collect();
    format!("[{}]", parts.join(", "))
}

fn comparison_blocks(baseline_path: &str, current_path: &str, comparison: &Comparison) -> Vec<Block> {
    let mut blocks = vec![
        Block::Heading(2, "Comparison".to_string()),
        Block::List(vec![
            format!("Baseline: `{baseline_path}`"),
            format!("Current: `{current_path}`"),
            format!("Matched: {}", comparison.matched),
            format!("Added: {}", comparison.added),
            format!("Removed: {}", comparison.removed),
            format!("Mean absolute delta: {:.4}", comparison.mean_abs_delta),
            format!("Max absolute delta: {:.4}", comparison.max_abs_delta),
            format!("Mean % change: {:.2}%", comparison.mean_pct_change),
        ]),
    ];
    if comparison.top_changes.is_empty() {
        blocks.push(Block::Text("No changed indices.".to_string()));
    } else {
        let rows = comparison
            .top_changes
            .iter()
            .map(|d| {
                vec![
                    d.index.to_string(),
                    format_values(&d.baseline),
                    format_values(&d.current),
                    format!("{:.4}", d.abs_delta),
                    format!("{:.2}%", d.pct_change),
                ]
            })
            .collect();
        blocks.push(Block::Heading(3, "Biggest changes".to_string()));
        blocks.push(Block::Table(vec!["Index", "Baseline", "Current", "Abs delta", "% change"], rows));
    }
    blocks
}

fn window_note(window: Option<(i64, i64)>) -> Block {
    match window {
        Some((start, end)) => Block::Text(format!(
            "Filtered to the incident window shared by both recordings: epoch ms `{start}`\u{2013}`{end}`."
        )),
        None => Block::Text(
            "Not filtered to a time window: one or both .spls recordings predate the wall-clock anchor (added in the .spls v3 format) — showing everything currently on record for this chart instead.".to_string(),
        ),
    }
}

fn bookmark_blocks(bookmarks: &[Annotation]) -> Vec<Block> {
    let mut blocks = vec![Block::Heading(2, format!("Bookmarks ({})", bookmarks.len()))];
    if bookmarks.is_empty() {
        blocks.push(Block::Text("No bookmarks in this window.".to_string()));
    } else {
        let items = bookmarks
            .iter()
            .map(|b| {
                let at = b.at_ms.unwrap_or(b.created_at);
                format!("`{at}` index {} \u{2014} {} (id {})", b.index, oneline(&b.label), b.id)
            })
            .collect();
        blocks.push(Block::List(items));
    }
    blocks
}

fn anomaly_blocks(anomalies: &[AnomalyEvent]) -> Vec<Block> {
    let mut blocks = vec![Block::Heading(2, format!("Anomalies logged ({})", anomalies.len()))];
    if anomalies.is_empty() {
        blocks.push(Block::Text("No anomalies logged in this window.".to_string()));
    } else {
        let items = anomalies
            .iter()
            .map(|a| format!("`{}` index {} = {:.4} (z={:.2})", a.at_ms, a.index, a.value, a.z))
            .collect();
        blocks.push(Block::List(items));
    }
    blocks
}

fn build_blocks(
    hid: u64,
    baseline_path: &str,
    current_path: &str,
    comparison: &Comparison,
    window: Option<(i64, i64)>,
    bookmarks: &[Annotation],
    anomalies: &[AnomalyEvent],
) -> Vec<Block> {
    let mut blocks = vec![
        Block::Heading(1, format!("Sera Pulse incident report \u{2014} chart {hid}")),
        Block::Text(format!("Generated at epoch ms `{}`.", crate::plot::now_ms())),
    ];
    blocks.extend(comparison_blocks(baseline_path, current_path, comparison));
    blocks.push(window_note(window));
    blocks.extend(bookmark_blocks(bookmarks));
    blocks.extend(anomaly_blocks(anomalies));
    blocks
}

fn render_markdown(blocks: &[Block]) -> String {
    let mut out = String::new();
    for block in blocks {
        match block {
            Block::Heading(level, text) => {
                out.push_str(&"#".repeat(*level as usize));
                out.push(' ');
                out.push_str(text);
                out.push_str("\n\n");
            }
            Block::Text(text) => {
                out.push_str(text);
                out.push_str("\n\n");
            }
            Block::List(items) => {
                for item in items {
                    out.push_str("- ");
                    out.push_str(item);
                    out.push('\n');
                }
                out.push('\n');
            }
            Block::Table(headers, rows) => {
                out.push_str(&format!("| {} |\n", headers.join(" | ")));
                out.push_str(&format!("|{}|\n", "---|".repeat(headers.len())));
                for row in rows {
                    out.push_str(&format!("| {} |\n", row.join(" | ")));
                }
                out.push('\n');
            }
        }
    }
    out
}

fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('"', "&quot;")
}

fn inline_html(s: &str) -> String {
    let mut out = String::new();
    for (i, part) in s.split('`').enumerate() {
        if i % 2 == 1 {
            out.push_str("<code>");
            out.push_str(&escape_html(part));
            out.push_str("</code>");
        } else {
            out.push_str(&escape_html(part));
        }
    }
    out
}

fn render_html(blocks: &[Block]) -> String {
    let mut body = String::new();
    for block in blocks {
        match block {
            Block::Heading(level, text) => {
                body.push_str(&format!("<h{level}>{}</h{level}>\n", inline_html(text)));
            }
            Block::Text(text) => {
                body.push_str(&format!("<p>{}</p>\n", inline_html(text)));
            }
            Block::List(items) => {
                body.push_str("<ul>\n");
                for item in items {
                    body.push_str(&format!("<li>{}</li>\n", inline_html(item)));
                }
                body.push_str("</ul>\n");
            }
            Block::Table(headers, rows) => {
                body.push_str("<table>\n<thead><tr>");
                for h in headers {
                    body.push_str(&format!("<th>{}</th>", escape_html(h)));
                }
                body.push_str("</tr></thead>\n<tbody>\n");
                for row in rows {
                    body.push_str("<tr>");
                    for cell in row {
                        body.push_str(&format!("<td>{}</td>", inline_html(cell)));
                    }
                    body.push_str("</tr>\n");
                }
                body.push_str("</tbody>\n</table>\n");
            }
        }
    }
    format!(
        "<!doctype html>\n<meta charset=\"utf-8\">\n<style>body{{font-family:sans-serif;max-width:60rem;margin:2rem auto;padding:0 1rem}}table{{border-collapse:collapse}}td,th{{border:1px solid #ccc;padding:.25rem .5rem;text-align:left}}code{{background:#f2f2f2;padding:.1rem .3rem;border-radius:.2rem}}</style>\n{body}"
    )
}

fn compute_window(baseline: &history::LoadedRecording, current: &history::LoadedRecording) -> Option<(i64, i64)> {
    let start = baseline.started_at_ms?;
    let end = current.ended_at_ms.or(current.started_at_ms)?;
    Some((start.min(end), start.max(end)))
}

fn in_window(at_ms: i64, window: Option<(i64, i64)>) -> bool {
    match window {
        Some((start, end)) => at_ms >= start && at_ms <= end,
        None => true,
    }
}

pub(crate) fn build_report(hid: u64, baseline_path: &str, current_path: &str, output_path: Option<&str>, format: &str) -> PyResult<String> {
    if format != "markdown" && format != "html" {
        return Err(pyo3::exceptions::PyValueError::new_err(format!(
            "seraplot: unknown incident report format '{format}', expected 'markdown' or 'html'"
        )));
    }
    let baseline = history::load_final_state(baseline_path)?;
    let current = history::load_final_state(current_path)?;
    let comparison = history::compare(&baseline.state, &current.state);
    let window = compute_window(&baseline, &current);

    let bookmarks: Vec<Annotation> = annotations::list(hid).into_iter().filter(|b| in_window(b.at_ms.unwrap_or(b.created_at), window)).collect();
    let anomalies: Vec<AnomalyEvent> = anomaly_registry::logged_events(hid).into_iter().filter(|a| in_window(a.at_ms, window)).collect();

    let blocks = build_blocks(hid, baseline_path, current_path, &comparison, window, &bookmarks, &anomalies);
    let report = if format == "html" { render_html(&blocks) } else { render_markdown(&blocks) };

    if let Some(path) = output_path {
        super::atomic_write(path, &report)
            .map_err(|e| pyo3::exceptions::PyOSError::new_err(format!("seraplot: failed to write {path}: {e}")))?;
    }
    Ok(report)
}

#[cfg(feature = "python")]
#[sera_doc_impl]
#[pyo3::pymethods]
impl Chart {
    #[pyo3(signature = (baseline_path, current_path, output_path = None, format = "markdown"))]
    #[sera_doc(
        category = "chart_method",
        module = "history",
        file = "seraplot/pulse-pricing.md",
        aliases("incident_report"),
        en = "Generates an incident report for this chart by composing three things Pulse already tracks: the quantitative diff between two .spls recordings (same engine as compare_history()), the bookmarks attached to this chart, and the anomalies logged for it. Bookmarks and anomalies are filtered to the shared time window between the two recordings when both were made with .spls v3+ (they carry a wall-clock anchor); older recordings fall back to reporting everything on record, clearly noted in the output. Returns the report as a string in the requested format; also writes it to output_path if given.",
        fr = "Génère un rapport d'incident pour ce chart en composant trois choses que Pulse suit déjà : le diff quantitatif entre deux enregistrements .spls (même moteur que compare_history()), les repères attachés à ce chart, et les anomalies enregistrées pour lui. Repères et anomalies sont filtrés à la fenêtre temporelle commune aux deux enregistrements quand les deux sont au format .spls v3+ (ils portent un ancrage horloge murale) ; les enregistrements plus anciens retombent sur tout ce qui est connu, clairement noté dans le résultat. Retourne le rapport sous forme de chaîne dans le format demandé ; l'écrit aussi dans output_path si fourni.",
        param(name = "baseline_path", ty = "str", en = "Path to the baseline .spls recording, same as compare_history().", fr = "Chemin de l'enregistrement .spls de référence, comme pour compare_history()."),
        param(name = "current_path", ty = "str", en = "Path to the .spls recording to compare against the baseline, same as compare_history().", fr = "Chemin de l'enregistrement .spls à comparer à la référence, comme pour compare_history()."),
        param(name = "output_path", ty = "str | None", en = "If set, also writes the report to this file path.", fr = "Si défini, écrit aussi le rapport à ce chemin de fichier."),
        param(name = "format", ty = "str", en = "'markdown' (default) or 'html'.", fr = "'markdown' (défaut) ou 'html'.")
    )]
    fn generate_incident_report(&self, baseline_path: &str, current_path: &str, output_path: Option<&str>, format: &str) -> PyResult<String> {
        let hid = super::require_hid(&self.html)?;
        build_report(hid, baseline_path, current_path, output_path, format)
    }

    #[pyo3(signature = (baseline_path, current_path, output_path = None, format = "markdown"))]
    fn incident_report(&self, baseline_path: &str, current_path: &str, output_path: Option<&str>, format: &str) -> PyResult<String> {
        self.generate_incident_report(baseline_path, current_path, output_path, format)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::plot::push_registry::{PushMeta, ScalarMeta};
    use crate::plot::record_registry;

    fn scalar_wire_frame(hid: u64, indices: &[u32], values: &[i16]) -> Vec<u8> {
        let n = indices.len() as u32;
        let mut frame = Vec::with_capacity(12 + indices.len() * 4 + values.len() * 2);
        frame.extend_from_slice(&(hid as u32).to_le_bytes());
        frame.extend_from_slice(&n.to_le_bytes());
        frame.extend_from_slice(&1u32.to_le_bytes());
        for i in indices {
            frame.extend_from_slice(&i.to_le_bytes());
        }
        for v in values {
            frame.extend_from_slice(&v.to_le_bytes());
        }
        frame
    }

    fn write_recording(hid: u64, path: &std::path::Path, offset: i16) {
        let meta = PushMeta::Scalar(ScalarMeta { min_val: 0.0, range_val: 1000.0, axis_px: 1000 });
        record_registry::start(hid, path.to_str().unwrap(), &meta).expect("start");
        let state = record_registry::get(hid).expect("registered");
        let indices: Vec<u32> = (0..10u32).collect();
        let values: Vec<i16> = indices.iter().map(|i| (*i as i16 * 50 + offset).clamp(0, 999)).collect();
        state.append(&scalar_wire_frame(hid, &indices, &values));
        record_registry::stop(hid);
    }

    fn two_recordings(tag: &str, base_hid: u64, cur_hid: u64) -> (std::path::PathBuf, std::path::PathBuf) {
        let base = std::env::temp_dir().join(format!("sp_incident_base_{tag}_{}.spls", std::process::id()));
        let cur = std::env::temp_dir().join(format!("sp_incident_cur_{tag}_{}.spls", std::process::id()));
        write_recording(base_hid, &base, 0);
        write_recording(cur_hid, &cur, 25);
        (base, cur)
    }

    fn v2_recording_without_anchor(path: &std::path::Path, indices: &[u32], values: &[i16]) {
        use std::io::Write;
        let mut f = std::fs::File::create(path).unwrap();
        f.write_all(b"SPLS").unwrap();
        f.write_all(&[2u8, 0, 0, 0]).unwrap();
        f.write_all(&[1u8]).unwrap();
        f.write_all(&0.0f64.to_le_bytes()).unwrap();
        f.write_all(&1000.0f64.to_le_bytes()).unwrap();
        f.write_all(&1000i32.to_le_bytes()).unwrap();
        let body = scalar_wire_frame(999, indices, values);
        f.write_all(&0u32.to_le_bytes()).unwrap();
        f.write_all(&(body.len() as u32).to_le_bytes()).unwrap();
        f.write_all(&body).unwrap();
    }

    #[test]
    fn report_composes_comparison_bookmarks_and_anomalies() {
        pyo3::prepare_freethreaded_python();
        let hid = 9_100_010u64;
        annotations::clear(hid);
        anomaly_registry::clear(hid);
        let base = std::env::temp_dir().join(format!("sp_incident_base_compose_{}.spls", std::process::id()));
        let cur = std::env::temp_dir().join(format!("sp_incident_cur_compose_{}.spls", std::process::id()));

        write_recording(9_100_110, &base, 0);
        std::thread::sleep(std::time::Duration::from_millis(20));

        annotations::add(hid, 3, "deploy started".to_string(), None).expect("add bookmark");
        Python::with_gil(|py| {
            anomaly_registry::set(hid, 5, 3.0, anomaly_registry::Method::Zscore, py.None());
            for _ in 0..10 {
                anomaly_registry::check_and_notify(py, hid, &[2], &[10.0]);
            }
            anomaly_registry::check_and_notify(py, hid, &[2], &[999_999.0]);
        });

        std::thread::sleep(std::time::Duration::from_millis(20));
        write_recording(9_100_111, &cur, 25);

        let report = build_report(hid, base.to_str().unwrap(), cur.to_str().unwrap(), None, "markdown").expect("build report");

        let _ = std::fs::remove_file(&base);
        let _ = std::fs::remove_file(&cur);
        annotations::clear(hid);
        anomaly_registry::clear(hid);

        assert!(report.contains("Sera Pulse incident report"));
        assert!(report.contains("deploy started"), "bookmark label should appear in the report");
        assert!(report.contains("Matched: 10"), "comparison stats should appear in the report, got:\n{report}");
        assert!(report.contains("index 2"), "the logged anomaly's index should appear in the report");
        assert!(report.contains("Filtered to the incident window"), "both recordings are fresh v3 so a real window should apply");
    }

    #[test]
    fn report_on_a_chart_with_nothing_extra_still_renders_cleanly() {
        let hid = 9_100_011u64;
        annotations::clear(hid);
        anomaly_registry::clear(hid);
        let (base, cur) = two_recordings("empty", 9_100_112, 9_100_113);

        let report = build_report(hid, base.to_str().unwrap(), cur.to_str().unwrap(), None, "markdown").expect("build report");

        let _ = std::fs::remove_file(&base);
        let _ = std::fs::remove_file(&cur);

        assert!(report.contains("No bookmarks in this window"), "empty bookmarks should render a clear placeholder, not nothing");
        assert!(report.contains("No anomalies logged in this window"), "empty anomalies should render a clear placeholder, not nothing");
    }

    #[test]
    fn output_path_writes_the_same_content_that_is_returned() {
        let hid = 9_100_012u64;
        annotations::clear(hid);
        let (base, cur) = two_recordings("write", 9_100_114, 9_100_115);
        let out_path = std::env::temp_dir().join(format!("sp_incident_report_{}.md", std::process::id()));
        let _ = std::fs::remove_file(&out_path);

        let report = build_report(hid, base.to_str().unwrap(), cur.to_str().unwrap(), Some(out_path.to_str().unwrap()), "markdown").expect("build report");
        let on_disk = std::fs::read_to_string(&out_path).expect("report file should exist");

        let _ = std::fs::remove_file(&base);
        let _ = std::fs::remove_file(&cur);
        let _ = std::fs::remove_file(&out_path);

        assert_eq!(report, on_disk, "the string returned to Python and the file written to disk should be identical");
    }

    #[test]
    fn missing_baseline_file_reports_a_clear_error_not_a_panic() {
        let hid = 9_100_013u64;
        let err = build_report(hid, "this/path/does/not/exist.spls", "this/path/does/not/exist.spls", None, "markdown").unwrap_err();
        pyo3::prepare_freethreaded_python();
        assert!(Python::with_gil(|py| err.value_bound(py).to_string().contains("failed to read")));
    }

    #[test]
    fn unknown_format_is_a_clear_error() {
        let hid = 9_100_016u64;
        let err = build_report(hid, "a.spls", "b.spls", None, "pdf").unwrap_err();
        pyo3::prepare_freethreaded_python();
        assert!(Python::with_gil(|py| err.value_bound(py).to_string().contains("unknown incident report format")));
    }

    #[test]
    fn html_output_escapes_untrusted_bookmark_text() {
        pyo3::prepare_freethreaded_python();
        let hid = 9_100_017u64;
        annotations::clear(hid);
        let base = std::env::temp_dir().join(format!("sp_incident_base_htmlesc_{}.spls", std::process::id()));
        let cur = std::env::temp_dir().join(format!("sp_incident_cur_htmlesc_{}.spls", std::process::id()));

        write_recording(9_100_120, &base, 0);
        std::thread::sleep(std::time::Duration::from_millis(20));
        annotations::add(hid, 1, "<script>alert(1)</script>".to_string(), None).expect("add bookmark");
        std::thread::sleep(std::time::Duration::from_millis(20));
        write_recording(9_100_121, &cur, 25);

        let report = build_report(hid, base.to_str().unwrap(), cur.to_str().unwrap(), None, "html").expect("build report");

        let _ = std::fs::remove_file(&base);
        let _ = std::fs::remove_file(&cur);
        annotations::clear(hid);

        assert!(report.starts_with("<!doctype html>"), "html format should produce an actual html document");
        assert!(!report.contains("<script>alert(1)</script>"), "a bookmark label must never appear as raw unescaped HTML: {report}");
        assert!(report.contains("&lt;script&gt;"), "the label should appear HTML-escaped instead");
    }

    #[test]
    fn bookmarks_outside_the_recorded_window_are_excluded() {
        let hid = 9_100_018u64;
        annotations::clear(hid);
        let (base, cur) = two_recordings("windowed", 9_100_122, 9_100_123);

        let window_start = history::load_final_state(base.to_str().unwrap()).expect("load baseline for window bounds").started_at_ms.expect("fresh recording should carry a wall-clock anchor");

        annotations::add(hid, 1, "inside the window".to_string(), Some(window_start)).expect("add current bookmark exactly at the window's inclusive lower bound");
        annotations::add(hid, 1, "long before this incident".to_string(), Some(window_start - 1_000_000)).expect("add ancient bookmark, ~11.5 days before the window");

        let report = build_report(hid, base.to_str().unwrap(), cur.to_str().unwrap(), None, "markdown").expect("build report");

        let _ = std::fs::remove_file(&base);
        let _ = std::fs::remove_file(&cur);
        annotations::clear(hid);

        assert!(report.contains("inside the window"), "a bookmark timestamped at the window's own lower bound should be included, got:\n{report}");
        assert!(!report.contains("long before this incident"), "a bookmark timestamped well before the window must be excluded, got:\n{report}");
    }

    #[test]
    fn recordings_without_a_wall_clock_anchor_fall_back_to_unfiltered() {
        let hid = 9_100_019u64;
        annotations::clear(hid);
        let base = std::env::temp_dir().join(format!("sp_incident_noanchor_base_{}.spls", std::process::id()));
        let cur = std::env::temp_dir().join(format!("sp_incident_noanchor_cur_{}.spls", std::process::id()));
        v2_recording_without_anchor(&base, &[0, 1], &[10, 20]);
        v2_recording_without_anchor(&cur, &[0, 1], &[15, 25]);

        annotations::add(hid, 0, "recorded ages ago or just now, can't tell".to_string(), None).expect("add");

        let report = build_report(hid, base.to_str().unwrap(), cur.to_str().unwrap(), None, "markdown").expect("build report");

        let _ = std::fs::remove_file(&base);
        let _ = std::fs::remove_file(&cur);
        annotations::clear(hid);

        assert!(report.contains("Not filtered to a time window"), "recordings without an anchor must fall back honestly instead of guessing, got:\n{report}");
        assert!(report.contains("recorded ages ago or just now"), "unfiltered fallback should still include the bookmark");
    }

    #[test]
    fn report_generation_stays_fast_with_realistic_volume() {
        pyo3::prepare_freethreaded_python();
        let hid = 9_100_014u64;
        annotations::clear(hid);
        anomaly_registry::clear(hid);

        let base_path = std::env::temp_dir().join(format!("sp_incident_perf_base_{}.spls", std::process::id()));
        let cur_path = std::env::temp_dir().join(format!("sp_incident_perf_cur_{}.spls", std::process::id()));
        let n = 50_000u32;
        let meta = PushMeta::Scalar(ScalarMeta { min_val: 0.0, range_val: 1000.0, axis_px: 1000 });
        let write_snapshot = |path: &std::path::Path, offset: i16| {
            record_registry::start(hid, path.to_str().unwrap(), &meta).expect("start");
            let state = record_registry::get(hid).expect("registered");
            let indices: Vec<u32> = (0..n).collect();
            let values: Vec<i16> = (0..n).map(|i| ((i % 900) as i16 + offset).clamp(0, 999)).collect();
            state.append(&scalar_wire_frame(hid, &indices, &values));
            record_registry::stop(hid);
        };

        write_snapshot(&base_path, 0);
        for i in 0..100 {
            annotations::add(hid, i, format!("bookmark {i}"), None).expect("add bookmark");
        }
        Python::with_gil(|py| {
            anomaly_registry::set(hid, 5, 3.0, anomaly_registry::Method::Zscore, py.None());
            for _ in 0..600u32 {
                for _ in 0..8 {
                    anomaly_registry::check_and_notify(py, hid, &[0], &[10.0]);
                }
                anomaly_registry::check_and_notify(py, hid, &[0], &[999_999.0]);
            }
        });
        let logged_before_render = anomaly_registry::logged_events(hid).len();
        write_snapshot(&cur_path, 25);

        let started = std::time::Instant::now();
        let report = build_report(hid, base_path.to_str().unwrap(), cur_path.to_str().unwrap(), None, "markdown").expect("build report");
        let elapsed = started.elapsed();

        let _ = std::fs::remove_file(&base_path);
        let _ = std::fs::remove_file(&cur_path);
        annotations::clear(hid);
        anomaly_registry::clear(hid);

        println!("generate_incident_report: {n} indices, 100 bookmarks, {logged_before_render} logged anomalies in {elapsed:?}");
        assert!(elapsed.as_millis() < 1000, "incident report generation got too slow: {elapsed:?}");
        assert_eq!(logged_before_render, 500, "the spike pattern should reliably saturate the 500-event log cap");
        assert!(report.contains("Bookmarks (100)"), "the 100 bookmarks generated between the two snapshots should land inside the window, not be filtered out, got a report of {} bytes", report.len());
        assert!(report.contains("Anomalies logged (500)"), "the 500 logged anomalies should land inside the window too");
        assert!(report.contains("Filtered to the incident window"), "both snapshots are fresh v3 recordings so a real window should apply");
    }
}
