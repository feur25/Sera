pub fn to_snake_case(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 4);
    let mut prev_lower_or_digit = false;
    for c in s.chars() {
        if c.is_uppercase() {
            if prev_lower_or_digit {
                out.push('_');
            }
            for lc in c.to_lowercase() {
                out.push(lc);
            }
            prev_lower_or_digit = false;
        } else {
            out.push(c);
            prev_lower_or_digit = c.is_lowercase() || c.is_numeric();
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::to_snake_case;

    #[test]
    fn converts_pascal_case() {
        assert_eq!(to_snake_case("Grid"), "grid");
        assert_eq!(to_snake_case("GridX"), "grid_x");
        assert_eq!(to_snake_case("ShowGrid"), "show_grid");
        assert_eq!(to_snake_case("SetBg"), "set_bg");
    }

    #[test]
    fn converts_camel_case() {
        assert_eq!(to_snake_case("gridX"), "grid_x");
        assert_eq!(to_snake_case("buildBar"), "build_bar");
    }

    #[test]
    fn leaves_already_snake_case_alone() {
        assert_eq!(to_snake_case("grid"), "grid");
        assert_eq!(to_snake_case("show_grid"), "show_grid");
        assert_eq!(to_snake_case("build_bar_chart"), "build_bar_chart");
    }

    #[test]
    fn handles_consecutive_uppercase_and_digits() {
        assert_eq!(to_snake_case("KDEChart"), "kdechart");
        assert_eq!(to_snake_case("bar3d"), "bar3d");
        assert_eq!(to_snake_case("Bar3D"), "bar3_d");
    }
}
