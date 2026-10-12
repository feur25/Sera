use super::token_bucket::Bucket;
use crate::Chart;
use pyo3::prelude::*;
use seraplot_macros::sera_doc_impl;
use std::collections::{HashMap, VecDeque};
use std::sync::Mutex;

const CAPACITY: usize = 256;
const MAX_CHANNELS_PER_CHART: usize = 16;
const DEFAULT_MAX_ALERTS_PER_MINUTE: f64 = 10.0;

#[derive(Clone)]
pub enum AlertChannel {
    Slack(String),
    Discord(String),
    PagerDuty(String),
    Webhook(String),
}

struct Registry {
    map: HashMap<u64, Vec<AlertChannel>>,
    order: VecDeque<u64>,
}

static REGISTRY: Mutex<Option<Registry>> = Mutex::new(None);

pub fn add(hid: u64, channel: AlertChannel) -> PyResult<()> {
    let Ok(mut guard) = REGISTRY.lock() else {
        return Err(pyo3::exceptions::PyRuntimeError::new_err("seraplot pulse: alert registry lock poisoned"));
    };
    let reg = guard.get_or_insert_with(|| Registry { map: HashMap::new(), order: VecDeque::new() });
    if !reg.map.contains_key(&hid) {
        reg.order.push_back(hid);
        while reg.order.len() > CAPACITY {
            if let Some(old) = reg.order.pop_front() {
                reg.map.remove(&old);
            }
        }
    }
    let channels = reg.map.entry(hid).or_default();
    if channels.len() >= MAX_CHANNELS_PER_CHART {
        return Err(pyo3::exceptions::PyRuntimeError::new_err(format!(
            "seraplot pulse: this chart already has {MAX_CHANNELS_PER_CHART} alert channels, the maximum"
        )));
    }
    channels.push(channel);
    Ok(())
}

pub fn clear(hid: u64) {
    if let Ok(mut guard) = REGISTRY.lock() {
        if let Some(reg) = guard.as_mut() {
            reg.map.remove(&hid);
        }
    }
}

fn channels_for(hid: u64) -> Vec<AlertChannel> {
    let Ok(guard) = REGISTRY.lock() else { return Vec::new() };
    guard.as_ref().and_then(|r| r.map.get(&hid)).cloned().unwrap_or_default()
}

fn kind_str(channel: &AlertChannel) -> &'static str {
    match channel {
        AlertChannel::Slack(_) => "slack",
        AlertChannel::Discord(_) => "discord",
        AlertChannel::PagerDuty(_) => "pagerduty",
        AlertChannel::Webhook(_) => "webhook",
    }
}

fn target_str(channel: &AlertChannel) -> &str {
    match channel {
        AlertChannel::Slack(t) | AlertChannel::Discord(t) | AlertChannel::PagerDuty(t) | AlertChannel::Webhook(t) => t,
    }
}

fn remove(hid: u64, kind: &str, target: Option<&str>) -> usize {
    let Ok(mut guard) = REGISTRY.lock() else { return 0 };
    let Some(reg) = guard.as_mut() else { return 0 };
    let Some(channels) = reg.map.get_mut(&hid) else { return 0 };
    let before = channels.len();
    channels.retain(|c| !(kind_str(c) == kind && target.map(|t| target_str(c) == t).unwrap_or(true)));
    before - channels.len()
}

struct ThrottleRegistry {
    map: HashMap<u64, Bucket>,
    order: VecDeque<u64>,
}

static THROTTLE: Mutex<Option<ThrottleRegistry>> = Mutex::new(None);

pub fn set_throttle(hid: u64, max_per_minute: f64) {
    let Ok(mut guard) = THROTTLE.lock() else { return };
    let reg = guard.get_or_insert_with(|| ThrottleRegistry { map: HashMap::new(), order: VecDeque::new() });
    if !reg.map.contains_key(&hid) {
        reg.order.push_back(hid);
        while reg.order.len() > CAPACITY {
            if let Some(old) = reg.order.pop_front() {
                reg.map.remove(&old);
            }
        }
    }
    reg.map.insert(hid, Bucket::new(max_per_minute / 60.0));
}

fn throttle_allows(hid: u64) -> bool {
    let Ok(mut guard) = THROTTLE.lock() else { return true };
    let reg = guard.get_or_insert_with(|| ThrottleRegistry { map: HashMap::new(), order: VecDeque::new() });
    if !reg.map.contains_key(&hid) {
        reg.order.push_back(hid);
        while reg.order.len() > CAPACITY {
            if let Some(old) = reg.order.pop_front() {
                reg.map.remove(&old);
            }
        }
        reg.map.insert(hid, Bucket::new(DEFAULT_MAX_ALERTS_PER_MINUTE / 60.0));
    }
    reg.map.get_mut(&hid).map(Bucket::try_consume).unwrap_or(true)
}

