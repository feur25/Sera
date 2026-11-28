use super::RegionSetEntry;
use crate::plot::map::svg_parser::{parse_named_region_svg, CountryShape};
use std::sync::OnceLock;

pub const SVG_WIDTH: f32 = 591.504;
pub const SVG_HEIGHT: f32 = 800.504;

const RAW_ID_TO_CODE: &[(&str, &str, &str)] = &[
    ("Baden__x26__Württemberg", "BW", "Baden-Württemberg"),
    ("Bayern", "BY", "Bayern"),
    ("Berlin", "BE", "Berlin"),
    ("Brandenburg", "BB", "Brandenburg"),
    ("Bremen", "HB", "Bremen"),
    ("Hamburg", "HH", "Hamburg"),
    ("Hessen", "HE", "Hessen"),
    ("Mecklenburg-Vorpommern", "MV", "Mecklenburg-Vorpommern"),
    ("Niedersachsen", "NI", "Niedersachsen"),
    ("Nordrhein-Westfalen", "NW", "Nordrhein-Westfalen"),
    ("Rheinland-Pfalz", "RP", "Rheinland-Pfalz"),
    ("Saarland", "SL", "Saarland"),
    ("Sachsen", "SN", "Sachsen"),
    ("Sachsen-Anhalt", "ST", "Sachsen-Anhalt"),
    ("Schleswig-Holstein", "SH", "Schleswig-Holstein"),
    ("Thüringen", "TH", "Thüringen"),
];

const REGIONS_GROUP: &[(&str, &[&str])] = &[
    ("North", &["SH", "HH", "HB", "NI", "MV"]),
    ("East", &["BE", "BB", "SN", "ST", "TH"]),
    ("West", &["NW", "HE", "RP", "SL"]),
    ("South", &["BW", "BY"]),
];

static STATES: OnceLock<Vec<CountryShape>> = OnceLock::new();

fn get_states() -> &'static Vec<CountryShape> {
    STATES.get_or_init(|| {
        let svg = super::pack::map_asset("europe/germany_states");
        let raw = parse_named_region_svg(&svg, "id");
        raw.into_iter()
            .filter_map(|shape| {
                RAW_ID_TO_CODE
                    .iter()
                    .find(|(raw_id, _, _)| *raw_id == shape.id)
                    .map(|(_, code, name)| CountryShape {
                        id: code.to_string(),
                        name: name.to_string(),
                        ..shape
                    })
            })
            .collect()
    })
}

pub fn lookup_state(key: &str) -> Option<&'static CountryShape> {
    super::find_by_id_or_name(get_states(), key)
}

pub fn all_states() -> &'static [CountryShape] {
    get_states()
}

pub fn german_regions() -> &'static [(&'static str, &'static [&'static str])] {
    REGIONS_GROUP
}

pub fn normalized_polygons(shape: &CountryShape) -> Vec<Vec<[f32; 2]>> {
    super::normalize_with(shape, SVG_WIDTH, SVG_HEIGHT)
}

inventory::submit! {
    RegionSetEntry {
        key: "germany_states",
        aliases: &["germany", "de", "deutschland", "bundeslander", "bundesländer", "german_states"],
        display_name: "Germany (states)",
        lookup: lookup_state,
        all: all_states,
        groups: german_regions,
        normalize: normalized_polygons,
        svg_width: SVG_WIDTH,
        svg_height: SVG_HEIGHT,
        to_latlon: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_all_sixteen_german_states() {
        assert_eq!(all_states().len(), 16);
    }

    #[test]
    fn every_declared_code_resolves_to_a_real_shape_with_polygons() {
        for (_, code, _) in RAW_ID_TO_CODE {
            let shape = lookup_state(code).unwrap_or_else(|| panic!("{code} must resolve"));
            assert!(!shape.polygons.is_empty(), "{code} has no polygons");
        }
    }

    #[test]
    fn lookup_is_case_insensitive_and_name_based_too() {
        assert!(lookup_state("by").is_some());
        assert!(lookup_state("Bayern").is_some());
        assert!(lookup_state("bayern").is_some());
    }

    #[test]
    fn regions_partition_every_state_with_no_overlap() {
        let mut seen = std::collections::HashSet::new();
        let mut total = 0;
        for (_, codes) in REGIONS_GROUP {
            for c in *codes {
                assert!(seen.insert(*c), "{c} appears in more than one German region group");
                total += 1;
            }
        }
        assert_eq!(total, 16);
    }
}
