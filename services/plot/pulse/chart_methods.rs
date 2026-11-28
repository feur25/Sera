use crate::_py::{extract_num_source, NumSource};
use crate::plot::canvas_points::{b64_encode, pack_indices_u32_raw, pack_points_i16_raw, pack_scalar_i16_raw};
use crate::plot::push_registry;
use crate::Chart;
use pyo3::prelude::*;
use pyo3::types::{PyAny, PyList};
use seraplot_macros::{sera_doc, sera_doc_impl};

fn require_meta(html: &str) -> PyResult<(u64, push_registry::PushMeta)> {
    let hid = push_registry::extract_hid(html).ok_or_else(|| {
        pyo3::exceptions::PyValueError::new_err(
            "push()/serve() require a chart built via one of the framework's native fast paths (enough points/bars to cross the render threshold)",
        )
    })?;
    let meta = push_registry::get(hid).ok_or_else(|| {
        pyo3::exceptions::PyValueError::new_err(
            "push()/push_vector()/serve() need incremental-update metadata that only charts built via a native fast path register (scatter/line/bubble/area for x/y point data, bar/heatmap for label/value data, candlestick for OHLC data, each above its render threshold); this chart either has a different type, was too small to use that path, or its metadata aged out of the registry",
        )
    })?;
    Ok((hid, meta))
}

fn extract_indices(v: &Bound<'_, PyAny>) -> PyResult<Vec<u32>> {
    extract_num_source::<u32>(v)
        .map(|s| s.as_slice().to_vec())
        .ok_or_else(|| pyo3::exceptions::PyTypeError::new_err("indices must be a list[int] or a numpy uint32 array"))
}

fn extract_coord<'py>(v: &Bound<'py, PyAny>) -> PyResult<NumSource<'py, f64>> {
    extract_num_source::<f64>(v)
        .ok_or_else(|| pyo3::exceptions::PyTypeError::new_err("x/y must be a list[float] or a numpy float64 array"))
}

use super::frame_codec::{build_binary_frame, build_replay_script};
#[cfg(test)]
use super::frame_codec::encode_replay_frames;

fn record_frame(hid: u64, rec: &crate::plot::record_registry::Record) -> Vec<u8> {
    let idx_raw = pack_indices_u32_raw(&rec.indices);
    let mut pts_raw = Vec::with_capacity(rec.values.len() * 2);
    for v in &rec.values {
        pts_raw.extend_from_slice(&v.to_le_bytes());
    }
    build_binary_frame(hid, &idx_raw, &pts_raw, rec.n, rec.vpp)
}

