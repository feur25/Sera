use crate::Chart;
use pdf_writer::types::{ActionType, AnnotationFlags, AnnotationType};
use pdf_writer::{Content, Finish, Name, Pdf, Rect, Ref, Str};
use pyo3::prelude::*;
use seraplot_macros::{sera_doc, sera_doc_impl};
use std::collections::HashMap;
use svg2pdf::usvg;

fn svg_to_pdf_bytes(svg: &str) -> Result<Vec<u8>, String> {
    let mut opts = usvg::Options::default();
    opts.fontdb_mut().load_system_fonts();
    let tree = usvg::Tree::from_str(svg, &opts).map_err(|e| format!("invalid SVG: {e}"))?;
    svg2pdf::to_pdf(&tree, svg2pdf::ConversionOptions::default(), svg2pdf::PageOptions::default())
        .map_err(|e| format!("SVG-to-PDF conversion failed: {e}"))
}

fn companion_html_name(path: &str) -> String {
    let stem = match path.rfind('.') {
        Some(i) => &path[..i],
        None => path,
    };
    let base = format!("{stem}.html");
    std::path::Path::new(&base)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or(&base)
        .to_string()
}

fn companion_html_path(path: &str) -> String {
    let stem = match path.rfind('.') {
        Some(i) => &path[..i],
        None => path,
    };
    format!("{stem}.html")
}

fn write_link_annotation(pdf: &mut Pdf, annot_id: Ref, page_id: Ref, rect: Rect, target_file_name: &str) {
    let mut annot = pdf.annotation(annot_id);
    annot.subtype(AnnotationType::Link);
    annot.rect(rect);
    annot.page(page_id);
    annot.flags(AnnotationFlags::PRINT);
    {
        let mut action = annot.action();
        action.action_type(ActionType::RemoteGoTo);
        action.new_window(true);
        {
            let mut fs = action.file_spec();
            fs.path(Str(target_file_name.as_bytes()));
            fs.finish();
        }
        action.finish();
    }
    annot.finish();
}

fn latin1_lossy(s: &str) -> Vec<u8> {
    s.chars().map(|c| if (c as u32) < 256 { c as u8 } else { b'?' }).collect()
}

