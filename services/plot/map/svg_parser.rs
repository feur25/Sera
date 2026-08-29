pub struct CountryShape {
    pub id: String,
    pub name: String,
    pub polygons: Vec<Vec<[f32; 2]>>,
}

pub fn parse_world_svg(svg: &str) -> Vec<CountryShape> {
    parse_region_svg(svg, "id")
}

pub fn parse_region_svg(svg: &str, code_attr: &str) -> Vec<CountryShape> {
    let mut shapes = Vec::with_capacity(260);

    let mut pos = 0;
    let len = svg.len();

    while pos < len {
        if let Some(p) = find_substr(svg, pos, "<path") {
            pos = p + 5;
            let end = match find_substr(svg, pos, ">") {
                Some(e) => e,
                None => break,
            };
            let tag = &svg[pos..end];

            let id = match extract_attr(tag, code_attr) {
                Some(v) if v.len() == 2 && v.chars().all(|c| c.is_ascii_alphabetic()) => {
                    v.to_ascii_uppercase()
                }
                _ => {
                    pos = end + 1;
                    continue;
                }
            };
            let name = extract_attr(tag, "title").unwrap_or_default();
            let d = match extract_attr(tag, "d") {
                Some(v) => v,
                None => {
                    pos = end + 1;
                    continue;
                }
            };

            let polygons = parse_path_d(&d);
            if !polygons.is_empty() {
                shapes.push(CountryShape { id, name, polygons });
            }
            pos = end + 1;
        } else {
            break;
        }
    }

    shapes
}

pub fn parse_named_region_svg(svg: &str, code_attr: &str) -> Vec<CountryShape> {
    let mut shapes = Vec::with_capacity(64);

    let mut pos = 0;
    let len = svg.len();

    while pos < len {
        if let Some(p) = find_substr(svg, pos, "<path") {
            pos = p + 5;
            let end = match find_substr(svg, pos, ">") {
                Some(e) => e,
                None => break,
            };
            let tag = &svg[pos..end];

            let id = match extract_attr(tag, code_attr) {
                Some(v)
                    if !v.is_empty()
                        && v.chars().count() <= 64
                        && v.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_') =>
                {
                    v
                }
                _ => {
                    pos = end + 1;
                    continue;
                }
            };
            let name = extract_attr(tag, "title").unwrap_or_default();
            let d = match extract_attr(tag, "d") {
                Some(v) => v,
                None => {
                    pos = end + 1;
                    continue;
                }
            };

            let polygons = parse_path_d(&d);
            if !polygons.is_empty() {
                shapes.push(CountryShape { id, name, polygons });
            }
            pos = end + 1;
        } else {
            break;
        }
    }

    shapes
}

fn find_substr(s: &str, start: usize, needle: &str) -> Option<usize> {
    s[start..].find(needle).map(|i| start + i)
}

fn extract_attr(tag: &str, attr: &str) -> Option<String> {
    let pattern = format!("{}=\"", attr);
    let mut search_from = 0usize;
    loop {
        let rel = tag[search_from..].find(&pattern)?;
        let abs = search_from + rel;
        let at_boundary = abs == 0 || matches!(tag.as_bytes()[abs - 1], b' ' | b'\t' | b'\n' | b'\r');
        if at_boundary {
            let val_start = abs + pattern.len();
            let val_end = tag[val_start..].find('"')? + val_start;
            return Some(tag[val_start..val_end].to_string());
        }
        search_from = abs + 1;
        if search_from >= tag.len() {
            return None;
        }
    }
}