#[cfg(feature = "python")]
#[sera_doc_impl]
#[pyo3::pymethods]
impl Chart {
    #[pyo3(signature = (indices, x, y = None))]
    #[sera_doc(
        category = "chart_method",
        module = "streaming",
        file = "seraplot/pulse-pricing.md",
        aliases("push_delta"),
        en = "Streams a delta update into an already-rendered chart instead of re-rendering it — updates the given indices in place, live in the browser if .serve() is active.",
        fr = "Streame une mise à jour delta dans un chart déjà rendu au lieu de le re-rendre — met à jour les indices donnés en place, en direct dans le navigateur si .serve() est actif.",
        param(name = "indices", ty = "list[int] | ndarray", en = "Indices of the points/bars/values to update.", fr = "Indices des points/barres/valeurs à mettre à jour."),
        param(name = "x", ty = "list[float] | ndarray", en = "New x values (point charts), or the new values themselves (bar/heatmap-style charts).", fr = "Nouvelles valeurs x (charts de points), ou les nouvelles valeurs elles-mêmes (charts type bar/heatmap)."),
        param(name = "y", ty = "list[float] | ndarray | None", en = "New y values, required for x/y point charts (scatter/line/bubble/area) and omitted otherwise.", fr = "Nouvelles valeurs y, requises pour les charts de points x/y (scatter/line/bubble/area), omises sinon.")
    )]
    fn push(
        &self,
        py: Python<'_>,
        indices: &Bound<'_, PyAny>,
        x: &Bound<'_, PyAny>,
        y: Option<&Bound<'_, PyAny>>,
    ) -> PyResult<(String, String)> {
        let (hid, meta) = require_meta(&self.html)?;
        let idx = extract_indices(indices)?;
        if !crate::plot::pulse_features::rate_limit::check(hid) {
            if let (push_registry::PushMeta::Scalar(sm), None) = (&meta, &y) {
                if crate::plot::pulse_features::downsample::is_enabled(hid) {
                    let vs = extract_coord(x)?;
                    crate::plot::pulse_features::downsample::absorb(hid, &idx, vs.as_slice());
                    crate::plot::pulse_features::metrics::incr("seraplot_pulse_push_downsampled_total", hid);
                    let idx_raw = pack_indices_u32_raw(&idx);
                    let pts_raw = pack_scalar_i16_raw(vs.as_slice(), sm.min_val, sm.range_val, sm.axis_px);
                    return Ok((b64_encode(&idx_raw), b64_encode(&pts_raw)));
                }
            }
            crate::plot::pulse_features::metrics::incr("seraplot_pulse_push_rejected_total", hid);
            return Err(pyo3::exceptions::PyRuntimeError::new_err(
                "seraplot pulse: push rate limit exceeded, set via set_push_rate_limit(); call it again with a higher value, pass downsample=True to set_push_rate_limit() to aggregate excess pushes into a summary instead (scalar/bar-style charts only), or clear_push_rate_limit() to remove it",
            ));
        }
        crate::plot::pulse_features::metrics::incr("seraplot_pulse_push_total", hid);
        let idx_raw = pack_indices_u32_raw(&idx);
        let (pts_raw, vpp): (Vec<u8>, u32) = match &meta {
            push_registry::PushMeta::Point(pm) => {
                let Some(y) = y else {
                    return Err(pyo3::exceptions::PyTypeError::new_err(
                        "this chart's push() needs both x and y (it was built from x/y point data)",
                    ));
                };
                let xs = extract_coord(x)?;
                let ys = extract_coord(y)?;
                crate::plot::anomaly_registry::check_and_notify(py, hid, &idx, ys.as_slice());
                let pts_raw = pack_points_i16_raw(
                    xs.as_slice(),
                    ys.as_slice(),
                    pm.min_x,
                    pm.range_x,
                    pm.min_y,
                    pm.range_y,
                    pm.plot_w,
                    pm.plot_h,
                );
                (pts_raw, 2)
            }
            push_registry::PushMeta::Scalar(sm) => {
                if y.is_some() {
                    return Err(pyo3::exceptions::PyTypeError::new_err(
                        "this chart's push() takes a single value array (it was built from label/value data): push(indices, values)",
                    ));
                }
                let vs = extract_coord(x)?;
                crate::plot::anomaly_registry::check_and_notify(py, hid, &idx, vs.as_slice());
                let pts_raw = pack_scalar_i16_raw(vs.as_slice(), sm.min_val, sm.range_val, sm.axis_px);
                (pts_raw, 1)
            }
            push_registry::PushMeta::Vector(_) => {
                return Err(pyo3::exceptions::PyTypeError::new_err(
                    "this chart was built from more than two parallel value arrays (e.g. OHLC): use push_vector(indices, [array1, array2, ...]) instead of push()",
                ));
            }
        };
        let needs_frame = crate::plot::record_registry::get(hid).is_some();
        #[cfg(feature = "webapp")]
        let needs_frame = needs_frame || push_registry::get_push_tx(hid).is_some();
        if needs_frame {
            if let push_registry::PushMeta::Scalar(sm) = &meta {
                let pending = crate::plot::pulse_features::downsample::take_pending(hid, &idx);
                if !pending.is_empty() {
                    let pending_idx: Vec<u32> = pending.iter().map(|(i, _)| *i).collect();
                    let pending_vals: Vec<f64> = pending.iter().map(|(_, v)| *v).collect();
                    let pending_idx_raw = pack_indices_u32_raw(&pending_idx);
                    let pending_pts_raw = pack_scalar_i16_raw(&pending_vals, sm.min_val, sm.range_val, sm.axis_px);
                    let catch_up_frame = build_binary_frame(hid, &pending_idx_raw, &pending_pts_raw, pending_idx.len() as u32, 1);
                    if let Some(rec) = crate::plot::record_registry::get(hid) {
                        rec.append(&catch_up_frame);
                    }
                    #[cfg(feature = "webapp")]
                    if let Some(tx) = push_registry::get_push_tx(hid) {
                        let _ = tx.send((crate::webapp::ws::OPCODE_BINARY, catch_up_frame));
                    }
                }
            }
            let frame = build_binary_frame(hid, &idx_raw, &pts_raw, idx.len() as u32, vpp);
            if let Some(rec) = crate::plot::record_registry::get(hid) {
                rec.append(&frame);
            }
            #[cfg(feature = "webapp")]
            if let Some(tx) = push_registry::get_push_tx(hid) {
                crate::plot::pulse_features::health::touch("serve", Some(hid));
                let _ = tx.send((crate::webapp::ws::OPCODE_BINARY, frame));
            }
        }
        Ok((b64_encode(&idx_raw), b64_encode(&pts_raw)))
    }

    #[sera_doc(
        category = "chart_method",
        module = "streaming",
        file = "seraplot/pulse-pricing.md",
        aliases("push_ohlc"),
        en = "Same as push(), for charts built from more than two parallel value arrays (OHLC/candlestick) — one new value per component, per index.",
        fr = "Identique à push(), pour les charts construits à partir de plus de deux tableaux de valeurs parallèles (OHLC/candlestick) — une nouvelle valeur par composant, par indice.",
        param(name = "indices", ty = "list[int] | ndarray", en = "Indices of the values to update.", fr = "Indices des valeurs à mettre à jour."),
        param(name = "components", ty = "list[list[float]]", en = "One list per value array (e.g. [open, high, low, close]), same order as the chart was built with.", fr = "Une liste par tableau de valeurs (ex : [open, high, low, close]), dans le même ordre qu'à la construction du chart.")
    )]
    fn push_vector(&self, py: Python<'_>, indices: &Bound<'_, PyAny>, components: Vec<Vec<f64>>) -> PyResult<(String, String)> {
        let (hid, meta) = require_meta(&self.html)?;
        if !crate::plot::pulse_features::rate_limit::check(hid) {
            crate::plot::pulse_features::metrics::incr("seraplot_pulse_push_rejected_total", hid);
            return Err(pyo3::exceptions::PyRuntimeError::new_err(
                "seraplot pulse: push rate limit exceeded, set via set_push_rate_limit(); call it again with a higher value or clear_push_rate_limit() to remove it",
            ));
        }
        crate::plot::pulse_features::metrics::incr("seraplot_pulse_push_total", hid);
        let vm = match &meta {
            push_registry::PushMeta::Vector(vm) => vm,
            _ => {
                return Err(pyo3::exceptions::PyTypeError::new_err(
                    "this chart was not built from more than two parallel value arrays: use push() instead of push_vector()",
                ));
            }
        };
        if components.len() as u32 != vm.vpp {
            return Err(pyo3::exceptions::PyValueError::new_err(format!(
                "this chart expects {} parallel value arrays, got {}",
                vm.vpp,
                components.len()
            )));
        }
        let idx = extract_indices(indices)?;
        let idx_raw = pack_indices_u32_raw(&idx);
        if let Some(last) = components.last() {
            crate::plot::anomaly_registry::check_and_notify(py, hid, &idx, last);
        }
        let refs: Vec<&[f64]> = components.iter().map(|c| c.as_slice()).collect();
        let pts_raw = crate::plot::canvas_points::pack_vector_i16_raw(&refs, vm.min_val, vm.range_val, vm.axis_px);
        let vpp = vm.vpp;

        let needs_frame = crate::plot::record_registry::get(hid).is_some();
        #[cfg(feature = "webapp")]
        let needs_frame = needs_frame || push_registry::get_push_tx(hid).is_some();
        if needs_frame {
            if let push_registry::PushMeta::Scalar(sm) = &meta {
                let pending = crate::plot::pulse_features::downsample::take_pending(hid, &idx);
                if !pending.is_empty() {
                    let pending_idx: Vec<u32> = pending.iter().map(|(i, _)| *i).collect();
                    let pending_vals: Vec<f64> = pending.iter().map(|(_, v)| *v).collect();
                    let pending_idx_raw = pack_indices_u32_raw(&pending_idx);
                    let pending_pts_raw = pack_scalar_i16_raw(&pending_vals, sm.min_val, sm.range_val, sm.axis_px);
                    let catch_up_frame = build_binary_frame(hid, &pending_idx_raw, &pending_pts_raw, pending_idx.len() as u32, 1);
                    if let Some(rec) = crate::plot::record_registry::get(hid) {
                        rec.append(&catch_up_frame);
                    }
                    #[cfg(feature = "webapp")]
                    if let Some(tx) = push_registry::get_push_tx(hid) {
                        let _ = tx.send((crate::webapp::ws::OPCODE_BINARY, catch_up_frame));
                    }
                }
            }
            let frame = build_binary_frame(hid, &idx_raw, &pts_raw, idx.len() as u32, vpp);
            if let Some(rec) = crate::plot::record_registry::get(hid) {
                rec.append(&frame);
            }
            #[cfg(feature = "webapp")]
            if let Some(tx) = push_registry::get_push_tx(hid) {
                crate::plot::pulse_features::health::touch("serve", Some(hid));
                let _ = tx.send((crate::webapp::ws::OPCODE_BINARY, frame));
            }
        }
        Ok((b64_encode(&idx_raw), b64_encode(&pts_raw)))
    }

    #[sera_doc(
        category = "chart_method",
        module = "streaming",
        file = "seraplot/pulse-pricing.md",
        aliases("firehose_append", "firehose_write"),
        en = "Appends one or more new readings to a chart built with sp.firehose(...) - the chart's own fixed-size ring buffer, server and browser alike, so it never grows unbounded no matter how long the stream runs. You never track write positions yourself: each call claims the next capacity-wrapped slot(s) and overwrites the oldest reading once the buffer is full. Internally this is push() with auto-computed indices, so it inherits the exact same rate limiting, anomaly detection, recording and serve() broadcast behaviour.",
        fr = "Ajoute une ou plusieurs nouvelles valeurs à un chart construit avec sp.firehose(...) - son propre tampon circulaire de taille fixe, côté serveur comme navigateur, qui ne grossit donc jamais quelle que soit la durée du flux. Vous ne suivez jamais vous-même les positions d'écriture : chaque appel réclame le(s) prochain(s) emplacement(s), avec retour au début une fois la capacité atteinte, en écrasant la valeur la plus ancienne. En interne il s'agit de push() avec des indices calculés automatiquement, donc il hérite exactement de la même limitation de débit, détection d'anomalies, enregistrement et diffusion serve().",
        param(name = "values", ty = "list[float] | ndarray", en = "New readings to append, oldest first.", fr = "Nouvelles valeurs à ajouter, la plus ancienne en premier.")
    )]
    fn firehose_push(&self, py: Python<'_>, values: &Bound<'_, PyAny>) -> PyResult<(String, String)> {
        let hid = push_registry::extract_hid(&self.html).ok_or_else(|| {
            pyo3::exceptions::PyValueError::new_err(
                "firehose_push() requires a chart built via sp.firehose(...)",
            )
        })?;
        let n = values.len()?;
        let idx = crate::plot::firehose_registry::next_indices(hid, n).ok_or_else(|| {
            pyo3::exceptions::PyValueError::new_err(
                "firehose_push() needs a chart built via sp.firehose(...) - no ring buffer is registered for this chart",
            )
        })?;
        let idx_list = PyList::new_bound(py, &idx);
        self.push(py, idx_list.as_any(), values, None)
    }

    #[sera_doc(
        category = "chart_method",
        module = "streaming",
        file = "seraplot/pulse-pricing.md",
        aliases("start_recording"),
        en = "Starts recording every subsequent push()/push_vector() delta to a binary .spls file, byte-identical to the wire format used by serve(). Replay it later with replay().",
        fr = "Démarre l'enregistrement de chaque delta push()/push_vector() suivant dans un fichier binaire .spls, identique octet pour octet au format réseau utilisé par serve(). À rejouer ensuite avec replay().",
        param(name = "path", ty = "str", en = "Destination file path for the recording.", fr = "Chemin du fichier de destination pour l'enregistrement.")
    )]
    pub(crate) fn record(&self, path: &str) -> PyResult<()> {
        let (hid, meta) = require_meta(&self.html)?;
        crate::plot::record_registry::start(hid, path, &meta)
            .map_err(|e| pyo3::exceptions::PyOSError::new_err(format!("seraplot: failed to start recording to {path}: {e}")))
    }

    #[sera_doc(
        category = "chart_method",
        module = "streaming",
        file = "seraplot/pulse-pricing.md",
        aliases("stop_recording"),
        en = "Stops the recording started by record(), if any.",
        fr = "Arrête l'enregistrement démarré par record(), s'il y en a un."
    )]
    pub(crate) fn stop_record(&self) -> PyResult<()> {
        let (hid, _) = require_meta(&self.html)?;
        crate::plot::record_registry::stop(hid);
        Ok(())
    }

    #[pyo3(signature = (callback, window = 50, threshold = 3.0, method = "zscore"))]
    #[sera_doc(
        category = "chart_method",
        module = "anomaly",
        file = "seraplot/pulse-pricing.md",
        aliases("watch_anomalies"),
        en = "Registers a rolling anomaly detector: callback(index, value, score) fires whenever a pushed value's score over the trailing window exceeds threshold. method='zscore' (default) flags values far from the rolling mean in standard deviations - fast, but one huge outlier already in the window can mask real anomalies afterwards (it drags the mean/stddev with it). method='iqr' flags values outside [Q1, Q3] by more than threshold interquartile ranges - more robust to that exact case, at the cost of sorting the window on every check. method='seasonal' keeps a separate window per hour-of-day (UTC) per index and compares each value only against its own hour's history - a value that's high for 3am but normal for 3pm won't be flagged just because it's high relative to the last few readings overall; costs up to window*24 memory per tracked index instead of window*1.",
        fr = "Enregistre un détecteur d'anomalies glissant : callback(index, value, score) se déclenche quand le score d'une valeur poussée, sur la fenêtre glissante, dépasse threshold. method='zscore' (défaut) signale les valeurs loin de la moyenne glissante en écarts-types — rapide, mais une seule valeur aberrante déjà dans la fenêtre peut ensuite masquer de vraies anomalies (elle entraîne la moyenne/l'écart-type avec elle). method='iqr' signale les valeurs hors de [Q1, Q3] de plus de threshold écarts interquartiles — plus robuste à ce cas précis, au prix d'un tri de la fenêtre à chaque vérification. method='seasonal' garde une fenêtre séparée par heure de la journée (UTC) et par indice, et compare chaque valeur uniquement à l'historique de sa propre heure — une valeur haute à 3h mais normale à 15h ne sera pas signalée juste parce qu'elle est haute par rapport aux dernières lectures toutes heures confondues ; coûte jusqu'à window*24 en mémoire par indice suivi au lieu de window*1.",
        param(name = "callback", ty = "Callable[[int, float, float], None]", en = "Called with (index, value, score) for each detected anomaly.", fr = "Appelé avec (index, value, score) pour chaque anomalie détectée."),
        param(name = "window", ty = "int", en = "Number of trailing values used to compute the rolling statistics (per hour-of-day bucket for 'seasonal').", fr = "Nombre de valeurs précédentes utilisées pour calculer les statistiques glissantes (par case horaire pour 'seasonal')."),
        param(name = "threshold", ty = "float", en = "Score magnitude above which a value is flagged as anomalous (standard deviations for 'zscore'/'seasonal', interquartile ranges for 'iqr').", fr = "Magnitude du score au-delà de laquelle une valeur est signalée comme anormale (écarts-types pour 'zscore'/'seasonal', écarts interquartiles pour 'iqr')."),
        param(name = "method", ty = "str", en = "'zscore' (default), 'iqr', or 'seasonal'.", fr = "'zscore' (défaut), 'iqr', ou 'seasonal'.")
    )]
    fn on_anomaly(&self, callback: PyObject, window: usize, threshold: f64, method: &str) -> PyResult<()> {
        let (hid, _) = require_meta(&self.html)?;
        let method = crate::plot::anomaly_registry::Method::parse(method)?;
        crate::plot::anomaly_registry::set(hid, window, threshold, method, callback);
        Ok(())
    }

    #[sera_doc(
        category = "chart_method",
        module = "anomaly",
        file = "seraplot/pulse-pricing.md",
        aliases("unwatch_anomalies"),
        en = "Removes the anomaly detector registered by on_anomaly(), if any.",
        fr = "Retire le détecteur d'anomalies enregistré par on_anomaly(), s'il y en a un."
    )]
    fn off_anomaly(&self) -> PyResult<()> {
        let (hid, _) = require_meta(&self.html)?;
        crate::plot::anomaly_registry::clear(hid);
        Ok(())
    }

    #[pyo3(signature = (path, speed = 1.0, callback = None))]
    #[sera_doc(
        category = "chart_method",
        module = "streaming",
        file = "seraplot/pulse-pricing.md",
        aliases("playback"),
        en = "Replays a .spls recording, resending its exact recorded pixel-space deltas — to any active serve() connection, and/or to callback(indices, values, vpp) for deterministic offline verification.",
        fr = "Rejoue un enregistrement .spls, en renvoyant exactement les deltas enregistrés en espace pixel — vers toute connexion serve() active, et/ou vers callback(indices, values, vpp) pour une vérification déterministe hors ligne.",
        param(name = "path", ty = "str", en = "Path to the .spls recording to replay.", fr = "Chemin de l'enregistrement .spls à rejouer."),
        param(name = "speed", ty = "float", en = "Playback speed multiplier; 0 replays every frame as fast as possible, with no waiting.", fr = "Multiplicateur de vitesse de lecture ; 0 rejoue chaque frame le plus vite possible, sans attente."),
        param(name = "callback", ty = "Callable[[list[int], list[float], int], None] | None", en = "Optional callback invoked with (indices, values, vpp) for each replayed frame.", fr = "Callback optionnel invoqué avec (indices, values, vpp) pour chaque frame rejouée.")
    )]
    fn replay(
        &self,
        py: Python<'_>,
        path: &str,
        speed: f64,
        callback: Option<&Bound<'_, PyAny>>,
    ) -> PyResult<usize> {
        let (hid, _) = require_meta(&self.html)?;
        #[cfg(not(feature = "webapp"))]
        let _ = hid;
        let records = crate::plot::record_registry::read_all(path)
            .map_err(|e| pyo3::exceptions::PyOSError::new_err(format!("seraplot: failed to read recording {path}: {e}")))?
            .records;

        let mut last_dt: u32 = 0;
        let mut count = 0usize;
        for rec in &records {
            if speed > 0.0 {
                let delta_ms = rec.dt_ms.saturating_sub(last_dt);
                if delta_ms > 0 {
                    let wait_s = (delta_ms as f64) / 1000.0 / speed;
                    py.allow_threads(|| std::thread::sleep(std::time::Duration::from_secs_f64(wait_s.max(0.0))));
                }
            }
            last_dt = rec.dt_ms;

            #[cfg(feature = "webapp")]
            if let Some(tx) = push_registry::get_push_tx(hid) {
                let _ = tx.send((crate::webapp::ws::OPCODE_BINARY, record_frame(hid, rec)));
            }

            if let Some(cb) = callback {
                cb.call1((rec.indices.clone(), rec.values.clone(), rec.vpp))?;
            }
            count += 1;
        }
        Ok(count)
    }

    #[sera_doc(
        category = "chart_method",
        module = "streaming",
        file = "seraplot/pulse-pricing.md",
        en = "Bundles this chart and a .spls recording into ONE portable HTML file that replays the exact recorded deltas client-side, on open — no server, no Python, opens standalone in any browser. Reuses the same sp_push_<id> wire function every native chart already embeds for push()/serve(), so no separate rendering engine is needed inside the file.",
        fr = "Regroupe ce chart et un enregistrement .spls en UN SEUL fichier HTML portable qui rejoue les deltas enregistrés côté client, à l'ouverture — sans serveur, sans Python, s'ouvre de façon autonome dans n'importe quel navigateur. Réutilise la même fonction réseau sp_push_<id> que chaque chart natif embarque déjà pour push()/serve(), donc aucun moteur de rendu séparé n'est nécessaire dans le fichier.",
        param(name = "html_path", ty = "str", en = "Destination .html file path.", fr = "Chemin du fichier .html de destination."),
        param(name = "spls_path", ty = "str", en = "Path to the .spls recording to embed and replay.", fr = "Chemin de l'enregistrement .spls à embarquer et rejouer.")
    )]
    pub(crate) fn export_standalone(&self, html_path: &str, spls_path: &str) -> PyResult<()> {
        let (hid, _) = require_meta(&self.html)?;
        let records = crate::plot::record_registry::read_all(spls_path)
            .map_err(|e| pyo3::exceptions::PyOSError::new_err(format!("seraplot: failed to read recording {spls_path}: {e}")))?
            .records;
        let script = build_replay_script(hid, &records);
        let final_html = crate::html::hover::inject_before_body(&self.html, &script);
        std::fs::write(html_path, final_html)
            .map_err(|e| pyo3::exceptions::PyOSError::new_err(format!("seraplot: failed to write {html_path}: {e}")))?;
        Ok(())
    }

    #[cfg(feature = "webapp")]
    #[pyo3(signature = (host = "127.0.0.1", port = 8787, username = None, password = None, collab = true))]
    #[sera_doc(
        category = "chart_method",
        module = "streaming",
        file = "seraplot/pulse-pricing.md",
        aliases("stream"),
        en = "Starts a local WebSocket server that streams this chart's push()/push_vector() updates live to any browser that opens it, over a binary frame protocol. Pass username/password to require HTTP Basic Auth before serving the page or opening the WebSocket. When collab is true (default), everyone connected also sees each other's live cursor, can pin a comment to a data point, and can trigger a shared replay of any recording in progress — Sera Live.",
        fr = "Démarre un serveur WebSocket local qui streame en direct les mises à jour push()/push_vector() de ce chart vers tout navigateur qui l'ouvre, via un protocole de trames binaires. Passer username/password exige une authentification HTTP Basic avant de servir la page ou d'ouvrir le WebSocket. Quand collab vaut true (défaut), tout le monde connecté voit aussi le curseur des autres en direct, peut épingler un commentaire sur un point de donnée, et peut déclencher un rejeu partagé de l'enregistrement en cours — Sera Live.",
        param(name = "host", ty = "str", en = "Address to bind the server on.", fr = "Adresse sur laquelle lier le serveur."),
        param(name = "port", ty = "int", en = "Port to bind the server on.", fr = "Port sur lequel lier le serveur."),
        param(name = "username", ty = "str | None", en = "If set together with password, requires HTTP Basic Auth to view the chart or connect.", fr = "Si défini avec password, exige une authentification HTTP Basic pour voir le chart ou se connecter."),
        param(name = "password", ty = "str | None", en = "See username.", fr = "Voir username."),
        param(name = "collab", ty = "bool", en = "Enables Sera Live: shared cursors, pinned comments, and a shared replay button visible to every connected browser. Set to False for a single-viewer stream with no presence broadcast at all.", fr = "Active Sera Live : curseurs partagés, commentaires épinglés, et un bouton de rejeu partagé visibles par chaque navigateur connecté. Mettre à False pour un flux mono-spectateur sans aucune diffusion de présence.")
    )]
    fn serve(&self, host: &str, port: u16, username: Option<&str>, password: Option<&str>, collab: bool) -> PyResult<()> {
        let (hid, _) = require_meta(&self.html)?;
        let addr = format!("{host}:{port}");
        let std_listener = crate::webapp::server::bind_listener(&addr).map_err(|e| {
            pyo3::exceptions::PyOSError::new_err(format!("seraplot: failed to bind {addr}: {e}"))
        })?;
        crate::plot::pulse_features::health::register("serve", Some(hid), host, port);
        let push_tx = push_registry::get_or_create_push_tx(hid);
        let auth = username.zip(password).map(|(u, p)| (u.to_string(), p.to_string()));
        let state = std::sync::Arc::new(ServeState {
            html: self.html.clone(),
            push_tx,
            auth,
            hid,
            collab,
            board: false,
            started_at_ms: crate::plot::now_ms(),
            session: std::sync::Mutex::new(None),
        });
        spawn_serve_thread(std_listener, state, "Chart.serve()");
        Ok(())
    }

    #[pyo3(signature = (indices, x, y = None))]
    fn push_delta(
        &self,
        py: Python<'_>,
        indices: &Bound<'_, PyAny>,
        x: &Bound<'_, PyAny>,
        y: Option<&Bound<'_, PyAny>>,
    ) -> PyResult<(String, String)> {
        self.push(py, indices, x, y)
    }

    fn push_ohlc(&self, py: Python<'_>, indices: &Bound<'_, PyAny>, components: Vec<Vec<f64>>) -> PyResult<(String, String)> {
        self.push_vector(py, indices, components)
    }

    fn firehose_append(&self, py: Python<'_>, values: &Bound<'_, PyAny>) -> PyResult<(String, String)> {
        self.firehose_push(py, values)
    }

    fn firehose_write(&self, py: Python<'_>, values: &Bound<'_, PyAny>) -> PyResult<(String, String)> {
        self.firehose_push(py, values)
    }

    fn start_recording(&self, path: &str) -> PyResult<()> {
        self.record(path)
    }

    fn stop_recording(&self) -> PyResult<()> {
        self.stop_record()
    }

    #[pyo3(signature = (callback, window = 50, threshold = 3.0, method = "zscore"))]
    fn watch_anomalies(&self, callback: PyObject, window: usize, threshold: f64, method: &str) -> PyResult<()> {
        self.on_anomaly(callback, window, threshold, method)
    }

    fn unwatch_anomalies(&self) -> PyResult<()> {
        self.off_anomaly()
    }

    #[pyo3(signature = (path, speed = 1.0, callback = None))]
    fn playback(&self, py: Python<'_>, path: &str, speed: f64, callback: Option<&Bound<'_, PyAny>>) -> PyResult<usize> {
        self.replay(py, path, speed, callback)
    }

    #[cfg(feature = "webapp")]
    #[pyo3(signature = (host = "127.0.0.1", port = 8787, username = None, password = None, collab = true))]
    fn stream(&self, host: &str, port: u16, username: Option<&str>, password: Option<&str>, collab: bool) -> PyResult<()> {
        self.serve(host, port, username, password, collab)
    }
}

