crate::plot_family! {
    pub enum ChartTheme default None {
        None    => "none" | "default" | "standard" | "light",
        Deluxe  => "deluxe" | "neon" | "dark" | "premium",
        Prism   => "prism" | "crystal" | "glass" | "rainbow",
        Aurora  => "aurora" | "cosmic" | "nebula" | "space" | "galaxy",
        Inferno => "inferno" | "fire" | "lava" | "ember" | "volcanic",
        Frost   => "frost" | "ice" | "arctic" | "winter" | "blizzard",
    }
}

fn extract_sp_id(html: &str) -> &str {
    html.find("id=\"sp")
        .and_then(|p| { let r = &html[p + 4..]; r.find('"').map(|q| &r[..q]) })
        .unwrap_or("sp-0")
}

pub fn apply_chart_theme(html: String, theme_str: &str) -> String {
    let t = ChartTheme::from_str(theme_str);
    if t == ChartTheme::None { return html; }
    if html.contains("class=\"c3w\"") {
        return apply_3d_theme(html, t);
    }
    inject_theme(html, t)
}

fn apply_3d_theme(html: String, t: ChartTheme) -> String {
    let (bg, canvas_f, glow, sh_a, sh_b) = match t {
        ChartTheme::Deluxe  => ("#060d1c", "saturate(1.9) hue-rotate(-20deg) brightness(0.92)", "0 0 0 1px rgba(0,200,255,0.18),0 0 60px rgba(0,180,255,0.5),0 0 120px rgba(0,80,200,0.25)", "rgba(0,200,255,0.12)", "rgba(0,60,180,0.09)"),
        ChartTheme::Prism   => ("#0d001a", "saturate(2.2) contrast(1.08) hue-rotate(30deg) brightness(0.9)", "0 0 0 1px rgba(200,80,255,0.18),0 0 55px rgba(180,60,255,0.55)", "rgba(200,80,255,0.13)", "rgba(100,0,200,0.1)"),
        ChartTheme::Aurora  => ("#01060f", "saturate(1.5) hue-rotate(195deg) brightness(1.0)", "0 0 0 1px rgba(80,170,255,0.16),0 0 50px rgba(80,160,255,0.5),0 0 100px rgba(40,80,200,0.22)", "rgba(100,180,255,0.11)", "rgba(60,100,220,0.09)"),
        ChartTheme::Inferno => ("#080000", "sepia(0.85) saturate(3.5) hue-rotate(-15deg) brightness(0.82)", "0 0 0 1px rgba(200,50,0,0.22),0 0 55px rgba(180,40,0,0.6),0 0 110px rgba(100,15,0,0.3)", "rgba(220,80,10,0.13)", "rgba(120,20,0,0.1)"),
        ChartTheme::Frost   => ("#000c1a", "saturate(1.5) hue-rotate(185deg) brightness(1.08)", "0 0 0 1px rgba(120,210,255,0.2),0 0 60px rgba(100,200,255,0.55),0 0 120px rgba(50,130,220,0.28)", "rgba(140,220,255,0.13)", "rgba(60,130,230,0.1)"),
        ChartTheme::None    => unreachable!(),
    };
    let extra_css = format!(
        "body{{background:{bg}!important}}\
         .c3w{{box-shadow:{glow}!important}}\
         .c3w canvas{{filter:{canvas_f}!important}}\
         .c3w::after{{content:'';position:absolute;inset:0;border-radius:12px;\
           background:radial-gradient(ellipse at 22% 22%,{sh_a} 0%,transparent 55%),\
                      radial-gradient(ellipse at 78% 78%,{sh_b} 0%,transparent 52%);\
           pointer-events:none;z-index:1}}"
    );
    html.replacen("</style>", &format!("{extra_css}</style>"), 1)
}

