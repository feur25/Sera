use super::config::BubbleConfig;
use crate::html::hover::{build_chart_html, slots_to_json, HoverSlot};
use crate::plot::statistical::common::{escape_xml, hex6, polar_point, push_b, push_f2, push_i, svg_open};
use std::f64::consts::PI;

#[crate::chart_demo(
    "labels=[\"Cheminee Rustique en Verre\",\"Cheminee Moderne en Fer\",\"Cheminee Moderne en Pierre\",\"Cheminee Moderne en Bois\",\"Cheminee Moderne en Charbon\",\"Cheminee Moderne en Mousse\",\"Cheminee Moderne en Argile\",\"Cheminee Moderne en Or\",\"Pont Rustique en Diamant\",\"Pont Rustique en Verre\",\"Pont Rustique en Or\",\"Pont Moderne en Diamant\",\"Pont Moderne en Charbon\",\"Pont Moderne en Verre\",\"Table Rustique en Fer\",\"Table Rustique en Diamant\",\"Table Rustique en Mousse\",\"Table Rustique en Verre\",\"Table Rustique en Argile\",\"Table Rustique en Or\",\"Table Moderne en Pierre\",\"Table Moderne en Mousse\",\"Table Moderne en Argile\",\"Mur Rustique en Pierre\",\"Mur Rustique en Bois\",\"Mur Rustique en Diamant\",\"Mur Rustique en Charbon\",\"Mur Rustique en Mousse\",\"Mur Rustique en Argile\",\"Mur Moderne en Fer\",\"Mur Moderne en Diamant\",\"Mur Moderne en Mousse\",\"Toit Rustique en Pierre\",\"Toit Rustique en Diamant\",\"Toit Rustique en Charbon\",\"Toit Rustique en Mousse\",\"Toit Rustique en Argile\",\"Toit Rustique en Or\",\"Toit Moderne en Pierre\",\"Toit Moderne en Bois\",\"Toit Moderne en Verre\",\"Toit Moderne en Or\",\"Statue Rustique en Diamant\",\"Statue Rustique en Charbon\",\"Statue Rustique en Mousse\",\"Statue Rustique en Argile\",\"Statue Rustique en Or\",\"Statue Moderne en Fer\",\"Statue Moderne en Pierre\",\"Statue Moderne en Diamant\",\"Statue Moderne en Charbon\",\"Statue Moderne en Mousse\",\"Statue Moderne en Argile\",\"Statue Moderne en Or\",\"Puits Rustique en Fer\",\"Puits Rustique en Bois\",\"Puits Rustique en Mousse\",\"Puits Rustique en Argile\",\"Puits Moderne en Pierre\",\"Puits Moderne en Verre\",\"Puits Moderne en Argile\",\"Fournaise Rustique en Pierre\",\"Fournaise Rustique en Bois\",\"Fournaise Rustique en Diamant\",\"Fournaise Rustique en Charbon\",\"Fournaise Rustique en Argile\",\"Fournaise Moderne en Pierre\",\"Fournaise Moderne en Bois\",\"Fournaise Moderne en Charbon\",\"Escalier Rustique en Fer\",\"Escalier Rustique en Pierre\",\"Escalier Rustique en Diamant\",\"Escalier Rustique en Charbon\",\"Escalier Rustique en Mousse\",\"Escalier Rustique en Argile\",\"Escalier Rustique en Or\",\"Escalier Moderne en Fer\",\"Escalier Moderne en Diamant\",\"Escalier Moderne en Charbon\",\"Cloture Rustique en Pierre\",\"Cloture Rustique en Bois\",\"Cloture Rustique en Diamant\",\"Cloture Rustique en Verre\",\"Cloture Rustique en Argile\",\"Cloture Rustique en Or\",\"Cloture Moderne en Fer\",\"Cloture Moderne en Bois\",\"Cloture Moderne en Diamant\",\"Cloture Moderne en Mousse\",\"Cloture Moderne en Argile\",\"Cloture Moderne en Or\",\"Tourelle Rustique en Fer\",\"Tourelle Rustique en Charbon\",\"Tourelle Rustique en Mousse\",\"Tourelle Rustique en Or\",\"Tourelle Moderne en Fer\",\"Tourelle Moderne en Diamant\",\"Tourelle Moderne en Or\",\"Portail Rustique en Pierre\",\"Portail Rustique en Diamant\",\"Portail Moderne en Diamant\",\"Portail Moderne en Verre\",\"Balcon Rustique en Fer\",\"Balcon Rustique en Charbon\",\"Balcon Rustique en Mousse\",\"Balcon Rustique en Verre\",\"Balcon Rustique en Argile\",\"Balcon Moderne en Pierre\",\"Balcon Moderne en Bois\",\"Balcon Moderne en Verre\",\"Fontaine Rustique en Diamant\",\"Fontaine Rustique en Charbon\",\"Fontaine Rustique en Argile\",\"Fontaine Rustique en Or\",\"Fontaine Moderne en Fer\",\"Fontaine Moderne en Charbon\",\"Fontaine Moderne en Mousse\",\"Fontaine Moderne en Verre\",\"Autel Rustique en Pierre\",\"Autel Rustique en Charbon\",\"Autel Rustique en Mousse\",\"Autel Rustique en Verre\",\"Autel Moderne en Pierre\",\"Autel Moderne en Charbon\",\"Autel Moderne en Verre\",\"Autel Moderne en Argile\",\"Silo Rustique en Pierre\",\"Silo Rustique en Diamant\",\"Silo Rustique en Charbon\",\"Silo Rustique en Or\",\"Silo Moderne en Fer\",\"Silo Moderne en Pierre\",\"Silo Moderne en Bois\",\"Silo Moderne en Charbon\",\"Silo Moderne en Mousse\",\"Silo Moderne en Argile\",\"Hangar Rustique en Fer\",\"Hangar Rustique en Pierre\",\"Hangar Rustique en Bois\",\"Hangar Rustique en Diamant\",\"Hangar Rustique en Charbon\",\"Hangar Moderne en Fer\",\"Hangar Moderne en Or\",\"Serre Rustique en Bois\",\"Serre Rustique en Diamant\",\"Serre Rustique en Verre\",\"Serre Rustique en Or\",\"Serre Moderne en Diamant\",\"Serre Moderne en Charbon\",\"Serre Moderne en Verre\",\"Kiosque Rustique en Fer\",\"Kiosque Rustique en Pierre\",\"Kiosque Rustique en Bois\",\"Kiosque Rustique en Mousse\",\"Kiosque Rustique en Or\",\"Kiosque Moderne en Fer\",\"Kiosque Moderne en Pierre\",\"Kiosque Moderne en Charbon\",\"Kiosque Moderne en Argile\",\"Kiosque Moderne en Or\",\"Belvedere Rustique en Bois\",\"Belvedere Rustique en Charbon\",\"Belvedere Rustique en Mousse\",\"Belvedere Moderne en Fer\",\"Belvedere Moderne en Pierre\",\"Belvedere Moderne en Bois\",\"Belvedere Moderne en Mousse\",\"Belvedere Moderne en Verre\",\"Belvedere Moderne en Argile\",\"Belvedere Moderne en Or\"], x_values=[1.0,2.0,3.0,4.0,5.0,6.0,7.0,8.0,9.0,10.0,11.0,12.0,13.0,14.0,15.0,16.0,17.0,18.0,19.0,20.0,21.0,22.0,23.0,24.0,25.0,26.0,27.0,28.0,29.0,30.0,31.0,32.0,33.0,34.0,35.0,36.0,37.0,38.0,39.0,40.0,41.0,42.0,43.0,44.0,45.0,46.0,47.0,48.0,49.0,50.0,51.0,52.0,53.0,54.0,55.0,56.0,57.0,58.0,59.0,60.0,61.0,62.0,63.0,64.0,65.0,66.0,67.0,68.0,69.0,70.0,71.0,72.0,73.0,74.0,75.0,76.0,77.0,78.0,79.0,80.0,81.0,82.0,83.0,84.0,85.0,86.0,87.0,88.0,89.0,90.0,91.0,92.0,93.0,94.0,95.0,96.0,97.0,98.0,99.0,100.0,101.0,102.0,103.0,104.0,105.0,106.0,107.0,108.0,109.0,110.0,111.0,112.0,113.0,114.0,115.0,116.0,117.0,118.0,119.0,120.0,121.0,122.0,123.0,124.0,125.0,126.0,127.0,128.0,129.0,130.0,131.0,132.0,133.0,134.0,135.0,136.0,137.0,138.0,139.0,140.0,141.0,142.0,143.0,144.0,145.0,146.0,147.0,148.0,149.0,150.0,151.0,152.0,153.0,154.0,155.0,156.0,157.0,158.0,159.0,160.0,161.0,162.0,163.0,164.0,165.0,166.0,167.0,168.0,169.0,170.0], sizes=[14.9,18.8,18.8,24.1,18.7,13.0,21.0,15.6,18.1,24.0,13.6,11.4,19.8,12.8,21.8,21.0,24.5,15.8,19.7,12.0,18.2,20.3,19.3,15.7,16.7,17.2,18.5,17.6,17.2,14.8,17.1,19.5,19.0,23.9,16.7,19.1,17.0,17.7,20.8,24.3,10.4,19.8,21.6,14.8,26.9,13.2,24.8,17.1,12.0,24.7,17.1,19.8,14.9,16.1,25.5,24.4,15.3,26.2,11.7,14.1,21.8,11.9,22.9,14.9,17.6,14.5,13.4,18.0,20.6,25.4,20.4,25.9,19.9,25.8,19.4,12.1,19.3,24.9,21.6,21.7,14.7,19.4,19.8,24.7,24.5,24.1,16.2,24.4,21.7,14.1,11.7,22.7,21.6,23.9,18.2,18.0,18.8,14.4,14.8,13.7,10.8,19.8,22.4,20.4,15.8,20.7,22.5,14.4,20.1,17.0,18.2,12.9,22.0,14.9,20.2,15.5,19.0,15.6,15.2,15.5,18.9,20.4,19.8,18.0,18.4,22.4,12.2,13.6,22.3,21.3,18.3,16.0,23.3,17.2,20.9,19.8,16.2,19.9,13.5,16.0,17.1,23.1,15.3,18.0,16.6,25.1,18.2,23.1,23.2,21.1,22.1,23.5,14.8,13.3,12.5,16.1,24.8,17.8,11.3,20.4,17.9,21.0,14.6,23.3,19.5,13.7,17.1,17.0,19.5,18.0], categories=[\"Cheminee Rustique\",\"Cheminee Moderne\",\"Cheminee Moderne\",\"Cheminee Moderne\",\"Cheminee Moderne\",\"Cheminee Moderne\",\"Cheminee Moderne\",\"Cheminee Moderne\",\"Pont Rustique\",\"Pont Rustique\",\"Pont Rustique\",\"Pont Moderne\",\"Pont Moderne\",\"Pont Moderne\",\"Table Rustique\",\"Table Rustique\",\"Table Rustique\",\"Table Rustique\",\"Table Rustique\",\"Table Rustique\",\"Table Moderne\",\"Table Moderne\",\"Table Moderne\",\"Mur Rustique\",\"Mur Rustique\",\"Mur Rustique\",\"Mur Rustique\",\"Mur Rustique\",\"Mur Rustique\",\"Mur Moderne\",\"Mur Moderne\",\"Mur Moderne\",\"Toit Rustique\",\"Toit Rustique\",\"Toit Rustique\",\"Toit Rustique\",\"Toit Rustique\",\"Toit Rustique\",\"Toit Moderne\",\"Toit Moderne\",\"Toit Moderne\",\"Toit Moderne\",\"Statue Rustique\",\"Statue Rustique\",\"Statue Rustique\",\"Statue Rustique\",\"Statue Rustique\",\"Statue Moderne\",\"Statue Moderne\",\"Statue Moderne\",\"Statue Moderne\",\"Statue Moderne\",\"Statue Moderne\",\"Statue Moderne\",\"Puits Rustique\",\"Puits Rustique\",\"Puits Rustique\",\"Puits Rustique\",\"Puits Moderne\",\"Puits Moderne\",\"Puits Moderne\",\"Fournaise Rustique\",\"Fournaise Rustique\",\"Fournaise Rustique\",\"Fournaise Rustique\",\"Fournaise Rustique\",\"Fournaise Moderne\",\"Fournaise Moderne\",\"Fournaise Moderne\",\"Escalier Rustique\",\"Escalier Rustique\",\"Escalier Rustique\",\"Escalier Rustique\",\"Escalier Rustique\",\"Escalier Rustique\",\"Escalier Rustique\",\"Escalier Moderne\",\"Escalier Moderne\",\"Escalier Moderne\",\"Cloture Rustique\",\"Cloture Rustique\",\"Cloture Rustique\",\"Cloture Rustique\",\"Cloture Rustique\",\"Cloture Rustique\",\"Cloture Moderne\",\"Cloture Moderne\",\"Cloture Moderne\",\"Cloture Moderne\",\"Cloture Moderne\",\"Cloture Moderne\",\"Tourelle Rustique\",\"Tourelle Rustique\",\"Tourelle Rustique\",\"Tourelle Rustique\",\"Tourelle Moderne\",\"Tourelle Moderne\",\"Tourelle Moderne\",\"Portail Rustique\",\"Portail Rustique\",\"Portail Moderne\",\"Portail Moderne\",\"Balcon Rustique\",\"Balcon Rustique\",\"Balcon Rustique\",\"Balcon Rustique\",\"Balcon Rustique\",\"Balcon Moderne\",\"Balcon Moderne\",\"Balcon Moderne\",\"Fontaine Rustique\",\"Fontaine Rustique\",\"Fontaine Rustique\",\"Fontaine Rustique\",\"Fontaine Moderne\",\"Fontaine Moderne\",\"Fontaine Moderne\",\"Fontaine Moderne\",\"Autel Rustique\",\"Autel Rustique\",\"Autel Rustique\",\"Autel Rustique\",\"Autel Moderne\",\"Autel Moderne\",\"Autel Moderne\",\"Autel Moderne\",\"Silo Rustique\",\"Silo Rustique\",\"Silo Rustique\",\"Silo Rustique\",\"Silo Moderne\",\"Silo Moderne\",\"Silo Moderne\",\"Silo Moderne\",\"Silo Moderne\",\"Silo Moderne\",\"Hangar Rustique\",\"Hangar Rustique\",\"Hangar Rustique\",\"Hangar Rustique\",\"Hangar Rustique\",\"Hangar Moderne\",\"Hangar Moderne\",\"Serre Rustique\",\"Serre Rustique\",\"Serre Rustique\",\"Serre Rustique\",\"Serre Moderne\",\"Serre Moderne\",\"Serre Moderne\",\"Kiosque Rustique\",\"Kiosque Rustique\",\"Kiosque Rustique\",\"Kiosque Rustique\",\"Kiosque Rustique\",\"Kiosque Moderne\",\"Kiosque Moderne\",\"Kiosque Moderne\",\"Kiosque Moderne\",\"Kiosque Moderne\",\"Belvedere Rustique\",\"Belvedere Rustique\",\"Belvedere Rustique\",\"Belvedere Moderne\",\"Belvedere Moderne\",\"Belvedere Moderne\",\"Belvedere Moderne\",\"Belvedere Moderne\",\"Belvedere Moderne\",\"Belvedere Moderne\"], x_categories=[\"Verre\",\"Fer\",\"Pierre\",\"Bois\",\"Charbon\",\"Mousse\",\"Argile\",\"Or\",\"Diamant\",\"Verre\",\"Or\",\"Diamant\",\"Charbon\",\"Verre\",\"Fer\",\"Diamant\",\"Mousse\",\"Verre\",\"Argile\",\"Or\",\"Pierre\",\"Mousse\",\"Argile\",\"Pierre\",\"Bois\",\"Diamant\",\"Charbon\",\"Mousse\",\"Argile\",\"Fer\",\"Diamant\",\"Mousse\",\"Pierre\",\"Diamant\",\"Charbon\",\"Mousse\",\"Argile\",\"Or\",\"Pierre\",\"Bois\",\"Verre\",\"Or\",\"Diamant\",\"Charbon\",\"Mousse\",\"Argile\",\"Or\",\"Fer\",\"Pierre\",\"Diamant\",\"Charbon\",\"Mousse\",\"Argile\",\"Or\",\"Fer\",\"Bois\",\"Mousse\",\"Argile\",\"Pierre\",\"Verre\",\"Argile\",\"Pierre\",\"Bois\",\"Diamant\",\"Charbon\",\"Argile\",\"Pierre\",\"Bois\",\"Charbon\",\"Fer\",\"Pierre\",\"Diamant\",\"Charbon\",\"Mousse\",\"Argile\",\"Or\",\"Fer\",\"Diamant\",\"Charbon\",\"Pierre\",\"Bois\",\"Diamant\",\"Verre\",\"Argile\",\"Or\",\"Fer\",\"Bois\",\"Diamant\",\"Mousse\",\"Argile\",\"Or\",\"Fer\",\"Charbon\",\"Mousse\",\"Or\",\"Fer\",\"Diamant\",\"Or\",\"Pierre\",\"Diamant\",\"Diamant\",\"Verre\",\"Fer\",\"Charbon\",\"Mousse\",\"Verre\",\"Argile\",\"Pierre\",\"Bois\",\"Verre\",\"Diamant\",\"Charbon\",\"Argile\",\"Or\",\"Fer\",\"Charbon\",\"Mousse\",\"Verre\",\"Pierre\",\"Charbon\",\"Mousse\",\"Verre\",\"Pierre\",\"Charbon\",\"Verre\",\"Argile\",\"Pierre\",\"Diamant\",\"Charbon\",\"Or\",\"Fer\",\"Pierre\",\"Bois\",\"Charbon\",\"Mousse\",\"Argile\",\"Fer\",\"Pierre\",\"Bois\",\"Diamant\",\"Charbon\",\"Fer\",\"Or\",\"Bois\",\"Diamant\",\"Verre\",\"Or\",\"Diamant\",\"Charbon\",\"Verre\",\"Fer\",\"Pierre\",\"Bois\",\"Mousse\",\"Or\",\"Fer\",\"Pierre\",\"Charbon\",\"Argile\",\"Or\",\"Bois\",\"Charbon\",\"Mousse\",\"Fer\",\"Pierre\",\"Bois\",\"Mousse\",\"Verre\",\"Argile\",\"Or\"], color_values=[0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,1.0,0.0,1.0,0.0,0.0,0.0,0.0,1.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,1.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,1.0,0.0,0.0,0.0,1.0,0.0,0.0,0.0,0.0,0.0,0.0,1.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,1.0,0.0,0.0,0.0,0.0,0.0,0.0,1.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,1.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,1.0,0.0,1.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,1.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,1.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,1.0], variant=\"radial_rows\", width=620, height=390, min_size=1.6, max_size=7"
)]

