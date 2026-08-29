use std::collections::HashMap;
use std::sync::OnceLock;

const MAGIC: &[u8; 4] = b"SPAK";
const VERSION: u16 = 1;

#[inline]
pub fn deflate(bytes: &[u8]) -> Vec<u8> {
    miniz_oxide::deflate::compress_to_vec(bytes, 10)
}

#[inline]
pub fn inflate(bytes: &[u8]) -> Vec<u8> {
    miniz_oxide::inflate::decompress_to_vec(bytes).unwrap_or_default()
}

#[inline]
fn read_u16(bytes: &[u8], at: usize) -> (u16, usize) {
    (u16::from_le_bytes([bytes[at], bytes[at + 1]]), at + 2)
}

#[inline]
fn read_u32(bytes: &[u8], at: usize) -> (u32, usize) {
    (
        u32::from_le_bytes([bytes[at], bytes[at + 1], bytes[at + 2], bytes[at + 3]]),
        at + 4,
    )
}

pub struct AssetPack {
    blob: &'static [u8],
    index: HashMap<String, (u32, u32)>,
}

impl AssetPack {
    pub fn load(blob: &'static [u8]) -> Self {
        let mut index = HashMap::new();
        if blob.len() < 10 || &blob[0..4] != MAGIC {
            return Self { blob, index };
        }
        let (version, at) = read_u16(blob, 4);
        if version != VERSION {
            return Self { blob, index };
        }
        let (count, mut at) = read_u32(blob, at);
        let mut entries: Vec<(String, u32)> = Vec::with_capacity(count as usize);
        for _ in 0..count {
            let (key_len, next) = read_u16(blob, at);
            at = next;
            let key = String::from_utf8_lossy(&blob[at..at + key_len as usize]).into_owned();
            at += key_len as usize;
            let (_raw_len, next) = read_u32(blob, at);
            at = next;
            let (comp_len, next) = read_u32(blob, at);
            at = next;
            entries.push((key, comp_len));
        }
        let mut cursor = at as u32;
        for (key, comp_len) in entries {
            index.insert(key, (cursor, comp_len));
            cursor += comp_len;
        }
        Self { blob, index }
    }

    pub fn get(&self, key: &str) -> Option<Vec<u8>> {
        let (offset, comp_len) = *self.index.get(key)?;
        let start = offset as usize;
        let end = start + comp_len as usize;
        Some(inflate(self.blob.get(start..end)?))
    }

    pub fn get_utf8(&self, key: &str) -> Option<String> {
        self.get(key).map(|bytes| String::from_utf8_lossy(&bytes).into_owned())
    }

    pub fn keys(&self) -> impl Iterator<Item = &str> {
        self.index.keys().map(|s| s.as_str())
    }

    pub fn len(&self) -> usize {
        self.index.len()
    }

    pub fn is_empty(&self) -> bool {
        self.index.is_empty()
    }
}

pub fn cached_pack(cell: &'static OnceLock<AssetPack>, blob: &'static [u8]) -> &'static AssetPack {
    cell.get_or_init(|| AssetPack::load(blob))
}

pub struct PackBuilder {
    entries: Vec<(String, Vec<u8>, u32)>,
}

impl PackBuilder {
    pub fn new() -> Self {
        Self { entries: Vec::new() }
    }

    pub fn add(mut self, key: &str, raw: &[u8]) -> Self {
        self.entries.push((key.to_string(), deflate(raw), raw.len() as u32));
        self
    }

    pub fn build(self) -> Vec<u8> {
        let mut out = Vec::new();
        out.extend_from_slice(MAGIC);
        out.extend_from_slice(&VERSION.to_le_bytes());
        out.extend_from_slice(&(self.entries.len() as u32).to_le_bytes());
        for (key, compressed, raw_len) in &self.entries {
            let key_bytes = key.as_bytes();
            out.extend_from_slice(&(key_bytes.len() as u16).to_le_bytes());
            out.extend_from_slice(key_bytes);
            out.extend_from_slice(&raw_len.to_le_bytes());
            out.extend_from_slice(&(compressed.len() as u32).to_le_bytes());
        }
        for (_, compressed, _) in &self.entries {
            out.extend_from_slice(compressed);
        }
        out
    }
}

impl Default for PackBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trips_a_single_entry_through_build_and_load() {
        let bytes = PackBuilder::new().add("a/b", b"hello world").build();
        let pack = AssetPack::load(Box::leak(bytes.into_boxed_slice()));
        assert_eq!(pack.get_utf8("a/b").as_deref(), Some("hello world"));
    }

    #[test]
    fn round_trips_multiple_entries_preserving_each_payload() {
        let bytes = PackBuilder::new()
            .add("one", b"first payload")
            .add("two", b"second, different payload")
            .add("three", &[0u8, 1, 2, 3, 255, 254])
            .build();
        let pack = AssetPack::load(Box::leak(bytes.into_boxed_slice()));
        assert_eq!(pack.len(), 3);
        assert_eq!(pack.get_utf8("one").as_deref(), Some("first payload"));
        assert_eq!(pack.get_utf8("two").as_deref(), Some("second, different payload"));
        assert_eq!(pack.get("three"), Some(vec![0u8, 1, 2, 3, 255, 254]));
    }

    #[test]
    fn get_returns_none_for_an_unknown_key() {
        let bytes = PackBuilder::new().add("known", b"x").build();
        let pack = AssetPack::load(Box::leak(bytes.into_boxed_slice()));
        assert!(pack.get("unknown").is_none());
    }

    #[test]
    fn load_on_garbage_bytes_yields_an_empty_pack_instead_of_panicking() {
        let pack = AssetPack::load(b"not a pack");
        assert!(pack.is_empty());
        assert!(pack.get("anything").is_none());
    }

    #[test]
    fn empty_builder_produces_a_pack_with_zero_entries() {
        let bytes = PackBuilder::new().build();
        let pack = AssetPack::load(Box::leak(bytes.into_boxed_slice()));
        assert!(pack.is_empty());
    }

    #[test]
    fn deflate_then_inflate_recovers_the_original_bytes() {
        let original = b"the quick brown fox jumps over the lazy dog".repeat(50);
        let packed = deflate(&original);
        assert!(packed.len() < original.len());
        assert_eq!(inflate(&packed), original);
    }
}
