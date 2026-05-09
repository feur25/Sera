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
    inject_theme(html, t)
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
        ChartTheme::Deluxe  => "saturate(1.6) brightness(1.1) drop-shadow(0 0 22px rgba(0,200,255,0.95))",
        ChartTheme::Prism   => "saturate(3) contrast(1.15) brightness(1.08) drop-shadow(0 0 12px rgba(200,80,255,0.7))",
        ChartTheme::Aurora  => "brightness(1.08) drop-shadow(0 0 11px rgba(80,170,255,0.65))",
        ChartTheme::Inferno => "brightness(1.05) drop-shadow(0 0 7px rgba(180,40,0,0.55))",
        ChartTheme::Frost   => "brightness(1.14) drop-shadow(0 0 18px rgba(120,210,255,0.85))",
        ChartTheme::None    => "",
    };

    let defs = build_filter(&fid, t);

    let sel = format!(
        ".{cls} svg rect[data-idx],.{cls} svg circle[data-idx],\
         .{cls} svg path[data-idx],.{cls} svg polygon[data-idx],\
         .{cls} svg ellipse[data-idx],.{cls} svg polyline[data-idx]"
    );

    let mut css = format!(
        ".{cls}{{background:{bg}!important;border-radius:12px}}\
         .{cls} .sp-bg{{fill:{bg}!important}}\
         {sel}{{filter:url(#{fid}) {extra}!important}}"
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
            "<filter id='{fid}' color-interpolation-filters='sRGB' x='-50%' y='-50%' width='200%' height='200%'>\
             <feColorMatrix in='SourceGraphic' type='matrix' \
               values='0.4 0.2 1.4 0 0  0.1 0.6 1.2 0 0  1.2 0.3 0.6 0 0  0 0 0 1 0' result='shifted'/>\
             <feGaussianBlur in='shifted' stdDeviation='2.5' result='soft'/>\
             <feSpecularLighting in='soft' surfaceScale='10' specularConstant='2.2' specularExponent='50' \
               lighting-color='#00d9ff' result='spec'>\
               <fePointLight x='60' y='-40' z='180'/>\
             </feSpecularLighting>\
             <feComposite in='spec' in2='shifted' operator='in' result='lit'/>\
             <feBlend in='shifted' in2='lit' mode='screen' result='gleam'/>\
             <feOffset in='gleam' dx='4' dy='6' result='s-off'/>\
             <feGaussianBlur in='s-off' stdDeviation='10' result='s-blur'/>\
             <feFlood flood-color='#000510' flood-opacity='0.9' result='s-col'/>\
             <feComposite in='s-col' in2='s-blur' operator='in' result='shadow'/>\
             <feGaussianBlur in='gleam' stdDeviation='14' result='glow1'/>\
             <feGaussianBlur in='gleam' stdDeviation='28' result='glow2'/>\
             <feBlend in='glow2' in2='glow1' mode='lighten' result='halo'/>\
             <feMerge>\
               <feMergeNode in='shadow'/>\
               <feMergeNode in='halo'/>\
               <feMergeNode in='gleam'/>\
             </feMerge>\
             </filter>"
        ),

        ChartTheme::Prism => format!(
            "<filter id='{fid}' color-interpolation-filters='sRGB' x='-40%' y='-40%' width='180%' height='180%'>\
             <feColorMatrix in='SourceGraphic' type='saturate' values='4.5' result='vivid'/>\
             <feConvolveMatrix in='vivid' order='3' \
               kernelMatrix='-1 -1 0 -1 6 -1 0 -1 -1' divisor='2' result='sharp'/>\
             <feOffset in='sharp' dx='-3' dy='-3' result='hi-off'/>\
             <feGaussianBlur in='hi-off' stdDeviation='3.5' result='hi-blur'/>\
             <feFlood flood-color='#ffffff' flood-opacity='0.95' result='white'/>\
             <feComposite in='white' in2='hi-blur' operator='in' result='highlight'/>\
             <feOffset in='sharp' dx='4' dy='5' result='sh-off'/>\
             <feGaussianBlur in='sh-off' stdDeviation='6' result='sh-blur'/>\
             <feFlood flood-color='#0a001a' flood-opacity='0.85' result='dark'/>\
             <feComposite in='dark' in2='sh-blur' operator='in' result='shadow'/>\
             <feConvolveMatrix in='vivid' order='3' \
               kernelMatrix='0 -5 0 -5 18 -5 0 -5 0' divisor='2' result='emboss'/>\
             <feBlend in='vivid' in2='emboss' mode='overlay' result='textured'/>\
             <feSpecularLighting in='textured' surfaceScale='16' specularConstant='3' specularExponent='80' \
               lighting-color='#ffffff' result='prism-spec'>\
               <feDistantLight azimuth='315' elevation='42'/>\
             </feSpecularLighting>\
             <feComposite in='prism-spec' in2='textured' operator='in' result='facets'/>\
             <feBlend in='textured' in2='facets' mode='screen' result='crystal'/>\
             <feMerge>\
               <feMergeNode in='shadow'/>\
               <feMergeNode in='highlight'/>\
               <feMergeNode in='crystal'/>\
             </feMerge>\
             </filter>"
        ),

        ChartTheme::Aurora => format!(
            "<filter id='{fid}' color-interpolation-filters='sRGB' x='-35%' y='-35%' width='170%' height='170%'>\
             <feColorMatrix in='SourceGraphic' type='matrix' \
               values='0.12 0.08 0.9 0 0.04  0.04 0.45 0.65 0 0.06  0.0 0.12 1.55 0 0.12  0 0 0 1 0' result='ice-col'/>\
             <feComposite in='ice-col' in2='SourceGraphic' operator='in' result='clipped'/>\
             <feGaussianBlur in='SourceGraphic' stdDeviation='10' result='dome'/>\
             <feSpecularLighting in='dome' surfaceScale='6' specularConstant='3.8' specularExponent='120' \
               lighting-color='#ddeeff' result='top-spec'>\
               <feDistantLight azimuth='315' elevation='64'/>\
             </feSpecularLighting>\
             <feComposite in='top-spec' in2='SourceGraphic' operator='in' result='top-lit'/>\
             <feSpecularLighting in='dome' surfaceScale='2' specularConstant='1.4' specularExponent='18' \
               lighting-color='#5599dd' result='fill-spec'>\
               <feDistantLight azimuth='200' elevation='28'/>\
             </feSpecularLighting>\
             <feComposite in='fill-spec' in2='SourceGraphic' operator='in' result='fill-lit'/>\
             <feSpecularLighting in='dome' surfaceScale='1.5' specularConstant='1.0' specularExponent='12' \
               lighting-color='#aa55ff' result='rim-spec'>\
               <feDistantLight azimuth='140' elevation='14'/>\
             </feSpecularLighting>\
             <feComposite in='rim-spec' in2='SourceGraphic' operator='in' result='rim-lit'/>\
             <feTurbulence type='fractalNoise' baseFrequency='0.6 0.75' numOctaves='5' seed='17' result='cryst-tex'/>\
             <feColorMatrix in='cryst-tex' type='saturate' values='0' result='cryst-grey'/>\
             <feBlend in='clipped' in2='cryst-grey' mode='overlay' result='textured'/>\
             <feComposite in='textured' in2='SourceGraphic' operator='in' result='textured-clipped'/>\
             <feBlend in='textured-clipped' in2='top-lit' mode='screen' result='s1'/>\
             <feBlend in='s1' in2='fill-lit' mode='screen' result='s2'/>\
             <feBlend in='s2' in2='rim-lit' mode='screen' result='full-lit'/>\
             <feOffset in='full-lit' dx='2' dy='6' result='s-off'/>\
             <feGaussianBlur in='s-off' stdDeviation='5' result='s-blur'/>\
             <feFlood flood-color='#000408' flood-opacity='0.93' result='s-col'/>\
             <feComposite in='s-col' in2='s-blur' operator='in' result='shadow'/>\
             <feMerge>\
               <feMergeNode in='shadow'/>\
               <feMergeNode in='full-lit'/>\
             </feMerge>\
             </filter>"
        ),

        ChartTheme::Inferno => format!(
            "<filter id='{fid}' color-interpolation-filters='sRGB' x='-35%' y='-35%' width='170%' height='170%'>\
             <feColorMatrix in='SourceGraphic' type='saturate' values='0' result='grey'/>\
             <feColorMatrix in='grey' type='matrix' \
               values='1.5 0 0 0 0.05  0.38 0 0 0 0.0  0.04 0 0 0 0.0  0 0 0 1 0' result='fire-col'/>\
             <feComposite in='fire-col' in2='SourceGraphic' operator='in' result='clipped'/>\
             <feGaussianBlur in='SourceGraphic' stdDeviation='10' result='dome'/>\
             <feSpecularLighting in='dome' surfaceScale='6' specularConstant='3.5' specularExponent='95' \
               lighting-color='#ffcc66' result='top-spec'>\
               <feDistantLight azimuth='315' elevation='62'/>\
             </feSpecularLighting>\
             <feComposite in='top-spec' in2='SourceGraphic' operator='in' result='top-lit'/>\
             <feSpecularLighting in='dome' surfaceScale='2.5' specularConstant='1.8' specularExponent='10' \
               lighting-color='#cc1500' result='rim-spec'>\
               <feDistantLight azimuth='135' elevation='12'/>\
             </feSpecularLighting>\
             <feComposite in='rim-spec' in2='SourceGraphic' operator='in' result='rim-lit'/>\
             <feTurbulence type='fractalNoise' baseFrequency='0.07 0.09' numOctaves='4' seed='7' result='lava-tex'/>\
             <feColorMatrix in='lava-tex' type='saturate' values='0' result='lava-grey'/>\
             <feBlend in='clipped' in2='lava-grey' mode='overlay' result='textured'/>\
             <feComposite in='textured' in2='SourceGraphic' operator='in' result='textured-clipped'/>\
             <feBlend in='textured-clipped' in2='top-lit' mode='screen' result='lit1'/>\
             <feBlend in='lit1' in2='rim-lit' mode='screen' result='full-lit'/>\
             <feOffset in='full-lit' dx='2' dy='6' result='s-off'/>\
             <feGaussianBlur in='s-off' stdDeviation='5' result='s-blur'/>\
             <feFlood flood-color='#050000' flood-opacity='0.93' result='s-col'/>\
             <feComposite in='s-col' in2='s-blur' operator='in' result='shadow'/>\
             <feMerge>\
               <feMergeNode in='shadow'/>\
               <feMergeNode in='full-lit'/>\
             </feMerge>\
             </filter>"
        ),

        ChartTheme::Frost => format!(
            "<filter id='{fid}' color-interpolation-filters='sRGB' x='-40%' y='-40%' width='180%' height='180%'>\
             <feColorMatrix in='SourceGraphic' type='saturate' values='0' result='grey'/>\
             <feColorMatrix in='grey' type='matrix' \
               values='0.18 0 0 0 0.04  0.55 0 0 0 0.09  1.9 0 0 0 0.22  0 0 0 1 0' result='ice-col'/>\
             <feComposite in='ice-col' in2='SourceGraphic' operator='in' result='clipped'/>\
             <feGaussianBlur in='SourceGraphic' stdDeviation='9' result='dome'/>\
             <feTurbulence type='turbulence' baseFrequency='0.018 0.024' numOctaves='2' seed='33' result='caustic-n'/>\
             <feDisplacementMap in='dome' in2='caustic-n' scale='14' xChannelSelector='R' yChannelSelector='G' result='warped-dome'/>\
             <feSpecularLighting in='warped-dome' surfaceScale='8' specularConstant='5.0' specularExponent='160' \
               lighting-color='#f4faff' result='main-spec'>\
               <feDistantLight azimuth='315' elevation='70'/>\
             </feSpecularLighting>\
             <feComposite in='main-spec' in2='SourceGraphic' operator='in' result='main-lit'/>\
             <feSpecularLighting in='dome' surfaceScale='3.5' specularConstant='2.2' specularExponent='28' \
               lighting-color='#70bbff' result='fill-spec'>\
               <feDistantLight azimuth='215' elevation='40'/>\
             </feSpecularLighting>\
             <feComposite in='fill-spec' in2='SourceGraphic' operator='in' result='fill-lit'/>\
             <feSpecularLighting in='dome' surfaceScale='2.5' specularConstant='2.0' specularExponent='9' \
               lighting-color='#b0e8ff' result='rim-spec'>\
               <feDistantLight azimuth='128' elevation='8'/>\
             </feSpecularLighting>\
             <feComposite in='rim-spec' in2='SourceGraphic' operator='in' result='rim-lit'/>\
             <feTurbulence type='fractalNoise' baseFrequency='0.48 0.72' numOctaves='6' seed='23' result='frost-tex'/>\
             <feColorMatrix in='frost-tex' type='saturate' values='0' result='frost-grey'/>\
             <feBlend in='clipped' in2='frost-grey' mode='overlay' result='textured'/>\
             <feComposite in='textured' in2='SourceGraphic' operator='in' result='tex-clipped'/>\
             <feBlend in='tex-clipped' in2='main-lit' mode='screen' result='s1'/>\
             <feBlend in='s1' in2='fill-lit' mode='screen' result='s2'/>\
             <feBlend in='s2' in2='rim-lit' mode='screen' result='full-lit'/>\
             <feGaussianBlur in='full-lit' stdDeviation='3.5' result='ice-glow'/>\
             <feBlend in='full-lit' in2='ice-glow' mode='screen' result='glowing'/>\
             <feOffset in='glowing' dx='2' dy='7' result='s-off'/>\
             <feGaussianBlur in='s-off' stdDeviation='7' result='s-blur'/>\
             <feFlood flood-color='#000510' flood-opacity='0.96' result='s-col'/>\
             <feComposite in='s-col' in2='s-blur' operator='in' result='shadow'/>\
             <feMerge>\
               <feMergeNode in='shadow'/>\
               <feMergeNode in='glowing'/>\
             </feMerge>\
             </filter>"
        ),

        ChartTheme::None => String::new(),
    }
}