#[cfg(feature = "webapp")]
const WS_BOOTSTRAP_JS: &str = "<script>(function(){var ws=new WebSocket((location.protocol==='https:'?'wss://':'ws://')+location.host+'/ws');ws.binaryType='arraybuffer';window.__spWS=ws;ws.onopen=function(){if(window.__spLiveOnOpen)window.__spLiveOnOpen();};ws.onmessage=function(e){if(typeof e.data==='string'){var msg;try{msg=JSON.parse(e.data);}catch(err){return;}if(window.__spLiveOnMessage)window.__spLiveOnMessage(msg);return;}var dv=new DataView(e.data);var hid=dv.getUint32(0,true),n=dv.getUint32(4,true),vpp=dv.getUint32(8,true);var idx=new Uint32Array(e.data,12,n);var pts=new Int16Array(e.data,12+n*4,n*vpp);var f=window['sp_apply_'+hid];if(typeof f==='function')f(idx,pts,vpp);};})();</script>";

#[cfg(feature = "webapp")]
const LIVE_JS: &str = "<script>(function(){ var MYSESS=0,MYNAME='Guest'+Math.floor(Math.random()*9000+1000),MYCOLOR='#334155'; var boardMode=!!window.__spBoardMode; var cursors={}; var marksById={}; var selectedId=null,selectedKind=null,selectMode=false,pinMode=false,drawMode=false; var svgNS='http://www.w3.org/2000/svg'; var themeStyle=document.createElement('style'); themeStyle.textContent=':root{--sp-bg:#f1f5f9;--sp-sidebar-bg:#0b0e18;--sp-sidebar-bg2:#12172a;--sp-sidebar-fg:#e2e8f0;--sp-sidebar-muted:#93a4c3;--sp-frame-bg:#ffffff;--sp-frame-border:#e2e8f0;--sp-frame-bar-bg:#f8fafc;--sp-frame-bar-fg:#334155;--sp-panel-bg:#ffffff;--sp-panel-fg:#0f172a;--sp-panel-border:#cbd5e1;--sp-panel-muted:#94a3b8;--sp-item-hover:rgba(255,255,255,.09);--sp-zoom-bg:#ffffff;--sp-zoom-fg:#475569;}[data-sp-theme=dark]{--sp-bg:#0d0f16;--sp-sidebar-bg:#05060a;--sp-sidebar-bg2:#0a0c14;--sp-sidebar-fg:#cbd5e1;--sp-sidebar-muted:#64748b;--sp-frame-bg:#171a23;--sp-frame-border:#2a2f3d;--sp-frame-bar-bg:#1d2130;--sp-frame-bar-fg:#cbd5e1;--sp-panel-bg:#171a23;--sp-panel-fg:#e5e7eb;--sp-panel-border:#2a2f3d;--sp-panel-muted:#7c8aa5;--sp-item-hover:rgba(255,255,255,.06);--sp-zoom-bg:#171a23;--sp-zoom-fg:#cbd5e1;}'; document.head.appendChild(themeStyle); var ovl=document.createElement('div'); ovl.style.cssText='position:absolute;left:0;top:0;pointer-events:none;z-index:99998'; document.body.appendChild(ovl); var svgLayer=document.createElementNS(svgNS,'svg'); svgLayer.setAttribute('style','position:absolute;left:0;top:0;width:1px;height:1px;overflow:visible;pointer-events:none;z-index:99996'); document.body.appendChild(svgLayer); var boardSvgLayer=null; var bar=document.createElement('div'); bar.style.cssText='position:fixed;top:8px;right:8px;display:flex;gap:4px;z-index:99999;font:11px -apple-system,Arial,sans-serif'; document.body.appendChild(bar); var timerBadge=document.createElement('div'); timerBadge.style.cssText='position:fixed;top:8px;left:50%;transform:translateX(-50%);z-index:99999;font:11px -apple-system,Arial,sans-serif;padding:5px 12px;border-radius:8px;background:rgba(15,23,42,.85);color:#fff'; document.body.appendChild(timerBadge); function lockSelection(v){document.body.style.userSelect=v?'none':'';document.body.style.webkitUserSelect=v?'none':'';} var pencilCursor=null; function buildPencilCursor(){ var s=document.createElementNS(svgNS,'svg'); s.setAttribute('width','24');s.setAttribute('height','24');s.setAttribute('viewBox','0 0 24 24'); var p=document.createElementNS(svgNS,'path'); p.setAttribute('fill','#3b82f6');p.setAttribute('stroke','#ffffff');p.setAttribute('stroke-width','1'); p.setAttribute('d','M3 21l1.6-5.6L15.7 4.3a2 2 0 0 1 2.8 0l1.2 1.2a2 2 0 0 1 0 2.8L8.6 19.4 3 21z'); s.appendChild(p); return 'url(data:image/svg+xml,'+encodeURIComponent(new XMLSerializer().serializeToString(s))+') 2 20,crosshair'; } function setCursor(v){ document.body.style.cursor=v; var vp=document.getElementById('sp-board-viewport'); if(vp)vp.style.cursor=v; } var toolbar=document.createElement('div'); toolbar.style.cssText='position:fixed;bottom:16px;left:50%;transform:translateX(-50%);display:flex;align-items:center;gap:2px;background:#1e1e1e;border-radius:12px;padding:6px;z-index:99999;box-shadow:0 6px 20px rgba(0,0,0,.35)'; document.body.appendChild(toolbar); function mkIconBtn(glyph,label){ var b=document.createElement('button'); b.textContent=glyph;b.title=label; b.style.cssText='width:34px;height:34px;border:0;border-radius:8px;background:transparent;color:#e5e7eb;font-size:15px;cursor:pointer;display:flex;align-items:center;justify-content:center'; toolbar.appendChild(b); return b; } function mkDivider(){ var d=document.createElement('div'); d.style.cssText='width:1px;height:20px;background:#3f3f46;margin:0 4px'; toolbar.appendChild(d); } function setActive(btn,v){btn.style.background=v?'#3b82f6':'transparent';btn.style.color=v?'#fff':'#e5e7eb';} var selBtn=mkIconBtn('↖','Select'); var pbtn=mkIconBtn('📌','Pin a comment'); var dbtn=mkIconBtn('✏','Draw'); var imgBtn=mkIconBtn('🖼','Add image'); mkDivider(); var recolorBtn=mkIconBtn('🎨','Recolor selected'); recolorBtn.style.opacity='0.35'; var trashBtn=mkIconBtn('🗑','Delete selected'); trashBtn.style.opacity='0.35'; mkDivider(); var cbtn=mkIconBtn('💬','Comments (0)'); var rbtn=null,sbtn=null; if(!boardMode){rbtn=mkIconBtn('▶','Replay together');} if(boardMode){sbtn=mkIconBtn('🗂','Sessions');} mkDivider(); var themeBtn=mkIconBtn('🌙','Dark mode'); function applyTheme(v){ document.documentElement.setAttribute('data-sp-theme',v); themeBtn.textContent=v==='dark'?'☀':'🌙'; themeBtn.title=v==='dark'?'Light mode':'Dark mode'; try{localStorage.setItem('sp-theme',v);}catch(e){} } var savedTheme=null; try{savedTheme=localStorage.getItem('sp-theme');}catch(e){} applyTheme(savedTheme||(window.matchMedia&&window.matchMedia('(prefers-color-scheme: dark)').matches?'dark':'light')); themeBtn.onclick=function(){applyTheme(document.documentElement.getAttribute('data-sp-theme')==='dark'?'light':'dark');}; function mkPopover(){ var el=document.createElement('div'); el.style.cssText='display:none;position:fixed;bottom:60px;left:50%;transform:translateX(-50%);align-items:center;gap:10px;background:#1e1e1e;border-radius:10px;padding:8px 14px;z-index:99999;box-shadow:0 6px 20px rgba(0,0,0,.35);font:11px -apple-system,Arial,sans-serif;color:#e5e7eb'; document.body.appendChild(el); return el; } function mkLabel(text,into){var s=document.createElement('span');s.textContent=text;into.appendChild(s);return s;} var drawPanel=mkPopover(); mkLabel('Color',drawPanel); var colorInput=document.createElement('input'); colorInput.type='color';colorInput.value='#3b82f6'; colorInput.style.cssText='width:28px;height:24px;border:0;background:none;cursor:pointer'; colorInput.oninput=function(){drawColor=colorInput.value;}; drawPanel.appendChild(colorInput); mkLabel('Thickness',drawPanel); var widthInput=document.createElement('input'); widthInput.type='range';widthInput.min='1';widthInput.max='16';widthInput.value='3'; widthInput.style.cssText='width:90px'; widthInput.oninput=function(){drawWidth=parseInt(widthInput.value,10);}; drawPanel.appendChild(widthInput); var recolorPanel=mkPopover(); mkLabel('Color',recolorPanel); var recolorInput=document.createElement('input'); recolorInput.type='color';recolorInput.value='#3b82f6'; recolorInput.style.cssText='width:28px;height:24px;border:0;background:none;cursor:pointer'; recolorInput.oninput=function(){if(selectedId!==null)liveSend('recolor:'+selectedId+':'+recolorInput.value);}; recolorPanel.appendChild(recolorInput); var fileInput=document.createElement('input'); fileInput.type='file';fileInput.accept='image/*';fileInput.style.display='none'; document.body.appendChild(fileInput); function closeAllPopovers(){ drawPanel.style.display='none'; recolorPanel.style.display='none'; panel.style.display='none'; if(sessionsPanel)sessionsPanel.style.display='none'; } function setSelectMode(v){ selectMode=v; if(v){setPinMode(false);setDrawMode(false);} setActive(selBtn,v); setCursor(v?'grab':''); if(!v)selectMark(null); } function setPinMode(v){ pinMode=v; if(v){setSelectMode(false);setDrawMode(false);} setActive(pbtn,v); setCursor(v?'crosshair':''); } function setDrawMode(v){ drawMode=v; if(v){setSelectMode(false);setPinMode(false);} setActive(dbtn,v); if(v&&!pencilCursor)pencilCursor=buildPencilCursor(); setCursor(v?pencilCursor:''); recolorPanel.style.display='none'; drawPanel.style.display=v?'flex':'none'; } selBtn.onclick=function(){setSelectMode(!selectMode);}; pbtn.onclick=function(){setPinMode(!pinMode);}; dbtn.onclick=function(){setDrawMode(!drawMode);}; imgBtn.onclick=function(){fileInput.click();}; trashBtn.onclick=function(){deleteSelected();}; recolorBtn.onclick=function(){ if(!(selectedKind==='pin'||selectedKind==='stroke'))return; var show=recolorPanel.style.display==='none'; panel.style.display='none'; if(sessionsPanel)sessionsPanel.style.display='none'; drawPanel.style.display='none'; recolorPanel.style.display=show?'flex':'none'; }; function target(){return document.getElementById('sp-board-viewport')||document.querySelector('svg,canvas')||document.body;} window.__spToolActive=function(){return pinMode||drawMode||selectMode;}; function liveSend(v){if(window.__spWS&&window.__spWS.readyState===1)window.__spWS.send(JSON.stringify({id:'sp-live',value:v}));} window.__spLiveSend=liveSend; function renderPresence(list){ bar.innerHTML=''; list.forEach(function(p){ var d=document.createElement('div'); d.textContent=p.n;d.title=p.n; d.style.cssText='background:'+p.c+';color:#fff;padding:3px 9px;border-radius:12px;font-weight:600'; bar.appendChild(d); }); } function ensureCursor(sess,name,color){ var el=cursors[sess]; if(!el){ el=document.createElement('div'); el.style.cssText='position:absolute;width:12px;height:12px;border-radius:50% 50% 50% 0;transform:translate(-2px,-2px) rotate(45deg);box-shadow:0 0 0 2px #fff,0 1px 4px rgba(0,0,0,.3);transition:left .09s linear,top .09s linear'; var lbl=document.createElement('div'); lbl.style.cssText='position:absolute;left:14px;top:-2px;font:10px -apple-system,Arial,sans-serif;color:#fff;padding:2px 6px;border-radius:5px;white-space:nowrap;transform:rotate(-45deg);transform-origin:left top'; lbl.textContent=name; el.appendChild(lbl); ovl.appendChild(el); cursors[sess]=el; } el.style.background=color;el.firstChild.style.background=color; return el; } function frameOf(anchor){return anchor>=0?document.getElementById('sp-board-frame-'+anchor):null;} function worldPoint(e){ var r=target().getBoundingClientRect(); var vx=e.clientX-r.left,vy=e.clientY-r.top; var t=window.__spBoardGetTransform?window.__spBoardGetTransform():{x:0,y:0,z:1}; return {inBounds:vx>=0&&vy>=0&&vx<=r.width&&vy<=r.height,wx:(vx-t.x)/t.z,wy:(vy-t.y)/t.z}; } function pointFor(e,anchor){ if(!boardMode){ var r=target().getBoundingClientRect(); var x=Math.round(e.clientX-r.left),y=Math.round(e.clientY-r.top); if(x<0||y<0||x>r.width||y>r.height)return null; return {x:x,y:y,a:-1}; } var wp=worldPoint(e); if(typeof anchor!=='number'){ var frameEl=e.target.closest('.sp-board-frame'); anchor=frameEl?parseInt(frameEl.id.slice('sp-board-frame-'.length),10):-1; if(isNaN(anchor))anchor=-1; }else if(!wp.inBounds){ return null; } var frame=frameOf(anchor); if(frame)return {x:Math.round(wp.wx-parseFloat(frame.dataset.x)),y:Math.round(wp.wy-parseFloat(frame.dataset.y)),a:anchor}; if(!wp.inBounds)return null; return {x:Math.round(wp.wx),y:Math.round(wp.wy),a:-1}; } function anchorHost(a){ if(!boardMode)return document.body; var frame=frameOf(a); if(frame)return frame; return window.__spBoardWorld||document.body; } function svgHost(a){ if(!boardMode)return svgLayer; var frame=frameOf(a); if(frame){ if(!frame._spSvg){ var s=document.createElementNS(svgNS,'svg'); s.setAttribute('style','position:absolute;left:0;top:0;width:100%;height:100%;overflow:visible;pointer-events:none;z-index:5'); frame.appendChild(s); frame._spSvg=s; } return frame._spSvg; } if(!boardSvgLayer){ boardSvgLayer=document.createElementNS(svgNS,'svg'); boardSvgLayer.setAttribute('style','position:absolute;left:0;top:0;width:1px;height:1px;overflow:visible;pointer-events:none;z-index:2'); (window.__spBoardWorld||document.body).appendChild(boardSvgLayer); } return boardSvgLayer; } var pinLog=[]; var panel=mkPopover(); panel.style.flexDirection='column';panel.style.alignItems='stretch';panel.style.padding='0'; panel.style.width='280px';panel.style.maxHeight='320px';panel.style.overflowY='auto'; panel.style.background='var(--sp-panel-bg)';panel.style.color='var(--sp-panel-fg)';panel.style.border='1px solid var(--sp-panel-border)'; function renderPanel(){ panel.innerHTML=''; if(!pinLog.length){ var empty=document.createElement('div'); empty.style.cssText='padding:10px;color:var(--sp-panel-muted)'; empty.textContent='No comments yet.'; panel.appendChild(empty); return; } pinLog.slice().reverse().forEach(function(p){ var row=document.createElement('div'); row.style.cssText='padding:8px 10px;border-bottom:1px solid var(--sp-panel-border);cursor:pointer'; var nm=document.createElement('div'); nm.style.cssText='font-weight:700;color:'+p.c; nm.textContent=p.n; var lb=document.createElement('div'); lb.style.cssText='color:var(--sp-panel-fg);margin-top:2px'; lb.textContent=p.label; row.appendChild(nm);row.appendChild(lb); row.onclick=function(){focusOnMark(p);}; panel.appendChild(row); }); } cbtn.onclick=function(){ var show=panel.style.display==='none'; closeAllPopovers(); panel.style.display=show?'flex':'none'; }; function focusOnMark(p){ if(!boardMode){window.scrollTo({top:Math.max(0,p.y-120),behavior:'smooth'});return;} var wx=p.x,wy=p.y; var frame=frameOf(p.a); if(frame){wx=parseFloat(frame.dataset.x)+p.x;wy=parseFloat(frame.dataset.y)+p.y;} if(window.__spBoardFocusAt)window.__spBoardFocusAt(wx,wy,1); } function highlight(el,on){if(!el)return;el.style.filter=on?'drop-shadow(0 0 5px #3b82f6)':'';} function selectMark(id){ if(selectedId!==null&&marksById[selectedId])highlight(marksById[selectedId].hl,false); selectedId=id; var m=id!==null?marksById[id]:null; selectedKind=m?m.kind:null; if(m)highlight(m.hl,true); trashBtn.style.opacity=id!==null?'1':'0.35'; var recolorable=selectedKind==='pin'||selectedKind==='stroke'; recolorBtn.style.opacity=recolorable?'1':'0.35'; if(recolorable&&m.c)recolorInput.value=m.c; if(!recolorable)recolorPanel.style.display='none'; } function deleteSelected(){if(selectedId===null)return;liveSend('unmark:'+selectedId);} function removeMark(id){ var m=marksById[id]; if(m&&m.el&&m.el.parentNode)m.el.parentNode.removeChild(m.el); if(m&&m.hit&&m.hit.parentNode)m.hit.parentNode.removeChild(m.hit); delete marksById[id]; if(selectedId===id)selectMark(null); var idx=pinLog.findIndex(function(p){return p.id===id;}); if(idx>=0){pinLog.splice(idx,1);cbtn.title='Comments ('+pinLog.length+')';renderPanel();} } function clearAllMarks(){ Object.keys(marksById).forEach(function(id){removeMark(Number(id));}); pinLog=[]; renderPanel(); } function applyRecolor(id,color){ var m=marksById[id]; if(!m)return; m.c=color; if(m.kind==='pin'){ m.hl.style.background=color; if(m.tag)m.tag.style.background=color; }else if(m.kind==='stroke'){ m.hl.setAttribute('stroke',color); } var log=pinLog.find(function(p){return p.id===id;}); if(log){log.c=color;renderPanel();} if(selectedId===id)recolorInput.value=color; } function repositionMarkVisual(id,x,y){ var m=marksById[id]; if(!m||m.kind!=='image')return; if(boardMode){ m.el.style.left=x+'px';m.el.style.top=y+'px'; }else{ var r=target().getBoundingClientRect(); m.el.style.left=(r.left+window.scrollX+x)+'px'; m.el.style.top=(r.top+window.scrollY+y)+'px'; } } function placePin(x,y,a,label,name,color,id){ var host=anchorHost(a); var wrap=document.createElement('div'); if(boardMode){ wrap.style.cssText='position:absolute;left:'+x+'px;top:'+y+'px;z-index:99997'; }else{ var r=target().getBoundingClientRect(); wrap.style.cssText='position:absolute;left:'+(r.left+window.scrollX+x)+'px;top:'+(r.top+window.scrollY+y)+'px;z-index:99997'; } var dot=document.createElement('div'); dot.style.cssText='width:9px;height:9px;border-radius:50%;background:'+color+';box-shadow:0 0 0 2px #fff,0 1px 3px rgba(0,0,0,.35);pointer-events:auto;cursor:pointer'; var tag=document.createElement('div'); tag.style.cssText='position:absolute;left:12px;top:-4px;background:'+color+';color:#fff;font:10px -apple-system,Arial,sans-serif;padding:2px 6px;border-radius:5px;white-space:nowrap;max-width:180px;overflow:hidden;text-overflow:ellipsis;pointer-events:auto;cursor:pointer'; tag.textContent=name+': '+label; wrap.appendChild(dot);wrap.appendChild(tag); host.appendChild(wrap); if(typeof id==='number'){ wrap.setAttribute('data-sp-mark','1'); var onClick=function(e){if(!selectMode)return;e.stopPropagation();selectMark(id);}; dot.onclick=onClick;tag.onclick=onClick; marksById[id]={el:wrap,hl:dot,tag:tag,kind:'pin',c:color}; pinLog.push({id:id,x:x,y:y,a:a,label:label,n:name,c:color}); cbtn.title='Comments ('+pinLog.length+')'; renderPanel(); } } function renderStroke(points,a,color,width,id){ var host=svgHost(a); var ptsAttr=points.map(function(p){return p[0]+','+p[1];}).join(' '); var w=width||3; var poly=document.createElementNS(svgNS,'polyline'); poly.setAttribute('points',ptsAttr); poly.setAttribute('fill','none');poly.setAttribute('stroke',color);poly.setAttribute('stroke-width',String(w)); poly.setAttribute('stroke-linecap','round');poly.setAttribute('stroke-linejoin','round'); poly.setAttribute('pointer-events','none'); host.appendChild(poly); if(typeof id==='number'){ var hit=document.createElementNS(svgNS,'polyline'); hit.setAttribute('points',ptsAttr); hit.setAttribute('fill','none');hit.setAttribute('stroke','transparent'); hit.setAttribute('stroke-width',String(Math.max(14,w+10))); hit.setAttribute('stroke-linecap','round');hit.setAttribute('stroke-linejoin','round'); hit.setAttribute('pointer-events','visibleStroke'); hit.setAttribute('data-sp-mark','1'); hit.style.cursor='pointer'; hit.onclick=function(e){if(!selectMode)return;e.stopPropagation();selectMark(id);}; host.appendChild(hit); marksById[id]={el:poly,hl:poly,hit:hit,kind:'stroke',c:color}; } return poly; } function makeMarkDraggable(el,id){ el.addEventListener('mousedown',function(e){ e.preventDefault();e.stopPropagation(); el.style.cursor='grabbing'; var startX=e.clientX,startY=e.clientY; var origLeft=parseFloat(el.style.left)||0,origTop=parseFloat(el.style.top)||0; var z=1; if(boardMode){var t=window.__spBoardGetTransform?window.__spBoardGetTransform():{x:0,y:0,z:1};z=t.z;} var curLeft=origLeft,curTop=origTop; function move(ev){ var dx=(ev.clientX-startX)/z,dy=(ev.clientY-startY)/z; curLeft=origLeft+dx;curTop=origTop+dy; el.style.left=curLeft+'px';el.style.top=curTop+'px'; } function up(){ window.removeEventListener('mousemove',move);window.removeEventListener('mouseup',up); el.style.cursor='grab'; var sendX=Math.round(curLeft),sendY=Math.round(curTop); if(!boardMode){ var r=target().getBoundingClientRect(); sendX=Math.round(curLeft-r.left-window.scrollX); sendY=Math.round(curTop-r.top-window.scrollY); } liveSend('move_mark:'+id+':'+sendX+':'+sendY); } window.addEventListener('mousemove',move);window.addEventListener('mouseup',up); }); } function placeImage(x,y,a,w,h,data,id){ var host=anchorHost(a); var img=document.createElement('img'); img.src=data; img.draggable=false; var pos; if(boardMode){ pos='left:'+x+'px;top:'+y+'px'; }else{ var r=target().getBoundingClientRect(); pos='left:'+(r.left+window.scrollX+x)+'px;top:'+(r.top+window.scrollY+y)+'px'; } img.style.cssText='position:absolute;'+pos+';width:'+w+'px;height:'+h+'px;z-index:99997;border-radius:6px;box-shadow:0 2px 10px rgba(0,0,0,.28);pointer-events:auto;user-select:none;cursor:grab'; img.setAttribute('draggable','false'); host.appendChild(img); if(typeof id==='number'){ img.setAttribute('data-sp-mark','1'); marksById[id]={el:img,hl:img,kind:'image'}; makeMarkDraggable(img,id); img.onclick=function(e){if(!selectMode)return;e.stopPropagation();selectMark(id);}; } } function loadImageFile(file,posEvent){ if(!file||!file.type||file.type.indexOf('image/')!==0)return; if(file.size>300000){window.alert('Image too large - keep it under ~300KB.');return;} var reader=new FileReader(); reader.onload=function(){ var dataUrl=reader.result; var probe=new Image(); probe.onload=function(){ var maxW=260; var scale=Math.min(1,maxW/probe.naturalWidth); var w=Math.max(20,Math.round(probe.naturalWidth*scale)); var h=Math.max(20,Math.round(probe.naturalHeight*scale)); var p=pointFor(posEvent)||{x:40,y:40,a:-1}; liveSend('image:'+p.x+':'+p.y+':'+p.a+':'+w+':'+h+':'+dataUrl); }; probe.src=dataUrl; }; reader.readAsDataURL(file); } fileInput.onchange=function(){ var file=fileInput.files&&fileInput.files[0]; fileInput.value=''; if(!file)return; var vp=target(); var r=vp.getBoundingClientRect(); loadImageFile(file,{clientX:r.left+r.width/2,clientY:r.top+r.height/2,target:vp}); }; document.addEventListener('dragover',function(e){ if(e.dataTransfer&&e.dataTransfer.types&&Array.prototype.indexOf.call(e.dataTransfer.types,'Files')!==-1)e.preventDefault(); }); document.addEventListener('drop',function(e){ if(!e.dataTransfer||!e.dataTransfer.files||!e.dataTransfer.files.length)return; e.preventDefault(); loadImageFile(e.dataTransfer.files[0],e); }); var drawing=false,drawPts=[],drawPreview=null,drawAnchor=-1,drawColor='#3b82f6',drawWidth=3; document.addEventListener('mousedown',function(e){ if(!drawMode||e.target.closest('button,input,select,textarea'))return; var p=pointFor(e); if(!p)return; e.preventDefault(); lockSelection(true); drawing=true;drawAnchor=p.a; drawPts=[[p.x,p.y]]; drawPreview=renderStroke(drawPts,drawAnchor,drawColor,drawWidth); }); document.addEventListener('mousemove',function(e){ if(!drawing)return; var p=pointFor(e,drawAnchor); if(!p)return; drawPts.push([p.x,p.y]); drawPreview.setAttribute('points',drawPts.map(function(pt){return pt[0]+','+pt[1];}).join(' ')); }); document.addEventListener('mouseup',function(){ if(!drawing)return; drawing=false; lockSelection(false); if(drawPreview){svgHost(drawAnchor).removeChild(drawPreview);drawPreview=null;} if(drawPts.length>=2)liveSend('stroke:'+drawColor+':'+drawWidth+':'+drawAnchor+':'+drawPts.map(function(p){return p[0]+','+p[1];}).join(';')); }); var lastCursorSend=0; document.addEventListener('mousemove',function(e){ var now=Date.now(); if(now-lastCursorSend<33)return; lastCursorSend=now; var r=target().getBoundingClientRect(); var x=Math.round(e.clientX-r.left),y=Math.round(e.clientY-r.top); if(x<-40||y<-40||x>r.width+40||y>r.height+40)return; liveSend('cursor:'+x+':'+y); }); document.addEventListener('click',function(e){ if(e.target.closest('button,input,select,textarea'))return; if(selectMode)selectMark(null); if(!pinMode&&!e.altKey)return; var p=pointFor(e); if(!p){if(pinMode)setPinMode(false);return;} var label=window.prompt('Pin a comment to this point:'); if(pinMode)setPinMode(false); if(!label)return; liveSend('pin:'+p.x+':'+p.y+':'+p.a+':'+label); }); document.addEventListener('keydown',function(e){if(e.key==='Delete')deleteSelected();}); var sessionsPanel=null; if(boardMode){ sessionsPanel=mkPopover(); sessionsPanel.style.flexDirection='column';sessionsPanel.style.alignItems='stretch';sessionsPanel.style.padding='0'; sessionsPanel.style.width='300px';sessionsPanel.style.maxHeight='340px';sessionsPanel.style.overflowY='auto'; sessionsPanel.style.background='var(--sp-panel-bg)';sessionsPanel.style.color='var(--sp-panel-fg)';sessionsPanel.style.border='1px solid var(--sp-panel-border)'; sbtn.onclick=function(){ var show=sessionsPanel.style.display==='none'; closeAllPopovers(); sessionsPanel.style.display=show?'flex':'none'; if(show)liveSend('list_sessions:'); }; } function renderSessions(payload){ if(!sessionsPanel)return; sessionsPanel.innerHTML=''; var header=document.createElement('div'); header.style.cssText='padding:8px 10px;font-weight:700;color:var(--sp-panel-fg);border-bottom:1px solid var(--sp-panel-border)'; header.textContent='Sessions'; sessionsPanel.appendChild(header); (payload.sessions||[]).forEach(function(s){ var row=document.createElement('div'); row.style.cssText='padding:8px 10px;border-bottom:1px solid var(--sp-panel-border);display:flex;align-items:center;justify-content:space-between;gap:8px;cursor:pointer'; var info=document.createElement('div'); info.style.cssText='overflow:hidden'; var nm=document.createElement('div'); nm.style.cssText='font-weight:'+(s.active?'700':'500')+';color:'+(s.active?'#3b82f6':'var(--sp-panel-fg)')+';cursor:pointer'; nm.textContent=s.name+(s.active?' (this board)':''); var meta=document.createElement('div'); meta.style.cssText='color:var(--sp-panel-muted)'; meta.textContent=s.count+' marks - '+new Date(s.mtime).toLocaleString(); info.appendChild(nm);info.appendChild(meta); var del=document.createElement('button'); del.textContent='🗑'; del.title='Delete this session'; del.style.cssText='border:0;background:transparent;cursor:pointer;font-size:14px'; del.onclick=function(e){e.stopPropagation();liveSend('delete_session:'+s.name);}; if(!s.active){ row.onclick=function(){liveSend('switch_session:'+s.name);}; row.title='Switch to this session'; }else{ row.style.cursor='default'; } row.appendChild(info);row.appendChild(del); sessionsPanel.appendChild(row); }); if(!(payload.sessions||[]).length){ var empty=document.createElement('div'); empty.style.cssText='padding:10px;color:var(--sp-panel-muted)'; empty.textContent='No saved sessions yet.'; sessionsPanel.appendChild(empty); } } var incidentStart=Date.now(); setInterval(function(){ var s=Math.floor((Date.now()-incidentStart)/1000); var mm=String(Math.floor(s/60)).padStart(2,'0'),ss=String(s%60).padStart(2,'0'); timerBadge.textContent='⏱ '+mm+':'+ss; },1000); function applyMark(m){ if(m.k==='pin')placePin(m.x,m.y,m.a,m.label,m.n,m.c,m.id); else if(m.k==='stroke')renderStroke(m.points,m.a,m.c,m.w,m.id); else if(m.k==='image')placeImage(m.x,m.y,m.a,m.w,m.h,m.data,m.id); } window.__spLiveOnOpen=function(){liveSend('hello:'+MYNAME);}; window.__spLiveOnMessage=function(msg){ if(msg.type==='update'&&msg.id==='sp-live-sync'){ var sync=JSON.parse(msg.html); MYSESS=sync.sess; renderPresence(sync.presence); var me=sync.presence.filter(function(p){return p.s===MYSESS;})[0]; if(me)MYCOLOR=me.c; if(sync.started_at)incidentStart=sync.started_at; (sync.marks||[]).forEach(applyMark); if(window.__spBoardApplyLayout)window.__spBoardApplyLayout(sync.layout||[]); return; } if(msg.type==='update'&&msg.id==='sp-board-sessions'){renderSessions(JSON.parse(msg.html));return;} if(msg.type!=='live')return; if(msg.k==='presence')renderPresence(msg.list); else if(msg.k==='cursor'){ if(msg.s===MYSESS)return; var r=target().getBoundingClientRect(); var el=ensureCursor(msg.s,msg.n,msg.c); el.style.left=(r.left+window.scrollX+msg.x)+'px'; el.style.top=(r.top+window.scrollY+msg.y)+'px'; } else if(msg.k==='unmark')removeMark(msg.id); else if(msg.k==='clear_marks')clearAllMarks(); else if(msg.k==='recolor')applyRecolor(msg.id,msg.c); else if(msg.k==='move_mark')repositionMarkVisual(msg.id,msg.x,msg.y); else if(msg.k==='layout'){if(window.__spBoardApplyLayout)window.__spBoardApplyLayout([{index:msg.i,x:msg.x,y:msg.y}]);} else if(msg.k==='session_switched'){ clearAllMarks(); (msg.marks||[]).forEach(applyMark); if(window.__spBoardApplyLayout)window.__spBoardApplyLayout(msg.layout||[]); if(sessionsPanel&&sessionsPanel.style.display!=='none')liveSend('list_sessions:'); } else applyMark(msg); }; if(rbtn)rbtn.onclick=function(){liveSend('replay');}; })();</script>";

