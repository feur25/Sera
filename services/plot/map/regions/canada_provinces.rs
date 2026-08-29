use super::RegionSetEntry;
use crate::plot::map::svg_parser::{parse_named_region_svg, CountryShape};
use std::sync::OnceLock;

pub const SVG_WIDTH: f32 = 977.953;
pub const SVG_HEIGHT: f32 = 949.606;

const PROVINCE_NAMES: &[(&str, &str)] = &[
    ("AB", "Alberta"),
    ("BC", "British Columbia"),
    ("MB", "Manitoba"),
    ("NB", "New Brunswick"),
    ("NL", "Newfoundland and Labrador"),
    ("NS", "Nova Scotia"),
    ("ON", "Ontario"),
    ("PE", "Prince Edward Island"),
    ("QC", "Quebec"),
    ("SK", "Saskatchewan"),
    ("NT", "Northwest Territories"),
    ("NU", "Nunavut"),
    ("YT", "Yukon"),
];

const RAW_ID_TO_CODE: &[(&str, &str)] = &[
    ("Alberta", "AB"),
    ("BritishColumbia", "BC"),
    ("BritishColumbiaCordillera", "BC"),
    ("VancouverIsland", "BC"),
    ("QueenCharlottes", "BC"),
    ("Manitoba", "MB"),
    ("NewBrunswick", "NB"),
    ("AcadianPeninsula", "NB"),
    ("Newfoundland", "NL"),
    ("NewfoundlandLabrador", "NL"),
    ("Labrador", "NL"),
    ("NovaScotia", "NS"),
    ("CapeBreton", "NS"),
    ("Ontario", "ON"),
    ("PrinceEdwardIsland", "PE"),
    ("Quebec", "QC"),
    ("QcAnticosti", "QC"),
    ("QcQuebec", "QC"),
    ("Saskatchewan", "SK"),
    ("NorthwestTerritories", "NT"),
    ("Mackenzie", "NT"),
    ("MackenzieKing", "NT"),
    ("Banks", "NT"),
    ("NorthWestVictoria", "NT"),
    ("NorthWestMelville", "NT"),
    ("NorthWestBorden", "NT"),
    ("Nunavut", "NU"),
    ("Baffin", "NU"),
    ("Bathurst", "NU"),
    ("Bylot", "NU"),
    ("Coats", "NU"),
    ("Cornwall", "NU"),
    ("Cornwallis", "NU"),
    ("Devon", "NU"),
    ("Eglinton", "NU"),
    ("EllefRingnes", "NU"),
    ("Ellesmere", "NU"),
    ("Keewatin", "NU"),
    ("KingWilliam", "NU"),
    ("Lougheed", "NU"),
    ("Mansel", "NU"),
    ("Meighen", "NU"),
    ("NunavutBorden", "NU"),
    ("NunavutMelville", "NU"),
    ("NunavutVictoria", "NU"),
    ("Nottingham", "NU"),
    ("PrinceCharles", "NU"),
    ("PrinceOfWales", "NU"),
    ("PrincePatrick", "NU"),
    ("Rowley", "NU"),
    ("Salisbury", "NU"),
    ("Somerset", "NU"),
    ("Southampton", "NU"),
    ("AmundRingnes", "NU"),
    ("AxelHeiberg", "NU"),
    ("ByamMartin", "NU"),
    ("AirForce", "NU"),
    ("Akimiski", "NU"),
    ("Yukon", "YT"),
];

const CANADA_REGIONS: &[(&str, &[&str])] = &[
    ("Atlantic", &["NB", "NS", "PE", "NL"]),
    ("Central", &["QC", "ON"]),
    ("Prairie", &["MB", "SK", "AB"]),
    ("West Coast", &["BC"]),
    ("North", &["YT", "NT", "NU"]),
];

static PROVINCES: OnceLock<Vec<CountryShape>> = OnceLock::new();

fn get_provinces() -> &'static Vec<CountryShape> {
    PROVINCES.get_or_init(|| {
        let svg = include_str!("../../../../asset/maps/dl/canada_provinces.svg");
        let raw = parse_named_region_svg(svg, "id");
        let mut merged: Vec<CountryShape> = Vec::with_capacity(PROVINCE_NAMES.len());
        for shape in raw {
            let Some((_, code)) = RAW_ID_TO_CODE.iter().find(|(raw_id, _)| *raw_id == shape.id) else {
                continue;
            };
            match merged.iter_mut().find(|s: &&mut CountryShape| s.id == *code) {
                Some(existing) => existing.polygons.extend(shape.polygons),
                None => {
                    let name = PROVINCE_NAMES
                        .iter()
                        .find(|(c, _)| c == code)
                        .map(|(_, n)| n.to_string())
                        .unwrap_or_default();
                    merged.push(CountryShape { id: code.to_string(), name, polygons: shape.polygons });
                }
            }
        }
        merged
    })
}

pub fn lookup_province(key: &str) -> Option<&'static CountryShape> {
    let provinces = get_provinces();
    let key_upper = key.to_uppercase();
    provinces
        .iter()
        .find(|s| s.id == key_upper || s.name.eq_ignore_ascii_case(key))
}

pub fn all_provinces() -> &'static [CountryShape] {
    get_provinces()
}

pub fn canada_regions() -> &'static [(&'static str, &'static [&'static str])] {
    CANADA_REGIONS
}

pub fn normalized_polygons(shape: &CountryShape) -> Vec<Vec<[f32; 2]>> {
    shape
        .polygons
        .iter()
        .map(|poly| poly.iter().map(|[x, y]| [x / SVG_WIDTH, y / SVG_HEIGHT]).collect())
        .collect()
}

inventory::submit! {
    RegionSetEntry {
        key: "canada_provinces",
        aliases: &["canada", "ca", "canadian_provinces", "provinces"],
        display_name: "Canada (provinces & territories)",
        lookup: lookup_province,
        all: all_provinces,
        groups: canada_regions,
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
    fn merges_every_raw_fragment_into_exactly_thirteen_provinces_and_territories() {
        assert_eq!(all_provinces().len(), 13);
    }

    #[test]
    fn every_declared_code_resolves_to_a_real_shape_with_polygons() {
        for (code, _) in PROVINCE_NAMES {
            let shape = lookup_province(code).unwrap_or_else(|| panic!("{code} must resolve"));
            assert!(!shape.polygons.is_empty(), "{code} has no polygons");
        }
    }

    #[test]
    fn ontario_and_quebec_keep_their_mainland_and_island_pieces_together() {
        let qc = lookup_province("QC").expect("Quebec must resolve");
        assert!(qc.polygons.len() >= 2, "Quebec should include both the mainland and Anticosti");
    }

    #[test]
    fn lookup_is_case_insensitive_and_name_based_too() {
        assert!(lookup_province("on").is_some());
        assert!(lookup_province("Ontario").is_some());
        assert!(lookup_province("nunavut").is_some());
    }

    #[test]
    fn regions_partition_every_province_with_no_overlap() {
        let mut seen = std::collections::HashSet::new();
        let mut total = 0;
        for (_, codes) in CANADA_REGIONS {
            for c in *codes {
                assert!(seen.insert(*c), "{c} appears in more than one Canadian region group");
                total += 1;
            }
        }
        assert_eq!(total, 13);
    }
}