fn parse_path_d(d: &str) -> Vec<Vec<[f32; 2]>> {
    let mut polygons: Vec<Vec<[f32; 2]>> = Vec::new();
    let mut current: Vec<[f32; 2]> = Vec::new();
    let mut cx: f64 = 0.0;
    let mut cy: f64 = 0.0;
    let mut subpath_sx: f64 = 0.0;
    let mut subpath_sy: f64 = 0.0;

    let tokens = tokenize_path(d);
    let mut i = 0;

    while i < tokens.len() {
        match tokens[i] {
            Token::MAbs => {
                if current.len() >= 3 {
                    polygons.push(std::mem::take(&mut current));
                } else {
                    current.clear();
                }
                i += 1;
                if i + 1 < tokens.len() {
                    if let (Token::Num(x), Token::Num(y)) = (tokens[i], tokens[i + 1]) {
                        cx = x;
                        cy = y;
                        subpath_sx = cx;
                        subpath_sy = cy;
                        current.push([cx as f32, cy as f32]);
                        i += 2;
                    }
                }
            }
            Token::MRel => {
                if current.len() >= 3 {
                    polygons.push(std::mem::take(&mut current));
                } else {
                    current.clear();
                }
                i += 1;
                if i + 1 < tokens.len() {
                    if let (Token::Num(dx), Token::Num(dy)) = (tokens[i], tokens[i + 1]) {
                        cx += dx;
                        cy += dy;
                        subpath_sx = cx;
                        subpath_sy = cy;
                        current.push([cx as f32, cy as f32]);
                        i += 2;
                    }
                }
            }
            Token::LAbs => {
                i += 1;
            }
            Token::LRel => {
                i += 1;
            }
            Token::VAbs => {
                i += 1;
                while i < tokens.len() {
                    if let Token::Num(y) = tokens[i] {
                        cy = y;
                        current.push([cx as f32, cy as f32]);
                        i += 1;
                    } else {
                        break;
                    }
                }
            }
            Token::VRel => {
                i += 1;
                while i < tokens.len() {
                    if let Token::Num(dy) = tokens[i] {
                        cy += dy;
                        current.push([cx as f32, cy as f32]);
                        i += 1;
                    } else {
                        break;
                    }
                }
            }
            Token::HAbs => {
                i += 1;
                while i < tokens.len() {
                    if let Token::Num(x) = tokens[i] {
                        cx = x;
                        current.push([cx as f32, cy as f32]);
                        i += 1;
                    } else {
                        break;
                    }
                }
            }
            Token::HRel => {
                i += 1;
                while i < tokens.len() {
                    if let Token::Num(dx) = tokens[i] {
                        cx += dx;
                        current.push([cx as f32, cy as f32]);
                        i += 1;
                    } else {
                        break;
                    }
                }
            }
            Token::Z => {
                if current.len() >= 3 {
                    polygons.push(std::mem::take(&mut current));
                } else {
                    current.clear();
                }
                cx = subpath_sx;
                cy = subpath_sy;
                i += 1;
            }
            Token::Num(val) => {
                if i + 1 < tokens.len() {
                    if let Token::Num(val2) = tokens[i + 1] {
                        cx += val;
                        cy += val2;
                        current.push([cx as f32, cy as f32]);
                        i += 2;
                        continue;
                    }
                }
                i += 1;
            }
        }
    }

    if current.len() >= 3 {
        polygons.push(current);
    }

    polygons
}

#[derive(Debug, Clone, Copy)]
enum Token {
    MAbs,
    MRel,
    LAbs,
    LRel,
    VAbs,
    VRel,
    HAbs,
    HRel,
    Z,
    Num(f64),
}

