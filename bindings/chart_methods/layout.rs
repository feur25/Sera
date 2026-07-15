use seraplot_macros::{sera_impl, sera_doc, sera_sig};
use crate::Chart;
use super::js::{SP_LEGEND_OFFSET_JS, SP_TITLE_OFFSET_JS, SP_X_LABELS_ROT_JS, SP_Y_LABELS_ROT_JS};

#[sera_impl]
impl Chart {
    #[sera_doc(
        category = "chart_method",
        aliases("shift_legend", "move_legend"),
        file = "charts/chart.md",
        en = "Shifts the legend by (dx, dy) pixels in SVG coordinates relative to its default position. Use dev() to find the right values interactively.",
        fr = "Deplace la legende de (dx, dy) pixels en coordonnees SVG par rapport a sa position par defaut. Utilisez dev() pour trouver les bonnes valeurs interactivement.",
        param(name = "dx", ty = "int", en = "Horizontal offset in SVG pixels.", fr = "Decalage horizontal en pixels SVG."),
        param(name = "dy", ty = "int", en = "Vertical offset in SVG pixels.", fr = "Decalage vertical en pixels SVG.")
    )]
    #[sera_sig(dx = 0, dy = 0)]
    pub fn legend_offset(&self, dx: i64, dy: i64) -> Chart {
        let snippet = format!(
            "<script>window.__sp_leg_off__={{dx:{},dy:{}}};{}</script></body>",
            dx, dy, SP_LEGEND_OFFSET_JS
        );
        self.propagate(self.html.replacen("</body>", &snippet, 1))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("shift_title", "move_title"),
        file = "charts/chart.md",
        en = "Shifts the chart title by (dx, dy) pixels in SVG coordinates relative to its default position. Use dev() to find the right values interactively.",
        fr = "Deplace le titre du graphique de (dx, dy) pixels en coordonnees SVG par rapport a sa position par defaut. Utilisez dev() pour trouver les bonnes valeurs interactivement.",
        param(name = "dx", ty = "int", en = "Horizontal offset in SVG pixels.", fr = "Decalage horizontal en pixels SVG."),
        param(name = "dy", ty = "int", en = "Vertical offset in SVG pixels.", fr = "Decalage vertical en pixels SVG.")
    )]
    #[sera_sig(dx = 0, dy = 0)]
    pub fn title_offset(&self, dx: i64, dy: i64) -> Chart {
        let snippet = format!(
            "<script>window.__sp_ttl_off__={{dx:{},dy:{}}};{}</script></body>",
            dx, dy, SP_TITLE_OFFSET_JS
        );
        self.propagate(self.html.replacen("</body>", &snippet, 1))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("rotate_x_labels", "x_label_angle", "tilt_x"),
        file = "charts/chart.md",
        en = "Rotates the X-axis tick labels by the given angle in degrees. Positive = clockwise. Use dev() to find the right value interactively.",
        fr = "Fait pivoter les etiquettes de l'axe X de l'angle donne en degres. Positif = sens horaire. Utilisez dev() pour trouver la bonne valeur.",
        param(name = "degrees", ty = "int", en = "Rotation angle in degrees (-90 to 90).", fr = "Angle de rotation en degres (-90 a 90).")
    )]
    #[sera_sig(degrees = 0)]
    pub fn x_labels_rotation(&self, degrees: i32) -> Chart {
        let snippet = format!(
            "<script>window.__sp_xlrot__={};{}</script></body>",
            degrees, SP_X_LABELS_ROT_JS
        );
        self.propagate(self.html.replacen("</body>", &snippet, 1))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("rotate_y_labels", "y_label_angle", "tilt_y"),
        file = "charts/chart.md",
        en = "Rotates the Y-axis tick labels by the given angle in degrees. Use dev() to find the right value interactively.",
        fr = "Fait pivoter les etiquettes de l'axe Y de l'angle donne en degres. Utilisez dev() pour trouver la bonne valeur.",
        param(name = "degrees", ty = "int", en = "Rotation angle in degrees (-90 to 90).", fr = "Angle de rotation en degres (-90 a 90).")
    )]
    #[sera_sig(degrees = 0)]
    pub fn y_labels_rotation(&self, degrees: i32) -> Chart {
        let snippet = format!(
            "<script>window.__sp_ylrot__={};{}</script></body>",
            degrees, SP_Y_LABELS_ROT_JS
        );
        self.propagate(self.html.replacen("</body>", &snippet, 1))
    }
}