fn multi_svg_to_pdf_bytes(svgs: &[String], titles: Option<&[String]>, companions: Option<&[String]>) -> Result<Vec<u8>, String> {
    if svgs.is_empty() {
        return Err("need at least one chart".to_string());
    }

    let mut alloc = Ref::new(1);
    let catalog_id = alloc.bump();
    let page_tree_id = alloc.bump();
    let font_id = alloc.bump();

    let mut pdf = Pdf::new();
    pdf.catalog(catalog_id).pages(page_tree_id);
    pdf.type1_font(font_id).base_font(Name(b"Helvetica"));

    let font_name = Name(b"F1");
    let svg_name = Name(b"S1");
    let mut page_ids = Vec::with_capacity(svgs.len());
    let mut pending_links: Vec<(Ref, Ref, Rect, String)> = Vec::new();

    for (i, svg) in svgs.iter().enumerate() {
        let mut opts = usvg::Options::default();
        opts.fontdb_mut().load_system_fonts();
        let tree = usvg::Tree::from_str(svg, &opts).map_err(|e| format!("invalid SVG on page {}: {e}", i + 1))?;
        let size = tree.size();
        let (width, height) = (size.width().max(1.0), size.height().max(1.0));

        let (svg_chunk, svg_ref) = svg2pdf::to_chunk(&tree, svg2pdf::ConversionOptions::default())
            .map_err(|e| format!("SVG-to-PDF conversion failed on page {}: {e}", i + 1))?;
        let mut remap: HashMap<Ref, Ref> = HashMap::new();
        let renumbered = svg_chunk.renumber(|old| *remap.entry(old).or_insert_with(|| alloc.bump()));
        let mapped_svg_ref = *remap.get(&svg_ref).expect("svg_ref was just assigned a mapping above");

        let page_id = alloc.bump();
        let content_id = alloc.bump();

        let mut content = Content::new();
        content.save_state();
        content.transform([width, 0.0, 0.0, height, 0.0, 0.0]);
        content.x_object(svg_name);
        content.restore_state();
        if let Some(title) = titles.and_then(|ts| ts.get(i)) {
            content.begin_text();
            content.set_font(font_name, 14.0);
            content.next_line(12.0, (height - 20.0).max(0.0));
            content.show(Str(&latin1_lossy(title)));
            content.end_text();
        }
        pdf.stream(content_id, &content.finish()).finish();

        let annot_id = companions.and_then(|c| c.get(i)).map(|_| alloc.bump());

        let mut page = pdf.page(page_id);
        page.media_box(Rect::new(0.0, 0.0, width, height));
        page.parent(page_tree_id);
        page.contents(content_id);
        if let Some(id) = annot_id {
            page.annotations([id]);
        }
        let mut resources = page.resources();
        resources.x_objects().pair(svg_name, mapped_svg_ref);
        resources.fonts().pair(font_name, font_id);
        resources.finish();
        page.finish();

        if let Some(id) = annot_id {
            let target = companions.and_then(|c| c.get(i)).unwrap();
            pending_links.push((id, page_id, Rect::new(0.0, 0.0, width, height), target.clone()));
        }

        pdf.extend(&renumbered);
        page_ids.push(page_id);
    }

    pdf.pages(page_tree_id).count(page_ids.len() as i32).kids(page_ids);
    for (annot_id, page_id, rect, target) in &pending_links {
        write_link_annotation(&mut pdf, *annot_id, *page_id, *rect, target);
    }
    Ok(pdf.finish())
}

fn per_page_companion_names(path: &str, n: usize) -> Vec<String> {
    let stem = match path.rfind('.') {
        Some(i) => &path[..i],
        None => path,
    };
    (0..n)
        .map(|i| {
            let base = if n == 1 { format!("{stem}.html") } else { format!("{stem}_p{}.html", i + 1) };
            std::path::Path::new(&base).file_name().and_then(|n| n.to_str()).unwrap_or(&base).to_string()
        })
        .collect()
}

fn per_page_companion_paths(path: &str, n: usize) -> Vec<String> {
    let stem = match path.rfind('.') {
        Some(i) => &path[..i],
        None => path,
    };
    (0..n).map(|i| if n == 1 { format!("{stem}.html") } else { format!("{stem}_p{}.html", i + 1) }).collect()
}

fn svg_size(svg: &str) -> Result<(f32, f32), String> {
    let mut opts = usvg::Options::default();
    opts.fontdb_mut().load_system_fonts();
    let tree = usvg::Tree::from_str(svg, &opts).map_err(|e| format!("invalid SVG: {e}"))?;
    let size = tree.size();
    Ok((size.width().max(1.0), size.height().max(1.0)))
}

fn svg_to_png_bytes(svg: &str, scale: f32) -> Result<Vec<u8>, String> {
    let (w, h) = svg_size(svg)?;
    let pw = ((w * scale) as u32).max(1);
    let ph = ((h * scale) as u32).max(1);
    let img = crate::plot::pulse::features::studio::replay::rasterize(svg, pw, ph)?;
    let mut buf = Vec::new();
    img.write_to(&mut std::io::Cursor::new(&mut buf), image::ImageFormat::Png)
        .map_err(|e| format!("PNG encoding failed: {e}"))?;
    Ok(buf)
}

