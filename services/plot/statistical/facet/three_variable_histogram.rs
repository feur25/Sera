#[crate::chart_demo(
    "family=\"joint\", x=[1.2,2.4,2.1,3.6,3.1,3.9,4.2,4.6,4.4,4.9,5.5,5.1,5.8,2.2,3.3,3.7], \
y=[1.1,2.3,3.2,2.4,3.6,4.1,3.3,4.7,5.2,3.9,4.4,5.6,6.1,1.4,2.5,4.2], \
facet_by=[\"Alpha\",\"Alpha\",\"Alpha\",\"Alpha\",\"Alpha\",\"Alpha\",\"Alpha\",\"Alpha\",\"Beta\",\"Beta\",\"Beta\",\"Beta\",\"Beta\",\"Beta\",\"Beta\",\"Beta\"], cols=2"
)]
pub fn demo_marker(_input: &str) -> String {
    String::new()
}