pub fn render(cfg: &BubbleConfig) -> String {
    let n = cfg.x_values.len().min(cfg.sizes.len()).min(cfg.categories.len());
    if n == 0 {
        return String::new();
    }

    let mut rows: Vec<String> = Vec::new();
    for c in &cfg.categories[..n] {
        if !rows.iter().any(|r| r == c) {
            rows.push(c.clone());
        }
    }
    let n_rows = rows.len();
    if n_rows == 0 {
        return String::new();
    }
    let row_of = |cat: &str| -> usize { rows.iter().position(|r| r == cat).unwrap_or(0) };

    let has_x_cats = cfg.x_categories.len() >= n;
    let mut cols: Vec<String> = Vec::new();
    if has_x_cats {
        for c in &cfg.x_categories[..n] {
            if !cols.iter().any(|x| x == c) {
                cols.push(c.clone());
            }
        }
    }
    let n_cols = cols.len();
    let col_of = |cat: &str| -> usize { cols.iter().position(|c| c == cat).unwrap_or(0) };

    let mut xmin = f64::INFINITY;
    let mut xmax = f64::NEG_INFINITY;
    let mut smin = f64::INFINITY;
    let mut smax = f64::NEG_INFINITY;
    for i in 0..n {
        xmin = xmin.min(cfg.x_values[i]);
        xmax = xmax.max(cfg.x_values[i]);
        let s = cfg.sizes[i].abs();
        smin = smin.min(s);
        smax = smax.max(s);
    }
    let xr = (xmax - xmin).max(1e-9);
    let sr = (smax - smin).max(1e-9);

    let w = cfg.width;
    let h = cfg.height;
    let label_w = 150.0;
    let top_margin = 34.0;
    let bottom_margin = 22.0;
    let usable_top = top_margin;
    let usable_bottom = h as f64 - bottom_margin;
    let cy = (usable_top + usable_bottom) / 2.0;
    let outer_r = ((usable_bottom - usable_top) / 2.0)
        .min(w as f64 - label_w - 60.0)
        .max(10.0);
    let cx = label_w + 24.0;
    let inner_floor = (outer_r * 0.02).max(3.0);
    let hub_r = outer_r * 0.09;

    let list_top = cy - outer_r;
    let list_bottom = cy + outer_r;
    let row_step = if n_rows > 1 { (list_bottom - list_top) / (n_rows - 1) as f64 } else { 0.0 };

    let mut label_ys = Vec::with_capacity(n_rows);
    let mut row_radii = Vec::with_capacity(n_rows);
    for ri in 0..n_rows {
        let ly = list_top + ri as f64 * row_step;
        label_ys.push(ly);
        row_radii.push((ly - cy).abs().max(inner_floor));
    }

    let start_angle = -PI / 2.0;
    let end_angle = PI / 2.0;
    let mid_angle = 0.0;
    let sweep = PI;

    let color_normal = if cfg.color_low == 0x636EFA { 0x94A3B8 } else { cfg.color_low };
    let color_record = if cfg.color_high == 0xF43F5E { 0xE03131 } else { cfg.color_high };

    let mut buf = Vec::<u8>::with_capacity(n * 200 + 8192);
    svg_open(&mut buf, w, h);

    let half_arc = |buf: &mut Vec<u8>, r: f64| {
        let (x0, y0) = polar_point(cx, cy, start_angle, r);
        let (xm, ym) = polar_point(cx, cy, mid_angle, r);
        let (x1, y1) = polar_point(cx, cy, end_angle, r);
        push_b(buf, b"<path fill=\"none\" d=\"M");
        push_f2(buf, x0);
        push_b(buf, b",");
        push_f2(buf, y0);
        push_b(buf, b" A");
        push_f2(buf, r);
        push_b(buf, b",");
        push_f2(buf, r);
        push_b(buf, b" 0 0,1 ");
        push_f2(buf, xm);
        push_b(buf, b",");
        push_f2(buf, ym);
        push_b(buf, b" A");
        push_f2(buf, r);
        push_b(buf, b",");
        push_f2(buf, r);
        push_b(buf, b" 0 0,1 ");
        push_f2(buf, x1);
        push_b(buf, b",");
        push_f2(buf, y1);
        push_b(buf, b"\"/>");
    };

    push_b(&mut buf, b"<line x1=\"");
    push_f2(&mut buf, cx);
    push_b(&mut buf, b"\" y1=\"");
    push_f2(&mut buf, list_top);
    push_b(&mut buf, b"\" x2=\"");
    push_f2(&mut buf, cx);
    push_b(&mut buf, b"\" y2=\"");
    push_f2(&mut buf, list_bottom);
    push_b(&mut buf, b"\" stroke=\"#cbd5e1\" stroke-width=\"1\"/>");

    push_b(&mut buf, b"<g stroke=\"#e6eaf0\" stroke-width=\"1\">");
    for &r in &row_radii {
        half_arc(&mut buf, r);
    }
    push_b(&mut buf, b"</g>");

    let tick_count = if has_x_cats { n_cols } else { 31 };
    let tick_segs = tick_count.saturating_sub(1).max(1);
    let tick_angle = |k: usize| -> f64 { start_angle + sweep * k as f64 / tick_segs as f64 };
    let tick_text = |k: usize| -> String {
        if has_x_cats {
            cols[k].clone()
        } else {
            format!("{:.0}", xmin + (xmax - xmin) * k as f64 / tick_segs as f64)
        }
    };
    let label_every = if has_x_cats { 1 } else { (tick_segs / 10).max(1) };
    let angle_of_point = |i: usize| -> f64 {
        if has_x_cats {
            tick_angle(col_of(&cfg.x_categories[i]))
        } else {
            start_angle + ((cfg.x_values[i] - xmin) / xr) * sweep
        }
    };

    push_b(&mut buf, b"<g stroke=\"#eef1f6\" stroke-width=\"1\">");
    for k in 0..tick_count {
        let a = tick_angle(k);
        let (x0, y0) = polar_point(cx, cy, a, hub_r);
        let (x1, y1) = polar_point(cx, cy, a, outer_r);
        push_b(&mut buf, b"<line x1=\"");
        push_f2(&mut buf, x0);
        push_b(&mut buf, b"\" y1=\"");
        push_f2(&mut buf, y0);
        push_b(&mut buf, b"\" x2=\"");
        push_f2(&mut buf, x1);
        push_b(&mut buf, b"\" y2=\"");
        push_f2(&mut buf, y1);
        push_b(&mut buf, b"\"/>");
    }
    push_b(&mut buf, b"</g>");

    push_b(&mut buf, b"<g fill=\"none\" stroke=\"#94a3b8\" stroke-width=\"1.4\">");
    half_arc(&mut buf, outer_r);
    half_arc(&mut buf, hub_r);
    push_b(&mut buf, b"</g>");

    push_b(&mut buf, b"<g fill=\"#94a3b8\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"7\">");
    for k in (0..tick_count).step_by(label_every) {
        let a = tick_angle(k);
        let (tx, ty) = polar_point(cx, cy, a, outer_r + 9.0);
        let deg = a * 180.0 / PI;
        push_b(&mut buf, b"<text x=\"");
        push_f2(&mut buf, tx);
        push_b(&mut buf, b"\" y=\"");
        push_f2(&mut buf, ty);
        push_b(&mut buf, b"\" transform=\"rotate(");
        push_f2(&mut buf, deg);
        push_b(&mut buf, b" ");
        push_f2(&mut buf, tx);
        push_b(&mut buf, b" ");
        push_f2(&mut buf, ty);
        push_b(&mut buf, b")\">");
        escape_xml(&mut buf, &tick_text(k));
        push_b(&mut buf, b"</text>");
    }
    push_b(&mut buf, b"</g>");

    let mut slots: Vec<HoverSlot> = Vec::with_capacity(n);
    for i in 0..n {
        let ri = row_of(&cfg.categories[i]);
        let a = angle_of_point(i);
        let r = row_radii[ri];
        let (px, py) = polar_point(cx, cy, a, r);
        let sn = (cfg.sizes[i].abs() - smin) / sr;
        let radius = cfg.min_size + sn * (cfg.max_size - cfg.min_size);
        let is_record = cfg.color_values.get(i).copied().unwrap_or(0.0) >= 0.5;
        let color = if is_record { color_record } else { color_normal };
        let hx = hex6(color);

        push_b(&mut buf, b"<circle data-idx=\"");
        push_i(&mut buf, i as i32);
        push_b(&mut buf, b"\" cx=\"");
        push_f2(&mut buf, px);
        push_b(&mut buf, b"\" cy=\"");
        push_f2(&mut buf, py);
        push_b(&mut buf, b"\" r=\"");
        push_f2(&mut buf, radius);
        push_b(&mut buf, b"\" fill=\"#");
        buf.extend_from_slice(&hx);
        push_b(&mut buf, b"\" fill-opacity=\"");
        push_f2(&mut buf, if is_record { 0.92 } else { 0.62 });
        push_b(&mut buf, b"\"");
        if is_record {
            push_b(&mut buf, b" stroke=\"#");
            buf.extend_from_slice(&hex6(color_record));
            push_b(&mut buf, b"\" stroke-width=\"1.4\" stroke-opacity=\"0.9\"");
        }
        push_b(&mut buf, b"/>");

        let label = cfg.labels.get(i).map(|s| s.as_str()).unwrap_or("");
        let mut slot = HoverSlot::new(if label.is_empty() { cfg.categories[i].clone() } else { label.to_string() })
            .kv("Row", cfg.categories[i].clone());
        slot = if has_x_cats {
            slot.kv("Column", cfg.x_categories[i].clone())
        } else {
            slot.kv("Position", format!("{:.0}", cfg.x_values[i]))
        };
        slot = slot.kv("Size", format!("{:.1}", cfg.sizes[i]));
        if is_record {
            slot = slot.kv("Record", "yes".to_string());
        }
        slots.push(slot);
    }

    let row_font = (row_step * 0.82).clamp(5.5, 9.0);
    for ri in 0..n_rows {
        let ly = label_ys[ri];
        push_b(&mut buf, b"<line x1=\"");
        push_f2(&mut buf, label_w - 6.0);
        push_b(&mut buf, b"\" y1=\"");
        push_f2(&mut buf, ly);
        push_b(&mut buf, b"\" x2=\"");
        push_f2(&mut buf, cx - 2.0);
        push_b(&mut buf, b"\" y2=\"");
        push_f2(&mut buf, ly);
        push_b(&mut buf, b"\" stroke=\"#dbe1ea\" stroke-width=\"1\"/>");
        push_b(&mut buf, b"<circle cx=\"");
        push_f2(&mut buf, cx);
        push_b(&mut buf, b"\" cy=\"");
        push_f2(&mut buf, ly);
        push_b(&mut buf, b"\" r=\"1.4\" fill=\"#94a3b8\"/>");
        push_b(&mut buf, b"<text x=\"");
        push_f2(&mut buf, label_w - 10.0);
        push_b(&mut buf, b"\" y=\"");
        push_f2(&mut buf, ly + 3.0);
        push_b(&mut buf, b"\" text-anchor=\"end\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"");
        push_f2(&mut buf, row_font);
        push_b(&mut buf, b"\" fill=\"#64748b\">");
        escape_xml(&mut buf, &rows[ri]);
        push_b(&mut buf, b"</text>");
    }

    if !cfg.title.is_empty() {
        push_b(&mut buf, b"<text x=\"");
        push_f2(&mut buf, cx);
        push_b(&mut buf, b"\" y=\"22\" text-anchor=\"middle\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"12\" font-weight=\"700\" fill=\"#1e293b\" letter-spacing=\"2\">");
        escape_xml(&mut buf, cfg.title);
        push_b(&mut buf, b"</text>");
    }

    let leg_x = w - 150;
    let leg_y = h - 84;
    push_b(&mut buf, b"<text x=\"");
    push_i(&mut buf, leg_x);
    push_b(&mut buf, b"\" y=\"");
    push_i(&mut buf, leg_y - 14);
    push_b(&mut buf, b"\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"8.5\" font-weight=\"700\" fill=\"#475569\">SIZE</text>");
    let sizes_legend = [cfg.min_size, (cfg.min_size + cfg.max_size) / 2.0, cfg.max_size];
    for (k, &r) in sizes_legend.iter().enumerate() {
        let sy = leg_y - r as i32;
        push_b(&mut buf, b"<circle cx=\"");
        push_i(&mut buf, leg_x + k as i32 * 34);
        push_b(&mut buf, b"\" cy=\"");
        push_i(&mut buf, sy);
        push_b(&mut buf, b"\" r=\"");
        push_f2(&mut buf, r);
        push_b(&mut buf, b"\" fill=\"none\" stroke=\"#94a3b8\" stroke-opacity=\"0.7\" stroke-width=\"1\"/>");
        push_b(&mut buf, b"<text x=\"");
        push_i(&mut buf, leg_x + k as i32 * 34);
        push_b(&mut buf, b"\" y=\"");
        push_i(&mut buf, leg_y + 12);
        push_b(&mut buf, b"\" text-anchor=\"middle\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"7\" fill=\"#94a3b8\">");
        let s = format!("{:.0}", r);
        buf.extend_from_slice(s.as_bytes());
        push_b(&mut buf, b"</text>");
    }
    push_b(&mut buf, b"<circle cx=\"");
    push_i(&mut buf, leg_x);
    push_b(&mut buf, b"\" cy=\"");
    push_i(&mut buf, leg_y + 26);
    push_b(&mut buf, b"\" r=\"5\" fill=\"#");
    buf.extend_from_slice(&hex6(color_normal));
    push_b(&mut buf, b"\" fill-opacity=\"0.62\"/>");
    push_b(&mut buf, b"<text x=\"");
    push_i(&mut buf, leg_x + 10);
    push_b(&mut buf, b"\" y=\"");
    push_i(&mut buf, leg_y + 30);
    push_b(&mut buf, b"\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"8\" fill=\"#475569\">Normal</text>");
    push_b(&mut buf, b"<circle cx=\"");
    push_i(&mut buf, leg_x + 70);
    push_b(&mut buf, b"\" cy=\"");
    push_i(&mut buf, leg_y + 26);
    push_b(&mut buf, b"\" r=\"5\" fill=\"#");
    buf.extend_from_slice(&hex6(color_record));
    push_b(&mut buf, b"\" fill-opacity=\"0.92\" stroke=\"#");
    buf.extend_from_slice(&hex6(color_record));
    push_b(&mut buf, b"\" stroke-width=\"1.4\"/>");
    push_b(&mut buf, b"<text x=\"");
    push_i(&mut buf, leg_x + 80);
    push_b(&mut buf, b"\" y=\"");
    push_i(&mut buf, leg_y + 30);
    push_b(&mut buf, b"\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"8\" fill=\"#475569\">Record</text>");

    push_b(&mut buf, b"</svg>");
    let svg = unsafe { String::from_utf8_unchecked(buf) };
    build_chart_html(cfg.title, &svg, &slots_to_json(&slots))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::plot::statistical::bubble::config::BubbleConfig;

    fn cfg<'a>(
        x: &'a [f64],
        sizes: &'a [f64],
        cats: &'a [String],
        colv: &'a [f64],
    ) -> BubbleConfig<'a> {
        BubbleConfig {
            title: "Test",
            x_values: x,
            sizes,
            categories: cats,
            color_values: colv,
            width: 900,
            height: 700,
            ..BubbleConfig::default()
        }
    }

    fn synth(n: usize, n_rows: usize) -> (Vec<f64>, Vec<f64>, Vec<String>, Vec<f64>) {
        let x: Vec<f64> = (0..n).map(|i| i as f64).collect();
        let sizes: Vec<f64> = (0..n).map(|i| 5.0 + (i % 7) as f64 * 4.0).collect();
        let cats: Vec<String> = (0..n).map(|i| format!("Row {}", i % n_rows)).collect();
        let colv: Vec<f64> = (0..n).map(|i| if i % 11 == 0 { 1.0 } else { 0.0 }).collect();
        (x, sizes, cats, colv)
    }

    #[test]
    fn renders_one_bubble_per_point_across_the_row_rings() {
        let (x, sizes, cats, colv) = synth(40, 6);
        let html = render(&cfg(&x, &sizes, &cats, &colv));
        assert!(!html.is_empty());
        assert_eq!(html.matches("<circle data-idx=").count(), 40);
        assert!(html.contains("class=\"sp-bg\""));
        assert!(html.contains("viewBox=\"0 0 900 700\""));
    }

    #[test]
    fn record_points_get_the_record_color_and_a_stroke() {
        let (x, sizes, cats, colv) = synth(20, 4);
        let html = render(&cfg(&x, &sizes, &cats, &colv));
        assert!(html.contains("Record</text>"));
        assert!(html.matches("stroke-width=\"1.4\" stroke-opacity=\"0.9\"").count() > 0);
    }

    #[test]
    fn each_distinct_row_gets_its_own_labeled_ring() {
        let (x, sizes, cats, colv) = synth(30, 5);
        let html = render(&cfg(&x, &sizes, &cats, &colv));
        for r in 0..5 {
            assert!(html.contains(&format!(">Row {r}<")));
        }
    }

    #[test]
    fn empty_input_returns_empty_string() {
        let x: Vec<f64> = vec![];
        let sizes: Vec<f64> = vec![];
        let cats: Vec<String> = vec![];
        let colv: Vec<f64> = vec![];
        assert!(render(&cfg(&x, &sizes, &cats, &colv)).is_empty());
    }

    #[test]
    fn per_point_x_categories_turn_the_axis_into_a_real_material_correlation() {
        let structures = ["cheminee", "pont", "table"];
        let materials = ["fer", "pierre", "bois", "diamant", "charbon", "mousse"];
        let mut x = Vec::new();
        let mut sizes = Vec::new();
        let mut cats = Vec::new();
        let mut mats = Vec::new();
        let mut colv = Vec::new();
        for (si, s) in structures.iter().enumerate() {
            for (mi, m) in materials.iter().enumerate() {
                x.push((si * materials.len() + mi) as f64);
                sizes.push(6.0 + mi as f64);
                cats.push(s.to_string());
                mats.push(m.to_string());
                colv.push(0.0);
            }
        }
        let html = render(&BubbleConfig {
            x_categories: &mats,
            ..cfg(&x, &sizes, &cats, &colv)
        });
        for m in &materials {
            assert!(html.contains(&format!(">{m}<")));
        }
        assert_eq!(html.matches("<line").count(), materials.len() + 1 + structures.len());
        assert_eq!(html.matches("<circle data-idx=").count(), structures.len() * materials.len());
    }

    #[test]
    fn without_x_categories_the_axis_falls_back_to_numeric_ticks() {
        let (x, sizes, cats, colv) = synth(30, 5);
        let html = render(&cfg(&x, &sizes, &cats, &colv));
        assert!(html.contains(">0<"));
        assert!(!html.contains(">fer<"));
    }

    #[test]
    fn perf_rendering_a_large_radial_row_chart_stays_fast() {
        let (x, sizes, cats, colv) = synth(1200, 60);
        let start = std::time::Instant::now();
        let html = render(&cfg(&x, &sizes, &cats, &colv));
        let elapsed = start.elapsed();
        assert!(elapsed.as_millis() < 200, "rendering took too long: {elapsed:?}");
        assert!(!html.is_empty());
    }
}
