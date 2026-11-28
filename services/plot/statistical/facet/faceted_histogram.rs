#[crate::chart_demo(
    "family=\"histogram\", values=[12,15,18,14,20,22,19,25,9,11,14,10,13,16,12,19,21,24,20,26,23,17,15,20], \
facet_by=[\"Weekday\",\"Weekday\",\"Weekday\",\"Weekday\",\"Weekday\",\"Weekday\",\"Weekday\",\"Weekday\",\"Weekday\",\"Weekday\",\"Weekday\",\"Weekday\", \
\"Weekend\",\"Weekend\",\"Weekend\",\"Weekend\",\"Weekend\",\"Weekend\",\"Weekend\",\"Weekend\",\"Weekend\",\"Weekend\",\"Weekend\",\"Weekend\"], cols=2"
)]
pub fn demo_marker(_input: &str) -> String {
    String::new()
}
