#[crate::chart_demo(
    "family=\"line\", labels=[\"Mon\",\"Tue\",\"Wed\",\"Thu\",\"Fri\",\"Sat\",\"Sun\",\"Mon\",\"Tue\",\"Wed\",\"Thu\",\"Fri\",\"Sat\",\"Sun\",\"Mon\",\"Tue\",\"Wed\",\"Thu\",\"Fri\",\"Sat\",\"Sun\"], \
values=[4,5,6,5,7,8,7,10,11,9,12,13,14,12,3,4,3,5,4,6,5], \
facet_by=[\"North\",\"North\",\"North\",\"North\",\"North\",\"North\",\"North\",\"South\",\"South\",\"South\",\"South\",\"South\",\"South\",\"South\",\"East\",\"East\",\"East\",\"East\",\"East\",\"East\",\"East\"], cols=3"
)]
pub fn demo_marker(_input: &str) -> String {
    String::new()
}