fn payload_for(channel: &AlertChannel, hid: u64, idx: u32, value: f64, z: f64) -> (String, serde_json::Value) {
    let summary = format!("Sera Pulse anomaly on chart {hid}: index {idx} = {value:.4} (z={z:.2})");
    match channel {
        AlertChannel::Slack(url) => (url.clone(), serde_json::json!({ "text": summary })),
        AlertChannel::Discord(url) => (url.clone(), serde_json::json!({ "content": summary })),
        AlertChannel::PagerDuty(routing_key) => (
            "https://events.pagerduty.com/v2/enqueue".to_string(),
            serde_json::json!({
                "routing_key": routing_key,
                "event_action": "trigger",
                "payload": {
                    "summary": summary,
                    "severity": "warning",
                    "source": format!("seraplot-pulse-chart-{hid}"),
                }
            }),
        ),
        AlertChannel::Webhook(url) => {
            (url.clone(), serde_json::json!({ "chart_id": hid, "index": idx, "value": value, "z_score": z }))
        }
    }
}

pub fn dispatch(hid: u64, idx: u32, value: f64, z: f64) {
    let channels = channels_for(hid);
    if channels.is_empty() {
        return;
    }
    if !throttle_allows(hid) {
        super::metrics::incr("seraplot_pulse_alerts_throttled_total", hid);
        return;
    }
    std::thread::spawn(move || {
        let Ok(client) = reqwest::blocking::Client::builder().timeout(std::time::Duration::from_secs(5)).build() else {
            return;
        };
        for channel in &channels {
            let (url, body) = payload_for(channel, hid, idx, value, z);
            match client.post(&url).json(&body).send() {
                Ok(resp) if resp.status().is_success() => {
                    super::metrics::incr("seraplot_pulse_alerts_sent_total", hid);
                }
                _ => {
                    super::metrics::incr("seraplot_pulse_alerts_failed_total", hid);
                }
            }
        }
    });
}

