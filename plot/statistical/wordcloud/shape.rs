
pub const DEMO_KWARGS: &str = "words=[\"rust\",\"python\",\"wasm\",\"plot\",\"data\",\"viz\",\"chart\",\"graph\",\"fast\",\"native\",\"async\",\"macro\",\"trait\",\"enum\",\"crate\"], frequencies=[42,38,30,28,25,22,18,15,12,10,9,8,7,6,5]";

crate::plot_family! {
    pub enum WordCloudShape default Rect {
        Rect    => "rect" | "rectangle" | "default" | "spiral" | "box",
        Circle  => "circle" | "round" | "disk" | "ball",
        Heart   => "heart" | "love" | "valentine" | "cardioid",
        Bird    => "bird" | "twitter" | "tweet" | "swallow",
        Glasses => "glasses" | "sunglasses" | "shades" | "specs",
        Diamond => "diamond" | "rhombus" | "kite",
        Star    => "star" | "polygon" | "celestial",
    }
}


