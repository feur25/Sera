mod canvas;
mod engine;
mod renderers;
mod resample;
#[cfg(feature = "sera-report")]
pub(crate) mod replay;
mod tween;
pub(crate) mod video;

#[cfg(feature = "python")]
use crate::Chart;
pub(crate) use engine::GifOptions;
#[cfg(feature = "python")]
use pyo3::prelude::*;
#[cfg(feature = "python")]
use seraplot_macros::sera_doc_impl;
#[cfg(feature = "python")]
use video::VideoFormat;

pub(crate) fn read_recording(spls_path: &str) -> Result<crate::plot::record_registry::Recording, String> {
    crate::plot::record_registry::read_all(spls_path).map_err(|e| format!("seraplot: failed to read {spls_path}: {e}"))
}

#[cfg(feature = "python")]
fn read_recording_py(spls_path: &str) -> PyResult<crate::plot::record_registry::Recording> {
    read_recording(spls_path).map_err(pyo3::exceptions::PyOSError::new_err)
}

pub(crate) fn build_breakpoints(at_ms: Vec<u32>, on_value: Vec<(u32, f64)>, pause_ms: u32) -> Vec<engine::Breakpoint> {
    let from_time = at_ms.into_iter().map(|ms| engine::Breakpoint { trigger: engine::BreakpointTrigger::AtMs(ms), pause_ms });
    let from_value = on_value
        .into_iter()
        .map(|(index, threshold)| engine::Breakpoint { trigger: engine::BreakpointTrigger::ValueAtLeast(index, threshold), pause_ms });
    from_time.chain(from_value).collect()
}

pub(crate) fn render_gif(recording: &crate::plot::record_registry::Recording, opts: &GifOptions) -> Result<Vec<u8>, String> {
    engine::render_gif(recording, opts)
}

pub(crate) fn render_timed_frames(
    recording: &crate::plot::record_registry::Recording,
    opts: &GifOptions,
) -> Result<Vec<(u32, image::RgbaImage)>, String> {
    engine::render_timed_frames(recording, opts)
}

pub(crate) fn count_frames(recording: &crate::plot::record_registry::Recording) -> usize {
    engine::count_frames(recording)
}

pub(crate) fn render_single_frame(
    recording: &crate::plot::record_registry::Recording,
    opts: &GifOptions,
    frame_index: usize,
) -> Result<image::RgbaImage, String> {
    engine::render_single_frame(recording, opts, frame_index)
}

pub(crate) fn parse_video_format(s: &str) -> Result<video::VideoFormat, String> {
    video::VideoFormat::parse(s)
}

pub(crate) fn encode_video(timed_frames: &[(u32, image::RgbaImage)], fps: u32, format: video::VideoFormat, out_path: &str) -> Result<(), String> {
    video::encode_video(timed_frames, fps, format, out_path)
}

pub(crate) fn even_dim(n: u32) -> u32 {
    video::even(n)
}

