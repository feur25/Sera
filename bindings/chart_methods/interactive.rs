use seraplot_macros::{sera_impl, sera_doc};
use crate::Chart;
use super::js::SP_DEV_JS;

#[sera_impl]
impl Chart {
    #[sera_doc(
        category = "chart_method",
        aliases("devmode", "inspector", "layout_inspector"),
        file = "charts/chart.md",
        en = "Activates dev mode: a floating panel with 6 tabs — MOVE (drag legend/title), ROTATE (x/y label angle), SCALE (drag a bar's height), COLOR (click any element to recolor it), BG (chart background), STYLE (font size, opacity) — every change is live in the browser and generates the matching Python snippet (.legend_offset(), .title_offset(), .x_labels_rotation(), .y_labels_rotation(), .bar_scale(), .highlight(), .set_bg(), .set_font_size(), .set_opacity()) ready to copy and chain in place of .dev().",
        fr = "Active le mode dev : un panneau flottant a 6 onglets — MOVE (deplacer legende/titre), ROTATE (angle des labels x/y), SCALE (etirer la hauteur d'une barre), COLOR (cliquer un element pour le recolorer), BG (fond du graphique), STYLE (taille de police, opacite) — chaque changement est visible en direct dans le navigateur et genere l'extrait Python correspondant (.legend_offset(), .title_offset(), .x_labels_rotation(), .y_labels_rotation(), .bar_scale(), .highlight(), .set_bg(), .set_font_size(), .set_opacity()) pret a copier et a chainer a la place de .dev()."
    )]
    pub fn dev(&self) -> Chart {
        self.propagate(self.html.replacen(
            "</body>",
            &format!("<script>{}</script></body>", SP_DEV_JS),
            1,
        ))
    }
}