fn build_docx_bytes(svg: &str, title: &str, companion_name: Option<&str>) -> Result<Vec<u8>, String> {
    let (w, h) = svg_size(svg)?;
    let png_bytes = svg_to_png_bytes(svg, 2.0)?;
    let emu_w = ((w * 9525.0) as u32).max(1);
    let emu_h = ((h * 9525.0) as u32).max(1);
    let pic = docx_rs::Pic::new(&png_bytes).size(emu_w, emu_h);

    let mut docx = docx_rs::Docx::new();
    if !title.is_empty() {
        docx = docx.add_paragraph(docx_rs::Paragraph::new().add_run(docx_rs::Run::new().add_text(title).bold().size(28)));
    }
    docx = docx.add_paragraph(docx_rs::Paragraph::new().add_run(docx_rs::Run::new().add_image(pic)));
    if let Some(name) = companion_name {
        docx = docx.add_paragraph(docx_rs::Paragraph::new().add_hyperlink(
            docx_rs::Hyperlink::new(name, docx_rs::HyperlinkType::External)
                .add_run(docx_rs::Run::new().add_text("Open interactive version").color("2563EB").underline("single")),
        ));
    }
    let mut cursor = std::io::Cursor::new(Vec::new());
    docx.build().pack(&mut cursor).map_err(|e| format!("docx packaging failed: {e:?}"))?;
    Ok(cursor.into_inner())
}

#[sera_doc_impl]
#[pyo3::pymethods]
impl Chart {
    #[pyo3(signature = (path, interactive = true))]
    #[sera_doc(
        category = "chart_method",
        module = "report",
        file = "seraplot/pulse-pricing.md",
        en = "Exports this chart as a native, vector PDF page — no headless browser involved, just the chart's own SVG converted directly to PDF. Requires the chart to have real SVG content (charts that crossed their native canvas-rendering threshold have none; use export_svg()/export_png() to check first).",
        fr = "Exporte ce chart en PDF vectoriel natif — sans navigateur headless, juste le SVG du chart converti directement en PDF. Nécessite que le chart ait un vrai contenu SVG (les charts ayant dépassé leur seuil de rendu canvas natif n'en ont pas ; utilisez export_svg()/export_png() pour vérifier au préalable).",
        param(name = "path", ty = "str", en = "Destination .pdf file path.", fr = "Chemin du fichier .pdf de destination."),
        param(name = "interactive", ty = "bool", en = "When true (default), also writes a companion .html file next to the PDF (this chart's real interactive version) and embeds a clickable link on the PDF page that opens it in most PDF readers. When false, writes a plain static PDF with no companion file.", fr = "Si vrai (défaut), écrit aussi un fichier .html compagnon à côté du PDF (la vraie version interactive de ce chart) et intègre un lien cliquable sur la page PDF qui l'ouvre dans la plupart des lecteurs PDF. Si faux, écrit un PDF statique simple sans fichier compagnon.")
    )]
    fn export_pdf(&self, path: &str, interactive: bool) -> PyResult<()> {
        let svg = self.to_svg().ok_or_else(|| {
            pyo3::exceptions::PyValueError::new_err(
                "seraplot: export_pdf() needs real SVG content, but this chart has none — it crossed its native canvas-rendering threshold (large bar/heatmap/line/scatter/candlestick/icicle charts render via <canvas>+JS instead of <svg>, the same limitation export_svg()/export_png() already have). Use a smaller chart, or export_png() for a raster fallback.",
            )
        })?;
        let pdf = if interactive {
            let companion_path = companion_html_path(path);
            std::fs::write(&companion_path, &self.html).map_err(|e| {
                pyo3::exceptions::PyOSError::new_err(format!("seraplot: failed to write {companion_path}: {e}"))
            })?;
            multi_svg_to_pdf_bytes(&[svg], None, Some(&[companion_html_name(path)]))
        } else {
            svg_to_pdf_bytes(&svg)
        }
        .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(format!("seraplot: {e}")))?;
        super::atomic_write(path, &pdf)
            .map_err(|e| pyo3::exceptions::PyOSError::new_err(format!("seraplot: failed to write {path}: {e}")))?;
        Ok(())
    }

    #[pyo3(signature = (path, interactive = true))]
    #[sera_doc(
        category = "chart_method",
        module = "report",
        file = "seraplot/pulse-pricing.md",
        en = "Exports this chart as a Word .docx document — the chart embedded as a real image, ready to drop into a written report. Same SVG-content requirement as export_pdf().",
        fr = "Exporte ce chart en document Word .docx — le chart intégré en vraie image, prêt à glisser dans un rapport écrit. Même exigence de contenu SVG que export_pdf().",
        param(name = "path", ty = "str", en = "Destination .docx file path.", fr = "Chemin du fichier .docx de destination."),
        param(name = "interactive", ty = "bool", en = "When true (default), also writes a companion .html file (this chart's real interactive version) and adds a clickable hyperlink under the embedded image that opens it. When false, the .docx has only the static image.", fr = "Si vrai (défaut), écrit aussi un fichier .html compagnon (la vraie version interactive de ce chart) et ajoute un lien cliquable sous l'image intégrée qui l'ouvre. Si faux, le .docx ne contient que l'image statique.")
    )]
    fn export_docx(&self, path: &str, interactive: bool) -> PyResult<()> {
        let svg = self.to_svg().ok_or_else(|| {
            pyo3::exceptions::PyValueError::new_err(
                "seraplot: export_docx() needs real SVG content, but this chart has none — it crossed its native canvas-rendering threshold. Use a smaller chart, or export_png() for a raster fallback.",
            )
        })?;
        let companion_name = if interactive {
            let companion_path = companion_html_path(path);
            std::fs::write(&companion_path, &self.html).map_err(|e| {
                pyo3::exceptions::PyOSError::new_err(format!("seraplot: failed to write {companion_path}: {e}"))
            })?;
            Some(companion_html_name(path))
        } else {
            None
        };
        let docx = build_docx_bytes(&svg, "", companion_name.as_deref())
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(format!("seraplot: {e}")))?;
        super::atomic_write(path, &docx)
            .map_err(|e| pyo3::exceptions::PyOSError::new_err(format!("seraplot: failed to write {path}: {e}")))?;
        Ok(())
    }
}

