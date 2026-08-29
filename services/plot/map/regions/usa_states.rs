use super::RegionSetEntry;
use crate::plot::map::svg_parser::{parse_region_svg, CountryShape};
use std::sync::OnceLock;

const STATE_NAMES: &[(&str, &str)] = &[
    ("AL", "Alabama"), ("AK", "Alaska"), ("AZ", "Arizona"), ("AR", "Arkansas"),
    ("CA", "California"), ("CO", "Colorado"), ("CT", "Connecticut"), ("DE", "Delaware"),
    ("FL", "Florida"), ("GA", "Georgia"), ("HI", "Hawaii"), ("ID", "Idaho"),
    ("IL", "Illinois"), ("IN", "Indiana"), ("IA", "Iowa"), ("KS", "Kansas"),
    ("KY", "Kentucky"), ("LA", "Louisiana"), ("ME", "Maine"), ("MD", "Maryland"),
    ("MA", "Massachusetts"), ("MI", "Michigan"), ("MN", "Minnesota"), ("MS", "Mississippi"),
    ("MO", "Missouri"), ("MT", "Montana"), ("NE", "Nebraska"), ("NV", "Nevada"),
    ("NH", "New Hampshire"), ("NJ", "New Jersey"), ("NM", "New Mexico"), ("NY", "New York"),
    ("NC", "North Carolina"), ("ND", "North Dakota"), ("OH", "Ohio"), ("OK", "Oklahoma"),
    ("OR", "Oregon"), ("PA", "Pennsylvania"), ("RI", "Rhode Island"), ("SC", "South Carolina"),
    ("SD", "South Dakota"), ("TN", "Tennessee"), ("TX", "Texas"), ("UT", "Utah"),
    ("VT", "Vermont"), ("VA", "Virginia"), ("WA", "Washington"), ("WV", "West Virginia"),
    ("WI", "Wisconsin"), ("WY", "Wyoming"), ("DC", "District of Columbia"),
];

const CENSUS_REGIONS: &[(&str, &[&str])] = &[
    ("Northeast", &["CT", "ME", "MA", "NH", "RI", "VT", "NJ", "NY", "PA"]),
    ("Midwest", &["IL", "IN", "MI", "OH", "WI", "IA", "KS", "MN", "MO", "NE", "ND", "SD"]),
    ("South", &["DE", "FL", "GA", "MD", "NC", "SC", "VA", "DC", "WV", "AL", "KY", "MS", "TN", "AR", "LA", "OK", "TX"]),
    ("West", &["AZ", "CO", "ID", "MT", "NV", "NM", "UT", "WY", "AK", "CA", "HI", "OR", "WA"]),
];

static STATES: OnceLock<Vec<CountryShape>> = OnceLock::new();

fn state_name(code: &str) -> &'static str {
    STATE_NAMES
        .iter()
        .find(|(c, _)| *c == code)
        .map(|(_, n)| *n)
        .unwrap_or("")
}

fn get_states() -> &'static Vec<CountryShape> {
    STATES.get_or_init(|| {
        let svg = super::pack::map_asset("north-america/usa_states");
        parse_region_svg(&svg, "class")
            .into_iter()
            .map(|shape| CountryShape {
                name: state_name(&shape.id).to_string(),
                ..shape
            })
            .collect()
    })
}

pub fn lookup_state(key: &str) -> Option<&'static CountryShape> {
    let states = get_states();
    let key_upper = key.to_uppercase();
    states
        .iter()
        .find(|s| s.id == key_upper || s.name.eq_ignore_ascii_case(key))
}

pub fn all_states() -> &'static [CountryShape] {
    get_states()
}

pub fn census_regions() -> &'static [(&'static str, &'static [&'static str])] {
    CENSUS_REGIONS
}

const SVG_WIDTH: f32 = 959.0;
const SVG_HEIGHT: f32 = 593.0;

pub fn normalized_polygons(shape: &CountryShape) -> Vec<Vec<[f32; 2]>> {
    shape
        .polygons
        .iter()
        .map(|poly| poly.iter().map(|[x, y]| [x / SVG_WIDTH, y / SVG_HEIGHT]).collect())
        .collect()
}

inventory::submit! {
    RegionSetEntry {
        key: "usa_states",
        aliases: &["usa", "us", "us_states", "united_states", "states"],
        display_name: "United States (states)",
        lookup: lookup_state,
        all: all_states,
        groups: census_regions,
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
    fn all_states_returns_fifty_states_plus_dc() {
        assert_eq!(all_states().len(), 51);
    }

    #[test]
    fn lookup_state_resolves_by_postal_code_case_insensitively() {
        assert_eq!(lookup_state("ca").unwrap().name, "California");
        assert_eq!(lookup_state("TX").unwrap().name, "Texas");
    }

    #[test]
    fn lookup_state_resolves_by_full_name_case_insensitively() {
        assert_eq!(lookup_state("new york").unwrap().id, "NY");
    }

    #[test]
    fn lookup_state_returns_none_for_a_non_state() {
        assert!(lookup_state("ZZ").is_none());
    }

    #[test]
    fn every_state_has_a_backfilled_name_and_at_least_one_polygon() {
        for state in all_states() {
            assert!(!state.name.is_empty(), "{} has no backfilled name", state.id);
            assert!(!state.polygons.is_empty(), "{} has no polygons", state.id);
        }
    }

    #[test]
    fn census_regions_cover_every_state_and_dc_exactly_once() {
        let mut seen: Vec<&str> = Vec::new();
        for (_, codes) in census_regions() {
            for code in *codes {
                assert!(!seen.contains(code), "{code} appears in more than one census region");
                seen.push(code);
            }
        }
        assert_eq!(seen.len(), 51, "census regions must cover exactly 50 states + DC, covered {}", seen.len());
        for (code, _) in STATE_NAMES {
            assert!(seen.contains(code), "{code} is missing from every census region");
        }
    }

    #[test]
    fn usa_states_region_set_is_registered_and_resolvable() {
        let entry = super::super::resolve("usa").expect("usa alias must resolve");
        assert_eq!(entry.key, "usa_states");
        assert_eq!((entry.all)().len(), 51);
    }
}