fn tokenize_path(d: &str) -> Vec<Token> {
    let mut tokens = Vec::with_capacity(d.len() / 4);
    let bytes = d.as_bytes();
    let len = bytes.len();
    let mut i = 0;

    while i < len {
        let b = bytes[i];
        match b {
            b'M' => {
                tokens.push(Token::MAbs);
                i += 1;
            }
            b'm' => {
                tokens.push(Token::MRel);
                i += 1;
            }
            b'L' => {
                tokens.push(Token::LAbs);
                i += 1;
            }
            b'l' => {
                tokens.push(Token::LRel);
                i += 1;
            }
            b'V' => {
                tokens.push(Token::VAbs);
                i += 1;
            }
            b'v' => {
                tokens.push(Token::VRel);
                i += 1;
            }
            b'H' => {
                tokens.push(Token::HAbs);
                i += 1;
            }
            b'h' => {
                tokens.push(Token::HRel);
                i += 1;
            }
            b'z' | b'Z' => {
                tokens.push(Token::Z);
                i += 1;
            }
            b'-' | b'0'..=b'9' | b'.' => {
                let start = i;
                if b == b'-' {
                    i += 1;
                }
                let mut has_dot = false;
                let mut has_e = false;
                while i < len {
                    let c = bytes[i];
                    if c == b'.' && !has_dot && !has_e {
                        has_dot = true;
                        i += 1;
                    } else if (c == b'e' || c == b'E') && !has_e {
                        has_e = true;
                        i += 1;
                        if i < len && (bytes[i] == b'-' || bytes[i] == b'+') {
                            i += 1;
                        }
                    } else if c.is_ascii_digit() {
                        i += 1;
                    } else {
                        break;
                    }
                }
                if let Ok(val) = d[start..i].parse::<f64>() {
                    tokens.push(Token::Num(val));
                }
            }
            b',' | b' ' | b'\n' | b'\r' | b'\t' => {
                i += 1;
            }
            _ => {
                i += 1;
            }
        }
    }

    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_world_svg_keeps_extracting_uppercase_id_attributes() {
        let svg = r#"<path d="m 0,0 10,0 0,10 z" title="Testland" id="TL" />"#;
        let shapes = parse_world_svg(svg);
        assert_eq!(shapes.len(), 1);
        assert_eq!(shapes[0].id, "TL");
        assert_eq!(shapes[0].name, "Testland");
    }

    #[test]
    fn extract_attr_does_not_confuse_the_tail_of_id_with_the_d_attribute() {
        let svg = r#"<path id="Thueringen" d="m 0,0 10,0 0,10 z">"#;
        let shapes = parse_named_region_svg(svg, "id");
        assert_eq!(shapes.len(), 1);
        assert_eq!(shapes[0].id, "Thueringen");
        assert_eq!(shapes[0].polygons[0].len(), 3);
    }

    #[test]
    fn parse_named_region_svg_keeps_a_long_raw_identifier_verbatim() {
        let svg = r#"<path id="state-ac" d="m 0,0 10,0 0,10 z">"#;
        let shapes = parse_named_region_svg(svg, "id");
        assert_eq!(shapes.len(), 1);
        assert_eq!(shapes[0].id, "state-ac");
    }

    #[test]
    fn parse_named_region_svg_accepts_unicode_letters_in_the_identifier() {
        let svg = r#"<path id="Thüringen" d="m 0,0 10,0 0,10 z">"#;
        let shapes = parse_named_region_svg(svg, "id");
        assert_eq!(shapes.len(), 1);
        assert_eq!(shapes[0].id, "Thüringen");
    }

    #[test]
    fn parse_named_region_svg_skips_a_path_with_no_matching_attribute() {
        let svg = r#"<path d="m 0,0 10,0 0,10 z">"#;
        let shapes = parse_named_region_svg(svg, "id");
        assert!(shapes.is_empty());
    }

    #[test]
    fn parse_region_svg_reads_a_lowercase_class_as_the_code_and_uppercases_it() {
        let svg = r#"<path class="tl" d="m 0,0 10,0 0,10 z">"#;
        let shapes = parse_region_svg(svg, "class");
        assert_eq!(shapes.len(), 1);
        assert_eq!(shapes[0].id, "TL");
    }

    #[test]
    fn parse_region_svg_handles_a_plain_closing_bracket_without_a_self_closing_slash() {
        let svg = r#"<path class="ny" d="m 5,5 h 10 v 10 h -10 z">"#;
        let shapes = parse_region_svg(svg, "class");
        assert_eq!(shapes.len(), 1);
        assert_eq!(shapes[0].polygons[0].len(), 4);
    }

    #[test]
    fn parse_path_d_handles_horizontal_and_vertical_relative_lineto() {
        let polys = parse_path_d("m 0,0 h 10 v 5 h -10 z");
        assert_eq!(polys.len(), 1);
        assert_eq!(polys[0], vec![[0.0, 0.0], [10.0, 0.0], [10.0, 5.0], [0.0, 5.0]]);
    }

    #[test]
    fn parse_path_d_handles_repeated_implicit_h_and_v_coordinates() {
        let polys = parse_path_d("m 0,0 h 5 5 v 2 3 z");
        assert_eq!(polys[0], vec![[0.0, 0.0], [5.0, 0.0], [10.0, 0.0], [10.0, 2.0], [10.0, 5.0]]);
    }

    #[test]
    fn parse_path_d_handles_absolute_horizontal_and_vertical_lineto() {
        let polys = parse_path_d("m 3,3 H 9 V 1 z");
        assert_eq!(polys[0], vec![[3.0, 3.0], [9.0, 3.0], [9.0, 1.0]]);
    }

    #[test]
    fn parse_region_svg_on_the_real_usa_states_asset_finds_every_state_and_dc() {
        let svg = include_str!("../../../asset/usa_states.svg");
        let shapes = parse_region_svg(svg, "class");
        let ids: std::collections::HashSet<&str> = shapes.iter().map(|s| s.id.as_str()).collect();
        assert!(ids.len() >= 51, "expected at least 50 states + DC, found {}: {ids:?}", ids.len());
        for expect in ["CA", "NY", "TX", "AK", "HI", "DC"] {
            assert!(ids.contains(expect), "missing {expect} in parsed usa_states.svg: {ids:?}");
        }
        for shape in &shapes {
            assert!(!shape.polygons.is_empty(), "{} parsed with zero polygons", shape.id);
            assert!(shape.polygons.iter().all(|p| p.len() >= 3), "{} has a degenerate polygon", shape.id);
        }
    }
}