#[cfg(feature = "webapp")]
static NEXT_LIVE_SESSION: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);

#[cfg(feature = "webapp")]
fn spawn_serve_thread(std_listener: std::net::TcpListener, state: std::sync::Arc<ServeState>, label: &'static str) {
    std::thread::spawn(move || {
        let rt = match tokio::runtime::Runtime::new() {
            Ok(rt) => rt,
            Err(e) => {
                eprintln!("seraplot: failed to start tokio runtime for {label}: {e}");
                return;
            }
        };
        rt.block_on(async move {
            let listener = match tokio::net::TcpListener::from_std(std_listener) {
                Ok(l) => l,
                Err(e) => {
                    eprintln!("seraplot: {label} listener setup failed: {e}");
                    return;
                }
            };
            if let Err(e) = crate::webapp::server::run_server(listener, state).await {
                eprintln!("seraplot: {label} server error: {e}");
            }
        });
    });
}

#[cfg(feature = "webapp")]
#[pyfunction]
#[pyo3(signature = (charts, titles = None, live_urls = None, session = None, host = "127.0.0.1", port = 8787, username = None, password = None, collab = true))]
#[sera_doc(
    category = "pulse_function",
    module = "board",
    file = "seraplot/pulse-pricing.md",
    en = "Serves several charts together, live, on one infinite pannable/zoomable canvas — Figma-style: a sidebar lists every chart, each sits in its own draggable frame that can be freely repositioned or overlapped, scroll to zoom in/out with no fixed limit either way. Everyone who opens the page shares live cursors, can pin a comment, drop an image, or freehand-draw anywhere on the board (glued to a chart's frame if placed on top of it, following it if the frame moves), exactly like Chart.serve(collab=True). Pins, images and drawings are saved to a named session on disk (today's date by default) and restored automatically the next time the same session is served, frame positions included — a small session panel in the toolbar lists every saved session with a delete button.",
    fr = "Sert plusieurs charts ensemble, en direct, sur une seule toile déplaçable et zoomable à l'infini — façon Figma : une barre latérale liste chaque chart, chacun dans son propre cadre déplaçable librement ou superposable, molette pour zoomer/dézoomer sans limite fixe. Toute personne qui ouvre la page partage les curseurs en direct, peut épingler un commentaire, déposer une image, ou dessiner à main levée n'importe où sur la toile (collé au cadre d'un chart si posé dessus, le suivant si le cadre est déplacé), exactement comme Chart.serve(collab=True). Épingles, images et dessins sont sauvegardés dans une session nommée sur disque (la date du jour par défaut) et restaurés automatiquement la prochaine fois que la même session est servie, positions des cadres incluses — un petit panneau de sessions dans la barre d'outils liste chaque session sauvegardée avec un bouton de suppression.",
    param(name = "charts", ty = "list[Chart]", en = "Charts to place on the board, one frame each.", fr = "Charts à placer sur la toile, un cadre chacun."),
    param(name = "titles", ty = "list[str] | None", en = "Optional per-frame title shown in the sidebar and the frame's title bar; must have exactly one entry per chart if given.", fr = "Titre optionnel par cadre affiché dans la barre latérale et la barre de titre du cadre ; doit avoir exactement une entrée par chart si fourni."),
    param(name = "live_urls", ty = "list[str | None] | None", en = "Optional, one entry per chart: the URL of that same chart's own .serve() (e.g. 'http://127.0.0.1:8788') if you want it to keep streaming live inside its board frame instead of showing a frozen snapshot. None (or omit) for any chart that should stay static. You serve each such chart yourself first, on its own port.", fr = "Optionnel, une entrée par chart : l'URL du propre .serve() de ce chart (ex : 'http://127.0.0.1:8788') si vous voulez qu'il continue à streamer en direct dans son cadre au lieu d'un instantané figé. None (ou omis) pour un chart qui doit rester statique. Vous servez chaque chart concerné vous-même d'abord, sur son propre port."),
    param(name = "session", ty = "str | None", en = "Name of the session to save pins/drawings/layout under and restore from, e.g. '2026-08-09'. Defaults to today's date so re-running the same script the same day naturally resumes where you left off.", fr = "Nom de la session sous laquelle sauvegarder/restaurer épingles, dessins et disposition, ex : '2026-08-09'. Par défaut la date du jour, pour que relancer le même script le même jour reprenne naturellement là où vous en étiez."),
    param(name = "host", ty = "str", en = "Address to bind the server on.", fr = "Adresse sur laquelle lier le serveur."),
    param(name = "port", ty = "int", en = "Port to bind the server on.", fr = "Port sur lequel lier le serveur."),
    param(name = "username", ty = "str | None", en = "If set together with password, requires HTTP Basic Auth to view the board or connect.", fr = "Si défini avec password, exige une authentification HTTP Basic pour voir la toile ou se connecter."),
    param(name = "password", ty = "str | None", en = "See username.", fr = "Voir username."),
    param(name = "collab", ty = "bool", en = "Enables shared cursors and pinned comments across everyone viewing the board. Set to False to serve the board with no presence broadcast at all.", fr = "Active les curseurs partagés et les commentaires épinglés entre toutes les personnes qui regardent la toile. Mettre à False pour servir la toile sans aucune diffusion de présence.")
)]
pub fn serve_board(
    charts: Vec<PyRef<'_, Chart>>,
    titles: Option<Vec<String>>,
    live_urls: Option<Vec<Option<String>>>,
    session: Option<&str>,
    host: &str,
    port: u16,
    username: Option<&str>,
    password: Option<&str>,
    collab: bool,
) -> PyResult<()> {
    if charts.is_empty() {
        return Err(pyo3::exceptions::PyValueError::new_err("seraplot: serve_board() needs at least one chart"));
    }
    if let Some(t) = &titles {
        if t.len() != charts.len() {
            return Err(pyo3::exceptions::PyValueError::new_err(format!(
                "seraplot: titles has {} entries but {} charts were given - they must match in length",
                t.len(),
                charts.len()
            )));
        }
    }
    if let Some(u) = &live_urls {
        if u.len() != charts.len() {
            return Err(pyo3::exceptions::PyValueError::new_err(format!(
                "seraplot: live_urls has {} entries but {} charts were given - they must match in length",
                u.len(),
                charts.len()
            )));
        }
    }
    let entries: Vec<(String, String)> = charts
        .iter()
        .enumerate()
        .map(|(i, chart)| {
            let title = titles.as_ref().map(|t| t[i].clone()).unwrap_or_else(|| format!("Chart {}", i + 1));
            (title, chart.html.clone())
        })
        .collect();
    let live_urls = live_urls.unwrap_or_else(|| vec![None; entries.len()]);
    let html = crate::plot::pulse_features::board::build_board_html(&entries, &live_urls);
    let addr = format!("{host}:{port}");
    let std_listener = crate::webapp::server::bind_listener(&addr)
        .map_err(|e| pyo3::exceptions::PyOSError::new_err(format!("seraplot: failed to bind {addr}: {e}")))?;
    let hid = crate::html::hover::html_id();
    crate::plot::pulse_features::health::register("serve_board", Some(hid), host, port);
    let push_tx = push_registry::get_or_create_push_tx(hid);
    let auth = username.zip(password).map(|(u, p)| (u.to_string(), p.to_string()));
    let session_name = session.map(|s| s.to_string()).unwrap_or_else(super::board_sessions::today_session_name);
    let session_path = super::board_sessions::session_path(&session_name);
    if let Some(parent) = session_path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if let Some(path_str) = session_path.to_str() {
        super::live_marks::enable_persistence(hid, path_str);
    }
    let state = std::sync::Arc::new(ServeState {
        html,
        push_tx,
        auth,
        hid,
        collab,
        board: true,
        started_at_ms: crate::plot::now_ms(),
        session: std::sync::Mutex::new(Some(session_name)),
    });
    spawn_serve_thread(std_listener, state, "serve_board()");
    Ok(())
}

