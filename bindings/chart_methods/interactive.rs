use seraplot_macros::{sera_impl, sera_doc};
use crate::Chart;
use super::js::SP_DEV_JS;

#[sera_impl]
impl Chart {
    #[sera_doc(
        category = "chart_method",
        aliases("devmode", "inspector", "layout_inspector"),
        file = "charts/chart.md",
        en = "Activates dev mode: makes the legend and title draggable in the browser, shows a floating panel with real-time SVG coordinates, and generates a Python snippet you can copy. When you find the right positions, replace .dev() with .legend_offset(dx, dy) and/or .title_offset(dx, dy).",
        fr = "Active le mode dev : rend la legende et le titre depla?ables dans le navigateur, affiche un panneau flottant avec les coordonnees SVG en temps reel, et genere un extrait Python a copier. Une fois les bonnes positions trouvees, remplacez .dev() par .legend_offset(dx, dy) et/ou .title_offset(dx, dy)."
    )]
    pub fn dev(&self) -> Chart {
        self.propagate(self.html.replacen(
            "</body>",
            &format!("<script>{}</script></body>", SP_DEV_JS),
            1,
        ))
    }
}
