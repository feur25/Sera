use crate::core::asset_pack::{cached_pack, AssetPack};
use std::sync::OnceLock;

static MAP_PACK_BYTES: &[u8] = include_bytes!("../../../../asset/maps.spak");
static MAP_PACK: OnceLock<AssetPack> = OnceLock::new();

fn pack() -> &'static AssetPack {
    cached_pack(&MAP_PACK, MAP_PACK_BYTES)
}

pub fn map_asset(key: &str) -> String {
    pack().get_utf8(key).unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn maps_root() -> PathBuf {
        let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let direct = manifest.join("asset").join("maps");
        if direct.is_dir() {
            direct
        } else {
            manifest.join("src").join("asset").join("maps")
        }
    }

    fn walk(dir: &std::path::Path, out: &mut Vec<PathBuf>) {
        let Ok(entries) = std::fs::read_dir(dir) else { return };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                walk(&path, out);
            } else if path.extension().and_then(|e| e.to_str()) == Some("svg") {
                out.push(path);
            }
        }
    }

    fn pack_key_for(file: &std::path::Path, root: &std::path::Path) -> String {
        let rel = file.strip_prefix(root).expect("file must live under maps_root");
        let joined = rel
            .components()
            .map(|c| c.as_os_str().to_string_lossy().into_owned())
            .collect::<Vec<_>>()
            .join("/");
        joined.strip_suffix(".svg").unwrap_or(&joined).to_string()
    }

    #[test]
    fn map_pack_loads_with_at_least_one_hundred_entries() {
        assert!(pack().len() >= 100, "expected a fully populated map pack, got {}", pack().len());
    }

    #[test]
    fn map_asset_returns_valid_svg_markup_for_a_known_key() {
        let svg = map_asset("world/world");
        assert!(svg.contains("<svg"), "expected valid svg markup for world/world");
    }

    #[test]
    fn map_asset_returns_empty_string_for_an_unknown_key() {
        assert_eq!(map_asset("does/not/exist"), "");
    }

    #[test]
    fn every_svg_under_asset_maps_has_a_matching_pack_entry() {
        let root = maps_root();
        let mut files = Vec::new();
        walk(&root, &mut files);
        assert!(files.len() >= 100, "expected at least 100 source svg files, found {}", files.len());
        for file in &files {
            let key = pack_key_for(file, &root);
            assert!(!map_asset(&key).is_empty(), "no pack entry for source file {file:?} (key {key})");
        }
    }

    #[test]
    fn every_pack_entry_matches_its_source_svg_file_byte_for_byte() {
        let root = maps_root();
        let mut checked = 0usize;
        for key in pack().keys() {
            let path = root.join(format!("{key}.svg"));
            let expected = std::fs::read_to_string(&path)
                .unwrap_or_else(|_| panic!("missing source file for pack key {key}: {path:?}"));
            assert_eq!(map_asset(key), expected, "pack entry {key} is stale, regenerate maps.spak");
            checked += 1;
        }
        assert!(checked >= 100, "expected at least 100 verified pack entries, got {checked}");
    }

    #[test]
    #[ignore]
    fn regenerate_map_asset_pack() {
        let root = maps_root();
        let mut files = Vec::new();
        walk(&root, &mut files);
        files.sort();
        let mut builder = crate::core::asset_pack::PackBuilder::new();
        for file in &files {
            let key = pack_key_for(file, &root);
            let raw = std::fs::read(file).unwrap_or_else(|e| panic!("cannot read {file:?}: {e}"));
            builder = builder.add(&key, &raw);
        }
        let bytes = builder.build();
        let out = root.parent().expect("maps_root must have a parent").join("maps.spak");
        std::fs::write(&out, &bytes).unwrap_or_else(|e| panic!("cannot write {out:?}: {e}"));
        println!("packed {} svg assets into {:?} ({} bytes)", files.len(), out, bytes.len());
    }
}
