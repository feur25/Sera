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
        }

        impl Default for $name {
            fn default() -> Self { $name::$default }
        }
    };
}