fn inject_theme(html: String, t: ChartTheme) -> String {
    let id = extract_sp_id(&html).to_owned();
    let fid = format!("{}-{}", t.name(), id.trim_start_matches("sp"));
    let cls = format!("sp-t-{}", t.name());

    let (bg, dark_ui) = match t {
        ChartTheme::Deluxe  => ("#060d1c", true),
        ChartTheme::Prism   => ("#0d001a", false),
        ChartTheme::Aurora  => ("#01060f", true),
        ChartTheme::Inferno => ("#080000", true),
        ChartTheme::Frost   => ("#000c1a", true),
        ChartTheme::None    => ("transparent", false),
    };

    let extra = match t {
        ChartTheme::Deluxe  => "saturate(1.3) brightness(1.05) drop-shadow(0 0 8px rgba(0,200,255,0.5))",
        ChartTheme::Prism   => "saturate(2) brightness(1.05) drop-shadow(0 0 6px rgba(200,80,255,0.45))",
        ChartTheme::Aurora  => "brightness(1.04) drop-shadow(0 0 6px rgba(80,170,255,0.4))",
        ChartTheme::Inferno => "brightness(1.04) drop-shadow(0 0 5px rgba(160,40,0,0.45))",
        ChartTheme::Frost   => "brightness(1.06) drop-shadow(0 0 7px rgba(100,200,255,0.45))",
        ChartTheme::None    => "",
    };

    let fid_f = format!("{fid}-f");
    let defs = format!("{}{}", build_filter(&fid, t), build_flat_filter(&fid_f, t));

    let sel_r = format!(
        ".{cls} svg circle[data-idx],.{cls} svg ellipse[data-idx],\
         .{cls} svg path[data-idx],.{cls} svg polygon[data-idx],.{cls} svg polyline[data-idx],\
         .{cls} svg circle[data-series],.{cls} svg path[data-series],\
         .{cls} svg g[data-series] polygon,.{cls} svg g[data-series] path,\
         .{cls} svg g[data-series] circle"
    );
    let sel_f = format!(".{cls} svg rect[data-idx]");

    let mut css = format!(
        ".{cls}{{background:{bg}!important;border-radius:12px}}\
         .{cls} .sp-bg{{fill:{bg}!important}}\
         {sel_r}{{filter:url(#{fid}) {extra}!important}}\
         {sel_f}{{filter:url(#{fid_f}) {extra}!important}}"
    );

    if dark_ui {
        css.push_str(&format!(
            ".{cls} .sp-ttl{{fill:#dce8f5!important}}\
             .{cls} text{{fill:#8aaac8!important}}\
             .{cls} .sp-yt,.{cls} .sp-xt,.{cls} .sp-xl,.{cls} .sp-yl{{fill:#6b8aaa!important}}\
             .{cls} .sp-ax-x,.{cls} .sp-ax-y{{stroke:#1f3250!important}}\
             .{cls} .sp-gl{{stroke:#0e1e35!important}}\
             .{cls} svg line:not([data-idx]){{stroke:#1a2e48!important}}"
        ));
    }

    let mut h = html.replace(&format!("id=\"{id}\""), &format!("id=\"{id}\" class=\"{cls}\""));
    h = h.replacen("</title>", &format!("</title><style>{css}</style>"), 1);
    h = h.replacen("</svg>", &format!("<defs>{defs}</defs></svg>"), 1);
    h = h.replace("filter=\"url(#dlxgf)\"", &format!("filter=\"url(#{fid})\""));
    h = h.replace("filter=\"url(#prsf)\"", &format!("filter=\"url(#{fid})\""));
    h
}