#[pyfunction]
#[pyo3(signature = (charts, path, titles = None, interactive = true))]
#[sera_doc(
    category = "pulse_function",
    module = "report",
    file = "seraplot/pulse-pricing.md",
    en = "Combines several charts into one multi-page, native vector PDF - one page per chart, in the order given, each page sized to that chart's own drawing (no forced A4/scaling). Same SVG-content requirement as export_pdf() (charts that crossed their native canvas-rendering threshold have none).",
    fr = "Combine plusieurs charts en un seul PDF vectoriel natif multi-page — une page par chart, dans l'ordre donné, chaque page dimensionnée sur le dessin propre à ce chart (pas de mise à l'échelle A4 forcée). Même exigence de contenu SVG que export_pdf() (les charts ayant dépassé leur seuil de rendu canvas natif n'en ont pas).",
    param(name = "charts", ty = "list[Chart]", en = "Charts to combine, one page each, in order.", fr = "Charts à combiner, une page chacun, dans l'ordre."),
    param(name = "path", ty = "str", en = "Destination .pdf file path.", fr = "Chemin du fichier .pdf de destination."),
    param(name = "titles", ty = "list[str] | None", en = "Optional per-page title drawn at the top of each page; must have exactly one entry per chart if given. Non-Latin-1 characters are rendered as '?'.", fr = "Titre optionnel par page dessiné en haut de chaque page ; doit avoir exactement une entrée par chart si fourni. Les caractères hors Latin-1 sont rendus en « ? »."),
    param(name = "interactive", ty = "bool", en = "When true (default), each page also gets its own companion .html file (that page's chart, fully interactive) and a clickable link on the page that opens it. When false, plain static pages, no companion files.", fr = "Si vrai (défaut), chaque page reçoit aussi son propre fichier .html compagnon (le chart de cette page, pleinement interactif) et un lien cliquable sur la page qui l'ouvre. Si faux, pages statiques simples, aucun fichier compagnon.")
)]
pub fn export_pdf_report(charts: Vec<PyRef<'_, Chart>>, path: &str, titles: Option<Vec<String>>, interactive: bool) -> PyResult<()> {
    if charts.is_empty() {
        return Err(pyo3::exceptions::PyValueError::new_err("seraplot: export_pdf_report() needs at least one chart"));
    }
    if let Some(t) = &titles {
        if t.len() != charts.len() {
            return Err(pyo3::exceptions::PyValueError::new_err(format!(
                "seraplot: titles has {} entries but {} charts were given - they must match in length",
                t.len(),
                charts.len()
            )));
        }
    }
    let mut svgs = Vec::with_capacity(charts.len());
    for (i, chart) in charts.iter().enumerate() {
        let svg = chart.to_svg().ok_or_else(|| {
            pyo3::exceptions::PyValueError::new_err(format!(
                "seraplot: export_pdf_report() needs real SVG content for every chart, but chart at index {i} has none - it crossed its native canvas-rendering threshold. Use export_svg() to check a chart first."
            ))
        })?;
        svgs.push(svg);
    }
    let companion_names = if interactive {
        let names = per_page_companion_names(path, charts.len());
        let paths = per_page_companion_paths(path, charts.len());
        for (p, chart) in paths.iter().zip(charts.iter()) {
            std::fs::write(p, &chart.html)
                .map_err(|e| pyo3::exceptions::PyOSError::new_err(format!("seraplot: failed to write {p}: {e}")))?;
        }
        Some(names)
    } else {
        None
    };
    let pdf = multi_svg_to_pdf_bytes(&svgs, titles.as_deref(), companion_names.as_deref())
        .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(format!("seraplot: {e}")))?;
    super::atomic_write(path, &pdf)
        .map_err(|e| pyo3::exceptions::PyOSError::new_err(format!("seraplot: failed to write {path}: {e}")))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="200" height="100" viewBox="0 0 200 100">