#[cfg(feature = "webapp")]
struct ServeState {
    html: String,
    push_tx: tokio::sync::broadcast::Sender<(u8, Vec<u8>)>,
    auth: Option<(String, String)>,
    hid: u64,
    collab: bool,
    board: bool,
    started_at_ms: i64,
    session: std::sync::Mutex<Option<String>>,
}

#[cfg(feature = "webapp")]
impl ServeState {
    fn presence_of(&self, session: u64, now_ms: i64) -> (String, String) {
        super::live_registry::prune_and_list(self.hid, now_ms)
            .into_iter()
            .find(|p| p.session == session)
            .map(|p| (p.name, p.color_hex))
            .unwrap_or_else(|| {
                let name = format!("Guest{session}");
                let color = super::live_registry::color_for(&name);
                (name, color)
            })
    }

    fn broadcast_live(&self, payload: serde_json::Value) {
        let _ = self.push_tx.send((crate::webapp::ws::OPCODE_TEXT, payload.to_string().into_bytes()));
    }

    fn active_session(&self) -> Option<String> {
        self.session.lock().unwrap_or_else(|e| e.into_inner()).clone()
    }

    fn sessions_json(&self) -> String {
        let active = self.active_session();
        let list: Vec<serde_json::Value> = super::board_sessions::list_sessions()
            .into_iter()
            .map(|s| {
                serde_json::json!({
                    "name": s.name,
                    "mtime": s.mtime_ms,
                    "count": s.mark_count,
                    "active": active.as_deref() == Some(s.name.as_str()),
                })
            })
            .collect();
        serde_json::json!({ "sessions": list, "active": active }).to_string()
    }
}

#[cfg(feature = "webapp")]
impl crate::webapp::server::EventDispatcher for ServeState {
    fn page_html(&self, path: &str) -> Option<String> {
        if path != "/" {
            return None;
        }
        let live_js = if self.collab { LIVE_JS } else { "" };
        let board_flag = if self.board { "<script>window.__spBoardMode=1;</script>" } else { "" };
        match self.html.strip_suffix("</body></html>") {
            Some(head) => Some(format!("{head}{board_flag}{WS_BOOTSTRAP_JS}{live_js}</body></html>")),
            None => Some(format!("{}{}{}{}", self.html, board_flag, WS_BOOTSTRAP_JS, live_js)),
        }
    }

