pub mod usa_states;

use crate::plot::map::svg_parser::CountryShape;

pub struct RegionSetEntry {
    pub key: &'static str,
    pub aliases: &'static [&'static str],
    pub display_name: &'static str,
    pub lookup: fn(&str) -> Option<&'static CountryShape>,
    pub all: fn() -> &'static [CountryShape],
    pub groups: fn() -> &'static [(&'static str, &'static [&'static str])],
    pub normalize: fn(&CountryShape) -> Vec<Vec<[f32; 2]>>,
    pub svg_width: f32,
    pub svg_height: f32,
}

inventory::collect!(RegionSetEntry);

pub fn resolve(key: &str) -> Option<&'static RegionSetEntry> {
    let k = key.trim().to_lowercase();
    if k.is_empty() {
        return default_region_set();
    }
    inventory::iter::<RegionSetEntry>().find(|e| e.key == k || e.aliases.iter().any(|a| *a == k))
}

pub fn all_region_sets() -> impl Iterator<Item = &'static RegionSetEntry> {
    inventory::iter::<RegionSetEntry>()
}

pub fn default_region_set() -> Option<&'static RegionSetEntry> {
    inventory::iter::<RegionSetEntry>().find(|e| e.key == "world")
}

fn world_groups() -> &'static [(&'static str, &'static [&'static str])] {
    crate::plot::map::world_data::region_groups()
}

inventory::submit! {
    RegionSetEntry {
        key: "world",
        aliases: &["countries", "global", "earth"],
        display_name: "World (countries)",
        lookup: crate::plot::map::world_data::lookup_country,
        all: crate::plot::map::world_data::all_countries,
        groups: world_groups,
        normalize: crate::plot::map::world_data::normalized_polygons,
        svg_width: 1009.6727,
        svg_height: 665.96301,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolve_finds_world_by_key_and_by_alias() {
        assert!(resolve("world").is_some());
        assert!(resolve("global").is_some());
        assert!(resolve("EARTH").is_some());
    }

    #[test]
    fn resolve_returns_none_for_an_unknown_region_set() {
        assert!(resolve("narnia").is_none());
    }

    #[test]
    fn resolve_falls_back_to_world_for_an_empty_key() {
        let entry = resolve("").expect("empty key must fall back to world");
        assert_eq!(entry.key, "world");
    }

    #[test]
    fn every_registered_region_set_has_a_working_lookup_and_non_empty_catalog() {
        for entry in all_region_sets() {
            let shapes = (entry.all)();
            assert!(!shapes.is_empty(), "{} registered with zero shapes", entry.key);
            let first = &shapes[0];
            let found = (entry.lookup)(&first.id);
            assert!(found.is_some(), "{}'s own lookup fn cannot find its own first entry {}", entry.key, first.id);
        }
    }

    #[test]
    fn no_two_region_sets_share_the_same_key() {
        let keys: Vec<&str> = all_region_sets().map(|e| e.key).collect();
        let unique: std::collections::HashSet<&str> = keys.iter().copied().collect();
        assert_eq!(keys.len(), unique.len(), "duplicate region set keys registered: {keys:?}");
    }
}