#[cfg(feature = "python")]
#[sera_doc_impl]
#[pyo3::pymethods]
impl Chart {
    #[pyo3(signature = (gif_path, spls_path, width = 800, height = 450, max_frames = 600, color = canvas::DEFAULT_RGB, sort_by_value = false, smooth = false, smooth_speed = 1.0, smooth_ease = "linear", reveal = false, reveal_steps = 120, reveal_fixed_scale = false, breakpoint_at_ms = vec![], breakpoint_on_value = vec![], breakpoint_pause_ms = 1000))]
    #[sera_doc(
        category = "chart_method",
        module = "studio",
        file = "seraplot/pulse-pricing.md",
        aliases("to_gif", "save_gif"),
        en = "Renders a .spls recording (from record()) as an animated GIF - one frame per recorded update. When the recording carries its original chart-construction source (true for any recording made since the source-tagged .spls v4 format), every frame is replayed through that chart family's own real renderer - a pictogram bar variant shows real icons, a themed heatmap shows its real color grid - instead of a generic substitute. Falls back automatically to a family-agnostic shape-based renderer (bar/dot/candle by push shape) for older .spls files or families without a replay mapping, so nothing ever fails outright. Only draws a slot/dot/candle for indices actually touched during the recording, so watching a handful of values update inside a 500+ element chart still produces a readable animation. sort_by_value=True re-ranks slots by current value every frame (highest first) for a 'bar chart race' look, compacting to just the racing rows and carrying each row's real label along with it - and assigning each row a color tied to its own identity rather than its current rank, so a row keeps its color as it overtakes others instead of colors swapping every time ranks change. smooth=True additionally tweens the geometry of matching elements between consecutive recorded states, so values glide and racing slots visibly slide past each other instead of snapping frame to frame - opt-in since it costs extra render time per transition. smooth_speed scales how many in-between steps a transition gets (2.0 = twice as fast/fewer steps, 0.5 = twice as slow/more steps); smooth_ease picks the pacing curve. reveal=True ignores the recording's real update timeline entirely and instead reveals the final touched state progressively in index order over reveal_steps frames, as if the chart were drawing itself - for a known, complete dataset rather than a live stream, this reads far better than replaying real push timestamps. reveal_fixed_scale=True additionally locks the axis to the final dataset's scale from the very first reveal frame, masking the not-yet-revealed portion instead of letting the axis auto-rescale as data appears - a cardiogram-style sweep across a fixed grid. breakpoint_at_ms and breakpoint_on_value each insert a held pause (breakpoint_pause_ms long) into the output the first time the recording's elapsed time, or a specific tracked index's decoded value, reaches the given mark - every later frame shifts forward by the pause to keep the rest of the timeline intact.",
        fr = "Rend un enregistrement .spls (issu de record()) en GIF anime - une frame par mise a jour enregistree. Quand l'enregistrement porte sa source de construction d'origine (vrai pour tout enregistrement fait depuis le format .spls v4 avec source), chaque frame est rejouee via le vrai rendu de cette famille de chart - un variant bar en pictogram montre de vraies icones, une heatmap themee montre sa vraie grille de couleurs - plutot qu'un substitut generique. Retombe automatiquement sur un rendu generique base sur la forme des donnees (barre/point/chandelier) pour les anciens fichiers .spls ou les familles sans correspondance de rejeu, pour ne jamais echouer completement. Ne dessine un emplacement/point/chandelier que pour les indices reellement touches pendant l'enregistrement, donc observer quelques valeurs evoluer dans un chart de 500+ elements produit quand meme une animation lisible. sort_by_value=True reclasse les emplacements par valeur courante a chaque frame (la plus grande en tete) pour un effet 'bar chart race', en compactant sur les seules lignes en course et en emportant le vrai libelle de chaque ligne avec elle - et en assignant a chaque ligne une couleur liee a sa propre identite plutot qu'a son rang courant, pour qu'une ligne garde sa couleur en depassant les autres au lieu que les couleurs s'echangent a chaque changement de rang. smooth=True anime en plus la geometrie des elements correspondants entre deux etats enregistres consecutifs, pour que les valeurs evoluent en douceur et que les emplacements en course glissent visiblement l'un devant l'autre plutot que de sauter d'une frame a l'autre - optionnel car cela coute un temps de rendu supplementaire par transition. smooth_speed ajuste le nombre d'etapes intermediaires d'une transition (2.0 = deux fois plus rapide/moins d'etapes, 0.5 = deux fois plus lent/plus d'etapes) ; smooth_ease choisit la courbe de rythme. reveal=True ignore entierement la vraie ligne de temps de mises a jour de l'enregistrement et revele plutot l'etat final touche progressivement dans l'ordre des indices sur reveal_steps frames, comme si le chart se dessinait lui-meme - pour un jeu de donnees connu et complet plutot qu'un flux temps reel, le rendu est bien meilleur qu'un rejeu des vrais timestamps de push. reveal_fixed_scale=True fige en plus l'echelle des axes sur le jeu de donnees final des la toute premiere frame de la revelation, en masquant la portion pas encore revelee plutot que de laisser l'echelle se reechelonner automatiquement - un balayage façon cardiogramme sur une grille fixe. breakpoint_at_ms et breakpoint_on_value inserent chacun une pause maintenue (de duree breakpoint_pause_ms) dans la sortie la premiere fois que le temps ecoule de l'enregistrement, ou la valeur decodee d'un indice suivi specifique, atteint le seuil donne - chaque frame suivante est decalee en avant de la duree de la pause pour garder le reste de la ligne de temps intact.",
        param(name = "gif_path", ty = "str", en = "Destination .gif file path.", fr = "Chemin du fichier .gif de destination."),
        param(name = "spls_path", ty = "str", en = "Path to the .spls recording produced by record().", fr = "Chemin de l'enregistrement .spls produit par record()."),
        param(name = "width", ty = "int", en = "GIF width in pixels.", fr = "Largeur du GIF en pixels."),
        param(name = "height", ty = "int", en = "GIF height in pixels.", fr = "Hauteur du GIF en pixels."),
        param(name = "max_frames", ty = "int", en = "Refuses to render recordings with more frames than this, to keep GIF size and render time bounded.", fr = "Refuse de rendre les enregistrements ayant plus de frames que cette limite, pour borner la taille du GIF et le temps de rendu."),
        param(name = "color", ty = "tuple[int, int, int]", en = "RGB color used for bars/dots/mini-bars in the generic-renderer fallback. Ignored whenever a family-correct replay renders the frame.", fr = "Couleur RVB utilisee pour les barres/points/mini-barres du rendu generique de secours. Ignoree des qu'un rejeu fidele a la famille rend la frame."),
        param(name = "sort_by_value", ty = "bool", en = "Re-rank slots by current value every frame instead of keeping a stable order.", fr = "Reclasse les emplacements par valeur courante a chaque frame au lieu de garder un ordre stable."),
        param(name = "smooth", ty = "bool", en = "Tween element geometry between consecutive recorded states instead of snapping - smooth growth and smooth slot-swapping during a race.", fr = "Anime la geometrie des elements entre deux etats enregistres consecutifs au lieu de sauter - croissance et echanges d'emplacement en douceur pendant une course."),
        param(name = "smooth_speed", ty = "float", en = "Multiplier on how many in-between steps a smooth transition gets. Higher is faster/fewer steps, lower is slower/more gradual. Only matters when smooth=True.", fr = "Multiplicateur sur le nombre d'etapes intermediaires d'une transition fluide. Plus haut = plus rapide/moins d'etapes, plus bas = plus lent/plus progressif. Compte seulement si smooth=True."),
        param(name = "smooth_ease", ty = "str", en = "Pacing curve for smooth transitions: 'linear', 'ease_in', 'ease_out', or 'ease_in_out'. Unknown names fall back to 'linear'. Only matters when smooth=True.", fr = "Courbe de rythme pour les transitions fluides : 'linear', 'ease_in', 'ease_out', ou 'ease_in_out'. Un nom inconnu retombe sur 'linear'. Compte seulement si smooth=True."),
        param(name = "reveal", ty = "bool", en = "Reveal the recording's final state progressively in index order over reveal_steps frames instead of replaying real push timestamps.", fr = "Revele l'etat final de l'enregistrement progressivement dans l'ordre des indices sur reveal_steps frames au lieu de rejouer les vrais timestamps de push."),
        param(name = "reveal_steps", ty = "int", en = "How many frames the progressive reveal is spread across. Only matters when reveal=True.", fr = "Sur combien de frames la revelation progressive est etalee. Compte seulement si reveal=True."),
        param(name = "reveal_fixed_scale", ty = "bool", en = "Lock the axis scale to the final complete dataset from the first reveal frame onward, instead of letting it auto-rescale as more of the data is revealed - only the drawn trace grows, like a cardiogram sweep. Only matters when reveal=True; silently has no effect without a replay source.", fr = "Fige l'echelle des axes sur le jeu de donnees final complet des la premiere frame de la revelation, au lieu de la laisser se reechelonner automatiquement au fur et a mesure - seul le trace dessine s'allonge, comme un balayage de cardiogramme. Compte seulement si reveal=True ; silencieusement sans effet sans source de rejeu."),
        param(name = "breakpoint_at_ms", ty = "list[int]", en = "Pause the output the first time the recording's elapsed time reaches each of these marks.", fr = "Met en pause la sortie la premiere fois que le temps ecoule de l'enregistrement atteint chacun de ces seuils."),
        param(name = "breakpoint_on_value", ty = "list[tuple[int, float]]", en = "Pause the output the first time the given tracked index's decoded value reaches or exceeds the given threshold, as (index, threshold) pairs.", fr = "Met en pause la sortie la premiere fois que la valeur decodee de l'indice suivi donne atteint ou depasse le seuil donne, sous forme de paires (indice, seuil)."),
        param(name = "breakpoint_pause_ms", ty = "int", en = "How long each triggered breakpoint holds the frame, in milliseconds.", fr = "Duree en millisecondes pendant laquelle chaque breakpoint declenche maintient la frame.")
    )]
    #[allow(clippy::too_many_arguments)]
    fn export_gif(
        &self,
        gif_path: &str,
        spls_path: &str,
        width: u32,
        height: u32,
        max_frames: usize,
        color: (u8, u8, u8),
        sort_by_value: bool,
        smooth: bool,
        smooth_speed: f64,
        smooth_ease: &str,
        reveal: bool,
        reveal_steps: u32,
        reveal_fixed_scale: bool,
        breakpoint_at_ms: Vec<u32>,
        breakpoint_on_value: Vec<(u32, f64)>,
        breakpoint_pause_ms: u32,
    ) -> PyResult<()> {
        super::features::require_hid(&self.html)?;
        let recording = read_recording_py(spls_path)?;
        let opts = GifOptions {
            width: width.max(50),
            height: height.max(50),
            max_frames,
            color,
            sort_by_value,
            smooth,
            smooth_speed,
            smooth_ease: smooth_ease.to_string(),
            reveal,
            reveal_steps: reveal_steps.max(1),
            reveal_fixed_scale,
            breakpoints: build_breakpoints(breakpoint_at_ms, breakpoint_on_value, breakpoint_pause_ms),
            ..GifOptions::default()
        };
        let gif = engine::render_gif(&recording, &opts).map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("seraplot: {e}")))?;
        super::frame_codec::atomic_write(gif_path, &gif)
            .map_err(|e| pyo3::exceptions::PyOSError::new_err(format!("seraplot: failed to write {gif_path}: {e}")))?;
        Ok(())
    }

    #[sera_doc(
        category = "chart_method",
        module = "studio",
        file = "seraplot/pulse-pricing.md",
        aliases("gif_length"),
        en = "Returns how many frames a .spls recording contains, without rendering anything - useful for picking a sensible max_frames before calling export_gif().",
        fr = "Retourne le nombre de frames que contient un enregistrement .spls, sans rien rendre - utile pour choisir un max_frames raisonnable avant d'appeler export_gif()."
    )]
    fn gif_frame_count(&self, spls_path: &str) -> PyResult<usize> {
        super::features::require_hid(&self.html)?;
        let recording = read_recording_py(spls_path)?;
        Ok(engine::count_frames(&recording))
    }

    #[pyo3(signature = (spls_path, frame_index, png_path, width = 800, height = 450, color = canvas::DEFAULT_RGB, sort_by_value = false))]
    #[sera_doc(
        category = "chart_method",
        module = "studio",
        file = "seraplot/pulse-pricing.md",
        en = "Renders one specific frame of a .spls recording as a standalone PNG, without encoding a whole GIF - useful to preview or debug what a recording looks like before committing to export_gif(). frame_index=0 is the first recorded update. Same shape-based, family-agnostic rendering as export_gif().",
        fr = "Rend une frame precise d'un enregistrement .spls sous forme de PNG autonome, sans encoder un GIF entier - utile pour previsualiser ou deboguer le rendu d'un enregistrement avant d'appeler export_gif(). frame_index=0 est la premiere mise a jour enregistree. Meme rendu base sur la forme des donnees, independant de la famille de chart, que export_gif().",
        param(name = "spls_path", ty = "str", en = "Path to the .spls recording produced by record().", fr = "Chemin de l'enregistrement .spls produit par record()."),
        param(name = "frame_index", ty = "int", en = "Which recorded update to render, 0-based.", fr = "Quelle mise a jour enregistree rendre, indexee a partir de 0."),
        param(name = "png_path", ty = "str", en = "Destination .png file path.", fr = "Chemin du fichier .png de destination."),
        param(name = "width", ty = "int", en = "PNG width in pixels.", fr = "Largeur du PNG en pixels."),
        param(name = "height", ty = "int", en = "PNG height in pixels.", fr = "Hauteur du PNG en pixels."),
        param(name = "color", ty = "tuple[int, int, int]", en = "RGB color used for bars/dots/mini-bars.", fr = "Couleur RVB utilisee pour les barres/points/mini-barres."),
        param(name = "sort_by_value", ty = "bool", en = "Re-rank slots by current value, same as export_gif()'s own sort_by_value.", fr = "Reclasse les emplacements par valeur courante, comme le sort_by_value de export_gif().")
    )]
    fn gif_preview_frame(&self, spls_path: &str, frame_index: usize, png_path: &str, width: u32, height: u32, color: (u8, u8, u8), sort_by_value: bool) -> PyResult<()> {
        super::features::require_hid(&self.html)?;
        let recording = read_recording_py(spls_path)?;
        let opts = GifOptions { width: width.max(50), height: height.max(50), color, sort_by_value, ..GifOptions::default() };
        let img =
            engine::render_single_frame(&recording, &opts, frame_index).map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("seraplot: {e}")))?;
        img.save(png_path).map_err(|e| pyo3::exceptions::PyOSError::new_err(format!("seraplot: failed to write {png_path}: {e}")))?;
        Ok(())
    }

    #[pyo3(signature = (gif_path, spls_path, width = 800, height = 450, max_frames = 600, color = canvas::DEFAULT_RGB, sort_by_value = false, smooth = false, smooth_speed = 1.0, smooth_ease = "linear", reveal = false, reveal_steps = 120, reveal_fixed_scale = false, breakpoint_at_ms = vec![], breakpoint_on_value = vec![], breakpoint_pause_ms = 1000))]
    #[allow(clippy::too_many_arguments)]
    fn to_gif(
        &self,
        gif_path: &str,
        spls_path: &str,
        width: u32,
        height: u32,
        max_frames: usize,
        color: (u8, u8, u8),
        sort_by_value: bool,
        smooth: bool,
        smooth_speed: f64,
        smooth_ease: &str,
        reveal: bool,
        reveal_steps: u32,
        reveal_fixed_scale: bool,
        breakpoint_at_ms: Vec<u32>,
        breakpoint_on_value: Vec<(u32, f64)>,
        breakpoint_pause_ms: u32,
    ) -> PyResult<()> {
        self.export_gif(gif_path, spls_path, width, height, max_frames, color, sort_by_value, smooth, smooth_speed, smooth_ease, reveal, reveal_steps, reveal_fixed_scale, breakpoint_at_ms, breakpoint_on_value, breakpoint_pause_ms)
    }

    #[pyo3(signature = (gif_path, spls_path, width = 800, height = 450, max_frames = 600, color = canvas::DEFAULT_RGB, sort_by_value = false, smooth = false, smooth_speed = 1.0, smooth_ease = "linear", reveal = false, reveal_steps = 120, reveal_fixed_scale = false, breakpoint_at_ms = vec![], breakpoint_on_value = vec![], breakpoint_pause_ms = 1000))]
    #[allow(clippy::too_many_arguments)]
    fn save_gif(
        &self,
        gif_path: &str,
        spls_path: &str,
        width: u32,
        height: u32,
        max_frames: usize,
        color: (u8, u8, u8),
        sort_by_value: bool,
        smooth: bool,
        smooth_speed: f64,
        smooth_ease: &str,
        reveal: bool,
        reveal_steps: u32,
        reveal_fixed_scale: bool,
        breakpoint_at_ms: Vec<u32>,
        breakpoint_on_value: Vec<(u32, f64)>,
        breakpoint_pause_ms: u32,
    ) -> PyResult<()> {
        self.export_gif(gif_path, spls_path, width, height, max_frames, color, sort_by_value, smooth, smooth_speed, smooth_ease, reveal, reveal_steps, reveal_fixed_scale, breakpoint_at_ms, breakpoint_on_value, breakpoint_pause_ms)
    }

    fn gif_length(&self, spls_path: &str) -> PyResult<usize> {
        self.gif_frame_count(spls_path)
    }

    #[pyo3(signature = (video_path, spls_path, format = "mp4", fps = 30, width = 800, height = 450, max_frames = 600, color = canvas::DEFAULT_RGB, sort_by_value = false, smooth = false, smooth_speed = 1.0, smooth_ease = "linear", reveal = false, reveal_steps = 120, reveal_fixed_scale = false, breakpoint_at_ms = vec![], breakpoint_on_value = vec![], breakpoint_pause_ms = 1000))]
    #[sera_doc(
        category = "chart_method",
        module = "studio",
        file = "seraplot/pulse-pricing.md",
        aliases("to_video", "save_video"),
        en = "Renders a .spls recording (from record()) as a real video file in the format of your choice (mp4/H.264, webm/VP9, mov, mkv or avi - all H.264 except webm), suitable for posting to Twitter/LinkedIn or embedding on a page - the same family-correct replay (falling back to a generic shape-based renderer when unavailable) as export_gif(), just with a constant frame rate and a real video codec instead of a GIF palette. Resamples the recording's real update timeline onto a fixed fps grid: bursts of rapid updates collapse into the latest state, quiet gaps repeat the last known frame - never speeds up or drops real state changes. sort_by_value=True re-ranks slots by current value every frame (highest first) for a 'bar chart race' video, keeping each row's own color as it overtakes others. smooth=True tweens element geometry between recorded states at the video's own fps, so growth and slot swaps glide instead of snapping; smooth_speed and smooth_ease control how that glide paces itself. reveal=True replaces the real timeline with a progressive reveal of the final touched state in index order across reveal_steps frames - a curve or a set of bars drawing itself, ideal for a known/complete dataset. reveal_fixed_scale=True keeps the axis locked to the final dataset's scale throughout, masking what has not yet been revealed rather than auto-rescaling - a cardiogram-style sweep. breakpoint_at_ms/breakpoint_on_value hold the video on a frame for breakpoint_pause_ms the first time elapsed time, or a tracked index's value, reaches a given mark. Requires ffmpeg to be installed and on PATH (not bundled - a real external encoder, not a partial reimplementation); raises a clear error naming that requirement if it is missing, rather than failing silently.",
        fr = "Rend un enregistrement .spls (issu de record()) en un vrai fichier video dans le format de votre choix (mp4/H.264, webm/VP9, mov, mkv ou avi - tous en H.264 sauf webm), adapte a une publication Twitter/LinkedIn ou a l'integration dans une page - le meme rejeu fidele a la famille (avec repli sur un rendu generique base sur la forme des donnees si indisponible) que export_gif(), juste avec un frame rate constant et un vrai codec video plutot qu'une palette GIF. Rééchantillonne la vraie ligne de temps de l'enregistrement sur une grille a fps fixe : les rafales de mises a jour rapides se compressent sur le dernier etat, les silences repetent la derniere frame connue - jamais d'accéleration ni de perte d'un vrai changement d'etat. sort_by_value=True reclasse les emplacements par valeur courante a chaque frame (la plus grande en tete) pour une video 'bar chart race', en gardant la couleur propre de chaque ligne pendant qu'elle depasse les autres. smooth=True anime la geometrie des elements entre les etats enregistres au fps meme de la video, pour que croissances et echanges d'emplacement glissent au lieu de sauter ; smooth_speed et smooth_ease controlent le rythme de ce glissement. reveal=True remplace la vraie ligne de temps par une revelation progressive de l'etat final touche dans l'ordre des indices sur reveal_steps frames - une courbe ou un jeu de barres qui se dessine, ideal pour un jeu de donnees connu/complet. reveal_fixed_scale=True garde l'echelle des axes figee sur le jeu de donnees final tout du long, en masquant ce qui n'est pas encore revele plutot que de reechelonner - un balayage façon cardiogramme. breakpoint_at_ms/breakpoint_on_value maintiennent la video sur une frame pendant breakpoint_pause_ms la premiere fois que le temps ecoule, ou la valeur d'un indice suivi, atteint un seuil donne. Necessite que ffmpeg soit installe et accessible sur le PATH (non embarque - un vrai encodeur externe, pas une reimplementation partielle) ; leve une erreur claire nommant cette exigence s'il est absent, plutot que d'echouer silencieusement.",
        param(name = "video_path", ty = "str", en = "Destination video file path, extension matching format=.", fr = "Chemin du fichier video de destination, extension cohérente avec format=."),
        param(name = "spls_path", ty = "str", en = "Path to the .spls recording produced by record().", fr = "Chemin de l'enregistrement .spls produit par record()."),
        param(name = "format", ty = "str", en = "One of 'mp4', 'webm', 'mov', 'mkv', 'avi'.", fr = "Un parmi 'mp4', 'webm', 'mov', 'mkv', 'avi'."),
        param(name = "fps", ty = "int", en = "Constant output frame rate.", fr = "Frame rate constant de sortie."),
        param(name = "width", ty = "int", en = "Video width in pixels (rounded up to an even number if needed).", fr = "Largeur video en pixels (arrondie au nombre pair superieur si besoin)."),
        param(name = "height", ty = "int", en = "Video height in pixels (rounded up to an even number if needed).", fr = "Hauteur video en pixels (arrondie au nombre pair superieur si besoin)."),
        param(name = "max_frames", ty = "int", en = "Refuses to render recordings with more recorded updates than this, to keep render time bounded.", fr = "Refuse de rendre les enregistrements ayant plus de mises a jour enregistrees que cette limite, pour borner le temps de rendu."),
        param(name = "color", ty = "tuple[int, int, int]", en = "RGB color used for bars/dots/mini-bars in the generic-renderer fallback. Ignored whenever a family-correct replay renders the frame.", fr = "Couleur RVB utilisee pour les barres/points/mini-barres du rendu generique de secours. Ignoree des qu'un rejeu fidele a la famille rend la frame."),
        param(name = "sort_by_value", ty = "bool", en = "Re-rank slots by current value every frame, for a bar chart race look.", fr = "Reclasse les emplacements par valeur courante a chaque frame, pour un effet bar chart race."),
        param(name = "smooth", ty = "bool", en = "Tween element geometry between consecutive recorded states at the target fps instead of snapping.", fr = "Anime la geometrie des elements entre deux etats enregistres consecutifs au fps cible au lieu de sauter."),
        param(name = "smooth_speed", ty = "float", en = "Multiplier on how many in-between steps a smooth transition gets. Higher is faster/fewer steps, lower is slower/more gradual. Only matters when smooth=True.", fr = "Multiplicateur sur le nombre d'etapes intermediaires d'une transition fluide. Plus haut = plus rapide/moins d'etapes, plus bas = plus lent/plus progressif. Compte seulement si smooth=True."),
        param(name = "smooth_ease", ty = "str", en = "Pacing curve for smooth transitions: 'linear', 'ease_in', 'ease_out', or 'ease_in_out'. Unknown names fall back to 'linear'. Only matters when smooth=True.", fr = "Courbe de rythme pour les transitions fluides : 'linear', 'ease_in', 'ease_out', ou 'ease_in_out'. Un nom inconnu retombe sur 'linear'. Compte seulement si smooth=True."),
        param(name = "reveal", ty = "bool", en = "Reveal the recording's final state progressively in index order over reveal_steps frames instead of replaying real push timestamps.", fr = "Revele l'etat final de l'enregistrement progressivement dans l'ordre des indices sur reveal_steps frames au lieu de rejouer les vrais timestamps de push."),
        param(name = "reveal_steps", ty = "int", en = "How many frames the progressive reveal is spread across. Only matters when reveal=True.", fr = "Sur combien de frames la revelation progressive est etalee. Compte seulement si reveal=True."),
        param(name = "reveal_fixed_scale", ty = "bool", en = "Lock the axis scale to the final complete dataset from the first reveal frame onward, instead of letting it auto-rescale as more of the data is revealed - only the drawn trace grows, like a cardiogram sweep. Only matters when reveal=True; silently has no effect without a replay source.", fr = "Fige l'echelle des axes sur le jeu de donnees final complet des la premiere frame de la revelation, au lieu de la laisser se reechelonner automatiquement au fur et a mesure - seul le trace dessine s'allonge, comme un balayage de cardiogramme. Compte seulement si reveal=True ; silencieusement sans effet sans source de rejeu."),
        param(name = "breakpoint_at_ms", ty = "list[int]", en = "Pause the output the first time the recording's elapsed time reaches each of these marks.", fr = "Met en pause la sortie la premiere fois que le temps ecoule de l'enregistrement atteint chacun de ces seuils."),
        param(name = "breakpoint_on_value", ty = "list[tuple[int, float]]", en = "Pause the output the first time the given tracked index's decoded value reaches or exceeds the given threshold, as (index, threshold) pairs.", fr = "Met en pause la sortie la premiere fois que la valeur decodee de l'indice suivi donne atteint ou depasse le seuil donne, sous forme de paires (indice, seuil)."),
        param(name = "breakpoint_pause_ms", ty = "int", en = "How long each triggered breakpoint holds the frame, in milliseconds.", fr = "Duree en millisecondes pendant laquelle chaque breakpoint declenche maintient la frame.")
    )]
    #[allow(clippy::too_many_arguments)]
    fn export_video(
        &self,
        video_path: &str,
        spls_path: &str,
        format: &str,
        fps: u32,
        width: u32,
        height: u32,
        max_frames: usize,
        color: (u8, u8, u8),
        sort_by_value: bool,
        smooth: bool,
        smooth_speed: f64,
        smooth_ease: &str,
        reveal: bool,
        reveal_steps: u32,
        reveal_fixed_scale: bool,
        breakpoint_at_ms: Vec<u32>,
        breakpoint_on_value: Vec<(u32, f64)>,
        breakpoint_pause_ms: u32,
    ) -> PyResult<()> {
        super::features::require_hid(&self.html)?;
        let video_format = VideoFormat::parse(format).map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("seraplot: {e}")))?;
        let recording = read_recording_py(spls_path)?;
        let fps = fps.max(1);
        let opts = GifOptions {
            width: video::even(width.max(50)),
            height: video::even(height.max(50)),
            max_frames,
            color,
            sort_by_value,
            smooth,
            smooth_tick_ms: (1000 / fps).max(1),
            smooth_speed,
            smooth_ease: smooth_ease.to_string(),
            reveal,
            reveal_steps: reveal_steps.max(1),
            reveal_fixed_scale,
            breakpoints: build_breakpoints(breakpoint_at_ms, breakpoint_on_value, breakpoint_pause_ms),
            ..GifOptions::default()
        };
        let timed_frames =
            engine::render_timed_frames(&recording, &opts).map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("seraplot: {e}")))?;
        video::encode_video(&timed_frames, fps, video_format, video_path)
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(format!("seraplot: {e}")))?;
        Ok(())
    }

    #[pyo3(signature = (video_path, spls_path, fps = 30, width = 800, height = 450, max_frames = 600, color = canvas::DEFAULT_RGB, sort_by_value = false, smooth = false, smooth_speed = 1.0, smooth_ease = "linear", reveal = false, reveal_steps = 120, reveal_fixed_scale = false, breakpoint_at_ms = vec![], breakpoint_on_value = vec![], breakpoint_pause_ms = 1000))]
    #[sera_doc(
        category = "chart_method",
        module = "studio",
        file = "seraplot/pulse-pricing.md",
        en = "Shorthand for export_video(video_path, spls_path, format='mp4', ...).",
        fr = "Raccourci pour export_video(video_path, spls_path, format='mp4', ...)."
    )]
    #[allow(clippy::too_many_arguments)]
    fn export_mp4(
        &self,
        video_path: &str,
        spls_path: &str,
        fps: u32,
        width: u32,
        height: u32,
        max_frames: usize,
        color: (u8, u8, u8),
        sort_by_value: bool,
        smooth: bool,
        smooth_speed: f64,
        smooth_ease: &str,
        reveal: bool,
        reveal_steps: u32,
        reveal_fixed_scale: bool,
        breakpoint_at_ms: Vec<u32>,
        breakpoint_on_value: Vec<(u32, f64)>,
        breakpoint_pause_ms: u32,
    ) -> PyResult<()> {
        self.export_video(video_path, spls_path, "mp4", fps, width, height, max_frames, color, sort_by_value, smooth, smooth_speed, smooth_ease, reveal, reveal_steps, reveal_fixed_scale, breakpoint_at_ms, breakpoint_on_value, breakpoint_pause_ms)
    }

    #[pyo3(signature = (video_path, spls_path, fps = 30, width = 800, height = 450, max_frames = 600, color = canvas::DEFAULT_RGB, sort_by_value = false, smooth = false, smooth_speed = 1.0, smooth_ease = "linear", reveal = false, reveal_steps = 120, reveal_fixed_scale = false, breakpoint_at_ms = vec![], breakpoint_on_value = vec![], breakpoint_pause_ms = 1000))]
    #[sera_doc(
        category = "chart_method",
        module = "studio",
        file = "seraplot/pulse-pricing.md",
        en = "Shorthand for export_video(video_path, spls_path, format='webm', ...).",
        fr = "Raccourci pour export_video(video_path, spls_path, format='webm', ...)."
    )]
    #[allow(clippy::too_many_arguments)]
    fn export_webm(
        &self,
        video_path: &str,
        spls_path: &str,
        fps: u32,
        width: u32,
        height: u32,
        max_frames: usize,
        color: (u8, u8, u8),
        sort_by_value: bool,
        smooth: bool,
        smooth_speed: f64,
        smooth_ease: &str,
        reveal: bool,
        reveal_steps: u32,
        reveal_fixed_scale: bool,
        breakpoint_at_ms: Vec<u32>,
        breakpoint_on_value: Vec<(u32, f64)>,
        breakpoint_pause_ms: u32,
    ) -> PyResult<()> {
        self.export_video(video_path, spls_path, "webm", fps, width, height, max_frames, color, sort_by_value, smooth, smooth_speed, smooth_ease, reveal, reveal_steps, reveal_fixed_scale, breakpoint_at_ms, breakpoint_on_value, breakpoint_pause_ms)
    }

    #[pyo3(signature = (video_path, spls_path, fps = 30, width = 800, height = 450, max_frames = 600, color = canvas::DEFAULT_RGB, sort_by_value = false, smooth = false, smooth_speed = 1.0, smooth_ease = "linear", reveal = false, reveal_steps = 120, reveal_fixed_scale = false, breakpoint_at_ms = vec![], breakpoint_on_value = vec![], breakpoint_pause_ms = 1000))]
    #[sera_doc(
        category = "chart_method",
        module = "studio",
        file = "seraplot/pulse-pricing.md",
        en = "Shorthand for export_video(video_path, spls_path, format='mov', ...).",
        fr = "Raccourci pour export_video(video_path, spls_path, format='mov', ...)."
    )]
    #[allow(clippy::too_many_arguments)]
    fn export_mov(
        &self,
        video_path: &str,
        spls_path: &str,
        fps: u32,
        width: u32,
        height: u32,
        max_frames: usize,
        color: (u8, u8, u8),
        sort_by_value: bool,
        smooth: bool,
        smooth_speed: f64,
        smooth_ease: &str,
        reveal: bool,
        reveal_steps: u32,
        reveal_fixed_scale: bool,
        breakpoint_at_ms: Vec<u32>,
        breakpoint_on_value: Vec<(u32, f64)>,
        breakpoint_pause_ms: u32,
    ) -> PyResult<()> {
        self.export_video(video_path, spls_path, "mov", fps, width, height, max_frames, color, sort_by_value, smooth, smooth_speed, smooth_ease, reveal, reveal_steps, reveal_fixed_scale, breakpoint_at_ms, breakpoint_on_value, breakpoint_pause_ms)
    }

    #[pyo3(signature = (video_path, spls_path, fps = 30, width = 800, height = 450, max_frames = 600, color = canvas::DEFAULT_RGB, sort_by_value = false, smooth = false, smooth_speed = 1.0, smooth_ease = "linear", reveal = false, reveal_steps = 120, reveal_fixed_scale = false, breakpoint_at_ms = vec![], breakpoint_on_value = vec![], breakpoint_pause_ms = 1000))]
    #[sera_doc(
        category = "chart_method",
        module = "studio",
        file = "seraplot/pulse-pricing.md",
        en = "Shorthand for export_video(video_path, spls_path, format='mkv', ...).",
        fr = "Raccourci pour export_video(video_path, spls_path, format='mkv', ...)."
    )]
    #[allow(clippy::too_many_arguments)]
    fn export_mkv(
        &self,
        video_path: &str,
        spls_path: &str,
        fps: u32,
        width: u32,
        height: u32,
        max_frames: usize,
        color: (u8, u8, u8),
        sort_by_value: bool,
        smooth: bool,
        smooth_speed: f64,
        smooth_ease: &str,
        reveal: bool,
        reveal_steps: u32,
        reveal_fixed_scale: bool,
        breakpoint_at_ms: Vec<u32>,
        breakpoint_on_value: Vec<(u32, f64)>,
        breakpoint_pause_ms: u32,
    ) -> PyResult<()> {
        self.export_video(video_path, spls_path, "mkv", fps, width, height, max_frames, color, sort_by_value, smooth, smooth_speed, smooth_ease, reveal, reveal_steps, reveal_fixed_scale, breakpoint_at_ms, breakpoint_on_value, breakpoint_pause_ms)
    }

    #[pyo3(signature = (video_path, spls_path, fps = 30, width = 800, height = 450, max_frames = 600, color = canvas::DEFAULT_RGB, sort_by_value = false, smooth = false, smooth_speed = 1.0, smooth_ease = "linear", reveal = false, reveal_steps = 120, reveal_fixed_scale = false, breakpoint_at_ms = vec![], breakpoint_on_value = vec![], breakpoint_pause_ms = 1000))]
    #[sera_doc(
        category = "chart_method",
        module = "studio",
        file = "seraplot/pulse-pricing.md",
        en = "Shorthand for export_video(video_path, spls_path, format='avi', ...).",
        fr = "Raccourci pour export_video(video_path, spls_path, format='avi', ...)."
    )]
    #[allow(clippy::too_many_arguments)]
    fn export_avi(
        &self,
        video_path: &str,
        spls_path: &str,
        fps: u32,
        width: u32,
        height: u32,
        max_frames: usize,
        color: (u8, u8, u8),
        sort_by_value: bool,
        smooth: bool,
        smooth_speed: f64,
        smooth_ease: &str,
        reveal: bool,
        reveal_steps: u32,
        reveal_fixed_scale: bool,
        breakpoint_at_ms: Vec<u32>,
        breakpoint_on_value: Vec<(u32, f64)>,
        breakpoint_pause_ms: u32,
    ) -> PyResult<()> {
        self.export_video(video_path, spls_path, "avi", fps, width, height, max_frames, color, sort_by_value, smooth, smooth_speed, smooth_ease, reveal, reveal_steps, reveal_fixed_scale, breakpoint_at_ms, breakpoint_on_value, breakpoint_pause_ms)
    }

    #[pyo3(signature = (video_path, spls_path, format = "mp4", fps = 30, width = 800, height = 450, max_frames = 600, color = canvas::DEFAULT_RGB, sort_by_value = false, smooth = false, smooth_speed = 1.0, smooth_ease = "linear", reveal = false, reveal_steps = 120, reveal_fixed_scale = false, breakpoint_at_ms = vec![], breakpoint_on_value = vec![], breakpoint_pause_ms = 1000))]
    #[allow(clippy::too_many_arguments)]
    fn to_video(
        &self,
        video_path: &str,
        spls_path: &str,
        format: &str,
        fps: u32,
        width: u32,
        height: u32,
        max_frames: usize,
        color: (u8, u8, u8),
        sort_by_value: bool,
        smooth: bool,
        smooth_speed: f64,
        smooth_ease: &str,
        reveal: bool,
        reveal_steps: u32,
        reveal_fixed_scale: bool,
        breakpoint_at_ms: Vec<u32>,
        breakpoint_on_value: Vec<(u32, f64)>,
        breakpoint_pause_ms: u32,
    ) -> PyResult<()> {
        self.export_video(video_path, spls_path, format, fps, width, height, max_frames, color, sort_by_value, smooth, smooth_speed, smooth_ease, reveal, reveal_steps, reveal_fixed_scale, breakpoint_at_ms, breakpoint_on_value, breakpoint_pause_ms)
    }

    #[pyo3(signature = (video_path, spls_path, format = "mp4", fps = 30, width = 800, height = 450, max_frames = 600, color = canvas::DEFAULT_RGB, sort_by_value = false, smooth = false, smooth_speed = 1.0, smooth_ease = "linear", reveal = false, reveal_steps = 120, reveal_fixed_scale = false, breakpoint_at_ms = vec![], breakpoint_on_value = vec![], breakpoint_pause_ms = 1000))]
    #[allow(clippy::too_many_arguments)]
    fn save_video(
        &self,
        video_path: &str,
        spls_path: &str,
        format: &str,
        fps: u32,
        width: u32,
        height: u32,
        max_frames: usize,
        color: (u8, u8, u8),
        sort_by_value: bool,
        smooth: bool,
        smooth_speed: f64,
        smooth_ease: &str,
        reveal: bool,
        reveal_steps: u32,
        reveal_fixed_scale: bool,
        breakpoint_at_ms: Vec<u32>,
        breakpoint_on_value: Vec<(u32, f64)>,
        breakpoint_pause_ms: u32,
    ) -> PyResult<()> {
        self.export_video(video_path, spls_path, format, fps, width, height, max_frames, color, sort_by_value, smooth, smooth_speed, smooth_ease, reveal, reveal_steps, reveal_fixed_scale, breakpoint_at_ms, breakpoint_on_value, breakpoint_pause_ms)
    }
}