<rect x="0" y="0" width="200" height="100" fill="#ffffff"/>
<circle cx="50" cy="50" r="30" fill="#636EFA"/>
<text x="100" y="50" font-size="14" fill="#1a202c">seraplot</text>
</svg>"##;

    #[test]
    fn valid_svg_converts_to_a_structurally_real_pdf() {
        let pdf = svg_to_pdf_bytes(SAMPLE_SVG).expect("conversion should succeed on valid SVG");
        assert!(pdf.starts_with(b"%PDF-"), "output must start with the PDF magic header");
        let tail = &pdf[pdf.len().saturating_sub(1024)..];
        assert!(
            tail.windows(5).any(|w| w == b"%%EOF"),
            "output must end with a PDF end-of-file marker"
        );
        assert!(pdf.len() > 200, "a real vector page should be more than a trivial stub");
    }

    #[test]
    fn malformed_svg_is_rejected_with_a_clear_error_not_a_panic() {
        let result = svg_to_pdf_bytes("not valid svg at all");
        assert!(result.is_err());
    }

    #[test]
    fn different_svg_content_produces_different_pdf_bytes() {
        let a = svg_to_pdf_bytes(SAMPLE_SVG).unwrap();
        let b = svg_to_pdf_bytes(
            r##"<svg xmlns="http://www.w3.org/2000/svg" width="200" height="100" viewBox="0 0 200 100"><rect x="0" y="0" width="200" height="100" fill="#000000"/></svg>"##,
        )
        .unwrap();
        assert_ne!(a, b, "visually different SVG input should not produce byte-identical PDFs");
    }

    #[test]
    fn empty_svg_document_still_produces_a_valid_pdf_shell() {
        let svg = r#"<svg xmlns="http://www.w3.org/2000/svg" width="10" height="10" viewBox="0 0 10 10"></svg>"#;
        let pdf = svg_to_pdf_bytes(svg).expect("an empty-but-valid SVG should still convert");
        assert!(pdf.starts_with(b"%PDF-"));
    }

    const SAMPLE_SVG_2: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="300" height="150" viewBox="0 0 300 150">