    fn on_event(&self, session: u64, component_id: &str, value: &str) -> Vec<(String, String)> {
        if !self.collab || component_id != "sp-live" {
            return Vec::new();
        }
        let now = crate::plot::now_ms();
        let (kind, rest) = value.split_once(':').unwrap_or((value, ""));
        match kind {
            "hello" => {
                let name = if rest.is_empty() { format!("Guest{session}") } else { rest.to_string() };
                super::live_registry::join_or_touch(self.hid, session, &name, now);
                let list = super::live_registry::prune_and_list(self.hid, now);
                self.broadcast_live(serde_json::json!({ "type": "live", "k": "presence", "list": list }));
                let marks = super::live_marks::list(self.hid);
                let layout = super::live_marks::list_layout(self.hid);
                let sync =
                    serde_json::json!({ "sess": session, "presence": list, "marks": marks, "layout": layout, "started_at": self.started_at_ms })
                        .to_string();
                vec![("sp-live-sync".to_string(), sync)]
            }
            "cursor" => {
                super::live_registry::touch(self.hid, session, now);
                let (x, y) = rest
                    .split_once(':')
                    .map(|(x, y)| (x.parse::<i32>().unwrap_or(0), y.parse::<i32>().unwrap_or(0)))
                    .unwrap_or((0, 0));
                let (name, color) = self.presence_of(session, now);
                self.broadcast_live(serde_json::json!({ "type": "live", "k": "cursor", "s": session, "x": x, "y": y, "n": name, "c": color }));
                Vec::new()
            }
            "pin" => {
                if !super::live_registry::allow(self.hid, super::live_registry::Action::Pin) {
                    return Vec::new();
                }
                let Some((x_s, rest2)) = rest.split_once(':') else { return Vec::new() };
                let Some((y_s, rest3)) = rest2.split_once(':') else { return Vec::new() };
                let Some((a_s, label)) = rest3.split_once(':') else { return Vec::new() };
                let (Ok(x), Ok(y)) = (x_s.parse::<i32>(), y_s.parse::<i32>()) else { return Vec::new() };
                let anchor = a_s.parse::<i32>().unwrap_or(-1);
                if label.is_empty() {
                    return Vec::new();
                }
                let packed = ((x.clamp(0, 0xFFFF) as u32) << 16) | (y.clamp(0, 0xFFFF) as u32);
                if crate::plot::pulse_features::annotations::add(self.hid, packed, label.to_string(), Some(now)).is_err() {
                    return Vec::new();
                }
                let (name, color) = self.presence_of(session, now);
                let id = super::live_marks::add(
                    self.hid,
                    super::live_marks::Mark::Pin { x, y, a: anchor, label: label.to_string(), n: name.clone(), c: color.clone() },
                );
                self.broadcast_live(serde_json::json!({ "type": "live", "k": "pin", "id": id, "x": x, "y": y, "a": anchor, "label": label, "n": name, "c": color }));
                Vec::new()
            }
            "stroke" => {
                if !super::live_registry::allow(self.hid, super::live_registry::Action::Stroke) {
                    return Vec::new();
                }
                let Some((color, rest2)) = rest.split_once(':') else { return Vec::new() };
                let Some((w_s, rest3)) = rest2.split_once(':') else { return Vec::new() };
                let Some((a_s, points_s)) = rest3.split_once(':') else { return Vec::new() };
                if color.is_empty() || points_s.is_empty() {
                    return Vec::new();
                }
                let width = w_s.parse::<i32>().unwrap_or(3).clamp(1, 24);
                let anchor = a_s.parse::<i32>().unwrap_or(-1);
                let points: Vec<(i32, i32)> = points_s
                    .split(';')
                    .filter_map(|p| {
                        let (xs, ys) = p.split_once(',')?;
                        Some((xs.parse::<i32>().ok()?, ys.parse::<i32>().ok()?))
                    })
                    .take(300)
                    .collect();
                if points.len() < 2 {
                    return Vec::new();
                }
                let id = super::live_marks::add(
                    self.hid,
                    super::live_marks::Mark::Stroke { points: points.clone(), a: anchor, w: width, c: color.to_string() },
                );
                self.broadcast_live(serde_json::json!({ "type": "live", "k": "stroke", "id": id, "points": points, "a": anchor, "w": width, "c": color }));
                Vec::new()
            }
            "unmark" => {
                if !super::live_registry::allow(self.hid, super::live_registry::Action::Unmark) {
                    return Vec::new();
                }
                let Ok(id) = rest.parse::<u64>() else { return Vec::new() };
                if super::live_marks::remove(self.hid, id) {
                    self.broadcast_live(serde_json::json!({ "type": "live", "k": "unmark", "id": id }));
                }
                Vec::new()
            }
            "recolor" => {
                if !super::live_registry::allow(self.hid, super::live_registry::Action::Recolor) {
                    return Vec::new();
                }
                let Some((id_s, color)) = rest.split_once(':') else { return Vec::new() };
                let (Ok(id), false) = (id_s.parse::<u64>(), color.is_empty()) else { return Vec::new() };
                if super::live_marks::recolor(self.hid, id, color) {
                    self.broadcast_live(serde_json::json!({ "type": "live", "k": "recolor", "id": id, "c": color }));
                }
                Vec::new()
            }
            "move_mark" => {
                if !super::live_registry::allow(self.hid, super::live_registry::Action::Transform) {
                    return Vec::new();
                }
                let Some((id_s, rest2)) = rest.split_once(':') else { return Vec::new() };
                let Some((x_s, y_s)) = rest2.split_once(':') else { return Vec::new() };
                let (Ok(id), Ok(x), Ok(y)) = (id_s.parse::<u64>(), x_s.parse::<i32>(), y_s.parse::<i32>()) else { return Vec::new() };
                if super::live_marks::reposition(self.hid, id, x, y) {
                    self.broadcast_live(serde_json::json!({ "type": "live", "k": "move_mark", "id": id, "x": x, "y": y }));
                }
                Vec::new()
            }
            "image" => {
                if !super::live_registry::allow(self.hid, super::live_registry::Action::Image) {
                    return Vec::new();
                }
                let parts: Vec<&str> = rest.splitn(6, ':').collect();
                let [x_s, y_s, a_s, w_s, h_s, data] = parts[..] else { return Vec::new() };
                let (Ok(x), Ok(y), Ok(w), Ok(h)) = (x_s.parse::<i32>(), y_s.parse::<i32>(), w_s.parse::<i32>(), h_s.parse::<i32>()) else {
                    return Vec::new();
                };
                if data.is_empty() {
                    return Vec::new();
                }
                let anchor = a_s.parse::<i32>().unwrap_or(-1);
                let Some(id) = super::live_marks::add_image(self.hid, x, y, anchor, w, h, data.to_string()) else {
                    return Vec::new();
                };
                self.broadcast_live(serde_json::json!({ "type": "live", "k": "image", "id": id, "x": x, "y": y, "a": anchor, "w": w, "h": h, "data": data }));
                Vec::new()
            }
            "layout" => {
                if !self.board || !super::live_registry::allow(self.hid, super::live_registry::Action::Transform) {
                    return Vec::new();
                }
                let Some((idx_s, rest2)) = rest.split_once(':') else { return Vec::new() };
                let Some((x_s, y_s)) = rest2.split_once(':') else { return Vec::new() };
                let (Ok(index), Ok(x), Ok(y)) = (idx_s.parse::<i32>(), x_s.parse::<i32>(), y_s.parse::<i32>()) else { return Vec::new() };
                super::live_marks::set_layout(self.hid, index, x, y);
                self.broadcast_live(serde_json::json!({ "type": "live", "k": "layout", "i": index, "x": x, "y": y }));
                Vec::new()
            }
            "list_sessions" => {
                if !self.board {
                    return Vec::new();
                }
                vec![("sp-board-sessions".to_string(), self.sessions_json())]
            }
            "delete_session" => {
                if !self.board || !super::live_registry::allow(self.hid, super::live_registry::Action::Unmark) {
                    return Vec::new();
                }
                let deleted = super::board_sessions::delete_session(rest);
                if deleted && self.active_session().as_deref() == Some(rest) {
                    super::live_marks::disable_persistence(self.hid);
                    super::live_marks::clear(self.hid);
                    self.broadcast_live(serde_json::json!({ "type": "live", "k": "clear_marks" }));
                }
                vec![("sp-board-sessions".to_string(), self.sessions_json())]
            }
            "switch_session" => {
                if !self.board || rest.is_empty() || !super::live_registry::allow(self.hid, super::live_registry::Action::Unmark) {
                    return Vec::new();
                }
                let path = super::board_sessions::session_path(rest);
                if let Some(parent) = path.parent() {
                    let _ = std::fs::create_dir_all(parent);
                }
                let Some(path_str) = path.to_str() else { return Vec::new() };
                super::live_marks::switch_session(self.hid, path_str);
                *self.session.lock().unwrap_or_else(|e| e.into_inner()) = Some(rest.to_string());
                let marks = super::live_marks::list(self.hid);
                let layout = super::live_marks::list_layout(self.hid);
                self.broadcast_live(serde_json::json!({ "type": "live", "k": "session_switched", "name": rest, "marks": marks, "layout": layout }));
                Vec::new()
            }
            "replay" => {
                if !super::live_registry::allow(self.hid, super::live_registry::Action::Replay) {
                    return Vec::new();
                }
                if let Some(path) = crate::plot::record_registry::path_of(self.hid) {
                    if let Ok(recording) = crate::plot::record_registry::read_all(&path) {
                        for rec in &recording.records {
                            let _ = self.push_tx.send((crate::webapp::ws::OPCODE_BINARY, record_frame(self.hid, rec)));
                        }
                    }
                }
                Vec::new()
            }
            _ => Vec::new(),
        }
    }

    fn open_session(&self) -> u64 {
        NEXT_LIVE_SESSION.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
    }

    fn on_disconnect(&self, session: u64) {
        if !self.collab {
            return;
        }
        super::live_registry::leave(self.hid, session);
        let now = crate::plot::now_ms();
        let list = super::live_registry::prune_and_list(self.hid, now);
        self.broadcast_live(serde_json::json!({ "type": "live", "k": "presence", "list": list }));
    }

    fn credentials(&self) -> Option<(String, String)> {
        self.auth.clone()
    }

    fn subscribe(&self) -> tokio::sync::broadcast::Receiver<(u8, Vec<u8>)> {
        self.push_tx.subscribe()
    }

    fn timer_intervals(&self) -> Vec<f64> {
        if self.collab {
            vec![4.0]
        } else {
            Vec::new()
        }
    }

    fn tick_timer(&self, _index: usize) {
        if !self.collab {
            return;
        }
        let now = crate::plot::now_ms();
        let list = super::live_registry::prune_and_list(self.hid, now);
        self.broadcast_live(serde_json::json!({ "type": "live", "k": "presence", "list": list }));
    }
}

#[cfg(test)]
mod export_standalone_tests {
    use super::*;
    use crate::plot::record_registry::Record;
    use base64::Engine;

    fn rec(dt_ms: u32, indices: Vec<u32>, values: Vec<i16>) -> Record {
        let n = indices.len() as u32;
        Record { dt_ms, n, vpp: 1, indices, values }
    }

    fn decode_u32s(b64: &str) -> Vec<u32> {
        let raw = base64::engine::general_purpose::STANDARD.decode(b64).unwrap();
        raw.chunks_exact(4).map(|c| u32::from_le_bytes(c.try_into().unwrap())).collect()
    }

    fn decode_i16s(b64: &str) -> Vec<i16> {
        let raw = base64::engine::general_purpose::STANDARD.decode(b64).unwrap();
        raw.chunks_exact(2).map(|c| i16::from_le_bytes(c.try_into().unwrap())).collect()
    }

    #[test]
    fn empty_recording_encodes_to_an_empty_frame_array() {
        assert_eq!(encode_replay_frames(&[]), "[]");
    }

    #[test]
    fn single_frame_round_trips_indices_and_values_exactly() {
        let records = vec![rec(5, vec![7, 9], vec![100, -200])];
        let json = encode_replay_frames(&records);
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        let frame = &parsed[0];
        assert_eq!(decode_u32s(frame["a"].as_str().unwrap()), vec![7, 9]);
        assert_eq!(decode_i16s(frame["b"].as_str().unwrap()), vec![100, -200]);
    }

    #[test]
    fn wait_times_are_deltas_between_consecutive_dt_ms_not_absolute_timestamps() {
        let records = vec![
            rec(10, vec![0], vec![1]),
            rec(35, vec![1], vec![2]),
            rec(35, vec![2], vec![3]),
            rec(1000, vec![3], vec![4]),
        ];
        let json = encode_replay_frames(&records);
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        let waits: Vec<u64> = parsed.as_array().unwrap().iter().map(|f| f["w"].as_u64().unwrap()).collect();
        assert_eq!(waits, vec![10, 25, 0, 965]);
    }

    #[test]
    fn out_of_order_dt_ms_does_not_panic_and_clamps_to_zero_wait() {
        let records = vec![rec(50, vec![0], vec![1]), rec(10, vec![1], vec![2])];
        let json = encode_replay_frames(&records);
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed[1]["w"].as_u64().unwrap(), 0);
    }

    #[test]
    fn script_references_the_correct_hid_and_frame_data() {
        let records = vec![rec(3, vec![42], vec![7])];
        let script = build_replay_script(99, &records);
        assert!(script.contains("sp_push_99"));
        assert!(script.contains("</script></body>"));
        assert!(!script.contains("<html>"), "the fragment should not duplicate the document shell");
    }

    #[test]
    fn multi_frame_json_is_valid_and_preserves_order() {
        let records = vec![
            rec(0, vec![0], vec![10]),
            rec(5, vec![1], vec![20]),
            rec(12, vec![2], vec![30]),
        ];
        let json = encode_replay_frames(&records);
        let parsed: serde_json::Value = serde_json::from_str(&json).expect("must be valid JSON");
        let arr = parsed.as_array().unwrap();
        assert_eq!(arr.len(), 3);
        assert_eq!(decode_u32s(arr[0]["a"].as_str().unwrap()), vec![0]);
        assert_eq!(decode_u32s(arr[1]["a"].as_str().unwrap()), vec![1]);
        assert_eq!(decode_u32s(arr[2]["a"].as_str().unwrap()), vec![2]);
    }
}

#[cfg(all(test, feature = "webapp"))]
mod sera_live_tests {
    use super::*;
    use crate::webapp::server::{bind_listener, run_server, EventDispatcher};
    use crate::webapp::ws::{accept_key, decode_frame, DecodedFrame};
    use std::sync::atomic::{AtomicU16, Ordering};
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpStream;

    fn next_port() -> u16 {
        static NEXT: AtomicU16 = AtomicU16::new(19300);
        NEXT.fetch_add(1, Ordering::Relaxed)
    }

    fn state(hid: u64, collab: bool) -> std::sync::Arc<ServeState> {
        std::sync::Arc::new(ServeState {
            html: format!("<html><body><div id=\"spp{hid}\"></div></body></html>"),
            push_tx: push_registry::get_or_create_push_tx(hid),
            auth: None,
            hid,
            collab,
            board: false,
            started_at_ms: crate::plot::now_ms(),
            session: std::sync::Mutex::new(None),
        })
    }

