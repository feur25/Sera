#[macro_export]
macro_rules! declare_named_region_set {
    (
        $mod_name:ident,
        svg = $svg_file:literal,
        width = $width:expr,
        height = $height:expr,
        key = $key:literal,
        aliases = $aliases:expr,
        display = $display:literal,
        groups = $groups:expr
    ) => {
        pub mod $mod_name {
            use $crate::plot::map::regions::RegionSetEntry;
            use $crate::plot::map::svg_parser::{parse_named_region_svg, CountryShape};
            use std::sync::OnceLock;

            pub const SVG_WIDTH: f32 = $width;
            pub const SVG_HEIGHT: f32 = $height;

            static SHAPES: OnceLock<Vec<CountryShape>> = OnceLock::new();

            fn get_shapes() -> &'static Vec<CountryShape> {
                SHAPES.get_or_init(|| {
                    let svg = $crate::plot::map::regions::pack::map_asset($svg_file);
                    parse_named_region_svg(&svg, "id")
                })
            }

            pub fn lookup(key: &str) -> Option<&'static CountryShape> {
                $crate::plot::map::regions::find_by_id_or_name(get_shapes(), key)
            }

            pub fn all() -> &'static [CountryShape] {
                get_shapes()
            }

            pub fn groups() -> &'static [(&'static str, &'static [&'static str])] {
                $groups
            }

            pub fn normalized_polygons(shape: &CountryShape) -> Vec<Vec<[f32; 2]>> {
                $crate::plot::map::regions::normalize_with(shape, SVG_WIDTH, SVG_HEIGHT)
            }

            inventory::submit! {
                RegionSetEntry {
                    key: $key,
                    aliases: $aliases,
                    display_name: $display,
                    lookup,
                    all,
                    groups,
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
                fn has_at_least_one_shape_and_every_shape_has_polygons() {
                    let shapes = all();
                    assert!(!shapes.is_empty(), "{} registered with zero shapes", $key);
                    assert!(shapes.iter().all(|s| !s.polygons.is_empty()));
                }

                #[test]
                fn every_shape_has_a_non_empty_id_and_name() {
                    for s in all() {
                        assert!(!s.id.is_empty());
                        assert!(!s.name.is_empty());
                    }
                }

                #[test]
                fn ids_are_unique() {
                    let shapes = all();
                    let ids: std::collections::HashSet<&str> = shapes.iter().map(|s| s.id.as_str()).collect();
                    assert_eq!(ids.len(), shapes.len(), "{} has duplicate ids", $key);
                }

                #[test]
                fn lookup_resolves_every_shape_by_its_own_id() {
                    for s in all() {
                        assert!(lookup(&s.id).is_some(), "{} cannot resolve its own id {}", $key, s.id);
                    }
                }
            }
        }
    };
}