<rect x="0" y="0" width="300" height="150" fill="#111111"/>
<circle cx="80" cy="80" r="40" fill="#22c55e"/>
</svg>"##;

    fn count_occurrences(haystack: &[u8], needle: &[u8]) -> usize {
        haystack.windows(needle.len()).filter(|w| *w == needle).count()
    }

    #[test]
    fn latin1_lossy_passes_through_ascii_and_accents_but_replaces_the_rest() {
        assert_eq!(latin1_lossy("Sales"), b"Sales".to_vec());
        assert_eq!(latin1_lossy("caf\u{e9}"), vec![b'c', b'a', b'f', 0xe9]);
        assert_eq!(latin1_lossy("\u{4e2d}\u{6587}"), vec![b'?', b'?']);
    }

    #[test]
    fn rejects_an_empty_chart_list() {
        let err = multi_svg_to_pdf_bytes(&[], None, None).unwrap_err();
        assert!(err.contains("at least one chart"));
    }

    #[test]
    fn companion_html_name_swaps_the_extension_and_strips_any_directory() {
        assert_eq!(companion_html_name("out/report.pdf"), "report.html");
        assert_eq!(companion_html_name("report"), "report.html");
        assert_eq!(companion_html_name("nested/dir/x.docx"), "x.html");
    }

    #[test]
    fn companion_html_path_swaps_only_the_extension_keeping_the_directory() {
        assert_eq!(companion_html_path("out/report.pdf"), "out/report.html");
        assert_eq!(companion_html_path("report"), "report.html");
    }

    #[test]
    fn per_page_companion_names_uses_a_bare_name_for_a_single_page_and_indexed_names_for_several() {
        assert_eq!(per_page_companion_names("r.pdf", 1), vec!["r.html".to_string()]);
        assert_eq!(
            per_page_companion_names("out/r.pdf", 3),
            vec!["r_p1.html".to_string(), "r_p2.html".to_string(), "r_p3.html".to_string()]
        );
    }

    #[test]
    fn per_page_companion_paths_keep_the_original_directory() {
        assert_eq!(
            per_page_companion_paths("out/r.pdf", 2),
            vec!["out/r_p1.html".to_string(), "out/r_p2.html".to_string()]
        );
    }

    #[test]
    fn a_companion_link_annotation_makes_the_pdf_larger_and_adds_an_annots_entry() {
        let svgs = vec![SAMPLE_SVG.to_string()];
        let plain = multi_svg_to_pdf_bytes(&svgs, None, None).unwrap();
        let linked = multi_svg_to_pdf_bytes(&svgs, None, Some(&["chart.html".to_string()])).unwrap();
        assert!(linked.len() > plain.len(), "a page with a link annotation should be larger than one without");
        assert!(
            String::from_utf8_lossy(&linked).contains("/Annots"),
            "expected an /Annots entry when a companion link is requested"
        );
        assert!(!String::from_utf8_lossy(&plain).contains("/Annots"), "no link requested, no /Annots entry expected");
    }

    #[test]
    fn svg_to_png_bytes_produces_a_real_png_at_the_requested_scale() {
        let png = svg_to_png_bytes(SAMPLE_SVG, 1.0).expect("valid SVG should rasterize");
        assert_eq!(&png[..8], &[0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a], "must start with the PNG magic header");
    }

    #[test]
    fn build_docx_bytes_produces_a_real_zip_shaped_docx() {
        let docx = build_docx_bytes(SAMPLE_SVG, "Sales", None).expect("valid SVG should build a docx");
        assert_eq!(&docx[..4], b"PK\x03\x04", "a .docx is a zip archive and must start with the zip local-file magic");
    }

    #[test]
    fn build_docx_bytes_with_a_companion_link_is_larger_than_without() {
        let without = build_docx_bytes(SAMPLE_SVG, "Sales", None).unwrap();
        let with = build_docx_bytes(SAMPLE_SVG, "Sales", Some("chart.html")).unwrap();
        assert!(with.len() > without.len(), "adding a hyperlink paragraph should add real content to the docx");
    }

    #[test]
    fn build_docx_bytes_rejects_malformed_svg_with_a_clear_error_not_a_panic() {
        let err = build_docx_bytes("not valid svg", "t", None).unwrap_err();
        assert!(!err.is_empty());
    }

    #[test]
    fn single_page_report_is_a_structurally_real_pdf_with_exactly_one_page() {
        let svgs = vec![SAMPLE_SVG.to_string()];
        let pdf = multi_svg_to_pdf_bytes(&svgs, None, None).expect("single-chart report should succeed");
        assert!(pdf.starts_with(b"%PDF-"));
        assert_eq!(count_occurrences(&pdf, b"/Type /Page\n"), 1, "expected exactly one page object, got: {}", String::from_utf8_lossy(&pdf));
    }

    #[test]
    fn multi_page_report_has_one_page_object_per_chart_and_is_larger_than_any_single_page() {
        let svgs = vec![SAMPLE_SVG.to_string(), SAMPLE_SVG_2.to_string(), SAMPLE_SVG.to_string()];
        let pdf = multi_svg_to_pdf_bytes(&svgs, None, None).expect("three-chart report should succeed");
        assert!(pdf.starts_with(b"%PDF-"));
        assert_eq!(count_occurrences(&pdf, b"/Type /Page\n"), 3, "expected exactly 3 page objects for 3 charts");

        let single = multi_svg_to_pdf_bytes(&svgs[..1], None, None).unwrap();
        assert!(pdf.len() > single.len(), "a 3-page report should be larger than a 1-page report");
    }

    #[test]
    fn each_page_gets_its_own_distinct_object_ids_no_collisions() {
        let svgs = vec![SAMPLE_SVG.to_string(), SAMPLE_SVG.to_string()];
        let pdf = multi_svg_to_pdf_bytes(&svgs, None, None).expect("two identical-content charts should still each get their own objects");
        let tail = &pdf[pdf.len().saturating_sub(2048)..];
        assert!(tail.windows(5).any(|w| w == b"%%EOF"));
        assert_eq!(count_occurrences(&pdf, b"/Type /Page\n"), 2);
    }

    #[test]
    fn titles_must_match_chart_count_or_be_omitted_entirely() {
        let svgs = vec![SAMPLE_SVG.to_string(), SAMPLE_SVG.to_string()];
        let with_matching_titles = multi_svg_to_pdf_bytes(&svgs, Some(&["Q1".to_string(), "Q2".to_string()]), None);
        assert!(with_matching_titles.is_ok());
        let without_titles = multi_svg_to_pdf_bytes(&svgs, None, None);
        assert!(without_titles.is_ok());
    }

    #[test]
    fn a_title_changes_the_page_content_stream_bytes() {
        let svgs = vec![SAMPLE_SVG.to_string()];
        let untitled = multi_svg_to_pdf_bytes(&svgs, None, None).unwrap();
        let titled = multi_svg_to_pdf_bytes(&svgs, Some(&["Revenue Report".to_string()]), None).unwrap();
        assert_ne!(untitled, titled, "adding a title should change the produced PDF bytes");
    }

    #[test]
    fn malformed_svg_at_any_position_is_rejected_with_a_clear_error_not_a_panic() {
        let svgs = vec![SAMPLE_SVG.to_string(), "not valid svg at all".to_string()];
        let err = multi_svg_to_pdf_bytes(&svgs, None, None).unwrap_err();
        assert!(err.contains("page 2"), "error should identify which page failed, got: {err}");
    }
}
