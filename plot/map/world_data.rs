use std::sync::OnceLock;
use super::svg_parser::{CountryShape, parse_world_svg};

const SVG_WIDTH: f32 = 1009.6727;
const SVG_HEIGHT: f32 = 665.96301;
const GEO_MIN_LON: f64 = -169.110266;
const GEO_MAX_LAT: f64 = 83.600842;
const GEO_MAX_LON: f64 = 190.486279;
const GEO_MIN_LAT: f64 = -58.508473;

static COUNTRIES: OnceLock<Vec<CountryShape>> = OnceLock::new();

fn get_countries() -> &'static Vec<CountryShape> {
    COUNTRIES.get_or_init(|| {
        let svg = include_str!("../../asset/world.svg");
        parse_world_svg(svg)
    })
}

pub fn lookup_country(key: &str) -> Option<&'static CountryShape> {
    let countries = get_countries();
    let key_upper = key.to_uppercase();
    countries.iter().find(|c| c.id == key_upper || c.name.eq_ignore_ascii_case(key))
}

pub fn all_countries() -> &'static [CountryShape] {
    get_countries()
}

pub fn normalized_polygons(shape: &CountryShape) -> Vec<Vec<[f32; 2]>> {
    shape.polygons.iter().map(|poly| {
        poly.iter().map(|[x, y]| {
            [x / SVG_WIDTH, y / SVG_HEIGHT]
        }).collect()
    }).collect()
}

pub fn svg_to_latlon(sx: f32, sy: f32) -> (f64, f64) {
    let nx = sx as f64 / SVG_WIDTH as f64;
    let ny = sy as f64 / SVG_HEIGHT as f64;
    let lon = GEO_MIN_LON + nx * (GEO_MAX_LON - GEO_MIN_LON);
    let lat = GEO_MAX_LAT - ny * (GEO_MAX_LAT - GEO_MIN_LAT);
    (lat, lon)
}

pub fn polygon_centroid(poly: &[[f32; 2]]) -> [f32; 2] {
    if poly.is_empty() { return [0.0, 0.0]; }
    let mut sx: f64 = 0.0;
    let mut sy: f64 = 0.0;
    for p in poly {
        sx += p[0] as f64;
        sy += p[1] as f64;
    }
    let n = poly.len() as f64;
    [(sx / n) as f32, (sy / n) as f32]
}

pub fn shape_centroid(shape: &CountryShape) -> [f32; 2] {
    let mut best_poly: Option<&Vec<[f32; 2]>> = None;
    let mut best_len = 0;
    for poly in &shape.polygons {
        if poly.len() > best_len {
            best_len = poly.len();
            best_poly = Some(poly);
        }
    }
    match best_poly {
        Some(poly) => polygon_centroid(poly),
        None => [0.0, 0.0],
    }
}

pub fn countries_in_region(region: &str) -> Vec<&'static str> {
    let region_map: &[(&str, &[&str])] = &[
        ("Africa", &["DZ","AO","BJ","BW","BF","BI","CV","CM","CF","TD","KM","CG","CD","CI","DJ","EG","GQ","ER","SZ","ET","GA","GM","GH","GN","GW","KE","LS","LR","LY","MG","MW","ML","MR","MU","MA","MZ","NA","NE","NG","RW","ST","SN","SC","SL","SO","ZA","SS","SD","TZ","TG","TN","UG","ZM","ZW"]),
        ("Europe", &["AL","AD","AT","BY","BE","BA","BG","HR","CY","CZ","DK","EE","FI","FR","DE","GR","HU","IS","IE","IT","XK","LV","LI","LT","LU","MT","MD","MC","ME","NL","MK","NO","PL","PT","RO","RU","SM","RS","SK","SI","ES","SE","CH","UA","GB","VA"]),
        ("Asia", &["AF","AM","AZ","BH","BD","BT","BN","KH","CN","GE","IN","ID","IR","IQ","IL","JP","JO","KZ","KW","KG","LA","LB","MY","MV","MN","MM","NP","KP","OM","PK","PS","PH","QA","SA","SG","KR","LK","SY","TW","TJ","TH","TL","TR","TM","AE","UZ","VN","YE"]),
        ("Americas", &["AG","AR","BS","BB","BZ","BO","BR","CA","CL","CO","CR","CU","DM","DO","EC","SV","GD","GT","GY","HT","HN","JM","MX","NI","PA","PY","PE","KN","LC","VC","SR","TT","US","UY","VE"]),
        ("Oceania", &["AU","FJ","KI","MH","FM","NR","NZ","PW","PG","WS","SB","TO","TV","VU"]),
    ];

    let r = region.to_lowercase();
    for (name, codes) in region_map {
        if name.to_lowercase() == r {
            return codes.iter().map(|s| *s).collect();
        }
    }
    Vec::new()
}

pub fn point_in_polygon(px: f32, py: f32, poly: &[[f32; 2]]) -> bool {
    let n = poly.len();
    if n < 3 { return false; }
    let mut inside = false;
    let mut j = n - 1;
    for i in 0..n {
        let (xi, yi) = (poly[i][0], poly[i][1]);
        let (xj, yj) = (poly[j][0], poly[j][1]);
        if ((yi > py) != (yj > py)) && (px < (xj - xi) * (py - yi) / (yj - yi) + xi) {
            inside = !inside;
        }
        j = i;
    }
    inside
}

pub fn hit_test_country(nx: f32, ny: f32, shape: &CountryShape) -> bool {
    let polys = normalized_polygons(shape);
    for poly in &polys {
        if point_in_polygon(nx, ny, poly) {
            return true;
        }
    }
    false
}

pub fn detect_hovered_country(
    mouse_nx: f32,
    mouse_ny: f32,
    labels: &[String],
    visible_indices: &[usize],
) -> Option<usize> {
    let mut label_map: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
    for &idx in visible_indices {
        if idx < labels.len() {
            label_map.insert(labels[idx].to_uppercase(), idx);
        }
    }

    for shape in all_countries() {
        let entry = label_map.get(&shape.id).or_else(|| label_map.get(&shape.name.to_uppercase()));
        if let Some(&idx) = entry {
            if hit_test_country(mouse_nx, mouse_ny, shape) {
                return Some(idx);
            }
        }
    }
    None
}

pub fn all_regions() -> Vec<&'static str> {
    vec!["Africa", "Europe", "Asia", "Americas", "Oceania"]
}

pub fn region_for_country(code: &str) -> Option<&'static str> {
    let code_upper = code.to_uppercase();
    for region in all_regions() {
        let codes = countries_in_region(region);
        if codes.iter().any(|c| c.to_uppercase() == code_upper) {
            return Some(region);
        }
    }
    None
}


