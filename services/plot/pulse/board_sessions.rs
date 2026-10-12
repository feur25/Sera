use std::path::PathBuf;

fn civil_from_days(z: i64) -> (i64, u32, u32) {
    let z = z + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = (z - era * 146097) as u64;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    let y = if m <= 2 { y + 1 } else { y };
    (y, m, d)
}

pub fn today_session_name() -> String {
    let days = crate::plot::now_ms().div_euclid(86_400_000);
    let (y, m, d) = civil_from_days(days);
    format!("{y:04}-{m:02}-{d:02}")
}

pub fn boards_root() -> PathBuf {
    let home = std::env::var("USERPROFILE").or_else(|_| std::env::var("HOME")).unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home).join(".seraplot").join("boards")
}

fn sanitize(name: &str) -> String {
    let cleaned: String = name.chars().map(|c| if c.is_alphanumeric() || c == '-' || c == '_' { c } else { '_' }).collect();
    if cleaned.is_empty() {
        "board".to_string()
    } else {
        cleaned
    }
}

pub fn session_path(name: &str) -> PathBuf {
    boards_root().join(format!("{}.json", sanitize(name)))
}

pub struct SessionInfo {
    pub name: String,
    pub mtime_ms: i64,
    pub mark_count: usize,
}

pub fn list_sessions() -> Vec<SessionInfo> {
    let root = boards_root();
    let Ok(entries) = std::fs::read_dir(&root) else { return Vec::new() };
    let mut out = Vec::new();
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("json") {
            continue;
        }
        let Some(stem) = path.file_stem().and_then(|s| s.to_str()) else { continue };
        let mtime_ms = entry
            .metadata()
            .ok()
            .and_then(|m| m.modified().ok())
            .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
            .map(|d| d.as_millis() as i64)
            .unwrap_or(0);
        let mark_count = path.to_str().map(super::live_marks::peek_mark_count).unwrap_or(0);
        out.push(SessionInfo { name: stem.to_string(), mtime_ms, mark_count });
    }
    out.sort_by(|a, b| b.mtime_ms.cmp(&a.mtime_ms));
    out
}

pub fn delete_session(name: &str) -> bool {
    let path = session_path(name);
    std::fs::remove_file(&path).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn civil_from_days_matches_the_unix_epoch() {
        assert_eq!(civil_from_days(0), (1970, 1, 1));
    }

    #[test]
    fn civil_from_days_matches_a_known_date() {
        assert_eq!(civil_from_days(20674), (2026, 8, 9));
    }

    #[test]
    fn today_session_name_has_the_expected_shape() {
        let name = today_session_name();
        assert_eq!(name.len(), 10);
        assert_eq!(name.chars().nth(4), Some('-'));
        assert_eq!(name.chars().nth(7), Some('-'));
    }

    #[test]
    fn sanitize_replaces_unsafe_characters_but_keeps_dashes_and_underscores() {
        assert_eq!(sanitize("2026-08-09"), "2026-08-09");
        assert_eq!(sanitize("war room / q3"), "war_room___q3");
        assert_eq!(sanitize(""), "board");
    }

    #[test]
    fn session_path_is_stable_for_the_same_name() {
        assert_eq!(session_path("2026-08-09"), session_path("2026-08-09"));
    }

    #[test]
    fn list_and_delete_round_trip_against_real_files() {
        let root = boards_root();
        std::fs::create_dir_all(&root).unwrap();
        let name = "test_session_round_trip_unique_xyz";
        let path = session_path(name);
        std::fs::write(&path, "[]").unwrap();

        let sessions = list_sessions();
        assert!(sessions.iter().any(|s| s.name == name));

        assert!(delete_session(name));
        assert!(!std::path::Path::new(&path).exists());
        assert!(!delete_session(name));
    }
}