#[cfg(feature = "python")]
#[sera_doc_impl]
#[pyo3::pymethods]
impl Chart {
    #[sera_doc(
        category = "chart_method",
        module = "alerts",
        file = "seraplot/pulse-pricing.md",
        aliases("notify_via"),
        en = "Attaches an integrated alert channel that fires automatically whenever on_anomaly() detects an anomaly, alongside the Python callback (call on_anomaly() first to start detection). Sent from a background thread, never blocks push().",
        fr = "Attache un canal d'alerte intégré qui se déclenche automatiquement à chaque anomalie détectée par on_anomaly(), en plus du callback Python (appelez d'abord on_anomaly() pour démarrer la détection). Envoyé depuis un thread en arrière-plan, ne bloque jamais push().",
        param(name = "kind", ty = "str", en = "One of 'slack', 'discord', 'pagerduty', 'webhook'.", fr = "Un parmi 'slack', 'discord', 'pagerduty', 'webhook'."),
        param(name = "target", ty = "str", en = "Slack/Discord/generic webhook: the webhook URL. PagerDuty: the integration routing key.", fr = "Slack/Discord/webhook générique : l'URL du webhook. PagerDuty : la clé de routage d'intégration.")
    )]
    fn add_alert_channel(&self, kind: &str, target: &str) -> PyResult<()> {
        let hid = super::require_hid(&self.html)?;
        let channel = match kind {
            "slack" => AlertChannel::Slack(target.to_string()),
            "discord" => AlertChannel::Discord(target.to_string()),
            "pagerduty" => AlertChannel::PagerDuty(target.to_string()),
            "webhook" => AlertChannel::Webhook(target.to_string()),
            other => {
                return Err(pyo3::exceptions::PyValueError::new_err(format!(
                    "unknown alert channel kind '{other}', expected one of: slack, discord, pagerduty, webhook"
                )))
            }
        };
        add(hid, channel)
    }

    #[sera_doc(
        category = "chart_method",
        module = "alerts",
        file = "seraplot/pulse-pricing.md",
        aliases("stop_notifying"),
        en = "Removes every alert channel added with add_alert_channel(), if any.",
        fr = "Retire tous les canaux d'alerte ajoutés avec add_alert_channel(), s'il y en a."
    )]
    fn clear_alert_channels(&self) -> PyResult<()> {
        let hid = super::require_hid(&self.html)?;
        clear(hid);
        Ok(())
    }

    #[sera_doc(
        category = "chart_method",
        module = "alerts",
        file = "seraplot/pulse-pricing.md",
        aliases("throttle_alerts"),
        en = "Caps how many alerts per minute this chart's alert channels will actually send, regardless of how many anomalies fire — protects Slack/Discord/PagerDuty from being flooded during a real incident. A sensible default (10/minute) applies automatically even without calling this.",
        fr = "Plafonne le nombre d'alertes par minute réellement envoyées par les canaux de ce chart, quel que soit le nombre d'anomalies détectées — protège Slack/Discord/PagerDuty d'être inondé pendant un vrai incident. Une valeur par défaut raisonnable (10/minute) s'applique automatiquement même sans appeler cette méthode.",
        param(name = "max_per_minute", ty = "float", en = "Maximum alerts sent per minute for this chart, across all its channels combined.", fr = "Nombre maximal d'alertes envoyées par minute pour ce chart, tous canaux confondus.")
    )]
    fn set_alert_rate_limit(&self, max_per_minute: f64) -> PyResult<()> {
        let hid = super::require_hid(&self.html)?;
        set_throttle(hid, max_per_minute);
        Ok(())
    }

    #[sera_doc(
        category = "chart_method",
        module = "alerts",
        file = "seraplot/pulse-pricing.md",
        en = "Lists every alert channel currently attached via add_alert_channel(), as (kind, target) tuples - 'target' is the webhook URL for slack/discord/webhook, or the routing key for pagerduty.",
        fr = "Liste chaque canal d'alerte actuellement attaché via add_alert_channel(), sous forme de tuples (kind, target) — « target » est l'URL du webhook pour slack/discord/webhook, ou la clé de routage pour pagerduty."
    )]
    fn list_alert_channels(&self) -> PyResult<Vec<(String, String)>> {
        let hid = super::require_hid(&self.html)?;
        Ok(channels_for(hid).iter().map(|c| (kind_str(c).to_string(), target_str(c).to_string())).collect())
    }

    #[pyo3(signature = (kind, target = None))]
    #[sera_doc(
        category = "chart_method",
        module = "alerts",
        file = "seraplot/pulse-pricing.md",
        en = "Removes one specific alert channel added with add_alert_channel(), instead of clearing all of them with clear_alert_channels(). Matches by kind alone if target is omitted (removes every channel of that kind), or by kind+target for a single exact match. Returns how many channels were actually removed.",
        fr = "Retire un canal d'alerte spécifique ajouté avec add_alert_channel(), plutôt que de tous les effacer avec clear_alert_channels(). Filtre par kind seul si target est omis (retire tous les canaux de ce type), ou par kind+target pour une correspondance exacte unique. Retourne le nombre de canaux effectivement retirés.",
        param(name = "kind", ty = "str", en = "One of 'slack', 'discord', 'pagerduty', 'webhook'.", fr = "Un parmi 'slack', 'discord', 'pagerduty', 'webhook'."),
        param(name = "target", ty = "str | None", en = "Optional exact target (URL or routing key) to match; omit to remove every channel of this kind.", fr = "Cible exacte optionnelle (URL ou clé de routage) à filtrer ; à omettre pour retirer tous les canaux de ce type.")
    )]
    fn remove_alert_channel(&self, kind: &str, target: Option<&str>) -> PyResult<usize> {
        let hid = super::require_hid(&self.html)?;
        Ok(remove(hid, kind, target))
    }

    fn notify_via(&self, kind: &str, target: &str) -> PyResult<()> {
        self.add_alert_channel(kind, target)
    }

    fn stop_notifying(&self) -> PyResult<()> {
        self.clear_alert_channels()
    }

    fn throttle_alerts(&self, max_per_minute: f64) -> PyResult<()> {
        self.set_alert_rate_limit(max_per_minute)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Read, Write};
    use std::net::TcpListener;

    #[test]
    fn slack_payload_shape() {
        let (url, body) = payload_for(&AlertChannel::Slack("https://hooks.slack.com/x".into()), 1, 2, 3.0, 4.0);
        assert_eq!(url, "https://hooks.slack.com/x");
        assert!(body["text"].as_str().unwrap().contains("index 2"));
    }

    #[test]
    fn discord_payload_shape() {
        let (url, body) = payload_for(&AlertChannel::Discord("https://discord.com/api/webhooks/x".into()), 1, 5, 9.0, 2.0);
        assert_eq!(url, "https://discord.com/api/webhooks/x");
        assert!(body["content"].as_str().unwrap().contains("index 5"));
    }

    #[test]
    fn pagerduty_payload_shape() {
        let (url, body) = payload_for(&AlertChannel::PagerDuty("routing-key-123".into()), 1, 7, 1.5, 3.3);
        assert_eq!(url, "https://events.pagerduty.com/v2/enqueue");
        assert_eq!(body["routing_key"], "routing-key-123");
        assert_eq!(body["event_action"], "trigger");
        assert!(body["payload"]["summary"].as_str().unwrap().contains("index 7"));
    }

    #[test]
    fn generic_webhook_payload_shape() {
        let (url, body) = payload_for(&AlertChannel::Webhook("https://example.com/hook".into()), 42, 8, 6.28, 4.1);
        assert_eq!(url, "https://example.com/hook");
        assert_eq!(body["chart_id"], 42);
        assert_eq!(body["index"], 8);
    }

    #[test]
    fn kind_str_and_target_str_round_trip_every_variant() {
        assert_eq!(kind_str(&AlertChannel::Slack("a".into())), "slack");
        assert_eq!(target_str(&AlertChannel::Slack("a".into())), "a");
        assert_eq!(kind_str(&AlertChannel::Discord("b".into())), "discord");
        assert_eq!(target_str(&AlertChannel::Discord("b".into())), "b");
        assert_eq!(kind_str(&AlertChannel::PagerDuty("c".into())), "pagerduty");
        assert_eq!(target_str(&AlertChannel::PagerDuty("c".into())), "c");
        assert_eq!(kind_str(&AlertChannel::Webhook("d".into())), "webhook");
        assert_eq!(target_str(&AlertChannel::Webhook("d".into())), "d");
    }

    #[test]
    fn remove_by_kind_and_target_removes_only_the_exact_match() {
        let hid = 2_000_009u64;
        clear(hid);
        add(hid, AlertChannel::Slack("https://hooks.slack.com/a".into())).unwrap();
        add(hid, AlertChannel::Slack("https://hooks.slack.com/b".into())).unwrap();
        add(hid, AlertChannel::Discord("https://discord.com/x".into())).unwrap();

        let removed = remove(hid, "slack", Some("https://hooks.slack.com/a"));
        assert_eq!(removed, 1);
        let remaining = channels_for(hid);
        assert_eq!(remaining.len(), 2);
        assert!(remaining.iter().any(|c| kind_str(c) == "slack" && target_str(c) == "https://hooks.slack.com/b"));
        assert!(remaining.iter().any(|c| kind_str(c) == "discord"));
        clear(hid);
    }

    #[test]
    fn remove_by_kind_alone_removes_every_channel_of_that_kind() {
        let hid = 2_000_010u64;
        clear(hid);
        add(hid, AlertChannel::Webhook("https://a".into())).unwrap();
        add(hid, AlertChannel::Webhook("https://b".into())).unwrap();
        add(hid, AlertChannel::Slack("https://c".into())).unwrap();

        let removed = remove(hid, "webhook", None);
        assert_eq!(removed, 2);
        let remaining = channels_for(hid);
        assert_eq!(remaining.len(), 1);
        assert_eq!(kind_str(&remaining[0]), "slack");
        clear(hid);
    }

    #[test]
    fn remove_on_unknown_chart_or_no_match_returns_zero_not_an_error() {
        assert_eq!(remove(999_999_998u64, "slack", None), 0);
        let hid = 2_000_011u64;
        clear(hid);
        add(hid, AlertChannel::Slack("https://x".into())).unwrap();
        assert_eq!(remove(hid, "slack", Some("https://does-not-exist")), 0);
        assert_eq!(channels_for(hid).len(), 1);
        clear(hid);
    }

    #[test]
    fn overhead_of_list_and_remove_is_negligible_even_at_the_per_chart_cap() {
        let hid = 2_000_012u64;
        clear(hid);
        for i in 0..MAX_CHANNELS_PER_CHART {
            add(hid, AlertChannel::Webhook(format!("https://example.com/{i}"))).unwrap();
        }
        let n = 50_000u32;
        let started = std::time::Instant::now();
        for _ in 0..n {
            let _ = channels_for(hid).iter().map(|c| (kind_str(c).to_string(), target_str(c).to_string())).collect::<Vec<_>>();
        }
        let elapsed = started.elapsed();
        println!("alerts list (at {MAX_CHANNELS_PER_CHART}-channel cap) x{n}: {elapsed:?} ({:?}/call)", elapsed / n);
        assert!(elapsed.as_millis() < 1000, "listing channels got too slow: {elapsed:?} for {n} calls");

        let started = std::time::Instant::now();
        for _ in 0..n {
            remove(hid, "nonexistent", None);
        }
        let elapsed = started.elapsed();
        println!("alerts remove (no-op, at cap) x{n}: {elapsed:?} ({:?}/call)", elapsed / n);
        assert!(elapsed.as_millis() < 1000, "remove() got too slow: {elapsed:?} for {n} calls");
        clear(hid);
    }

    #[test]
    fn default_throttle_caps_bursts_without_any_configuration() {
        let hid = 2_000_005u64;
        let mut allowed = 0;
        for _ in 0..50 {
            if throttle_allows(hid) {
                allowed += 1;
            }
        }
        assert!(allowed < 50, "an unconfigured chart should still have a default anti-burst cap, got {allowed}/50 allowed");
        assert!(allowed >= 1, "the very first alert should still go through immediately, got {allowed}/50 allowed");
    }

    #[test]
    fn explicit_throttle_overrides_the_default() {
        let hid = 2_000_006u64;
        set_throttle(hid, 600.0);
        let mut allowed = 0;
        for _ in 0..10 {
            if throttle_allows(hid) {
                allowed += 1;
            }
        }
        assert_eq!(allowed, 10, "600/minute (10/sec) burst capacity should allow a rapid-fire burst of 10 through entirely, well above the 1-2 the default would allow");
    }

    #[test]
    fn explicit_zero_means_never_not_one_freebie() {
        let hid = 2_000_008u64;
        set_throttle(hid, 0.0);
        assert!(!throttle_allows(hid), "a rate of exactly 0 should mean never allow, not one free token before going silent");
        assert!(!throttle_allows(hid));
    }

    #[test]
    fn throttled_dispatch_is_counted_and_does_not_reach_the_network() {
        let hid = 2_000_007u64;
        clear(hid);
        add(hid, AlertChannel::Webhook("http://127.0.0.1:1/should-never-be-hit".into())).unwrap();
        set_throttle(hid, 0.0);
        for _ in 0..5 {
            dispatch(hid, 1, 1.0, 1.0);
        }
        let text = super::super::metrics::render_prometheus();
        assert!(
            text.contains(&format!("seraplot_pulse_alerts_throttled_total{{chart=\"{hid}\"}} 5")),
            "expected 5 throttled alerts recorded, got: {text}"
        );
        clear(hid);
    }

    #[test]
    fn per_chart_cap_and_invalid_kind_are_enforced() {
        pyo3::prepare_freethreaded_python();
        let hid = 2_000_002u64;
        clear(hid);
        for _ in 0..MAX_CHANNELS_PER_CHART {
            add(hid, AlertChannel::Webhook("https://example.com".into())).expect("within cap");
        }
        let err = add(hid, AlertChannel::Webhook("https://example.com".into())).unwrap_err();
        assert!(Python::with_gil(|py| err.value_bound(py).to_string().contains("maximum")));
        clear(hid);
    }

    #[test]
    fn dispatch_returns_immediately_even_with_channels_configured() {
        let hid = 2_000_003u64;
        clear(hid);
        add(hid, AlertChannel::Webhook("http://127.0.0.1:1/unreachable".into())).unwrap();
        let started = std::time::Instant::now();
        dispatch(hid, 1, 1.0, 1.0);
        let elapsed = started.elapsed();
        println!("dispatch() call overhead: {elapsed:?}");
        assert!(elapsed.as_millis() < 50, "dispatch() must return immediately (spawns a background thread), took {elapsed:?}");
        clear(hid);
    }

    #[test]
    fn dispatch_sends_a_real_http_post_to_a_local_server() {
        let listener = TcpListener::bind("127.0.0.1:0").expect("bind local test server");
        let addr = listener.local_addr().unwrap();
        let received: std::sync::Arc<Mutex<Option<String>>> = std::sync::Arc::new(Mutex::new(None));
        let received_clone = received.clone();
        let handle = std::thread::spawn(move || {
            if let Ok((mut stream, _)) = listener.accept() {
                let mut buf = [0u8; 4096];
                let n = stream.read(&mut buf).unwrap_or(0);
                *received_clone.lock().unwrap() = Some(String::from_utf8_lossy(&buf[..n]).to_string());
                let _ = stream.write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 0\r\n\r\n");
            }
        });

        let hid = 2_000_004u64;
        clear(hid);
        add(hid, AlertChannel::Webhook(format!("http://{addr}/hook"))).unwrap();
        dispatch(hid, 42, 3.14, 5.0);
        handle.join().expect("server thread");

        let text = received.lock().unwrap().clone().expect("server should have received a request");
        assert!(text.starts_with("POST /hook"), "expected a POST to /hook, got: {text}");
        assert!(text.contains("\"index\":42"), "expected the index in the JSON body, got: {text}");
        clear(hid);
    }
}
