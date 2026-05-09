pub struct CountryShape {
    pub id: String,
    pub name: String,
    pub polygons: Vec<Vec<[f32; 2]>>,
}

pub fn parse_world_svg(svg: &str) -> Vec<CountryShape> {
    let mut countries = Vec::with_capacity(260);

    let mut pos = 0;
    let bytes = svg.as_bytes();
    let len = bytes.len();

    while pos < len {
        if let Some(p) = find_substr(svg, pos, "<path") {
            pos = p + 5;
            let end = match find_substr(svg, pos, "/>") {
                Some(e) => e,
                None => break,
            };
            let tag = &svg[pos..end];

            let id = match extract_attr(tag, "id") {
                Some(v) if v.len() == 2 && v.chars().all(|c| c.is_ascii_uppercase()) => v,
                _ => { pos = end + 2; continue; }
            };
            let name = extract_attr(tag, "title").unwrap_or_default();
            let d = match extract_attr(tag, "d") {
                Some(v) => v,
                None => { pos = end + 2; continue; }
            };

            let polygons = parse_path_d(&d);
            if !polygons.is_empty() {
                countries.push(CountryShape { id, name, polygons });
            }
            pos = end + 2;
        } else {
            break;
        }
    }

    countries
}

fn find_substr(s: &str, start: usize, needle: &str) -> Option<usize> {
    s[start..].find(needle).map(|i| start + i)
}

fn extract_attr(tag: &str, attr: &str) -> Option<String> {
    let pattern = format!("{}=\"", attr);
    let start = tag.find(&pattern)?;
    let val_start = start + pattern.len();
    let val_end = tag[val_start..].find('"')? + val_start;
    Some(tag[val_start..val_end].to_string())
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
            b'M' => { tokens.push(Token::MAbs); i += 1; }
            b'm' => { tokens.push(Token::MRel); i += 1; }
            b'L' => { tokens.push(Token::LAbs); i += 1; }
            b'l' => { tokens.push(Token::LRel); i += 1; }
            b'z' | b'Z' => { tokens.push(Token::Z); i += 1; }
            b'-' | b'0'..=b'9' | b'.' => {
                let start = i;
                if b == b'-' { i += 1; }
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
            b',' | b' ' | b'\n' | b'\r' | b'\t' => { i += 1; }
            _ => { i += 1; }
        }
    }

    tokens
}


