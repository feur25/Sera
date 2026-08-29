pub mod africa;
pub mod asia;
pub mod brazil_states;
pub mod europe;
pub mod generic;
pub mod germany_states;
pub mod north_america;
pub mod oceania;
pub mod south_america;
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
    pub to_latlon: Option<fn(f32, f32) -> (f64, f64)>,
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

pub fn group_codes(region: &RegionSetEntry, group: &str) -> Option<&'static [&'static str]> {
    let g = group.trim().to_lowercase();
    if g.is_empty() {
        return None;
    }
    (region.groups)()
        .iter()
        .find(|(name, _)| name.to_lowercase() == g)
        .map(|(_, codes)| *codes)
}

pub fn centroid_of(region: &RegionSetEntry, shape: &CountryShape) -> [f32; 2] {
    let polys = (region.normalize)(shape);
    let mut best: Option<&Vec<[f32; 2]>> = None;
    let mut best_len = 0;
    for poly in &polys {
        if poly.len() > best_len {
            best_len = poly.len();
            best = Some(poly);
        }
    }
    match best {
        Some(poly) if !poly.is_empty() => {
            let mut sx = 0.0f32;
            let mut sy = 0.0f32;
            for p in poly {
                sx += p[0];
                sy += p[1];
            }
            [sx / poly.len() as f32, sy / poly.len() as f32]
        }
        _ => [0.5, 0.5],
    }
}

pub fn shapes_in_group(region: &RegionSetEntry, group: &str) -> Vec<&'static CountryShape> {
    match group_codes(region, group) {
        Some(codes) => (region.all)()
            .iter()
            .filter(|shape| codes.contains(&shape.id.as_str()))
            .collect(),
        None => (region.all)().iter().collect(),
    }
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
        to_latlon: Some(crate::plot::map::world_data::svg_to_latlon),
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
    fn shapes_in_group_restricts_usa_states_to_the_west_census_region() {
        let usa = resolve("usa_states").expect("usa_states must be registered");
        let west = shapes_in_group(usa, "West");
        assert_eq!(west.len(), 13);
        assert!(west.iter().any(|s| s.id == "CA"));
        assert!(!west.iter().any(|s| s.id == "NY"));
    }

    #[test]
    fn shapes_in_group_is_case_insensitive() {
        let usa = resolve("usa_states").expect("usa_states must be registered");
        assert_eq!(shapes_in_group(usa, "west").len(), shapes_in_group(usa, "WEST").len());
    }

    #[test]
    fn shapes_in_group_returns_everything_for_an_unknown_group_name() {
        let usa = resolve("usa_states").expect("usa_states must be registered");
        assert_eq!(shapes_in_group(usa, "atlantis").len(), (usa.all)().len());
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
    fn map_regions_json_lists_every_registered_region_set_with_its_own_shape_count() {
        let json = crate::map_regions();
        let regions = json["regions"].as_array().expect("regions must be an array");
        assert_eq!(regions.len(), all_region_sets().count());
        let usa = regions.iter().find(|r| r["key"] == "usa_states").expect("usa_states must be listed");
        assert_eq!(usa["count"].as_u64(), Some(51));
        assert!(usa["groups"].as_array().unwrap().contains(&serde_json::Value::String("West".to_string())));
    }

    #[test]
    fn no_two_region_sets_share_the_same_key() {
        let keys: Vec<&str> = all_region_sets().map(|e| e.key).collect();
        let unique: std::collections::HashSet<&str> = keys.iter().copied().collect();
        assert_eq!(keys.len(), unique.len(), "duplicate region set keys registered: {keys:?}");
    }
}
