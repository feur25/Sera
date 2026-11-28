crate::plot_family! {
    pub enum Scene3DVariant default Default {
        Default => "default" | "classic" | "categorical",
        Terrain => "terrain" | "mountain" | "dense",
        Tower   => "tower" | "cylinder" | "skyline",
        Radial  => "radial" | "circular" | "polar",
        Podium  => "podium" | "stepped" | "amphitheater",
    }
}

crate::plot_family! {
    pub enum Orientation3D default Iso {
        Iso        => "iso" | "isometric" | "default",
        Horizontal => "horizontal" | "side" | "h",
        Vertical   => "vertical" | "top" | "v" | "bird",
        Front      => "front" | "flat",
    }
}

impl Orientation3D {
    pub fn angles(self) -> (f64, f64) {
        match self {
            Orientation3D::Iso => (0.785, 0.6),
            Orientation3D::Horizontal => (0.785, 0.08),
            Orientation3D::Vertical => (0.785, 1.48),
            Orientation3D::Front => (0.05, 0.18),
        }
    }
}

pub struct Scene3DEntry {
    pub name: &'static str,
    pub en: &'static str,
    pub fr: &'static str,
}

inventory::collect!(Scene3DEntry);

pub fn iter_entries() -> impl Iterator<Item = &'static Scene3DEntry> {
    inventory::iter::<Scene3DEntry>()
}

inventory::submit! {
    Scene3DEntry {
        name: "default",
        en: "Each series keeps its own categorical palette color, rendered as shaded 3D bars on a flat grid.",
        fr: "Chaque série garde sa propre couleur de palette catégorielle, rendue en barres 3D ombrées sur une grille plate.",
    }
}
inventory::submit! {
    Scene3DEntry {
        name: "terrain",
        en: "The data is resampled onto a dense grid and rendered as hundreds of thin colormap-colored columns, forming a continuous mountain-of-bars terrain.",
        fr: "Les données sont réinterpolées sur une grille dense et rendues en centaines de fines colonnes colorées par hauteur, formant un terrain continu en montagne de barres.",
    }
}
inventory::submit! {
    Scene3DEntry {
        name: "tower",
        en: "The data is resampled onto a dense field of round skyscraper-like towers instead of boxes, for a city-skyline reading of magnitude.",
        fr: "Les données sont réinterpolées sur un champ dense de tours cylindriques façon gratte-ciel au lieu de boites, pour une lecture des magnitudes en skyline urbaine.",
    }
}
inventory::submit! {
    Scene3DEntry {
        name: "radial",
        en: "The data is resampled onto a dense polar grid of rings and wedges radiating from the center, like a 3D radar/sonar sweep of the value field.",
        fr: "Les données sont réinterpolées sur une grille polaire dense d'anneaux et de secteurs partant du centre, comme un balayage radar/sonar 3D du champ de valeurs.",
    }
}
inventory::submit! {
    Scene3DEntry {
        name: "podium",
        en: "The data is resampled onto a dense grid and quantized into stepped terrace levels, like a rice-terrace contour map, to read value bands at a glance.",
        fr: "Les données sont réinterpolées sur une grille dense et quantifiées en paliers étagés, façon rizière en terrasses, pour lire les tranches de valeurs d'un coup d'oeil.",
    }
}