    async fn start(hid: u64, collab: bool) -> String {
        let port = next_port();
        let addr = format!("127.0.0.1:{port}");
        let listener = tokio::net::TcpListener::from_std(bind_listener(&addr).unwrap()).unwrap();
        tokio::spawn(run_server(listener, state(hid, collab)));
        tokio::time::sleep(std::time::Duration::from_millis(80)).await;
        addr
    }

    fn board_state(hid: u64, session: Option<&str>) -> std::sync::Arc<ServeState> {
        if let Some(name) = session {
            let path = crate::plot::pulse::board_sessions::session_path(name);
            if let Some(parent) = path.parent() {
                let _ = std::fs::create_dir_all(parent);
            }
            if let Some(path_str) = path.to_str() {
                crate::plot::pulse::live_marks::enable_persistence(hid, path_str);
            }
        }
        std::sync::Arc::new(ServeState {
            html: format!("<html><body><div id=\"spp{hid}\"></div></body></html>"),
            push_tx: push_registry::get_or_create_push_tx(hid),
            auth: None,
            hid,
            collab: true,
            board: true,
            started_at_ms: crate::plot::now_ms(),
            session: std::sync::Mutex::new(session.map(|s| s.to_string())),
        })
    }

    async fn start_board(hid: u64, session: Option<&str>) -> String {
        let port = next_port();
        let addr = format!("127.0.0.1:{port}");
        let listener = tokio::net::TcpListener::from_std(bind_listener(&addr).unwrap()).unwrap();
        tokio::spawn(run_server(listener, board_state(hid, session)));
        tokio::time::sleep(std::time::Duration::from_millis(80)).await;
        addr
    }

    async fn connect_ws(addr: &str) -> TcpStream {
        let mut stream = TcpStream::connect(addr).await.unwrap();
        let key = "dGhlIHNhbXBsZSBub25jZQ==";
        let req = format!("GET /ws HTTP/1.1\r\nHost: x\r\nUpgrade: websocket\r\nConnection: Upgrade\r\nSec-WebSocket-Key: {key}\r\n\r\n");
        stream.write_all(req.as_bytes()).await.unwrap();
        let mut buf = vec![0u8; 4096];
        let n = stream.read(&mut buf).await.unwrap();
        let resp = String::from_utf8_lossy(&buf[..n]);
        assert!(resp.contains("101 Switching Protocols"));
        assert!(resp.contains(&accept_key(key)));
        stream
    }

    fn masked_text_frame(text: &str) -> Vec<u8> {
        let mask = [0x12u8, 0x34, 0x56, 0x78];
        let mut frame = vec![0x80 | crate::webapp::ws::OPCODE_TEXT];
        let len = text.len();
        if len < 126 {
            frame.push(0x80 | (len as u8));
        } else if len <= 0xFFFF {
            frame.push(0x80 | 126);
            frame.push((len >> 8) as u8);
            frame.push(len as u8);
        } else {
            frame.push(0x80 | 127);
            frame.extend_from_slice(&(len as u64).to_be_bytes());
        }
        frame.extend_from_slice(&mask);
        for (i, b) in text.bytes().enumerate() {
            frame.push(b ^ mask[i % 4]);
        }
        frame
    }

    async fn send_live(stream: &mut TcpStream, value: &str) {
        let msg = serde_json::json!({ "id": "sp-live", "value": value }).to_string();
        stream.write_all(&masked_text_frame(&msg)).await.unwrap();
    }

    async fn read_frame(stream: &mut TcpStream) -> DecodedFrame {
        let mut buf = vec![0u8; 8192];
        loop {
            let n = tokio::time::timeout(std::time::Duration::from_secs(2), stream.read(&mut buf))
                .await
                .expect("timed out waiting for a frame")
                .unwrap();
            if let Some(frame) = decode_frame(&buf[..n]) {
                return frame;
            }
        }
    }

    async fn read_live_json(stream: &mut TcpStream) -> serde_json::Value {
        let frame = read_frame(stream).await;
        assert_eq!(frame.opcode, crate::webapp::ws::OPCODE_TEXT);
        serde_json::from_slice(&frame.payload).unwrap()
    }

    async fn read_frame_until<F: Fn(&DecodedFrame) -> bool>(stream: &mut TcpStream, pred: F) -> DecodedFrame {
        loop {
            let frame = read_frame(stream).await;
            if pred(&frame) {
                return frame;
            }
        }
    }

    async fn read_json_until<F: Fn(&serde_json::Value) -> bool>(stream: &mut TcpStream, pred: F) -> serde_json::Value {
        loop {
            let frame = read_frame_until(stream, |f| f.opcode == crate::webapp::ws::OPCODE_TEXT).await;
            let value: serde_json::Value = serde_json::from_slice(&frame.payload).unwrap();
            if pred(&value) {
                return value;
            }
        }
    }

    async fn read_frame_buffered(stream: &mut TcpStream, pending: &mut Vec<u8>) -> DecodedFrame {
        loop {
            if let Some(frame) = decode_frame(pending) {
                pending.drain(..frame.consumed);
                return frame;
            }
            let mut chunk = [0u8; 8192];
            let n = tokio::time::timeout(std::time::Duration::from_secs(2), stream.read(&mut chunk))
                .await
                .expect("timed out waiting for a frame")
                .unwrap();
            pending.extend_from_slice(&chunk[..n]);
        }
    }

    async fn read_json_until_buffered<F: Fn(&serde_json::Value) -> bool>(stream: &mut TcpStream, pending: &mut Vec<u8>, pred: F) -> serde_json::Value {
        loop {
            let frame = read_frame_buffered(stream, pending).await;
            if frame.opcode != crate::webapp::ws::OPCODE_TEXT {
                continue;
            }
            let value: serde_json::Value = serde_json::from_slice(&frame.payload).unwrap();
            if pred(&value) {
                return value;
            }
        }
    }

    fn is_sync_reply(v: &serde_json::Value) -> bool {
        v["type"] == "update" && v["id"] == "sp-live-sync"
    }

    #[tokio::test]
    async fn hello_gets_a_sync_reply_with_a_real_session_id_and_the_sender_in_presence() {
        let hid = 900_100;
        let addr = start(hid, true).await;
        let mut a = connect_ws(&addr).await;
        send_live(&mut a, "hello:Alice").await;
        let sync = read_live_json(&mut a).await;
        assert_eq!(sync["type"], "update");
        assert_eq!(sync["id"], "sp-live-sync");
        let inner: serde_json::Value = serde_json::from_str(sync["html"].as_str().unwrap()).unwrap();
        assert!(inner["sess"].as_u64().unwrap() > 0);
        let presence = inner["presence"].as_array().unwrap();
        assert_eq!(presence.len(), 1);
        assert_eq!(presence[0]["n"], "Alice");
    }

    #[tokio::test]
    async fn open_session_hands_out_distinct_increasing_ids_across_connections() {
        let hid = 900_101;
        let addr = start(hid, true).await;
        let mut a = connect_ws(&addr).await;
        let mut b = connect_ws(&addr).await;
        send_live(&mut a, "hello:A").await;
        let sess_a = serde_json::from_str::<serde_json::Value>(read_json_until(&mut a, is_sync_reply).await["html"].as_str().unwrap()).unwrap()["sess"].as_u64().unwrap();
        send_live(&mut b, "hello:B").await;
        let sess_b = serde_json::from_str::<serde_json::Value>(read_json_until(&mut b, is_sync_reply).await["html"].as_str().unwrap()).unwrap()["sess"].as_u64().unwrap();
        assert_ne!(sess_a, sess_b);
    }

    #[tokio::test]
    async fn a_cursor_move_from_one_client_is_broadcast_to_another_but_not_flagged_as_its_own() {
        let hid = 900_102;
        let addr = start(hid, true).await;
        let mut a = connect_ws(&addr).await;
        let mut b = connect_ws(&addr).await;
        send_live(&mut a, "hello:Alice").await;
        let _ = read_json_until(&mut a, is_sync_reply).await;
        send_live(&mut b, "hello:Bob").await;
        let _ = read_json_until(&mut b, is_sync_reply).await;

        send_live(&mut a, "cursor:123:45").await;
        let seen_by_b = read_json_until(&mut b, |v| v["k"] == "cursor").await;
        assert_eq!(seen_by_b["type"], "live");
        assert_eq!(seen_by_b["k"], "cursor");
        assert_eq!(seen_by_b["x"], 123);
        assert_eq!(seen_by_b["y"], 45);
        assert_eq!(seen_by_b["n"], "Alice");
    }

    #[tokio::test]
    async fn pinning_a_point_persists_a_real_annotation_and_broadcasts_it() {
        let hid = 900_103;
        let addr = start(hid, true).await;
        let mut a = connect_ws(&addr).await;
        let mut b = connect_ws(&addr).await;
        send_live(&mut a, "hello:Alice").await;
        let _ = read_json_until(&mut a, is_sync_reply).await;
        send_live(&mut b, "hello:Bob").await;
        let _ = read_json_until(&mut b, is_sync_reply).await;

        send_live(&mut a, "pin:120:45:-1:worth a look").await;
        let seen_by_b = read_json_until(&mut b, |v| v["k"] == "pin").await;
        assert_eq!(seen_by_b["k"], "pin");
        assert_eq!(seen_by_b["x"], 120);
        assert_eq!(seen_by_b["y"], 45);
        assert_eq!(seen_by_b["a"], -1);
        assert_eq!(seen_by_b["label"], "worth a look");

        let saved = crate::plot::pulse_features::annotations::list(hid);
        assert_eq!(saved.len(), 1);
        assert_eq!(saved[0].index, (120u32 << 16) | 45u32);
        assert_eq!(saved[0].label, "worth a look");
    }

    #[tokio::test]
    async fn a_pin_anchored_to_a_board_frame_carries_its_anchor_through_broadcast_and_replay() {
        let hid = 900_109;
        let addr = start(hid, true).await;
        let mut a = connect_ws(&addr).await;
        let mut b = connect_ws(&addr).await;
        send_live(&mut a, "hello:Alice").await;
        let _ = read_json_until(&mut a, is_sync_reply).await;
        send_live(&mut b, "hello:Bob").await;
        let _ = read_json_until(&mut b, is_sync_reply).await;

        send_live(&mut a, "pin:10:20:3:frame-local note").await;
        let seen_by_b = read_json_until(&mut b, |v| v["k"] == "pin").await;
        assert_eq!(seen_by_b["a"], 3);
        assert_eq!(seen_by_b["x"], 10);
        assert_eq!(seen_by_b["y"], 20);

        let mut c = connect_ws(&addr).await;
        send_live(&mut c, "hello:Carol").await;
        let sync = read_json_until(&mut c, is_sync_reply).await;
        let inner: serde_json::Value = serde_json::from_str(sync["html"].as_str().unwrap()).unwrap();
        let marks = inner["marks"].as_array().unwrap();
        assert_eq!(marks.len(), 1);
        assert_eq!(marks[0]["a"], 3);
        assert_eq!(marks[0]["label"], "frame-local note");
    }

    #[tokio::test]
    async fn a_stroke_broadcasts_its_points_and_anchor_and_is_replayed_to_late_joiners() {
        let hid = 900_110;
        let addr = start(hid, true).await;
        let mut a = connect_ws(&addr).await;
        let mut b = connect_ws(&addr).await;
        send_live(&mut a, "hello:Alice").await;
        let _ = read_json_until(&mut a, is_sync_reply).await;
        send_live(&mut b, "hello:Bob").await;
        let _ = read_json_until(&mut b, is_sync_reply).await;

        send_live(&mut a, "stroke:#3b82f6:6:-1:10,10;20,20;30,15").await;
        let seen_by_b = read_json_until(&mut b, |v| v["k"] == "stroke").await;
        assert_eq!(seen_by_b["a"], -1);
        assert_eq!(seen_by_b["w"], 6);
        assert_eq!(seen_by_b["c"], "#3b82f6");
        assert_eq!(seen_by_b["points"], serde_json::json!([[10, 10], [20, 20], [30, 15]]));

        let mut c = connect_ws(&addr).await;
        send_live(&mut c, "hello:Carol").await;
        let sync = read_json_until(&mut c, is_sync_reply).await;
        let inner: serde_json::Value = serde_json::from_str(sync["html"].as_str().unwrap()).unwrap();
        let marks = inner["marks"].as_array().unwrap();
        assert_eq!(marks.len(), 1);
        assert_eq!(marks[0]["k"], "stroke");
        assert_eq!(marks[0]["points"], serde_json::json!([[10, 10], [20, 20], [30, 15]]));
    }

    #[tokio::test]
    async fn recoloring_a_pin_updates_and_broadcasts_its_color_but_a_stroke_wide_recolor_of_an_unknown_id_is_ignored() {
        let hid = 900_120;
        let addr = start(hid, true).await;
        let mut a = connect_ws(&addr).await;
        let mut b = connect_ws(&addr).await;
        send_live(&mut a, "hello:Alice").await;
        let _ = read_json_until(&mut a, is_sync_reply).await;
        send_live(&mut b, "hello:Bob").await;
        let _ = read_json_until(&mut b, is_sync_reply).await;

        send_live(&mut a, "pin:1:2:-1:x").await;
        let seen = read_json_until(&mut b, |v| v["k"] == "pin").await;
        let id = seen["id"].as_u64().unwrap();

        send_live(&mut a, &format!("recolor:{id}:#ff0000")).await;
        let recolored = read_json_until(&mut b, |v| v["k"] == "recolor").await;
        assert_eq!(recolored["id"], id);
        assert_eq!(recolored["c"], "#ff0000");

        send_live(&mut a, "recolor:999999:#00ff00").await;
        send_live(&mut a, "hello:Alice").await;
        let sync = read_json_until(&mut a, is_sync_reply).await;
        let inner: serde_json::Value = serde_json::from_str(sync["html"].as_str().unwrap()).unwrap();
        let marks = inner["marks"].as_array().unwrap();
        assert_eq!(marks[0]["c"], "#ff0000");
    }

    #[tokio::test]
    async fn moving_an_image_mark_updates_and_broadcasts_its_position() {
        let hid = 900_121;
        let addr = start(hid, true).await;
        let mut a = connect_ws(&addr).await;
        let mut b = connect_ws(&addr).await;
        send_live(&mut a, "hello:Alice").await;
        let _ = read_json_until(&mut a, is_sync_reply).await;
        send_live(&mut b, "hello:Bob").await;
        let _ = read_json_until(&mut b, is_sync_reply).await;

        send_live(&mut a, "image:10:10:-1:40:40:data:image/png;base64,AAAA").await;
        let seen = read_json_until(&mut b, |v| v["k"] == "image").await;
        let id = seen["id"].as_u64().unwrap();
        assert_eq!(seen["data"], "data:image/png;base64,AAAA");
        assert_eq!(seen["w"], 40);

        send_live(&mut a, &format!("move_mark:{id}:70:80")).await;
        let moved = read_json_until(&mut b, |v| v["k"] == "move_mark").await;
        assert_eq!(moved["id"], id);
        assert_eq!(moved["x"], 70);
        assert_eq!(moved["y"], 80);
    }

