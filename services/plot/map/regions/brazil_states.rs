use super::RegionSetEntry;
use crate::plot::map::svg_parser::{parse_named_region_svg, CountryShape};
use std::sync::OnceLock;

pub const SVG_WIDTH: f32 = 220000.0;
pub const SVG_HEIGHT: f32 = 194010.0;

const STATE_NAMES: &[(&str, &str)] = &[
    ("AC", "Acre"),
    ("AL", "Alagoas"),
    ("AM", "Amazonas"),
    ("AP", "Amapá"),
    ("BA", "Bahia"),
    ("CE", "Ceará"),
    ("DF", "Distrito Federal"),
    ("ES", "Espírito Santo"),
    ("GO", "Goiás"),
    ("MA", "Maranhão"),
    ("MG", "Minas Gerais"),
    ("MS", "Mato Grosso do Sul"),
    ("MT", "Mato Grosso"),
    ("PA", "Pará"),
    ("PB", "Paraíba"),
    ("PE", "Pernambuco"),
    ("PI", "Piauí"),
    ("PR", "Paraná"),
    ("RJ", "Rio de Janeiro"),
    ("RN", "Rio Grande do Norte"),
    ("RO", "Rondônia"),
    ("RR", "Roraima"),
    ("RS", "Rio Grande do Sul"),
    ("SC", "Santa Catarina"),
    ("SE", "Sergipe"),
    ("SP", "São Paulo"),
    ("TO", "Tocantins"),
];

const BRAZIL_REGIONS: &[(&str, &[&str])] = &[
    ("Norte", &["AC", "AP", "AM", "PA", "RO", "RR", "TO"]),
    ("Nordeste", &["AL", "BA", "CE", "MA", "PB", "PE", "PI", "RN", "SE"]),
    ("Centro-Oeste", &["DF", "GO", "MT", "MS"]),
    ("Sudeste", &["ES", "MG", "RJ", "SP"]),
    ("Sul", &["PR", "RS", "SC"]),
];

static STATES: OnceLock<Vec<CountryShape>> = OnceLock::new();

fn get_states() -> &'static Vec<CountryShape> {
    STATES.get_or_init(|| {
        let svg = super::pack::map_asset("south-america/brazil_states");
        let raw = parse_named_region_svg(&svg, "id");
        raw.into_iter()
            .filter_map(|shape| {
                let code = shape.id.strip_prefix("state-")?.to_uppercase();
                let name = STATE_NAMES
                    .iter()
                    .find(|(c, _)| *c == code)
                    .map(|(_, n)| n.to_string())
                    .unwrap_or_default();
                if name.is_empty() {
                    return None;
                }
                Some(CountryShape { id: code, name, ..shape })
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

pub fn brazil_regions() -> &'static [(&'static str, &'static [&'static str])] {
    BRAZIL_REGIONS
}

pub fn normalized_polygons(shape: &CountryShape) -> Vec<Vec<[f32; 2]>> {
    super::normalize_with(shape, SVG_WIDTH, SVG_HEIGHT)
}

inventory::submit! {
    RegionSetEntry {
        key: "brazil_states",
        aliases: &["brazil", "br", "brasil", "brazil_uf", "estados_brasil"],
        display_name: "Brazil (states)",
        lookup: lookup_state,
        all: all_states,
        groups: brazil_regions,
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
    fn parses_all_twenty_seven_brazilian_states() {
        assert_eq!(all_states().len(), 27);
    }

    #[test]
    fn every_declared_code_resolves_to_a_real_shape_with_polygons() {
        for (code, _) in STATE_NAMES {
            let shape = lookup_state(code).unwrap_or_else(|| panic!("{code} must resolve"));
            assert!(!shape.polygons.is_empty(), "{code} has no polygons");
        }
    }

    #[test]
    fn lookup_is_case_insensitive_and_name_based_too() {
        assert!(lookup_state("sp").is_some());
        assert!(lookup_state("Sao Paulo").is_none());
        assert!(lookup_state("São Paulo").is_some());
    }

    #[test]
    fn regions_partition_every_state_with_no_overlap() {
        let mut seen = std::collections::HashSet::new();
        let mut total = 0;
        for (_, codes) in BRAZIL_REGIONS {
            for c in *codes {
                assert!(seen.insert(*c), "{c} appears in more than one Brazilian region group");
                total += 1;
            }
        }
        assert_eq!(total, 27);
    }
}