fn build_filter(fid: &str, t: ChartTheme) -> String {
    match t {
        ChartTheme::Deluxe => format!(
            "<filter id='{fid}' color-interpolation-filters='sRGB' x='-30%' y='-30%' width='160%' height='160%'>\
             <feColorMatrix in='SourceGraphic' type='matrix' \
               values='0.3 0.1 1.2 0 0  0.05 0.5 1.0 0 0  1.0 0.2 0.5 0 0  0 0 0 1 0' result='tinted'/>\
             <feComposite in='tinted' in2='SourceGraphic' operator='in' result='colored'/>\
             <feGaussianBlur in='SourceGraphic' stdDeviation='7' result='dome'/>\
             <feSpecularLighting in='dome' surfaceScale='7' specularConstant='2.0' specularExponent='60' \
               lighting-color='#b0f0ff' result='top-spec'>\
               <feDistantLight azimuth='310' elevation='58'/>\
             </feSpecularLighting>\
             <feComposite in='top-spec' in2='SourceGraphic' operator='in' result='top-lit'/>\
             <feSpecularLighting in='dome' surfaceScale='2' specularConstant='1.0' specularExponent='10' \
               lighting-color='#0044cc' result='rim-spec'>\
               <feDistantLight azimuth='130' elevation='10'/>\
             </feSpecularLighting>\
             <feComposite in='rim-spec' in2='SourceGraphic' operator='in' result='rim-lit'/>\
             <feTurbulence type='fractalNoise' baseFrequency='0.65 0.8' numOctaves='4' seed='5' result='tex'/>\
             <feColorMatrix in='tex' type='saturate' values='0' result='tex-g'/>\
             <feBlend in='colored' in2='tex-g' mode='soft-light' result='textured'/>\
             <feComposite in='textured' in2='SourceGraphic' operator='in' result='tex-clip'/>\
             <feBlend in='tex-clip' in2='top-lit' mode='screen' result='s1'/>\
             <feBlend in='s1' in2='rim-lit' mode='screen' result='full-lit'/>\
             <feOffset in='full-lit' dx='2' dy='4' result='s-off'/>\
             <feGaussianBlur in='s-off' stdDeviation='4' result='s-blur'/>\
             <feFlood flood-color='#000510' flood-opacity='0.88' result='s-col'/>\
             <feComposite in='s-col' in2='s-blur' operator='in' result='shadow'/>\
             <feMerge><feMergeNode in='shadow'/><feMergeNode in='full-lit'/></feMerge>\
             </filter>"
        ),

        ChartTheme::Prism => format!(
            "<filter id='{fid}' color-interpolation-filters='sRGB' x='-30%' y='-30%' width='160%' height='160%'>\
             <feColorMatrix in='SourceGraphic' type='saturate' values='3.5' result='vivid'/>\
             <feComposite in='vivid' in2='SourceGraphic' operator='in' result='colored'/>\
             <feGaussianBlur in='SourceGraphic' stdDeviation='7' result='dome'/>\
             <feSpecularLighting in='dome' surfaceScale='10' specularConstant='2.5' specularExponent='80' \
               lighting-color='#ffffff' result='top-spec'>\
               <feDistantLight azimuth='305' elevation='52'/>\
             </feSpecularLighting>\
             <feComposite in='top-spec' in2='SourceGraphic' operator='in' result='top-lit'/>\
             <feSpecularLighting in='dome' surfaceScale='3' specularConstant='1.2' specularExponent='14' \
               lighting-color='#cc44ff' result='rim-spec'>\
               <feDistantLight azimuth='130' elevation='12'/>\
             </feSpecularLighting>\
             <feComposite in='rim-spec' in2='SourceGraphic' operator='in' result='rim-lit'/>\
             <feTurbulence type='fractalNoise' baseFrequency='0.55 0.7' numOctaves='4' seed='11' result='tex'/>\
             <feColorMatrix in='tex' type='saturate' values='0' result='tex-g'/>\
             <feBlend in='colored' in2='tex-g' mode='soft-light' result='textured'/>\
             <feComposite in='textured' in2='SourceGraphic' operator='in' result='tex-clip'/>\
             <feBlend in='tex-clip' in2='top-lit' mode='screen' result='s1'/>\
             <feBlend in='s1' in2='rim-lit' mode='screen' result='full-lit'/>\
             <feOffset in='full-lit' dx='2' dy='4' result='s-off'/>\
             <feGaussianBlur in='s-off' stdDeviation='4' result='s-blur'/>\
             <feFlood flood-color='#0a001a' flood-opacity='0.88' result='s-col'/>\
             <feComposite in='s-col' in2='s-blur' operator='in' result='shadow'/>\
             <feMerge><feMergeNode in='shadow'/><feMergeNode in='full-lit'/></feMerge>\
             </filter>"
        ),

        ChartTheme::Aurora => format!(
            "<filter id='{fid}' color-interpolation-filters='sRGB' x='-30%' y='-30%' width='160%' height='160%'>\
             <feColorMatrix in='SourceGraphic' type='matrix' \
               values='0.08 0.05 0.85 0 0.03  0.03 0.38 0.55 0 0.05  0.0 0.08 1.4 0 0.1  0 0 0 1 0' result='tinted'/>\
             <feComposite in='tinted' in2='SourceGraphic' operator='in' result='colored'/>\
             <feGaussianBlur in='SourceGraphic' stdDeviation='8' result='dome'/>\
             <feSpecularLighting in='dome' surfaceScale='6' specularConstant='2.8' specularExponent='100' \
               lighting-color='#cce8ff' result='top-spec'>\
               <feDistantLight azimuth='315' elevation='60'/>\
             </feSpecularLighting>\
             <feComposite in='top-spec' in2='SourceGraphic' operator='in' result='top-lit'/>\
             <feSpecularLighting in='dome' surfaceScale='2' specularConstant='1.0' specularExponent='14' \
               lighting-color='#7744cc' result='rim-spec'>\
               <feDistantLight azimuth='145' elevation='10'/>\
             </feSpecularLighting>\
             <feComposite in='rim-spec' in2='SourceGraphic' operator='in' result='rim-lit'/>\
             <feTurbulence type='fractalNoise' baseFrequency='0.55 0.7' numOctaves='5' seed='17' result='tex'/>\
             <feColorMatrix in='tex' type='saturate' values='0' result='tex-g'/>\
             <feBlend in='colored' in2='tex-g' mode='soft-light' result='textured'/>\
             <feComposite in='textured' in2='SourceGraphic' operator='in' result='tex-clip'/>\
             <feBlend in='tex-clip' in2='top-lit' mode='screen' result='s1'/>\
             <feBlend in='s1' in2='rim-lit' mode='screen' result='full-lit'/>\
             <feOffset in='full-lit' dx='2' dy='4' result='s-off'/>\
             <feGaussianBlur in='s-off' stdDeviation='4' result='s-blur'/>\
             <feFlood flood-color='#000408' flood-opacity='0.9' result='s-col'/>\
             <feComposite in='s-col' in2='s-blur' operator='in' result='shadow'/>\
             <feMerge><feMergeNode in='shadow'/><feMergeNode in='full-lit'/></feMerge>\
             </filter>"
        ),

        ChartTheme::Inferno => format!(
            "<filter id='{fid}' color-interpolation-filters='sRGB' x='-30%' y='-30%' width='160%' height='160%'>\
             <feTurbulence type='fractalNoise' baseFrequency='0.028 0.055' numOctaves='5' seed='3' result='magma-n'/>\
             <feColorMatrix in='magma-n' type='saturate' values='0' result='magma-g'/>\
             <feDisplacementMap in='SourceGraphic' in2='magma-g' scale='9' xChannelSelector='R' yChannelSelector='G' result='displaced'/>\
             <feColorMatrix in='displaced' type='matrix' \
               values='1.0 0.3 0 0 0.04  0.0 0.5 0 0 0  0.0 0 0.3 0 0  0 0 0 1 0' result='lava-col'/>\
             <feComposite in='lava-col' in2='SourceGraphic' operator='in' result='colored'/>\
             <feTurbulence type='turbulence' baseFrequency='0.012 0.02' numOctaves='3' seed='9' result='bump-n'/>\
             <feGaussianBlur in='displaced' stdDeviation='6' result='dome'/>\
             <feDisplacementMap in='dome' in2='bump-n' scale='8' xChannelSelector='R' yChannelSelector='G' result='warped-dome'/>\
             <feSpecularLighting in='warped-dome' surfaceScale='9' specularConstant='2.0' specularExponent='50' \
               lighting-color='#ffe0a0' result='top-spec'>\
               <feDistantLight azimuth='305' elevation='55'/>\
             </feSpecularLighting>\
             <feComposite in='top-spec' in2='displaced' operator='in' result='top-lit'/>\
             <feSpecularLighting in='warped-dome' surfaceScale='4' specularConstant='1.2' specularExponent='9' \
               lighting-color='#cc2200' result='crack-spec'>\
               <feDistantLight azimuth='135' elevation='8'/>\
             </feSpecularLighting>\
             <feComposite in='crack-spec' in2='displaced' operator='in' result='crack-lit'/>\
             <feTurbulence type='fractalNoise' baseFrequency='0.055 0.09' numOctaves='3' seed='7' result='surface-n'/>\
             <feColorMatrix in='surface-n' type='saturate' values='0' result='surface-g'/>\
             <feBlend in='colored' in2='surface-g' mode='multiply' result='deep-tex'/>\
             <feComposite in='deep-tex' in2='displaced' operator='in' result='tex-clip'/>\
             <feBlend in='tex-clip' in2='top-lit' mode='screen' result='s1'/>\
             <feBlend in='s1' in2='crack-lit' mode='screen' result='full-lit'/>\
             <feOffset in='full-lit' dx='2' dy='4' result='s-off'/>\
             <feGaussianBlur in='s-off' stdDeviation='4' result='s-blur'/>\
             <feFlood flood-color='#060000' flood-opacity='0.9' result='s-col'/>\
             <feComposite in='s-col' in2='s-blur' operator='in' result='shadow'/>\
             <feMerge><feMergeNode in='shadow'/><feMergeNode in='full-lit'/></feMerge>\
             </filter>"
        ),

        ChartTheme::Frost => format!(
            "<filter id='{fid}' color-interpolation-filters='sRGB' x='-30%' y='-30%' width='160%' height='160%'>\
             <feColorMatrix in='SourceGraphic' type='saturate' values='0' result='grey'/>\
             <feColorMatrix in='grey' type='matrix' \
               values='0.12 0 0 0 0.03  0.42 0 0 0 0.07  1.6 0 0 0 0.18  0 0 0 1 0' result='tinted'/>\
             <feComposite in='tinted' in2='SourceGraphic' operator='in' result='colored'/>\
             <feTurbulence type='fractalNoise' baseFrequency='0.38 0.55' numOctaves='5' seed='23' result='frost-n'/>\
             <feGaussianBlur in='SourceGraphic' stdDeviation='8' result='dome'/>\
             <feDisplacementMap in='dome' in2='frost-n' scale='10' xChannelSelector='R' yChannelSelector='G' result='warped'/>\
             <feSpecularLighting in='warped' surfaceScale='9' specularConstant='3.0' specularExponent='120' \
               lighting-color='#eef8ff' result='top-spec'>\
               <feDistantLight azimuth='315' elevation='68'/>\
             </feSpecularLighting>\
             <feComposite in='top-spec' in2='SourceGraphic' operator='in' result='top-lit'/>\
             <feSpecularLighting in='dome' surfaceScale='3' specularConstant='1.2' specularExponent='20' \
               lighting-color='#5599dd' result='fill-spec'>\
               <feDistantLight azimuth='210' elevation='35'/>\
             </feSpecularLighting>\
             <feComposite in='fill-spec' in2='SourceGraphic' operator='in' result='fill-lit'/>\
             <feColorMatrix in='frost-n' type='saturate' values='0' result='frost-g'/>\
             <feBlend in='colored' in2='frost-g' mode='soft-light' result='textured'/>\
             <feComposite in='textured' in2='SourceGraphic' operator='in' result='tex-clip'/>\
             <feBlend in='tex-clip' in2='top-lit' mode='screen' result='s1'/>\
             <feBlend in='s1' in2='fill-lit' mode='screen' result='full-lit'/>\
             <feOffset in='full-lit' dx='2' dy='4' result='s-off'/>\
             <feGaussianBlur in='s-off' stdDeviation='4' result='s-blur'/>\
             <feFlood flood-color='#000510' flood-opacity='0.9' result='s-col'/>\
             <feComposite in='s-col' in2='s-blur' operator='in' result='shadow'/>\
             <feMerge><feMergeNode in='shadow'/><feMergeNode in='full-lit'/></feMerge>\
             </filter>"
        ),

        ChartTheme::None => String::new(),
    }
}

