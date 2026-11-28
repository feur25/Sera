#[crate::chart_demo(
    "family=\"bar\", labels=[\"A\",\"B\",\"A\",\"B\",\"A\",\"B\",\"A\",\"B\",\"A\",\"B\",\"A\",\"B\",\"A\",\"B\",\"A\",\"B\"], \
values=[3,5,4,6,2,7,5,4,6,3,8,5,4,6,3,5], \
facet_by=[\"g1\",\"g1\",\"g2\",\"g2\",\"g3\",\"g3\",\"g4\",\"g4\",\"g5\",\"g5\",\"g6\",\"g6\",\"g7\",\"g7\",\"g8\",\"g8\"], cols=4"
)]
pub fn demo_marker(_input: &str) -> String {
    String::new()
}