    #[tokio::test]
    async fn board_layout_updates_are_broadcast_and_included_in_the_sync_reply_for_late_joiners() {
        let hid = 900_122;
        let addr = start_board(hid, Some("layout-test-session")).await;
        let mut a = connect_ws(&addr).await;
        let mut b = connect_ws(&addr).await;
        send_live(&mut a, "hello:Alice").await;
        let _ = read_json_until(&mut a, is_sync_reply).await;
        send_live(&mut b, "hello:Bob").await;
        let _ = read_json_until(&mut b, is_sync_reply).await;

        send_live(&mut a, "layout:0:120:340").await;
        let seen = read_json_until(&mut b, |v| v["k"] == "layout").await;
        assert_eq!(seen["i"], 0);
        assert_eq!(seen["x"], 120);
        assert_eq!(seen["y"], 340);

        let mut c = connect_ws(&addr).await;
        send_live(&mut c, "hello:Carol").await;
        let sync = read_json_until(&mut c, is_sync_reply).await;
        let inner: serde_json::Value = serde_json::from_str(sync["html"].as_str().unwrap()).unwrap();
        let layout = inner["layout"].as_array().unwrap();
        assert!(layout.iter().any(|l| l["index"] == 0 && l["x"] == 120 && l["y"] == 340));
    }

    #[tokio::test]
    async fn an_oversized_image_upload_is_rejected_without_panicking() {
        let hid = 900_123;
        let addr = start(hid, true).await;
        let mut a = connect_ws(&addr).await;
        send_live(&mut a, "hello:Alice").await;
        let _ = read_json_until(&mut a, is_sync_reply).await;

        let huge_data = format!("data:image/png;base64,{}", "A".repeat(500_000));
        send_live(&mut a, &format!("image:0:0:-1:10:10:{huge_data}")).await;
        send_live(&mut a, "hello:Alice").await;
        let sync = read_json_until(&mut a, is_sync_reply).await;
        let inner: serde_json::Value = serde_json::from_str(sync["html"].as_str().unwrap()).unwrap();
        assert!(inner["marks"].as_array().unwrap().is_empty());
    }

    #[tokio::test]
    async fn unmarking_a_pin_removes_it_from_the_room_and_broadcasts_the_removal() {
        let hid = 900_112;
        let addr = start(hid, true).await;
        let mut a = connect_ws(&addr).await;
        let mut b = connect_ws(&addr).await;
        send_live(&mut a, "hello:Alice").await;
        let _ = read_json_until(&mut a, is_sync_reply).await;
        send_live(&mut b, "hello:Bob").await;
        let _ = read_json_until(&mut b, is_sync_reply).await;

        send_live(&mut a, "pin:1:2:-1:temp").await;
        let seen = read_json_until(&mut b, |v| v["k"] == "pin").await;
        let id = seen["id"].as_u64().unwrap();

        send_live(&mut a, &format!("unmark:{id}")).await;
        let removal = read_json_until(&mut b, |v| v["k"] == "unmark").await;
        assert_eq!(removal["id"], id);

        assert!(crate::plot::pulse::live_marks::list(hid).is_empty());
    }

    #[tokio::test]
    async fn a_board_lists_and_deletes_saved_sessions_and_clears_the_active_one_if_deleted() {
        let hid = 900_113;
        let session_name = format!("test-session-{hid}");
        let _ = crate::plot::pulse::board_sessions::delete_session(&session_name);
        let addr = start_board(hid, Some(&session_name)).await;
        let mut a = connect_ws(&addr).await;
        let mut pending = Vec::new();
        send_live(&mut a, "hello:Alice").await;
        let _ = read_json_until_buffered(&mut a, &mut pending, is_sync_reply).await;

        send_live(&mut a, "pin:1:2:-1:kept for the session file").await;
        let _ = read_json_until_buffered(&mut a, &mut pending, |v| v["k"] == "pin").await;

        let path = crate::plot::pulse::board_sessions::session_path(&session_name);
        assert!(path.exists(), "the session file should exist on disk after a pin");

        send_live(&mut a, "list_sessions:").await;
        let listing_msg = read_json_until_buffered(&mut a, &mut pending, |v| v["type"] == "update" && v["id"] == "sp-board-sessions").await;
        let listing: serde_json::Value = serde_json::from_str(listing_msg["html"].as_str().unwrap()).unwrap();
        let sessions = listing["sessions"].as_array().unwrap();
        assert!(sessions.iter().any(|s| s["name"] == session_name && s["active"] == true && s["count"] == 1));

        send_live(&mut a, &format!("delete_session:{session_name}")).await;
        let cleared = read_json_until_buffered(&mut a, &mut pending, |v| v.get("type") == Some(&serde_json::json!("live")) && v["k"] == "clear_marks").await;
        assert_eq!(cleared["k"], "clear_marks");

        assert!(!path.exists(), "the session file should be removed from disk");
        assert!(crate::plot::pulse::live_marks::list(hid).is_empty(), "the active room's marks should clear when its own session is deleted");
    }

    #[tokio::test]
    async fn switching_sessions_swaps_the_visible_marks_and_layout_for_everyone_in_the_room() {
        let hid = 900_124;
        let name_a = "switch-test-a";
        let name_b = "switch-test-b";
        let _ = crate::plot::pulse::board_sessions::delete_session(name_a);
        let _ = crate::plot::pulse::board_sessions::delete_session(name_b);
        let addr = start_board(hid, Some(name_a)).await;
        let mut a = connect_ws(&addr).await;
        let mut b = connect_ws(&addr).await;
        let mut pending_a = Vec::new();
        let mut pending_b = Vec::new();
        send_live(&mut a, "hello:Alice").await;
        let _ = read_json_until_buffered(&mut a, &mut pending_a, is_sync_reply).await;
        send_live(&mut b, "hello:Bob").await;
        let _ = read_json_until_buffered(&mut b, &mut pending_b, is_sync_reply).await;

        send_live(&mut a, "pin:1:2:-1:lives in session a").await;
        let _ = read_json_until_buffered(&mut a, &mut pending_a, |v| v["k"] == "pin").await;

        send_live(&mut a, &format!("switch_session:{name_b}")).await;
        let switched_a = read_json_until_buffered(&mut a, &mut pending_a, |v| v["k"] == "session_switched").await;
        assert_eq!(switched_a["name"], name_b);
        assert!(switched_a["marks"].as_array().unwrap().is_empty(), "the fresh session b starts with no marks");
        let switched_b = read_json_until_buffered(&mut b, &mut pending_b, |v| v["k"] == "session_switched").await;
        assert_eq!(switched_b["name"], name_b);

        send_live(&mut a, "pin:5:6:-1:lives in session b").await;
        let _ = read_json_until_buffered(&mut a, &mut pending_a, |v| v["k"] == "pin").await;

        send_live(&mut a, &format!("switch_session:{name_a}")).await;
        let back_to_a = read_json_until_buffered(&mut a, &mut pending_a, |v| v["k"] == "session_switched").await;
        let marks_back = back_to_a["marks"].as_array().unwrap();
        assert_eq!(marks_back.len(), 1);
        assert_eq!(marks_back[0]["label"], "lives in session a");

        let _ = crate::plot::pulse::board_sessions::delete_session(name_a);
        let _ = crate::plot::pulse::board_sessions::delete_session(name_b);
    }

    #[tokio::test]
    async fn a_malformed_stroke_or_pin_missing_the_anchor_field_is_dropped_not_panicking() {
        let hid = 900_111;
        let addr = start(hid, true).await;
        let mut a = connect_ws(&addr).await;
        send_live(&mut a, "hello:Alice").await;
        let _ = read_json_until(&mut a, is_sync_reply).await;
        send_live(&mut a, "stroke:#000:").await;
        send_live(&mut a, "pin:1:2:").await;
        send_live(&mut a, "hello:Alice").await;
        let sync = read_json_until(&mut a, is_sync_reply).await;
        let inner: serde_json::Value = serde_json::from_str(sync["html"].as_str().unwrap()).unwrap();
        assert!(inner["marks"].as_array().unwrap().is_empty());
    }

    #[tokio::test]
    async fn replay_broadcasts_the_full_recorded_history_as_real_binary_frames() {
        let hid = 900_104;
        let addr = start(hid, true).await;
        let path = std::env::temp_dir().join(format!("sp_live_replay_test_{}.spls", std::process::id())).to_string_lossy().to_string();
        let meta = push_registry::PushMeta::Scalar(push_registry::ScalarMeta { min_val: 0.0, range_val: 100.0, axis_px: 300 });
        crate::plot::record_registry::start(hid, &path, &meta).unwrap();
        let rec_state = crate::plot::record_registry::get(hid).unwrap();
        rec_state.append(&build_binary_frame(hid, &pack_indices_u32_raw(&[0, 1]), &[10, 0, 20, 0], 2, 1));
        crate::plot::record_registry::stop(hid);

        let mut a = connect_ws(&addr).await;
        let mut b = connect_ws(&addr).await;
        send_live(&mut a, "hello:Alice").await;
        let _ = read_json_until(&mut a, is_sync_reply).await;
        send_live(&mut b, "hello:Bob").await;
        let _ = read_json_until(&mut b, is_sync_reply).await;

        send_live(&mut a, "replay").await;
        let frame = read_frame_until(&mut b, |f| f.opcode == crate::webapp::ws::OPCODE_BINARY).await;
        assert_eq!(frame.opcode, crate::webapp::ws::OPCODE_BINARY);
        let got_hid = u32::from_le_bytes(frame.payload[0..4].try_into().unwrap());
        assert_eq!(got_hid as u64, hid);
        let _ = std::fs::remove_file(&path);
    }

    #[tokio::test]
    async fn collab_false_ignores_every_sp_live_event_and_disables_the_presence_timer() {
        let hid = 900_105;
        let addr = start(hid, false).await;
        let mut a = connect_ws(&addr).await;
        send_live(&mut a, "hello:Alice").await;
        let result = tokio::time::timeout(std::time::Duration::from_millis(300), a.read(&mut [0u8; 64])).await;
        assert!(result.is_err(), "no reply should ever arrive when collab is disabled");
        assert_eq!(state(hid, false).timer_intervals(), Vec::<f64>::new());
    }

    #[tokio::test]
    async fn a_stranger_component_id_is_ignored_even_with_collab_enabled() {
        let hid = 900_106;
        let addr = start(hid, true).await;
        let mut a = connect_ws(&addr).await;
        let msg = serde_json::json!({ "id": "not-sp-live", "value": "hello:Alice" }).to_string();
        a.write_all(&masked_text_frame(&msg)).await.unwrap();
        let result = tokio::time::timeout(std::time::Duration::from_millis(300), a.read(&mut [0u8; 64])).await;
        assert!(result.is_err(), "unrelated component ids must not produce a Sera Live reply");
    }

    #[tokio::test]
    async fn closing_a_connection_drops_its_presence_immediately_not_after_the_stale_window() {
        let hid = 900_107;
        let addr = start(hid, true).await;
        let mut a = connect_ws(&addr).await;
        let mut b = connect_ws(&addr).await;
        send_live(&mut a, "hello:Alice").await;
        let _ = read_json_until(&mut a, is_sync_reply).await;
        send_live(&mut b, "hello:Bob").await;
        let sync_b = read_json_until(&mut b, is_sync_reply).await;
        let presence_after_join = serde_json::from_str::<serde_json::Value>(sync_b["html"].as_str().unwrap()).unwrap();
        assert_eq!(presence_after_join["presence"].as_array().unwrap().len(), 2);

        drop(a);
        let update = read_json_until(&mut b, |v| v["k"] == "presence").await;
        let list = update["list"].as_array().unwrap();
        assert_eq!(list.len(), 1, "Alice's presence should disappear right away, not 15 seconds later");
        assert_eq!(list[0]["n"], "Bob");
    }

    #[tokio::test]
    async fn a_burst_of_pins_from_one_session_is_throttled_not_all_broadcast() {
        let hid = 900_108;
        let addr = start(hid, true).await;
        let mut a = connect_ws(&addr).await;
        let mut b = connect_ws(&addr).await;
        send_live(&mut a, "hello:Alice").await;
        let _ = read_json_until(&mut a, is_sync_reply).await;
        send_live(&mut b, "hello:Bob").await;
        let _ = read_json_until(&mut b, is_sync_reply).await;

        for i in 0..20 {
            send_live(&mut a, &format!("pin:{i}:10:-1:spam")).await;
        }
        let mut seen = 0;
        loop {
            let got = tokio::time::timeout(std::time::Duration::from_millis(400), async { read_json_until(&mut b, |v| v["k"] == "pin").await }).await;
            match got {
                Ok(_) => seen += 1,
                Err(_) => break,
            }
        }
        assert!(seen < 20, "an unthrottled burst of 20 pins must not all reach other clients, got {seen}");
    }

    #[tokio::test]
    async fn two_immediate_replay_clicks_only_broadcast_the_recording_once() {
        let hid = 900_109;
        let addr = start(hid, true).await;
        let path = std::env::temp_dir().join(format!("sp_live_replay_debounce_{}.spls", std::process::id())).to_string_lossy().to_string();
        let meta = push_registry::PushMeta::Scalar(push_registry::ScalarMeta { min_val: 0.0, range_val: 100.0, axis_px: 300 });
        crate::plot::record_registry::start(hid, &path, &meta).unwrap();
        let rec_state = crate::plot::record_registry::get(hid).unwrap();
        rec_state.append(&build_binary_frame(hid, &pack_indices_u32_raw(&[0]), &[10, 0], 1, 1));
        crate::plot::record_registry::stop(hid);

        let mut a = connect_ws(&addr).await;
        let mut b = connect_ws(&addr).await;
        send_live(&mut a, "hello:Alice").await;
        let _ = read_json_until(&mut a, is_sync_reply).await;
        send_live(&mut b, "hello:Bob").await;
        let _ = read_json_until(&mut b, is_sync_reply).await;

        send_live(&mut a, "replay").await;
        send_live(&mut a, "replay").await;
        let _ = read_frame_until(&mut b, |f| f.opcode == crate::webapp::ws::OPCODE_BINARY).await;
        let second = tokio::time::timeout(std::time::Duration::from_millis(400), async { read_frame_until(&mut b, |f| f.opcode == crate::webapp::ws::OPCODE_BINARY).await }).await;
        assert!(second.is_err(), "a second immediate replay click must be debounced, not broadcast twice");
        let _ = std::fs::remove_file(&path);
    }
}