fn build_flat_filter(fid: &str, t: ChartTheme) -> String {
    match t {
        ChartTheme::Deluxe => format!(
            "<filter id='{fid}' color-interpolation-filters='sRGB' x='-20%' y='-20%' width='140%' height='140%'>\
             <feColorMatrix in='SourceGraphic' type='matrix' \
               values='0.3 0.1 1.2 0 0  0.05 0.5 1.0 0 0  1.0 0.2 0.5 0 0  0 0 0 1 0' result='tinted'/>\
             <feComposite in='tinted' in2='SourceGraphic' operator='in' result='colored'/>\
             <feOffset in='SourceAlpha' dx='0' dy='3' result='shdn'/>\
             <feComposite in='SourceAlpha' in2='shdn' operator='out' result='top-strip'/>\
             <feGaussianBlur in='top-strip' stdDeviation='1.5' result='top-blur'/>\
             <feFlood flood-color='#88eeff' flood-opacity='0.9' result='edge-col'/>\
             <feComposite in='edge-col' in2='top-blur' operator='in' result='top-glow'/>\
             <feTurbulence type='fractalNoise' baseFrequency='0.06 0.03' numOctaves='3' seed='5' result='tex'/>\
             <feColorMatrix in='tex' type='saturate' values='0' result='tex-g'/>\
             <feBlend in='colored' in2='tex-g' mode='soft-light' result='textured'/>\
             <feComposite in='textured' in2='SourceGraphic' operator='in' result='tex-clip'/>\
             <feBlend in='tex-clip' in2='top-glow' mode='screen' result='lit'/>\
             <feOffset in='lit' dx='1' dy='4' result='s-off'/>\
             <feGaussianBlur in='s-off' stdDeviation='4' result='s-blur'/>\
             <feFlood flood-color='#000510' flood-opacity='0.9' result='s-col'/>\
             <feComposite in='s-col' in2='s-blur' operator='in' result='shadow'/>\
             <feMerge><feMergeNode in='shadow'/><feMergeNode in='lit'/></feMerge>\
             </filter>"
        ),

        ChartTheme::Prism => format!(
            "<filter id='{fid}' color-interpolation-filters='sRGB' x='-20%' y='-20%' width='140%' height='140%'>\
             <feColorMatrix in='SourceGraphic' type='saturate' values='3.5' result='vivid'/>\
             <feComposite in='vivid' in2='SourceGraphic' operator='in' result='colored'/>\
             <feOffset in='SourceAlpha' dx='0' dy='3' result='shdn'/>\
             <feComposite in='SourceAlpha' in2='shdn' operator='out' result='top-strip'/>\
             <feGaussianBlur in='top-strip' stdDeviation='1' result='top-blur'/>\
             <feFlood flood-color='#ffffff' flood-opacity='0.9' result='wht'/>\
             <feComposite in='wht' in2='top-blur' operator='in' result='top-glow'/>\
             <feTurbulence type='fractalNoise' baseFrequency='0.05 0.025' numOctaves='3' seed='11' result='tex'/>\
             <feColorMatrix in='tex' type='saturate' values='0' result='tex-g'/>\
             <feBlend in='colored' in2='tex-g' mode='soft-light' result='textured'/>\
             <feComposite in='textured' in2='SourceGraphic' operator='in' result='tex-clip'/>\
             <feBlend in='tex-clip' in2='top-glow' mode='screen' result='lit'/>\
             <feOffset in='lit' dx='1' dy='4' result='s-off'/>\
             <feGaussianBlur in='s-off' stdDeviation='4' result='s-blur'/>\
             <feFlood flood-color='#0a001a' flood-opacity='0.9' result='s-col'/>\
             <feComposite in='s-col' in2='s-blur' operator='in' result='shadow'/>\
             <feMerge><feMergeNode in='shadow'/><feMergeNode in='lit'/></feMerge>\
             </filter>"
        ),

        ChartTheme::Aurora => format!(
            "<filter id='{fid}' color-interpolation-filters='sRGB' x='-20%' y='-20%' width='140%' height='140%'>\
             <feColorMatrix in='SourceGraphic' type='saturate' values='0' result='grey'/>\
             <feColorMatrix in='grey' type='matrix' \
               values='0.12 0 0 0 0.03  0.42 0 0 0 0.07  1.6 0 0 0 0.18  0 0 0 1 0' result='col'/>\
             <feComposite in='col' in2='SourceGraphic' operator='in' result='colored'/>\
             <feOffset in='SourceAlpha' dx='0' dy='3' result='shdn'/>\
             <feComposite in='SourceAlpha' in2='shdn' operator='out' result='top-strip'/>\
             <feGaussianBlur in='top-strip' stdDeviation='1.5' result='top-blur'/>\
             <feFlood flood-color='#aaddff' flood-opacity='0.85' result='edge-col'/>\
             <feComposite in='edge-col' in2='top-blur' operator='in' result='top-glow'/>\
             <feTurbulence type='fractalNoise' baseFrequency='0.05 0.025' numOctaves='4' seed='17' result='tex'/>\
             <feColorMatrix in='tex' type='saturate' values='0' result='tex-g'/>\
             <feBlend in='colored' in2='tex-g' mode='soft-light' result='textured'/>\
             <feComposite in='textured' in2='SourceGraphic' operator='in' result='tex-clip'/>\
             <feBlend in='tex-clip' in2='top-glow' mode='screen' result='lit'/>\
             <feOffset in='lit' dx='1' dy='4' result='s-off'/>\
             <feGaussianBlur in='s-off' stdDeviation='4' result='s-blur'/>\
             <feFlood flood-color='#000408' flood-opacity='0.92' result='s-col'/>\
             <feComposite in='s-col' in2='s-blur' operator='in' result='shadow'/>\
             <feMerge><feMergeNode in='shadow'/><feMergeNode in='lit'/></feMerge>\
             </filter>"
        ),

        ChartTheme::Inferno => format!(
            "<filter id='{fid}' color-interpolation-filters='sRGB' x='-20%' y='-20%' width='140%' height='140%'>\
             <feTurbulence type='fractalNoise' baseFrequency='0.028 0.055' numOctaves='5' seed='3' result='magma-n'/>\
             <feColorMatrix in='magma-n' type='saturate' values='0' result='magma-g'/>\
             <feDisplacementMap in='SourceGraphic' in2='magma-g' scale='7' xChannelSelector='R' yChannelSelector='G' result='displaced'/>\
             <feColorMatrix in='displaced' type='matrix' \
               values='1.0 0.3 0 0 0.04  0.0 0.5 0 0 0  0.0 0 0.3 0 0  0 0 0 1 0' result='lava-col'/>\
             <feComposite in='lava-col' in2='SourceGraphic' operator='in' result='colored'/>\
             <feTurbulence type='fractalNoise' baseFrequency='0.055 0.09' numOctaves='3' seed='7' result='surface-n'/>\
             <feColorMatrix in='surface-n' type='saturate' values='0' result='surface-g'/>\
             <feBlend in='colored' in2='surface-g' mode='multiply' result='tex-blend'/>\
             <feComposite in='tex-blend' in2='SourceGraphic' operator='in' result='textured'/>\
             <feOffset in='SourceAlpha' dx='0' dy='4' result='shdn'/>\
             <feComposite in='SourceAlpha' in2='shdn' operator='out' result='top-strip'/>\
             <feGaussianBlur in='top-strip' stdDeviation='2' result='top-blur'/>\
             <feFlood flood-color='#ffcc44' flood-opacity='0.85' result='edge-col'/>\
             <feComposite in='edge-col' in2='top-blur' operator='in' result='top-glow'/>\
             <feBlend in='textured' in2='top-glow' mode='screen' result='lit'/>\
             <feOffset in='lit' dx='1' dy='4' result='s-off'/>\
             <feGaussianBlur in='s-off' stdDeviation='4' result='s-blur'/>\
             <feFlood flood-color='#060000' flood-opacity='0.92' result='s-col'/>\
             <feComposite in='s-col' in2='s-blur' operator='in' result='shadow'/>\
             <feMerge><feMergeNode in='shadow'/><feMergeNode in='lit'/></feMerge>\
             </filter>"
        ),

        ChartTheme::Frost => format!(
            "<filter id='{fid}' color-interpolation-filters='sRGB' x='-20%' y='-20%' width='140%' height='140%'>\
             <feColorMatrix in='SourceGraphic' type='saturate' values='0' result='grey'/>\
             <feColorMatrix in='grey' type='matrix' \
               values='0.12 0 0 0 0.04  0.42 0 0 0 0.08  1.6 0 0 0 0.2  0 0 0 1 0' result='col'/>\
             <feComposite in='col' in2='SourceGraphic' operator='in' result='colored'/>\
             <feOffset in='SourceAlpha' dx='0' dy='3' result='shdn'/>\
             <feComposite in='SourceAlpha' in2='shdn' operator='out' result='top-strip'/>\
             <feGaussianBlur in='top-strip' stdDeviation='1' result='top-blur'/>\
             <feFlood flood-color='#f0f8ff' flood-opacity='0.9' result='edge-col'/>\
             <feComposite in='edge-col' in2='top-blur' operator='in' result='top-glow'/>\
             <feTurbulence type='fractalNoise' baseFrequency='0.05 0.025' numOctaves='5' seed='23' result='tex'/>\
             <feColorMatrix in='tex' type='saturate' values='0' result='tex-g'/>\
             <feBlend in='colored' in2='tex-g' mode='soft-light' result='textured'/>\
             <feComposite in='textured' in2='SourceGraphic' operator='in' result='tex-clip'/>\
             <feBlend in='tex-clip' in2='top-glow' mode='screen' result='lit'/>\
             <feOffset in='lit' dx='1' dy='4' result='s-off'/>\
             <feGaussianBlur in='s-off' stdDeviation='4' result='s-blur'/>\
             <feFlood flood-color='#000510' flood-opacity='0.92' result='s-col'/>\
             <feComposite in='s-col' in2='s-blur' operator='in' result='shadow'/>\
             <feMerge><feMergeNode in='shadow'/><feMergeNode in='lit'/></feMerge>\
             </filter>"
        ),

        ChartTheme::None => String::new(),
    }
}




