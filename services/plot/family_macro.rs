pub struct ChartFamilyEntry {
    pub name: &'static str,
    pub keys_and_aliases: fn() -> &'static [(&'static str, &'static [&'static str])],
    pub default_key: fn() -> &'static str,
}

inventory::collect!(ChartFamilyEntry);

pub fn chart_families() -> impl Iterator<Item = &'static ChartFamilyEntry> {
    inventory::iter::<ChartFamilyEntry>()
}

#[macro_export]
macro_rules! plot_family {
    (
        $vis:vis enum $name:ident default $default:ident $(family $family:literal)? {
            $( $variant:ident => $key:literal $( | $alias:literal )* ),* $(,)?
        }
    ) => {
        #[derive(Clone, Copy, PartialEq, Eq, Debug)]
        $vis enum $name {
            $( $variant ),*
        }

        impl $name {
            pub fn from_str(s: &str) -> Self {
                let s = s.strip_prefix("fr_").or_else(|| s.strip_prefix("en_"))
                    .or_else(|| s.strip_prefix("fr-")).or_else(|| s.strip_prefix("en-"))
                    .unwrap_or(s);
                match s {
                    $( $key $( | $alias )* => $name::$variant, )*
                    _ => $name::$default,
                }
            }

            pub fn name(self) -> &'static str {
                match self {
                    $( $name::$variant => $key ),*
                }
            }

            pub fn all() -> &'static [$name] {
                &[ $( $name::$variant ),* ]
            }

            pub fn keys_and_aliases() -> &'static [(&'static str, &'static [&'static str])] {
                &[ $( ($key, &[ $( $alias ),* ]) ),* ]
            }

            pub fn default_key() -> &'static str {
                $name::$default.name()
            }
        }

        impl Default for $name {
            fn default() -> Self { $name::$default }
        }

        $(
            inventory::submit! {
                $crate::plot::family_macro::ChartFamilyEntry {
                    name: $family,
                    keys_and_aliases: $name::keys_and_aliases,
                    default_key: $name::default_key,
                }
            }
        )?
    };
}

#[cfg(test)]
mod inventory_tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn no_two_plot_family_invocations_registered_the_same_family_name() {
        let names: Vec<&str> = chart_families().map(|e| e.name).collect();
        let unique: HashSet<&str> = names.iter().copied().collect();
        assert_eq!(
            names.len(),
            unique.len(),
            "duplicate chart family names registered via plot_family!'s `family` clause: {names:?}"
        );
    }

    #[test]
    fn every_registered_family_has_at_least_one_variant_and_a_valid_default_key() {
        for entry in chart_families() {
            let keys = (entry.keys_and_aliases)();
            assert!(!keys.is_empty(), "family '{}' has zero variants", entry.name);
            let default_key = (entry.default_key)();
            assert!(
                keys.iter().any(|(k, _)| *k == default_key),
                "family '{}' default_key '{default_key}' is not one of its own variant keys",
                entry.name
            );
        }
    }

    #[test]
    fn at_least_the_known_2d_chart_families_are_registered() {
        let names: HashSet<&str> = chart_families().map(|e| e.name).collect();
        for expected in [
            "bar", "line", "scatter", "bubble", "histogram", "heatmap", "pie", "boxplot",
            "violin", "kde", "ridgeline", "radar", "slope", "funnel", "sunburst", "waterfall",
            "treemap", "candlestick", "dumbbell", "bullet", "gauge", "lollipop", "parallel",
            "wordcloud", "sankey", "chord", "circle_pack", "arc_diagram", "dendrogram", "venn",
            "correlogram", "hive", "pulse", "orbita", "eventplot", "gantt", "hexbin", "icicle",
            "parcats", "scatterternary", "splom", "stackplot", "plot_web",
        ] {
            assert!(
                names.contains(expected),
                "expected chart family '{expected}' not registered -- check its plot_family! \
                 invocation still has a `family \"...\"` clause"
            );
        }
    }
}
