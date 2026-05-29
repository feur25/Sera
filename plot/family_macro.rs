#[macro_export]
macro_rules! plot_family {
    (
        $vis:vis enum $name:ident default $default:ident {
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
    };
}


